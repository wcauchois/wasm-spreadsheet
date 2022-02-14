load("@build_bazel_rules_nodejs//:index.bzl", "nodejs_test")
load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "hello_world_lib_wasm",
    srcs = ["main.rs"],
    deps = [
        "@rules_rust//wasm_bindgen/raze:wasm_bindgen",
    ],
)

rust_wasm_bindgen(
    name = "hello_world_nodejs_wasm_bindgen",
    target = "nodejs",
    wasm_file = ":hello_world_lib_wasm",
)

nodejs_test(
    name = "hello_world_wasm_test",
    data = [
        ":hello_world_nodejs_wasm_bindgen",
    ],
    entry_point = "hello_world_wasm_test.js",
)
