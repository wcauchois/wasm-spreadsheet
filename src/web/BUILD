load("@npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@build_bazel_rules_nodejs//:index.bzl", "js_library")

ts_project(
    name = "lib",
    srcs = glob([
        "**/*.tsx",
        "**/*.ts",
    ]),
    deps = [
        "//src/engine:engine_lib_wasm_bindgen",
        "@npm//@types/codemirror",
        "@npm//@types/react",
        "@npm//@types/react-dom",
        "@npm//classnames",
        "@npm//codemirror",
        "@npm//immutable",
        "@npm//react",
        "@npm//react-codemirror2",
        "@npm//react-dom",
        "@npm//react-hotkeys-hook",
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
    srcs = glob(["**/*.css"]),
    config_file = "esbuild.config.mjs",
    deps = [
        ":bindgen_esbuild_plugin",
    ],
)

esbuild(
    name = "bundle",
    config = ":esbuild_config",
    entry_point = "index.tsx",
    output_css = "bundle.css",
    deps = [
        ":lib",
    ],
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
