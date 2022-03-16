import React from "react";
import { Controlled as CodeMirror } from "react-codemirror2";
import "codemirror/mode/commonlisp/commonlisp";
import "codemirror/lib/codemirror.css";

export interface FormulaEditorProps {
  contents: string;
  setContents(newContents: string): void;
}

export default function FormulaEditor({
  contents,
  setContents,
}: FormulaEditorProps) {
  return (
    <div className="formula-editor">
      <CodeMirror
        value={contents}
        options={{
          mode: "commonlisp",
        }}
        onBeforeChange={(editor, data, value) => {
          setContents(value);
        }}
      />
    </div>
  );
}
