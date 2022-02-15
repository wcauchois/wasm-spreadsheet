load("@npm//@bazel/esbuild:index.bzl", "esbuild", "esbuild_config")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@build_bazel_rules_nodejs//:index.bzl", "js_library")

ts_project(
    name = "lib",
    srcs = ["main.tsx"],
    # glob(["**/*.ts"])
    deps = [
        "//src/engine:engine_lib_wasm_bindgen",
        "@npm//@types/react",
        "@npm//@types/react-dom",
        "@npm//react",
        "@npm//react-dom",
    ],
)

js_library(
    name = "txt_array_plugin",
    srcs = [
        "txt-array-plugin.js",
    ],
)

esbuild_config(
    name = "esbuild_config",
    config_file = "esbuild.config.mjs",
    deps = [
        ":txt_array_plugin",
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

# yarn_install(
#     name = "yarn",
#     package_json = ":package.json",
#     yarn_lock = ":yarn.lock",
# )
