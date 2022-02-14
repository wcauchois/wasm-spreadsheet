load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "8e190ea711500bf076f8de6c4c2729ac0d676a992a3d8aefb409f1e786a3f080",
    strip_prefix = "rules_rust-c435cf4478fc6e097edc5dba0e71de6608ab77d8",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/c435cf4478fc6e097edc5dba0e71de6608ab77d8.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()
