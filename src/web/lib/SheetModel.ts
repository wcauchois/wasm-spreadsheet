import { JsSheet, JsSheetCellInfo } from "engine_lib_wasm_bindgen";

export default class SheetModel {
  private readonly underlying: JsSheet;

  constructor() {
    this.underlying = JsSheet.new();
  }

  getCell(row: number, col: number) {
    return this.underlying.get_cell(row, col);
  }

  setCell(row: number, col: number, contents: string) {
    this.underlying.set_cell(row, col, contents);
  }

  addListener(
    row: number,
    col: number,
    listener: (arg: JsSheetCellInfo) => void
  ) {
    this.underlying.add_listener(row, col, listener);
  }

  removeListener(
    row: number,
    col: number,
    listener: (arg: JsSheetCellInfo) => void
  ) {
    this.underlying.remove_listener(row, col, listener);
  }
}
