#!/usr/bin/env python3
# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

"""Narrows rules/ to a subset of the generated rules.

A rule source file is generated when rulegenerator's rules/ has a file of the
same name in the same directory; every other source file (src.c, the manual_*
directories, the hand rules of rules_manual) is kept as it is. In a generated
file, a rule is kept when the subset has it or when it has a target in
tgt_unsafe.rs or tgt_refcount.rs; a subset file that rules/ lacks is copied.
A file left without rules is deleted, as is a directory left without sources
that had some.

A subset file must be the generated file with some rules deleted.
"""

import argparse
import re
import shutil
import sys
from pathlib import Path

RE_RULE_NAME = re.compile(r"\busing\s+(t\d+)\s*=|\b(f\d+)\s*\(")
RE_TGT_NAME = re.compile(r"^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+([ft]\d+)\b", re.M)


def chunks(text):
    """The file's include block and its rules, which are separated by blank
    lines, as (name, chunk) pairs; the include block has no name."""
    for chunk in text.split("\n\n"):
        if not chunk.strip("\n"):
            continue
        names = {a or b for a, b in RE_RULE_NAME.findall(chunk)}
        if not names:
            assert all(
                line.startswith("#include") for line in chunk.strip("\n").split("\n")
            ), chunk
            yield None, chunk
            continue
        assert len(names) == 1, chunk
        yield names.pop(), chunk


def rule_names(path):
    return {name for name, _ in chunks(path.read_text()) if name}


def tgt_names(rule_dir):
    names = set()
    for tgt in rule_dir.glob("tgt_*.rs"):
        names |= set(RE_TGT_NAME.findall(tgt.read_text()))
    return names


def is_source(path):
    return path.name in ("src.c", "src.cpp") or (
        path.name.startswith("src_") and path.suffix == ".cpp"
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rules", type=Path, required=True, help="cpp2rust rules/")
    parser.add_argument(
        "--generated", type=Path, required=True, help="rulegenerator rules/"
    )
    parser.add_argument("--subset", type=Path, required=True, help="the rules subset")
    args = parser.parse_args()

    # A subset file that rules/ lacks is brought in as it is.
    for src in sorted(args.subset.glob("*/*")):
        dst = args.rules / src.relative_to(args.subset)
        if is_source(src) and not dst.exists():
            dst.parent.mkdir(exist_ok=True)
            shutil.copy2(src, dst)

    kept = removed = 0
    for rule_dir in sorted(p for p in args.rules.iterdir() if p.is_dir()):
        targets = tgt_names(rule_dir)
        sources = sorted(p for p in rule_dir.iterdir() if is_source(p))
        if not sources:
            continue
        for src in sources:
            rel = src.relative_to(args.rules)
            if not (args.generated / rel).is_file():
                continue
            subset_src = args.subset / rel
            subset = rule_names(subset_src) if subset_src.is_file() else set()
            text = src.read_text()
            missing = subset - rule_names(src)
            if missing:
                sys.exit(f"{subset_src}: rules not in {src}: {sorted(missing)}")
            out = []
            has_rules = False
            for name, chunk in chunks(text):
                if name is None or name in subset or name in targets:
                    out.append(chunk)
                    has_rules |= name is not None
                    kept += name is not None
                else:
                    removed += 1
            if has_rules:
                src.write_text("\n\n".join(c.strip("\n") for c in out) + "\n")
            else:
                src.unlink()
        if not any(is_source(p) for p in rule_dir.iterdir()):
            assert not targets, f"{rule_dir}: targets without rules"
            shutil.rmtree(rule_dir)
    print(f"kept {kept} generated rules, removed {removed}")


if __name__ == "__main__":
    main()
