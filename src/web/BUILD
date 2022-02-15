load("@npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@build_bazel_rules_nodejs//:index.bzl", "js_library")

ts_project(
    name = "lib",
    srcs = glob(["**/*.tsx"]),
    deps = [
        "//src/engine:engine_lib_wasm_bindgen",
        "@npm//@types/react",
        "@npm//@types/react-dom",
        "@npm//react",
        "@npm//react-dom",
    ],
)

js_library(
    name = "bindgen_esbuild_plugin",
    srcs = [
        "bindgen-plugin.js",
    ],
)

esbuild_config(
    name = "esbuild_config",
    config_file = "esbuild.config.mjs",
    deps = [
        ":bindgen_esbuild_plugin",
    ],
)

esbuild(
    name = "bundle",
    config = ":esbuild_config",
    entry_point = "main.tsx",
    deps = [":lib"],
)

http_server(
    name = "server",
    data = [
        "index.html",
        ":bundle",
        "//src/engine:engine_lib_wasm_bindgen",
    ],
    templated_args = ["."],
)
