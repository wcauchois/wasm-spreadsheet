#!/usr/bin/env python3
import argparse
import subprocess

parser = argparse.ArgumentParser(description="Task runner for wasm-spreadsheet")
subparsers = parser.add_subparsers(title="commands")


def snake_to_kebab_case(s):
    return s.replace("_", "-")


def cmd(func):
    global subparsers
    sub_parser = subparsers.add_parser(
        snake_to_kebab_case(func.__name__), help=func.__doc__
    )
    sub_parser.set_defaults(func=func)
    return func


@cmd
def build_wasm():
    "Build the WASM library"
    subprocess.check_call(
        "bazel build //src/engine:engine_lib_wasm_bindgen", shell=True
    )


@cmd
def serve():
    "Watch source code and run the development HTTP server"
    try:
        subprocess.check_call("ibazel run //src/web:server", shell=True)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    args = parser.parse_args()
    args.func()
