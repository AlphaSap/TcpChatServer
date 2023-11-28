#!/bin/bash

export RUST_LOG=debug
cargo run --bin server --release
