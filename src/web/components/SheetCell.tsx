import React, { useState } from "react";
import classNames from "classnames";

export interface SheetCellProps {
  contents: string;
  onClick(): void;
  focused: boolean;
  onStartEditing(): void;
  editing: boolean;
  onCancelEditing(): void;
  onFinishedEditing(newContents: string): void;
}

export default function SheetCell({
  contents,
  onClick,
  focused,
  onStartEditing,
  onCancelEditing,
  onFinishedEditing,
  editing,
}: SheetCellProps) {
  const [pendingContents, setPendingContents] = useState("");

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
