const path = require('path');

module.exports = {
  name: 'bindgen',
  setup(build) {
    build.onResolve({ filter: /^engine_lib_wasm_bindgen$/ }, args => {
      return { path: path.resolve("./bazel-out/darwin-fastbuild/bin/src/engine/engine_lib_wasm_bindgen.js") };
    });
  },
};
