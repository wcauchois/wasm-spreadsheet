import React from "react";
import ReactDOM from "react-dom";

// import * as foo from "../../bazel-bin/src/engine/engine_lib_wasm_bindgen";
import foo, { double } from "engine_lib_wasm_bindgen";

function App() {
  return (
    <div>
      Hello, world...
    </div>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);

async function runFoo() {
  console.log('run foo?');
  await foo("../engine/engine_lib_wasm_bindgen_bg.wasm");
  console.log('done?');

  console.log('double is', double(2));
}
runFoo();
