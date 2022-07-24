workspace(name = "beautiful-asm")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_jar")

# Vendor our own toolchain
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
BAZEL_TOOLCHAIN_TAG = "0.7.2"
BAZEL_TOOLCHAIN_SHA = "f7aa8e59c9d3cafde6edb372d9bd25fb4ee7293ab20b916d867cd0baaa642529"
http_archive(
    name = "com_grail_bazel_toolchain",
    sha256 = BAZEL_TOOLCHAIN_SHA,
    strip_prefix = "bazel-toolchain-{tag}".format(tag = BAZEL_TOOLCHAIN_TAG),
    canonical_id = BAZEL_TOOLCHAIN_TAG,
    url = "https://github.com/grailbio/bazel-toolchain/archive/{tag}.tar.gz".format(tag = BAZEL_TOOLCHAIN_TAG),
)
load("@com_grail_bazel_toolchain//toolchain:deps.bzl", "bazel_toolchain_dependencies")
bazel_toolchain_dependencies()
load("@com_grail_bazel_toolchain//toolchain:rules.bzl", "llvm_toolchain")
llvm_toolchain(
    name = "llvm_toolchain",
    llvm_version = "12.0.0",
)
load("@llvm_toolchain//:toolchains.bzl", "llvm_register_toolchains")
llvm_register_toolchains()

# Build ANTLR codegen
ANTLR4_VERSION = "4.10.1"
git_repository(
    name = "com_github_google_cel-cpp",
    commit = "1e0fd3d957a22e853c9b9bc9f682eaba67b9757f",
    remote = "https://github.com/google/cel-cpp",
    shallow_since = "1654569490 -0700"
)
http_archive(
    name = "antlr4_runtimes",
    build_file_content = """
package(default_visibility = ["//visibility:public"])
cc_library(
    name = "cpp",
    srcs = glob(["runtime/Cpp/runtime/src/**/*.cpp"]),
    hdrs = glob(["runtime/Cpp/runtime/src/**/*.h"]),
    includes = ["runtime/Cpp/runtime/src"],
)
  """,
    sha256 = "a320568b738e42735946bebc5d9d333170e14a251c5734e8b852ad1502efa8a2",
    strip_prefix = "antlr4-" + ANTLR4_VERSION,
    urls = ["https://github.com/antlr/antlr4/archive/v" + ANTLR4_VERSION + ".tar.gz"],
)
http_jar(
    name = "antlr4_jar",
    urls = ["https://www.antlr.org/download/antlr-" + ANTLR4_VERSION + "-complete.jar"],
    sha256 = "41949d41f20d31d5b8277187735dd755108df52b38db6c865108d3382040f918",
)

# Build C++ binaries
http_archive(
    name = "rules_cc",
    sha256 = "76737f9070decb43c9f1c6f1f7c4e40555de09f4312ad7e63f4f4727911d1478",
    strip_prefix = "rules_cc-9ec8187d589e7554e8cfe8d14b3917ebe4b94940",
    urls = ["https://github.com/bazelbuild/rules_cc/archive/9ec8187d589e7554e8cfe8d14b3917ebe4b94940.tar.gz"],
)

# Generate compilation databases
http_archive(
    name = "bazel_compdb",
    #patches = ["//foreign:comp_db.patch"],
    sha256 = "d51f8168954d4aa0ca984f53a1a6be298d827ff39303d10522dffb2a5c1942dc",
    strip_prefix = "bazel-compilation-database-0.5.0",
    urls = ["https://github.com/grailbio/bazel-compilation-database/archive/0.5.0.tar.gz"],
)

# ------ C++ LIBRARIES ------
http_archive(
    name = "com_github_fmtlib_fmt",
    build_file_content = """
load("@rules_cc//cc:defs.bzl", "cc_library")
cc_library(
    name = "fmt",
    hdrs = glob([
        "include/fmt/*.h",
    ]),
    defines = ["FMT_HEADER_ONLY"],
    includes = ["include"],
    visibility = ["//visibility:public"],
)
""",
    sha256 = "335b41fe614a35610ab92f8c1525e57cdbde75a72f2eb8e84949e4c25d8c31f8",
    strip_prefix = "fmt-d9f045fba18c6897ae0931a931450638560e3fd4",
    urls = ["https://github.com/fmtlib/fmt/archive/d9f045fba18c6897ae0931a931450638560e3fd4.tar.gz"],
)

http_archive(
    name = "com_github_gabime_spdlog",
    build_file_content = """
load("@rules_cc//cc:defs.bzl", "cc_library")
cc_library(
    name = "spdlog",
    hdrs = glob([
        "include/**/*.h",
    ]),
    defines = ["SPDLOG_FMT_EXTERNAL"],
    includes = ["include"],
    visibility = ["//visibility:public"],
    deps = ["@com_github_fmtlib_fmt//:fmt"],
)
""",
    sha256 = "d00ccce646e39286d7ca9c45ca13b22828e1a63ed5a2cb6769db936f1e2b1668",
    strip_prefix = "spdlog-729d7f6d8837b6693e7b378408518ea1710f80cb",
    urls = ["https://github.com/gabime/spdlog/archive/729d7f6d8837b6693e7b378408518ea1710f80cb.tar.gz"],
)

git_repository(
    name = "com_github_google_googletest",
    commit = "620659ed92829a88ee34134c782bf5b5aa5a0a0c",
    remote = "https://github.com/google/googletest",
    shallow_since = "1603130496 -0400",
)

load("//foreign:llvm.bzl", "llvm")
llvm(
    name = "llvm-darwin-x86_64",
    workspace_name = "beautiful-asm",
    host_triple = "x86_64-apple-darwin",
)
llvm(
    name = "llvm-linux-aarch64",
    workspace_name = "beautiful-asm",
    host_triple = "aarch64-linux-gnu",
)
llvm(
    name = "llvm-linux-x86_64",
    workspace_name = "beautiful-asm",
    host_triple = "x86_64-linux-gnu-ubuntu-18.04",
)
