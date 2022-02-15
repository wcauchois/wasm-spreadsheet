load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_shared_library", "rust_test")

package(default_visibility = ["//visibility:public"])

# TODO: Will need to use Raze (?) for rust deps: https://github.com/google/cargo-raze

rust_shared_library(
    name = "engine_lib_wasm",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo:console_error_panic_hook",
        "@rules_rust//wasm_bindgen/raze:wasm_bindgen",
    ],
)

rust_test(
    name = "engine_lib_wasm_test",
    crate = ":engine_lib_wasm",
)

rust_wasm_bindgen(
    name = "engine_lib_wasm_bindgen",
    target = "web",
    wasm_file = ":engine_lib_wasm",
)
