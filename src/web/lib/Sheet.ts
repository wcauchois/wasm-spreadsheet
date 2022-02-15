import { Sheet as EngineSheet } from "engine_lib_wasm_bindgen";

export default class Sheet {
  private readonly underlying: EngineSheet;

  constructor() {
    this.underlying = EngineSheet.new();
  }

  getCell(row: number, col: number) {
    return this.underlying.get_cell(row, col);
  }

  setCell(row: number, col: number, value: number) {
    this.underlying.set_cell(row, col, value);
  }
}
