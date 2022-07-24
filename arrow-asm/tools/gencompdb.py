#!/usr/bin/env python3

import argparse
import glob
import json
from pathlib import Path
import os
import subprocess


def generate_compilation_db(args):
    subprocess.check_call(
        [
            "bazel",
            "build",
            "--noshow_progress",
            "--noshow_loading_progress",
            "--aspects=@bazel_compdb//:aspects.bzl%compilation_database_aspect",
            "--output_groups=compdb_files,header_files",
        ]
        + args.bazel_targets
    )

    execroot = (
        subprocess.check_output(["bazel", "info", "execution_root"]).decode().strip()
    )

    compdb = []
    for compdb_file in Path(execroot).glob("**/*.compile_commands.json"):
        compdb.extend(
            json.loads(
                compdb_file.read_text()
                .replace("__EXEC_ROOT__", execroot)
                .replace("external/llvm_toolchain/bin/cc_wrapper.sh", "clang")
            )
        )
    return compdb


def compile_compilation_db(args, db):
    with open("compile_commands.json", "w") as db_file:
        json.dump(db, db_file, indent=2)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generation CMake-equivalent compilation database"
    )
    parser.add_argument("bazel_targets", nargs="*", default=["//..."])
    args = parser.parse_args()
    compile_compilation_db(args, generate_compilation_db(args))
