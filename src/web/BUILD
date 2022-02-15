# load("@npm//@bazel/esbuild:index.bzl", "esbuild")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")

ts_project(
    name = "web",
    srcs = ["main.tsx"],
)

http_server(
    name = "server",
    data = [
        "app.bundle.js",
        "index.html",
    ],
    templated_args = ["."],
)

# https://github.com/bazelbuild/rules_nodejs/blob/stable/examples/react_webpack/BUILD.bazel

# yarn_install(
#     name = "yarn",
#     package_json = ":package.json",
#     yarn_lock = ":yarn.lock",
# )
