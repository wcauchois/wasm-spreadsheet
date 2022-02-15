import Sheet from "../lib/Sheet";
import React, { useEffect, useRef } from "react";

export default function Main() {
  const sheet = useRef<Sheet>();

  useEffect(() => {
    sheet.current = new Sheet();
    (window as any).__sheet__ = sheet.current;
  }, [sheet]);

  return (
    <div>
      Hello world
    </div>
  )
}
