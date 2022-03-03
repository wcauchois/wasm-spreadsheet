import SheetModel from "../lib/SheetModel";
import React, { useEffect, useRef, useState } from "react";
import Sheet from "./Sheet";

export default function Main() {
  const [model] = useState(() => {
    const ret = new SheetModel();
    (window as any).__sheet__ = ret;
    return ret;
  });

  return (
    <div>
      <Sheet model={model} />
    </div>
  );
}
