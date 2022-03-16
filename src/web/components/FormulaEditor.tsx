import React from "react";
import { Controlled as CodeMirror } from "react-codemirror2";
import "codemirror/mode/commonlisp/commonlisp";
import "codemirror/lib/codemirror.css";

export default function FormulaEditor() {
  return (
    <div className="formula-editor">
      <CodeMirror
        value="(defun (x) (car foo))"
        options={{
          mode: "commonlisp",
        }}
        onBeforeChange={(editor, data, value) => {}}
      />
    </div>
  );
}
