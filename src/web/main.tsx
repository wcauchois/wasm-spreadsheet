import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";

import init, { double } from "engine_lib_wasm_bindgen";

function Main() {
  return (
    <div>
      Hello world, the double is {double(2)}
    </div>
  )
}

function App() {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    async function doInit() {
      await init("../engine/engine_lib_wasm_bindgen_bg.wasm");
      setInitialized(true);
    }
    doInit();
  }, []);

  return (
    <div>
      {initialized && <Main />}
    </div>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
