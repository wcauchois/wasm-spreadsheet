load("@rules_rust//wasm_bindgen:wasm_bindgen.bzl", "rust_wasm_bindgen")
load("@rules_rust//rust:defs.bzl", "rust_shared_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_shared_library(
    name = "engine_lib_wasm",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo:console_error_panic_hook",
        "//cargo:imbl",
        "//cargo:js_sys",
        "//cargo:lazy_static",
        "//cargo:nom",
        "//cargo:signals2",
        "@rules_rust//wasm_bindgen/raze:wasm_bindgen",
    ],
)

rust_test(
    name = "engine_lib_wasm_test",
    crate = ":engine_lib_wasm",
    # env = {
    #     "RUST_TEST_NOCAPTURE": "1",
    # },
)

rust_wasm_bindgen(
    name = "engine_lib_wasm_bindgen",
    target = "web",
    wasm_file = ":engine_lib_wasm",
)
