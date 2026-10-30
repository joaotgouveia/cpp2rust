#!/usr/bin/env python3
# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

"""Reports every rule-matching failure in translated outputs.

A system declaration that no rule matches is emitted under its mangled
name: `std_<qualified_name>` for types and `<name>_<n>` for functions and
values. Such a name is referenced in the output but never defined there, so
it is found without compiling the output.

With --log, a lit log is also read for:
  - names rustc could not resolve (`cannot find ... X`), which covers
    unmatched C types such as `sockaddr` that carry no mangled prefix;
  - tests whose translation crashed, whose outputs are stale and therefore
    cannot be trusted.

Exit status is 1 when anything is reported.
"""

import argparse
import collections
import re
import sys
from pathlib import Path

RE_COMMENT = re.compile(r"//[^\n]*|/\*.*?\*/", re.S)
RE_STRING = re.compile(r'b?"(?:\\.|[^"\\])*"')
RE_DEFINED = re.compile(
    r"\b(?:fn|struct|enum|union|type|const|static(?:\s+mut)?|trait|mod|macro_rules!)"
    r"\s+(\w+)"
)
RE_FIELD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:(?!:)", re.M)
RE_BINDING = re.compile(r"\b(?:let|for)\s+(?:mut\s+)?(\w+)")
RE_PARAM = re.compile(r"[(,]\s*(?:mut\s+)?(\w+)\s*:(?!:)")
RE_VARIANT = re.compile(r"^\s*(\w+)\s*(?:[,({=]|$)", re.M)
RE_MANGLED = re.compile(r"(?<![\w:.])(std_\w+|[A-Za-z_]\w*?_\d+)\b(?!\s*!)")
RE_CANNOT_FIND = re.compile(
    r"cannot find (type|struct, variant or union type|function|value|trait|macro)"
    r" `(\w+)`"
)
RE_TEST_BLOCK = re.compile(r"\*{20} TEST '(cpp2rust :: [^']+)' FAILED \*{20}\n")


def strip(text):
    return RE_STRING.sub('""', RE_COMMENT.sub("", text))


def defined_names(text):
    names = set(RE_DEFINED.findall(text))
    names |= set(RE_BINDING.findall(text))
    names |= set(RE_PARAM.findall(text))
    names |= set(RE_FIELD.findall(text))
    for body in re.findall(r"\benum\s+\w+[^{]*\{(.*?)\n\}", text, re.S):
        names |= set(RE_VARIANT.findall(body))
    return names


def scan_outputs(roots, since):
    found = collections.defaultdict(list)
    files = 0
    stale = 0
    for root in roots:
        for path in sorted(Path(root).rglob("out/*/*.rs")):
            if since is not None and path.stat().st_mtime < since:
                stale += 1
                continue
            files += 1
            text = strip(path.read_text(errors="replace"))
            defined = defined_names(text)
            for name in sorted(set(RE_MANGLED.findall(text)) - defined):
                found[name].append(str(path))
    return found, files, stale


def scan_log(path):
    unresolved = collections.defaultdict(set)
    crashed = []
    text = Path(path).read_text(errors="replace")
    parts = RE_TEST_BLOCK.split(text)[1:]
    for name, body in zip(parts[0::2], parts[1::2]):
        test = name.split("::", 1)[1].strip()
        body = body.split("\n********************")[0]
        for _, ident in RE_CANNOT_FIND.findall(body):
            unresolved[ident].add(test)
        if body.startswith("no out file") or body.startswith("cpp2rust failed"):
            reason = re.search(r"(LLVM ERROR: [^\n]*|Assertion `[^`]*`|"
                               r"what\(\): [^\n]*|Segmentation fault)", body)
            crashed.append((test, reason.group(1) if reason else "unknown"))
    return unresolved, crashed


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("roots", nargs="*", default=["tests"],
                        help="directories searched for out/<model>/*.rs")
    parser.add_argument("--log", action="append", default=[],
                        help="lit -vv log to read rustc errors and crashes from")
    parser.add_argument("--since", metavar="FILE",
                        help="only outputs modified after FILE (e.g. the lit log's start "
                             "marker); older outputs are stale and counted separately")
    parser.add_argument("--examples", type=int, default=1,
                        help="example files printed per name")
    args = parser.parse_args()

    status = 0
    since = Path(args.since).stat().st_mtime if args.since else None
    found, files, stale = scan_outputs(args.roots, since)
    print(f"# scanned {files} outputs under {', '.join(args.roots)}")
    if since is not None:
        print(f"# skipped {stale} stale outputs (older than {args.since})")
    print(f"# unmatched mangled names: {len(found)}")
    for name, paths in sorted(found.items(), key=lambda x: (-len(x[1]), x[0])):
        print(f"{len(paths):6}  {name}")
        for p in paths[: args.examples]:
            print(f"        {p}")
    status |= bool(found)

    for log in args.log:
        unresolved, crashed = scan_log(log)
        other = {k: v for k, v in unresolved.items() if k not in found}
        print(f"\n# {log}: unresolved names not already listed: {len(other)}")
        for name, tests in sorted(other.items(), key=lambda x: (-len(x[1]), x[0])):
            print(f"{len(tests):6}  {name}")
            for t in sorted(tests)[: args.examples]:
                print(f"        {t}")
        reasons = collections.Counter(r for _, r in crashed)
        print(f"\n# {log}: crashed translations (stale outputs): {len(crashed)}")
        for reason, n in reasons.most_common():
            example = next(t for t, r in crashed if r == reason)
            print(f"{n:6}  {reason[:150]}")
            print(f"        {example}")
        status |= bool(other) or bool(crashed)

    sys.exit(status)


if __name__ == "__main__":
    main()
