import SheetModel from "../lib/SheetModel";
import React, { useState } from "react";
import { Range } from "immutable";
import SheetCell from "./SheetCell";
import { useHotkeys } from "react-hotkeys-hook";
import { clamp } from "../lib/util";
import FormulaEditor from "./FormulaEditor";

const WIDTH = 10;
const HEIGHT = 10;

export default function Sheet({ model }: { model: SheetModel }) {
  const [focusedCell, rawSetFocusedCell] = useState<[number, number] | null>(
    null
  );
  const [editingCell, setEditingCell] = useState<[number, number] | null>(null);
  const [pendingContents, setPendingContents] = useState("");

  function setFocusedCell([newCol, newRow]: [number, number]) {
    if (focusedCell) {
      const [oldCol, oldRow] = focusedCell;
      if (newCol !== oldCol || newRow !== oldRow) {
        model.setCell(oldRow, oldCol, pendingContents);
        setPendingContents(model.getCell(newRow, newCol).source);
        setEditingCell(null);
      }
    }
    rawSetFocusedCell([newCol, newRow]);
  }

  const cells = Range(0, HEIGHT)
    .map((row) => (
      <tr className="sheet--row" key={`row-${row}`}>
        {Range(0, WIDTH)
          .map((col) => (
            <SheetCell
              key={`cell-${row}-${col}`}
              model={model}
              row={row}
              col={col}
              onFinishedEditing={() => {
                model.setCell(row, col, pendingContents);
                setEditingCell(null);
              }}
              pendingContents={pendingContents}
              setPendingContents={setPendingContents}
              onCancelEditing={() => {
                setEditingCell(null);
                setPendingContents(model.getCell(row, col).source);
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
      </tr>
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
    <div>
      <FormulaEditor
        contents={pendingContents}
        setContents={(newContents) => {
          if (focusedCell) {
            setPendingContents(newContents);
            setEditingCell(focusedCell);
          }
        }}
      />
      <table className="sheet">
        <tbody>{cells}</tbody>
      </table>
    </div>
  );
}
