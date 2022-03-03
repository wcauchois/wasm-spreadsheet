import SheetModel from "../lib/SheetModel";
import React, { useReducer, useState } from "react";
import Immutable, { Range } from "immutable";
import SheetCell from "./SheetCell";
import { useHotkeys } from "react-hotkeys-hook";
import { clamp } from "../lib/util";

const WIDTH = 10;
const HEIGHT = 10;

export default function Sheet({ model }: { model: SheetModel }) {
  const [cellNonces, incrCellNonce] = useReducer(
    (
      state: Immutable.Seq.Indexed<Immutable.Seq.Indexed<number>>,
      action: {
        row: number;
        col: number;
      }
    ) =>
      state.map((row, rowIndex) =>
        row.map((nonce, colIndex) =>
          action.row === rowIndex && action.col === colIndex ? nonce + 1 : nonce
        )
      ),
    Range(0, HEIGHT).map((_row) => Range(0, WIDTH).map((_col) => 1))
  );

  const [focusedCell, setFocusedCell] = useState<[number, number] | null>(null);
  const [editingCell, setEditingCell] = useState<[number, number] | null>(null);

  const cells = Range(0, HEIGHT)
    .map((row) => (
      <div className="sheet--row" key={`row-${row}`}>
        {Range(0, WIDTH)
          .map((col) => (
            <SheetCell
              key={`cell-${col}-${cellNonces.getIn([row, col])}`}
              contents={model.getCell(row, col) ?? ""}
              onFinishedEditing={(newContents) => {
                model.setCell(row, col, newContents);
                setEditingCell(null);
                incrCellNonce({ row, col });
              }}
              onCancelEditing={() => {
                setEditingCell(null);
              }}
              onClick={() => {
                setFocusedCell([col, row]);
              }}
              onStartEditing={() => {
                setEditingCell([col, row]);
              }}
              focused={
                !!focusedCell &&
                focusedCell[0] === col &&
                focusedCell[1] === row
              }
              editing={
                !!editingCell &&
                editingCell[0] === col &&
                editingCell[1] === row
              }
            />
          ))
          .toJS()}
      </div>
    ))
    .toJS();

  useHotkeys(
    "enter",
    () => {
      setEditingCell(focusedCell);
    },
    [focusedCell]
  );

  const keyToDir: Array<[key: string, dir: [number, number]]> = [
    ["left", [-1, 0]],
    ["right", [1, 0]],
    ["up", [0, -1]],
    ["down", [0, 1]],
  ];

  for (const [key, dir] of keyToDir) {
    useHotkeys(
      key,
      () => {
        if (focusedCell) {
          setFocusedCell([
            clamp(focusedCell[0] + dir[0], 0, WIDTH),
            clamp(focusedCell[1] + dir[1], 0, HEIGHT),
          ]);
        } else {
          setFocusedCell([0, 0]);
        }
      },
      [focusedCell]
    );
  }

  return (
    <div
      className="sheet"
      // onKeyDown={(e) => {
      //   if (e.currentTarget.tagName === "input") {
      //     return;
      //   }
      //   if (e.key === "Enter") {
      //     setEditingCell(focusedCell);
      //   }
      // }}
      // tabIndex={0}
    >
      {cells}
    </div>
  );
}
