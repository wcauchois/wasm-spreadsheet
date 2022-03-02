import React, { useState } from "react";

export interface SheetCellProps {
  contents: string;
  onClick(): void;
  focused: boolean;
  onStartEditing(): void;
  editing: boolean;
}

export default function SheetCell({
  contents,
  onClick,
  focused,
  onStartEditing,
  editing
}: SheetCellProps) {
  const [pendingContents, setPendingContents] = useState("");

  // TODO: classnames
  let className = "sheet--cell";
  if (focused) {
    className += " sheet--cell__focused";
  }
  return (
    <div className={className} onClick={onClick} onDoubleClick={() => {
      onStartEditing();
      setPendingContents(contents);
    }}>
      {editing ? <input autoFocus type="text" value={pendingContents} onChange={e => {
        setPendingContents(e.currentTarget.value);
      }} /> : <div>
        {contents}
      </div>}
    </div>
  );
}
