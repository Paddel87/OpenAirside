# Messergebnisse Spike 1.1

## Native Grundlinie (ohne Electron, 1000 Ticks)

| Arbeit | Threads | Schritt Mittel µs | p99 µs | Prüfsumme |
|---|---|---|---|---|
| 0 | 1 | 86 | 160 | `db519b3de6156cba` |
| 0 | 2 | 48 | 130 | `db519b3de6156cba` |
| 0 | 4 | 36 | 152 | `db519b3de6156cba` |
| 8 | 1 | 778 | 1210 | `e705586487649ab8` |
| 8 | 2 | 431 | 656 | `e705586487649ab8` |
| 8 | 4 | 257 | 479 | `e705586487649ab8` |
| 32 | 1 | 2692 | 3493 | `faf3efd41ef36267` |
| 32 | 2 | 1381 | 1773 | `faf3efd41ef36267` |
| 32 | 4 | 763 | 1072 | `faf3efd41ef36267` |

## Electron (10 s je Lauf, Mittel über Wiederholungen)

| Variante | Threads | Arbeit | Hz | Personen | Ticks/s | Schritt Mittel µs | p99 µs | Rückstand | FPS | Bild p99 ms | Lesen Renderer µs | Übergabe RTT ms (p99) | Prüfsumme |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| napi | 1 | 0 | max | 10000 | 4905 | 203 | 437 | 0/0 | 60.0 | 16.67 | 57 | 1.02 (1.89) | = nativ |
| napi | 4 | 0 | max | 10000 | 7906 | 126 | 395 | 0/0 | 60.0 | 16.67 | 68 | 1.04 (2.19) | = nativ |
| wasm1 | 1 | 0 | max | 10000 | 4034 | 238 | 430 | 0/0 | 59.8 | 16.67 | 41 | 0 (gemeinsamer Speicher) | `db519b3de6156cba` |
| wasmN | 4 | 0 | max | 10000 | 4368 | 227 | 538 | 0/0 | 60.0 | 16.67 | 42 | 0 (gemeinsamer Speicher) | `db519b3de6156cba` |
| napi | 1 | 8 | max | 10000 | 1068 | 933 | 1834 | 0/0 | 60.0 | 16.67 | 58 | 1.18 (3.36) | = nativ |
| napi | 4 | 8 | max | 10000 | 2482 | 401 | 1107 | 0/0 | 60.0 | 16.67 | 62 | 1.10 (2.58) | = nativ |
| wasm1 | 1 | 8 | max | 10000 | 726 | 1184 | 1735 | 0/0 | 60.0 | 16.67 | 44 | 0 (gemeinsamer Speicher) | `e705586487649ab8` |
| wasmN | 4 | 8 | max | 10000 | 1553 | 602 | 1333 | 0/0 | 60.0 | 16.67 | 47 | 0 (gemeinsamer Speicher) | `e705586487649ab8` |
| napi | 1 | 32 | max | 10000 | 342 | 2913 | 4250 | 0/0 | 60.0 | 16.67 | 58 | 1.15 (2.35) | = nativ |
| napi | 4 | 32 | max | 10000 | 1033 | 965 | 2010 | 0/0 | 60.0 | 16.67 | 57 | 1.21 (2.68) | = nativ |
| wasm1 | 1 | 32 | max | 10000 | 155 | 3810 | 5310 | 0/0 | 60.0 | 16.67 | 54 | 0 (gemeinsamer Speicher) | `faf3efd41ef36267` |
| wasmN | 4 | 32 | max | 10000 | 642 | 1325 | 2953 | 0/0 | 60.0 | 16.67 | 46 | 0 (gemeinsamer Speicher) | `faf3efd41ef36267` |
| napi | 4 | 8 | 60 | 10000 | 60 | 732 | 1360 | 1/0 | 60.0 | 16.67 | 56 | 1.15 (2.11) | = nativ |
| wasm1 | 1 | 8 | 60 | 10000 | 52 | 1902 | 2735 | 1/0 | 60.0 | 16.67 | 52 | 0 (gemeinsamer Speicher) | `e705586487649ab8` |
| wasmN | 4 | 8 | 60 | 10000 | 57 | 1008 | 2225 | 0/0 | 60.0 | 16.67 | 52 | 0 (gemeinsamer Speicher) | `e705586487649ab8` |
| napi | 4 | 8 | max | 50000 | 587 | 1698 | 3937 | 0/0 | 60.0 | 16.67 | 232 | 3.37 (5.78) | = nativ |
| wasmN | 4 | 8 | max | 50000 | 397 | 2343 | 4590 | 0/0 | 60.0 | 16.67 | 173 | 0 (gemeinsamer Speicher) | `e705586487649ab8` |
