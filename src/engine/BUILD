load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

package(default_visibility = ["//visibility:public"])

rust_shared_library(
    name = "engine_lib_wasm",
    srcs = ["main.rs"],
    deps = [
        "@rules_rust//wasm_bindgen/raze:wasm_bindgen",
    ],
)

rust_wasm_bindgen(
    name = "engine_lib_wasm_bindgen",
    target = "web",
    wasm_file = ":engine_lib_wasm",
)
