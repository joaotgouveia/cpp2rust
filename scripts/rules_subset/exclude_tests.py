#!/usr/bin/env python3
# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

"""Excludes the tests that use rules outside rules/.

checkusedrules (see CMakeLists.txt) lists, for each test, the library
declarations that no rule of the build's rules index matches. A test with any
that is not IGNORED, or on which checkusedrules fails, is added to its suite's
lit.local.cfg as an unsupported path, in a block that each run rewrites.

The tests are compiled with the flags lit gives them; the tests that lit
already skips are not checked.

Run in the container, after building cpp2rust and checkusedrules:
  scripts/rules_subset/exclude_tests.py \\
      --checkusedrules build/rules-subset/checkusedrules tests/libcxx tests/libstdcxx
"""

import argparse
import concurrent.futures
import copy
import importlib
import os
import re
import resource
import subprocess
import sys
import types
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tests" / "lit"))

from lit.BooleanExpression import BooleanExpression  # noqa: E402
# lit.formats exports the format class under the module's name.
fmt = importlib.import_module("lit.formats.Cpp2RustTest")

BEGIN = "# BEGIN tests that use rules outside the rules subset"
END = "# END tests that use rules outside the rules subset"
# Unmatched declarations that exclude no test: the library's internals, for
# which no rules are generated, which the tests reach through their allocators
# (test_allocator<std::_Rb_tree_node>), comparisons (std::__cmp_cat) and
# iterators (__gnu_cxx::__normal_iterator).
IGNORED = re.compile(r"\b__gnu_cxx::|\bstd::(?:\w+::)*_[_A-Z]")

EXCLUDED_REASON = {
    "unmatched": "# Use library declarations that no rule of the subset matches",
    "failed": "# checkusedrules fails on them",
}


def without_block(text):
    """The text of a lit.local.cfg without the block a previous run wrote."""
    if BEGIN not in text:
        return text
    head, rest = text.split(BEGIN, 1)
    return head.rstrip("\n") + "\n" + rest.split(END, 1)[1].lstrip("\n")


def load_config(config, cfg):
    config = copy.copy(config)
    config.unsupported_paths = set(getattr(config, "unsupported_paths", ()))
    config.available_features = set(config.available_features)
    code = compile(without_block(cfg.read_text()), str(cfg), "exec")
    exec(code, {"config": config, "__file__": str(cfg)})
    return config


def discover(suite_dir):
    """Yields (test path, its lit config) for every test lit runs."""
    base = types.SimpleNamespace(available_features=set(), unsupported=False)
    config = load_config(base, suite_dir / "lit.local.cfg")

    def walk(directory, config):
        cfg = directory / "lit.local.cfg"
        if directory != suite_dir and cfg.is_file():
            config = load_config(config, cfg)
        for entry in sorted(directory.iterdir()):
            if entry.is_dir():
                if entry.name not in getattr(config, "excludes", ()):
                    yield from walk(entry, config)
            elif (
                entry.suffix in (".cpp", ".cc", ".c")
                and not entry.name.endswith(fmt.IGNORED_SUFFIXES)
                and fmt.is_runnable_test(entry)
            ):
                yield entry, config

    yield from walk(suite_dir, config)


def skipped_by_lit(path, rel, config):
    if config.unsupported or rel in config.unsupported_paths:
        return True
    text = fmt.load_source_text(path)
    features = config.available_features
    if fmt.dg_unsupported_reason(text, features):
        return True
    if any(
        not BooleanExpression.evaluate(r, features)
        for r in fmt.parse_feature_list(fmt.RE_REQUIRES, text)
    ):
        return True
    return any(
        BooleanExpression.evaluate(u, features)
        for u in fmt.parse_feature_list(fmt.RE_UNSUPPORTED, text)
    )


def cxxflags(path, config):
    """The flags lit's TestContext.setup gives cpp2rust for the test."""
    text = fmt.load_source_text(path)
    flags = [f"-I{path.parent}", *getattr(config, "extra_cxxflags", [])]
    if default_std := getattr(config, "dg_default_std", None):
        flags.append(f"-std=gnu++{fmt.dg_std(text, default_std)}")
    test_flags = fmt.parse_additional_compile_flags(text, config.available_features)
    force_std = getattr(config, "force_std", None)
    if force_std and path.suffix != ".c":
        test_flags = [f for f in test_flags if not f.startswith("-std=")]
        test_flags.append(f"-std=gnu++{force_std}")
    return flags + test_flags


def no_core_dumps():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def check(checkusedrules, index, path, config):
    """None when every library declaration of the test matches a rule, else
    the reason for excluding it and the details."""
    flags = cxxflags(path, config)
    unmatched = set()
    for source in (path, *fmt.collect_support_sources(path, config)):
        result = subprocess.run(
            [
                checkusedrules,
                "--source",
                source,
                "--rules-index",
                index,
                *(f"--cxxflags={f}" for f in flags),
            ],
            capture_output=True,
            text=True,
            preexec_fn=no_core_dumps,
            check=False,
        )
        if result.returncode != 0:
            return "failed", [f"{source}: exit {result.returncode}"]
        unmatched |= {
            line
            for line in result.stderr.splitlines()
            if line and not IGNORED.search(line)
        }
    return ("unmatched", sorted(unmatched)) if unmatched else None


def write_block(cfg, excluded):
    text = without_block(cfg.read_text()).rstrip("\n") + "\n"
    if not any(excluded.values()):
        cfg.write_text(text)
        return
    block = [BEGIN]
    for reason, paths in excluded.items():
        if not paths:
            continue
        block += [EXCLUDED_REASON[reason], "config.unsupported_paths |= {"]
        block += [f'    "{p}",' for p in sorted(paths)]
        block += ["}"]
    block.append(END)
    cfg.write_text(text + "\n" + "\n".join(block) + "\n")


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("suites", nargs="+", type=Path, help="e.g. tests/libcxx")
    parser.add_argument("--checkusedrules", type=Path, required=True)
    parser.add_argument("--build", type=Path, default=ROOT / "build")
    parser.add_argument("-j", "--jobs", type=int, default=40)
    parser.add_argument("--report", type=Path, help="write each exclusion's details here")
    parser.add_argument(
        "--dry-run", action="store_true", help="leave lit.local.cfg as it is"
    )
    args = parser.parse_args()

    index = args.build / "rules_index"

    source_root = ROOT / "tests"
    # Written as the checks finish, so that a run can be followed.
    report = open(args.report, "w") if args.report else None
    for suite in args.suites:
        suite = suite.resolve()
        tests = []
        for path, config in discover(suite):
            rel = str(path.relative_to(source_root))
            if not skipped_by_lit(path, rel, config):
                tests.append((path, rel, config))
        excluded = {reason: [] for reason in EXCLUDED_REASON}
        with concurrent.futures.ThreadPoolExecutor(args.jobs) as pool:
            futures = {
                pool.submit(check, args.checkusedrules, index, path, config): rel
                for path, rel, config in tests
            }
            for done, future in enumerate(concurrent.futures.as_completed(futures), 1):
                rel = futures[future]
                if outcome := future.result():
                    reason, details = outcome
                    excluded[reason].append(rel)
                    if report:
                        print(rel, reason, *details, sep="\t", file=report, flush=True)
                if done % 500 == 0:
                    print(f"{suite.name}: {done}/{len(tests)} checked", flush=True)
        if not args.dry_run:
            write_block(suite / "lit.local.cfg", excluded)
        kept = len(tests) - sum(map(len, excluded.values()))
        counts = ", ".join(f"{len(v)} {k}" for k, v in excluded.items())
        print(f"{suite.name}: {len(tests)} tests, {kept} kept, excluded: {counts}")
    if report:
        report.close()


if __name__ == "__main__":
    main()
