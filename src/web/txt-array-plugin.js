const fs = require('fs');
const path = require('path');

module.exports = {
  name: 'txt',
  setup(build) {
    build.onResolve({ filter: /^engine_lib_wasm_bindgen$/ }, args => {
      console.log('hi bill', process.cwd(), fs.readdirSync('.'))
      return { path: path.resolve("./bazel-out/darwin-fastbuild/bin/src/engine/engine_lib_wasm_bindgen.js") };
    });
    // Load ".txt" files and return an array of words
    // build.onLoad({ filter: /\.txt$/ }, async (args) => {
    //   const text = await fs.promises.readFile(args.path, 'utf8');
    //   return {
    //     contents: JSON.stringify(text.split(/\s+/)),
    //     loader: 'json',
    //   }
    // });
  },
};
