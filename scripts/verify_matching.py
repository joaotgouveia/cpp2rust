#!/usr/bin/env python3
# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

"""Ten independent checks that a lit run left no rule-matching errors.

Each check looks for matching failures by a different means, so that a blind
spot of one method does not hide an error:

   1. check_matching.py over the fresh outputs (mangled names never defined)
   2. rustc errors in the log naming std_* items
   3. rustc errors in the log naming <name>_<n> items
   4. raw scan of fresh outputs for std_* identifiers, defined or not
   5. raw scan of fresh outputs for calls to <name>_<n>( never defined as fn
   6. translations that crashed (no output, so nothing was checked)
   7. "matches several rules" ambiguity errors
   8. "loaded rules do not compile" errors
   9. tests that lit ran but whose outputs are stale (older than the run)
  10. rustc unresolved names of any kind (cannot find ...) in the log

Usage: verify_matching.py --log LOG --since FILE [roots...]
Exit status is the number of failed checks.
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path

RE_BLOCK = re.compile(r"\*{20} TEST 'cpp2rust :: ([^']+)' FAILED \*{20}\n")
RE_FIND = re.compile(
    r"cannot find (?:type|struct, variant or union type|function|value|trait|macro)"
    r" `(\w+)`"
)
RE_STD_IDENT = re.compile(r"(?<![\w:.])(std_[A-Za-z_]\w*)\b")
RE_SUFFIX_CALL = re.compile(r"(?<![\w:.])([A-Za-z_]\w*?_\d+)\s*\(")
RE_FN_DEF = re.compile(r"\bfn\s+(\w+)")
RE_ANY_DEF = re.compile(
    r"\b(?:fn|struct|enum|union|type|const|static(?:\s+mut)?|trait|mod)\s+(\w+)"
)


def failures(log_text):
    parts = RE_BLOCK.split(log_text)[1:]
    for name, body in zip(parts[0::2], parts[1::2]):
        yield name, body.split("\n********************")[0]


def fresh_outputs(roots, since):
    for root in roots:
        for path in Path(root).rglob("out/*/*.rs"):
            if path.stat().st_mtime >= since:
                yield path


sys.path.insert(0, str(Path(__file__).parent))
from check_matching import strip as strip_literals  # noqa: E402  (lexer-based)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--log", required=True)
    parser.add_argument("--since", required=True)
    parser.add_argument("roots", nargs="*",
                        default=["tests/unit", "tests/ub", "tests/multi-file",
                                 "tests/libcxx", "tests/libstdcxx"])
    args = parser.parse_args()
    since = Path(args.since).stat().st_mtime
    log_text = Path(args.log).read_text(errors="replace")
    fails = list(failures(log_text))
    outputs = list(fresh_outputs(args.roots, since))
    results = []

    def report(number, title, items):
        items = sorted(set(items))
        results.append(not items)
        status = "OK" if not items else f"FAIL ({len(items)})"
        print(f"[{number:2}] {title}: {status}")
        for item in items[:15]:
            print(f"       {item}")

    checker = subprocess.run(
        [sys.executable, "-I", str(Path(__file__).parent / "check_matching.py"),
         *args.roots, "--since", args.since],
        capture_output=True, text=True)
    names = [l.split()[1] for l in checker.stdout.splitlines()
             if re.match(r"^\s+\d+\s+\S+$", l)]
    report(1, "check_matching.py unmatched mangled names", names)

    std_err, suffix_err, any_err = [], [], []
    for test, body in fails:
        for name in RE_FIND.findall(body):
            any_err.append(f"{name} ({test})")
            if name.startswith("std_"):
                std_err.append(f"{name} ({test})")
            elif re.search(r"_\d+$", name):
                suffix_err.append(f"{name} ({test})")
    report(2, "rustc: unresolved std_* items", std_err)
    report(3, "rustc: unresolved <name>_<n> items", suffix_err)

    raw_std, raw_calls = [], []
    for path in outputs:
        text = strip_literals(path.read_text(errors="replace"))
        defined = set(RE_ANY_DEF.findall(text))
        for ident in set(RE_STD_IDENT.findall(text)) - defined:
            raw_std.append(f"{ident} ({path})")
        fns = set(RE_FN_DEF.findall(text))
        for ident in set(RE_SUFFIX_CALL.findall(text)) - fns - defined:
            raw_calls.append(f"{ident} ({path})")
    report(4, "raw scan: undefined std_* identifiers", raw_std)
    report(5, "raw scan: calls to undefined <name>_<n>", raw_calls)

    crashed = [test for test, body in fails
               if body.startswith("no out file") or body.startswith("cpp2rust failed")]
    report(6, "crashed translations", crashed)
    report(7, "ambiguous rule matches",
           [t for t, b in fails if "matches several rules" in b])
    report(8, "loaded rules do not compile",
           [t for t, b in fails if "loaded rules do not compile" in b])

    stale = []
    for test, body in fails:
        if body.startswith("no out file") or body.startswith("cpp2rust failed"):
            continue
        path, model = test.rsplit("/", 1)
        src = Path("tests") / path
        if src.is_dir():
            out = src / "out" / model / (src.name + ".rs")
        else:
            out = src.parent / "out" / model / (src.stem + ".rs")
        if not out.exists() or out.stat().st_mtime < since:
            stale.append(str(out))
    report(9, "failed tests with stale outputs", stale)
    report(10, "rustc: unresolved names of any kind", any_err)

    failed = results.count(False)
    print(f"\n{10 - failed}/10 checks clean")
    sys.exit(failed)


if __name__ == "__main__":
    main()
