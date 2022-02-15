load("@npm//@bazel/esbuild:index.bzl", "esbuild")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")

ts_project(
    name = "lib",
    srcs = ["main.tsx"],
    deps = [
        "@npm//@types/react",
        "@npm//@types/react-dom",
        "@npm//react",
        "@npm//react-dom",
    ],
)

esbuild(
    name = "bundle",
    entry_point = "main.tsx",
    deps = [":lib"],
)

http_server(
    name = "server",
    data = [
        "index.html",
        ":bundle",
    ],
    templated_args = ["./src/web"],
)

# https://github.com/bazelbuild/rules_nodejs/blob/stable/examples/react_webpack/BUILD.bazel

# yarn_install(
#     name = "yarn",
#     package_json = ":package.json",
#     yarn_lock = ":yarn.lock",
# )
