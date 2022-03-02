import SheetModel from "../lib/SheetModel";
import React, { useEffect, useRef } from "react";
import Sheet from "./Sheet";

export default function Main() {
  const sheet = useRef<SheetModel>();

  useEffect(() => {
    sheet.current = new SheetModel();
    (window as any).__sheet__ = sheet.current;
  }, [sheet]);

  return (
    <div>
      <Sheet modelRef={sheet} />
    </div>
  )
}
