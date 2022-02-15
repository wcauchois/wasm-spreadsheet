load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# http_archive(
#     name = "bazel_skylib",
#     sha256 = "af87959afe497dc8dfd4c6cb66e1279cb98ccc84284619ebfec27d9c09a903de",
#     urls = [
#         "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.2.0/bazel-skylib-1.2.0.tar.gz",
#         "https://github.com/bazelbuild/bazel-skylib/releases/download/1.2.0/bazel-skylib-1.2.0.tar.gz",
#     ],
# )

# load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

# bazel_skylib_workspace()

http_archive(
    name = "build_bazel_rules_nodejs",
    sha256 = "c077680a307eb88f3e62b0b662c2e9c6315319385bc8c637a861ffdbed8ca247",
    urls = ["https://github.com/bazelbuild/rules_nodejs/releases/download/5.1.0/rules_nodejs-5.1.0.tar.gz"],
)

load("@build_bazel_rules_nodejs//:repositories.bzl", "build_bazel_rules_nodejs_dependencies")

build_bazel_rules_nodejs_dependencies()

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

load("@rules_rust//wasm_bindgen:repositories.bzl", "rust_wasm_bindgen_repositories")

rust_wasm_bindgen_repositories()

load("@build_bazel_rules_nodejs//:index.bzl", "node_repositories", "yarn_install")

node_repositories()

yarn_install(
    name = "npm",
    package_json = "//:package.json",
    yarn_lock = "//:yarn.lock",
)

load("@build_bazel_rules_nodejs//toolchains/esbuild:esbuild_repositories.bzl", "esbuild_repositories")

esbuild_repositories(npm_repository = "npm")
