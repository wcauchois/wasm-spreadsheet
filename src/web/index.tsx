import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import init from "engine_lib_wasm_bindgen";
import Main from "./components/Main";

import "./index.css";

function App() {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    async function doInit() {
      await init("../engine/engine_lib_wasm_bindgen_bg.wasm");
      setInitialized(true);
    }
    doInit();
  }, []);

  return <div>{initialized && <Main />}</div>;
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
