#!/usr/bin/env bash
# Spike 1.2 – full measurement series; writes results/*.jsonl.
# Voraussetzungen: bash 4+, Rust 1.98.1 (rust-toolchain.toml), Linux (/proc für Speichermessung).
# Plattformen: Linux (gemessen); macOS/Windows nicht unterstützt (Speichermessung über /proc).
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --quiet
R=target/release/run
mkdir -p results
: > results/recompute.jsonl
for bg in 1 2 4; do for inc in 0 1; do
  $R run ticks=6000 window=2000 bg=$bg incremental=$inc >> results/recompute.jsonl
done; done
: > results/layout.jsonl
for layout in aos split; do for t in 1 2 4; do
  $R run ticks=5000 window=5000 commands=0 layout=$layout threads=$t >> results/layout.jsonl
  $R run ticks=3000 window=3000 commands=0 layout=$layout threads=$t persons=50000 >> results/layout.jsonl
done; done
: > results/determinism.jsonl
for layout in aos split; do for t in 1 4; do
  $R run ticks=6000 window=6000 bg=4 incremental=1 layout=$layout threads=$t >> results/determinism.jsonl
done; done
$R run ticks=6000 window=6000 bg=2 incremental=1 layout=split threads=4 >> results/determinism.jsonl
$R run ticks=6000 window=3000 bg=4 incremental=0 w=1024 h=1024 gates=52 > results/large.jsonl
$R run ticks=6000 window=3000 bg=4 incremental=1 w=1024 h=1024 gates=52 >> results/large.jsonl
$R run ticks=60000 window=5000 bg=4 incremental=1 > results/longrun.jsonl
