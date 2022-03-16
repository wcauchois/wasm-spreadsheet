import React, { useEffect, useState } from "react";
import classNames from "classnames";
import SheetModel from "../lib/SheetModel";
import { JsSheetCellInfo } from "engine_lib_wasm_bindgen";

export interface SheetCellProps {
  model: SheetModel;
  row: number;
  col: number;
  onClick(): void;
  focused: boolean;
  onStartEditing(): void;
  editing: boolean;
  onCancelEditing(): void;
  onFinishedEditing(): void;
  pendingContents: string;
  setPendingContents(newContents: string): void;
}

export default function SheetCell({
  model,
  row,
  col,
  onClick,
  focused,
  onStartEditing,
  onCancelEditing,
  onFinishedEditing,
  editing,
  pendingContents,
  setPendingContents,
}: SheetCellProps) {
  const [contents, setContents] = useState(() => model.getCell(row, col).value);

  useEffect(() => {
    function listener(arg: JsSheetCellInfo) {
      setContents(arg.value);
    }
    model.addListener(row, col, listener);
    return () => {
      model.removeListener(row, col, listener);
    };
  }, [model, row, col]);

  const className = classNames("sheet--cell", {
    "sheet--cell__focused": focused,
  });

  return (
    <td
      className={className}
      onClick={onClick}
      onDoubleClick={() => {
        if (!editing) {
          onStartEditing();
        }
      }}
    >
      {editing ? (
        <input
          type="text"
          value={pendingContents}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              onCancelEditing();
            }
            if (e.key === "Enter") {
              onFinishedEditing();
            }
          }}
          onChange={(e) => {
            setPendingContents(e.currentTarget.value);
          }}
        />
      ) : (
        <div>{contents}</div>
      )}
    </td>
  );
}
