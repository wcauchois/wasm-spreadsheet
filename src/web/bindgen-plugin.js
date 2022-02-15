const path = require('path');

// Inspiration for an ESBuild plugin to load wasm-bindgen output:
// https://github.com/evanw/esbuild/issues/408#issuecomment-757555771

module.exports = {
  name: 'bindgen',
  setup(build) {
    build.onResolve({ filter: /^engine_lib_wasm_bindgen$/ }, args => {
      return {
        path: path.resolve("./bazel-out/darwin-fastbuild/bin/src/engine/engine_lib_wasm_bindgen.js")
      };
    });
  },
};
