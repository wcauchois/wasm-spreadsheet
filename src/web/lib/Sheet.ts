import { JsSheet } from "engine_lib_wasm_bindgen";

export default class Sheet {
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
}
