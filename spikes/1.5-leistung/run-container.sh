#!/usr/bin/env bash
# Spike 1.5: Messreihe im Container fuer Wegweiser mit fuenf Etagen und den
# Simulationsschritt im Fuenf-Etagen-Szenario (Kern aus spikes/1.2-wegweiser).
# Voraussetzungen: bash 4+, Rust laut rust-toolchain.toml des Spikes 1.2.
# Plattformen: Linux (liest /proc fuer Speicherwerte); macOS/Windows nicht unterstuetzt.
# Idempotenz: ueberschreibt results/fields-5.jsonl und results/run-5.jsonl.
set -euo pipefail
cd "$(dirname "$0")"
CORE=../1.2-wegweiser
cargo build --release --manifest-path "$CORE/Cargo.toml" --bin run
R="$CORE/target/release/run"
: > results/fields-5.jsonl
for size in 512 768 1024; do
  for gates in 17 45; do
    for t in 1 2 4; do
      "$R" fields scenario=5 w=$size h=$size gates=$gates threads=$t incremental=0 >> results/fields-5.jsonl
    done
  done
done
: > results/run-5.jsonl
for size in 512 768 1024; do
  "$R" run scenario=5 w=$size h=$size ticks=12000 window=3000 bg=4 threads=4 incremental=0 >> results/run-5.jsonl
done
# Ohne Bauaenderungen: reine Schrittzeit, ungebremst (hoechstmoegliche Tickrate).
for size in 512 1024; do
  "$R" run scenario=5 w=$size h=$size ticks=12000 window=3000 bg=4 threads=4 incremental=0 commands=0 >> results/run-5.jsonl
done
