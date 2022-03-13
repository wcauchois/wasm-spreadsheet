"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__bitmaps__2_1_0",
        url = "https://crates.io/api/v1/crates/bitmaps/2.1.0/download",
        type = "tar.gz",
        strip_prefix = "bitmaps-2.1.0",
        build_file = Label("//cargo/remote:BUILD.bitmaps-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bumpalo__3_9_1",
        url = "https://crates.io/api/v1/crates/bumpalo/3.9.1/download",
        type = "tar.gz",
        sha256 = "a4a45a46ab1f2412e53d3a0ade76ffad2025804294569aae387231a0cd6e0899",
        strip_prefix = "bumpalo-3.9.1",
        build_file = Label("//cargo/remote:BUILD.bumpalo-3.9.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__console_error_panic_hook__0_1_7",
        url = "https://crates.io/api/v1/crates/console_error_panic_hook/0.1.7/download",
        type = "tar.gz",
        sha256 = "a06aeb73f470f66dcdbf7223caeebb85984942f22f1adb2a088cf9668146bbbc",
        strip_prefix = "console_error_panic_hook-0.1.7",
        build_file = Label("//cargo/remote:BUILD.console_error_panic_hook-0.1.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__imbl__1_0_1",
        url = "https://crates.io/api/v1/crates/imbl/1.0.1/download",
        type = "tar.gz",
        strip_prefix = "imbl-1.0.1",
        build_file = Label("//cargo/remote:BUILD.imbl-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__js_sys__0_3_56",
        url = "https://crates.io/api/v1/crates/js-sys/0.3.56/download",
        type = "tar.gz",
        strip_prefix = "js-sys-0.3.56",
        build_file = Label("//cargo/remote:BUILD.js-sys-0.3.56.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_14",
        url = "https://crates.io/api/v1/crates/log/0.4.14/download",
        type = "tar.gz",
        sha256 = "51b9bbe6c47d51fc3e1a9b945965946b4c44142ab8792c50835a980d362c2710",
        strip_prefix = "log-0.4.14",
        build_file = Label("//cargo/remote:BUILD.log-0.4.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memchr__2_4_1",
        url = "https://crates.io/api/v1/crates/memchr/2.4.1/download",
        type = "tar.gz",
        strip_prefix = "memchr-2.4.1",
        build_file = Label("//cargo/remote:BUILD.memchr-2.4.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__minimal_lexical__0_2_1",
        url = "https://crates.io/api/v1/crates/minimal-lexical/0.2.1/download",
        type = "tar.gz",
        strip_prefix = "minimal-lexical-0.2.1",
        build_file = Label("//cargo/remote:BUILD.minimal-lexical-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nom__7_1_0",
        url = "https://crates.io/api/v1/crates/nom/7.1.0/download",
        type = "tar.gz",
        strip_prefix = "nom-7.1.0",
        build_file = Label("//cargo/remote:BUILD.nom-7.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_36",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.36/download",
        type = "tar.gz",
        sha256 = "c7342d5883fbccae1cc37a2353b09c87c9b0f3afd73f5fb9bba687a1f733b029",
        strip_prefix = "proc-macro2-1.0.36",
        build_file = Label("//cargo/remote:BUILD.proc-macro2-1.0.36.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_15",
        url = "https://crates.io/api/v1/crates/quote/1.0.15/download",
        type = "tar.gz",
        sha256 = "864d3e96a899863136fc6e99f3d7cae289dafe43bf2c5ac19b70df7210c0a145",
        strip_prefix = "quote-1.0.15",
        build_file = Label("//cargo/remote:BUILD.quote-1.0.15.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_core__0_6_3",
        url = "https://crates.io/api/v1/crates/rand_core/0.6.3/download",
        type = "tar.gz",
        strip_prefix = "rand_core-0.6.3",
        build_file = Label("//cargo/remote:BUILD.rand_core-0.6.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_xoshiro__0_6_0",
        url = "https://crates.io/api/v1/crates/rand_xoshiro/0.6.0/download",
        type = "tar.gz",
        strip_prefix = "rand_xoshiro-0.6.0",
        build_file = Label("//cargo/remote:BUILD.rand_xoshiro-0.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__signals2__0_3_2",
        url = "https://crates.io/api/v1/crates/signals2/0.3.2/download",
        type = "tar.gz",
        strip_prefix = "signals2-0.3.2",
        build_file = Label("//cargo/remote:BUILD.signals2-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__sized_chunks__0_6_5",
        url = "https://crates.io/api/v1/crates/sized-chunks/0.6.5/download",
        type = "tar.gz",
        strip_prefix = "sized-chunks-0.6.5",
        build_file = Label("//cargo/remote:BUILD.sized-chunks-0.6.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_86",
        url = "https://crates.io/api/v1/crates/syn/1.0.86/download",
        type = "tar.gz",
        sha256 = "8a65b3f4ffa0092e9887669db0eae07941f023991ab58ea44da8fe8e2d511c6b",
        strip_prefix = "syn-1.0.86",
        build_file = Label("//cargo/remote:BUILD.syn-1.0.86.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__typenum__1_15_0",
        url = "https://crates.io/api/v1/crates/typenum/1.15.0/download",
        type = "tar.gz",
        strip_prefix = "typenum-1.15.0",
        build_file = Label("//cargo/remote:BUILD.typenum-1.15.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_xid__0_2_2",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.2/download",
        type = "tar.gz",
        sha256 = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3",
        strip_prefix = "unicode-xid-0.2.2",
        build_file = Label("//cargo/remote:BUILD.unicode-xid-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__version_check__0_9_4",
        url = "https://crates.io/api/v1/crates/version_check/0.9.4/download",
        type = "tar.gz",
        strip_prefix = "version_check-0.9.4",
        build_file = Label("//cargo/remote:BUILD.version_check-0.9.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen__0_2_79",
        url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.79/download",
        type = "tar.gz",
        sha256 = "25f1af7423d8588a3d840681122e72e6a24ddbcb3f0ec385cac0d12d24256c06",
        strip_prefix = "wasm-bindgen-0.2.79",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-0.2.79.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_backend__0_2_79",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-backend/0.2.79/download",
        type = "tar.gz",
        sha256 = "8b21c0df030f5a177f3cba22e9bc4322695ec43e7257d865302900290bcdedca",
        strip_prefix = "wasm-bindgen-backend-0.2.79",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-backend-0.2.79.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro__0_2_79",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.79/download",
        type = "tar.gz",
        sha256 = "2f4203d69e40a52ee523b2529a773d5ffc1dc0071801c87b3d270b471b80ed01",
        strip_prefix = "wasm-bindgen-macro-0.2.79",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-0.2.79.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro_support__0_2_79",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.79/download",
        type = "tar.gz",
        sha256 = "bfa8a30d46208db204854cadbb5d4baf5fcf8071ba5bf48190c3e59937962ebc",
        strip_prefix = "wasm-bindgen-macro-support-0.2.79",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-support-0.2.79.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_shared__0_2_79",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.79/download",
        type = "tar.gz",
        sha256 = "3d958d035c4438e28c70e4321a2911302f10135ce78a9c7834c0cab4123d06a2",
        strip_prefix = "wasm-bindgen-shared-0.2.79",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-shared-0.2.79.bazel"),
    )
