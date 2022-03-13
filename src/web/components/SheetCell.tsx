import React, { useEffect, useState } from "react";
import classNames from "classnames";
import SheetModel from "../lib/SheetModel";

export interface SheetCellProps {
  model: SheetModel;
  row: number;
  col: number;
  onClick(): void;
  focused: boolean;
  onStartEditing(): void;
  editing: boolean;
  onCancelEditing(): void;
  onFinishedEditing(newContents: string): void;
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
}: SheetCellProps) {
  const [contents, setContents] = useState(() => model.getCell(row, col));
  const [pendingContents, setPendingContents] = useState("");

  useEffect(() => {
    function listener() {
      setTimeout(() => {
        setContents(model.getCell(row, col));
      }, 0);
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
        onStartEditing();
        setPendingContents(contents);
      }}
    >
      {editing ? (
        <input
          autoFocus
          type="text"
          value={pendingContents}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              onCancelEditing();
            }
            if (e.key === "Enter") {
              onFinishedEditing(pendingContents);
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
