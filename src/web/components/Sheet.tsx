import SheetModel from "../lib/SheetModel";
import React, { useState } from "react";
import { Range } from "immutable";
import SheetCell from "./SheetCell";

const WIDTH = 10;
const HEIGHT = 10;

export default function Sheet({
  modelRef
}: {
  modelRef: React.MutableRefObject<SheetModel | undefined>
}) {
  /*
  [
    [1, 2],
    [3, 4]
  ]
  */
  const [cellNonces, setCellNonces] = useState(() =>
    Range(0, HEIGHT).map((_col) => Range(0, WIDTH).map((_row) => 1))
  );

  const [focusedCell, setFocusedCell] = useState<[number, number] | null>(null);
  const [editingCell, setEditingCell] = useState<[number, number] | null>(null);

  const cells = Range(0, HEIGHT).map((col) => (
    <div className="sheet--row">
      {Range(0, WIDTH).map((row) => (
        <SheetCell
          key={cellNonces.getIn([col, row]) as number}
          contents="hi"
          onClick={() => {
            setFocusedCell([row, col]);
          }}
          onStartEditing={() => {
            setEditingCell([row, col]);
          }}
          focused={!!focusedCell && focusedCell[0] === row && focusedCell[1] === col}
          editing={!!editingCell && editingCell[0] === row && editingCell[1] === col}
        />
      )).toJS()}
    </div>
  )).toJS();

  return (
    <div className="sheet">
      {cells}
    </div>
  );
}
