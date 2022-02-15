#!/bin/bash
cargo generate-lockfile
cd cargo
cargo raze --verbose
