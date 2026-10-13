# Spike 1.1 – Betriebsart des Rust-Kerns

Fahrplan-Schritt 1.1 (Phase 1, ERKUNDUNG, Vergleichsstudie). **Wegwerf-Code.** Was davon weiterverwendet wird, wird in Phase 2 neu aufgebaut.

Frage: Läuft der Rust-Simulationskern als **natives Node-Modul (napi-rs)** in einem Electron-Utility-Prozess (Variante A) oder als **WebAssembly in einem Web Worker** des Renderers (Variante B)?

## Aufbau

| Teil | Inhalt |
|---|---|
| `sim-core/` | Gemeinsamer Kern: 10.000 Personen folgen Wegweisern (8 Ziele, Raster 512 × 512, Wände mit Türen), Bedürfnisse, Wartezeiten, Zufallszahlen je Person. Parallel mit rayon, deterministisch. `bench` misst ohne Electron. |
| `sim-napi/` | Variante A: napi-rs-Modul. Simulation auf eigenem Rust-Thread mit rayon-Pool, fertiges Bild wird per Pufferwechsel veröffentlicht. |
| `sim-wasm/` | Variante B: wasm-bindgen. `wasm1` = ein Thread (stabiles Rust), `wasmN` = mehrere Worker auf gemeinsamem WebAssembly-Speicher (Nightly-Rust, siehe Befund 3). |
| `app/` | Electron 44.7.0 mit Härtung (contextIsolation, Sandbox, kein nodeIntegration, eigenes Protokoll `app://` mit COOP/COEP). Renderer liest je Bild den neuesten Zustand und läuft 10 s. |
| `run-matrix.mjs`, `summarize.mjs` | Messreihe und Auswertung; Rohdaten in `results/matrix.jsonl`, Tabelle in `results/summary.md`. |

**Übergabe an den Renderer:**

- **A (napi):** Der Renderer fordert je Bild über einen `MessagePort` das neueste Bild an; der Utility-Prozess kopiert es (Kopie über Prozessgrenze). Ein `SharedArrayBuffer` kann nicht zwischen Prozessen geteilt werden, und ein natives Modul ist im Sandbox-Renderer nicht ladbar.
- **B (wasm):** Dreifachpuffer im `SharedArrayBuffer`; der Renderer liest ohne Kopie und ohne Nachricht.

**Arbeit (`work`):** Zusatzrechnung je Person und Tick, um eine schwerere spätere Simulation nachzubilden (0 = nur Wegweiser folgen; 8 ≈ 0,8 ms, 32 ≈ 2,7 ms je Tick auf einem Kern nativ).

## Ausführen

Voraussetzungen: rustup (installiert Rust 1.98.1 aus `rust-toolchain.toml`), für `wasmN` zusätzlich `nightly-2026-10-12` mit `rust-src`, `wasm-bindgen-cli` 0.2.128, Node 22+; unter Linux ohne Bildschirm `xvfb-run`.

```bash
cd spikes/1.1-betriebsart
cargo build --release -p sim-core -p sim-napi
mkdir -p app/native && cp target/release/libsim_napi.so app/native/sim_napi.node   # Linux; macOS: .dylib, Windows: .dll
cargo build --release -p sim-wasm --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir app/renderer/pkg-st target/wasm32-unknown-unknown/release/sim_wasm.wasm
RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-args=--shared-memory \
  -C link-args=--max-memory=1073741824 -C link-args=--import-memory -C link-args=--export=__wasm_init_tls \
  -C link-args=--export=__tls_size -C link-args=--export=__tls_align -C link-args=--export=__tls_base" \
  cargo +nightly-2026-10-12 build --release -p sim-wasm --target wasm32-unknown-unknown --target-dir target-mt \
  -Z build-std=panic_abort,std
wasm-bindgen --target web --out-dir app/renderer/pkg-mt target-mt/wasm32-unknown-unknown/release/sim_wasm.wasm
(cd app && npm ci)
node run-matrix.mjs 2 && node summarize.mjs
```

Ein einzelner Lauf: `cd app && npx electron . --variant=napi --threads=4 --work=8 --seconds=10` (als root zusätzlich `--no-sandbox`).

## Messumgebung

- Cloud-Container: Intel Xeon 2,1 GHz, **4 vCPU**, 15 GB RAM, **keine GPU** (Xvfb, Software-Darstellung), Linux. Electron als root mit `--no-sandbox` (Chromium verweigert sonst den Start; betrifft nur die OS-Sandbox, nicht `sandbox: true` des Renderers).
- **Nicht die Referenzhardware** (8 Kerne, Mittelklasse-Grafik). Die Zahlen taugen für den **Vergleich der Varianten**, nicht als Leistungsnachweis (der folgt in 1.5). Windows und macOS sind nicht gemessen.
- Datum: 2026-10-13. Zwei Wiederholungen je Lauf, 10 s je Lauf.

## Ergebnisse

Vollständige Tabelle: [`results/summary.md`](results/summary.md). Kernwerte (10.000 Personen, unbegrenzte Geschwindigkeit, Mittel aus 2 Läufen; Schrittzeit enthält Schreiben des Bildes):

| Arbeit | Variante | Schritt Mittel µs | p99 µs | Ticks/s | relativ zu napi |
|---|---|---|---|---|---|
| 8 | napi, 4 Threads | 401 | 1107 | 2482 | 1,00 |
| 8 | wasmN, 4 Threads | 602 | 1333 | 1553 | 1,50 × langsamer |
| 8 | napi, 1 Thread | 933 | 1834 | 1068 | – |
| 8 | wasm1, 1 Thread | 1184 | 1735 | 726 | 1,27 × langsamer als napi 1 Thread |
| 32 | napi, 4 Threads | 965 | 2010 | 1033 | 1,00 |
| 32 | wasmN, 4 Threads | 1325 | 2953 | 642 | 1,37 × langsamer |
| 32 | wasm1, 1 Thread | 3810 | 5310 | 155 | 3,9 × langsamer |
| 0 | wasmN, 4 Threads | 227 | 538 | 4368 | kein Gewinn gegenüber wasm1 (238 µs): Abstimmung kostet mehr als sie bringt |

**Darstellungsseite:** In allen 17 Konfigurationen 60,0 Bilder pro Sekunde, Bildabstand p99 16,67 ms, keine Bilder über 20 ms. Das Lesen aller 10.000 Datensätze im Renderer kostet 40–70 µs je Bild.

**Übergabe in Variante A:** Anfrage bis Ankunft des Bildes (160 KB) im Mittel 1,1 ms, p99 2–3,4 ms; Kopie im Utility-Prozess 0,14 ms. Bei 50.000 Personen (800 KB): 3,4 ms Mittel, 5,8 ms p99, Lesen im Renderer 0,23 ms. Das Bild ist dadurch im Mittel bis zu ein Bild älter als in Variante B.

**Festes Tempo 60 Ticks/s** (Arbeit 8): napi 60/s, wasmN 57/s, wasm1 52/s gemessen. Die Rechenlast erklärt die Abweichung bei wasm nicht (Schrittzeit unter 2 ms bei 16,7 ms Budget). **Vermutung, nicht geprüft:** Ungenauigkeit der Wartefunktion im Worker (`Atomics.wait` mit Zeitgrenze). Der Taktgeber wäre bei Wahl von B ohnehin neu zu bauen.

**Determinismus:** Nach 1000 Ticks mit 10.000 Personen sind die Prüfsummen von nativ (1, 2, 4 Threads), `wasm1` und `wasmN` **bitgleich** (`db519b3de6156cba`, `e705586487649ab8`, `faf3efd41ef36267` für Arbeit 0/8/32). Voraussetzung: nur IEEE-exakte Rechenoperationen (+ − × ÷ √), keine Winkelfunktionen.

## Befunde

1. **Leistung:** Nativ ist 1,27–1,50 × schneller als WebAssembly bei gleicher Threadzahl. Mehrere Kerne bringen in beiden Varianten den Hauptgewinn (napi 2,3–3,0 ×, wasm 1,9–2,9 × bei Arbeit 8–32).
2. **Übergabe:** Beide Wege halten 60 Bilder pro Sekunde. Variante B ist bei der Übergabe klar überlegen (keine Kopie, keine Nachricht). Variante A kostet je Bild eine Kopie über die Prozessgrenze; bei 10.000 Personen unkritisch, bei 50.000+ Objekten oder vollständigem Zustand je Bild (Fahrzeuge, Flugzeuge, Gepäck) zu beobachten → nur sichtbare oder geänderte Daten übertragen (Thema für 1.3).
3. **WebAssembly mit Threads braucht Nightly-Rust.** Stabiles Rust 1.98.1 meldet `atomics` als „unstable feature", und die vorkompilierte Standardbibliothek ist nicht threadfähig (u. a. Speicherverwaltung). Die offizielle wasm-bindgen-Anleitung nennt Nightly und `-Z build-std` (wasm-bindgen-Doku „Parallel Raytracing", abgerufen 2026-10-13). Das widerspricht der festen Rust-Version aus ADR-002 und „Reproduzierbarkeit vor Performance" (`CLAUDE.md` Abschnitt 6). Ohne Threads (`wasm1`, stabil) bleibt nur ein Kern: bei Arbeit 32 schon 3,9 × langsamer als napi mit 4 Threads.
4. **Abhängigkeitspflege napi-rs:** `napi-derive` 3.6.8 baut nicht mit der am 2026-10-05 erschienenen Unterabhängigkeit `napi-derive-backend` 6.1.5; Lösung: Lockfile auf 6.1.4 festgesetzt. napi-rs veröffentlicht mehrmals pro Woche; das Lockfile muss eingecheckt und bewusst aktualisiert werden.
5. **Build-Aufwand (nicht gemessen, nach Dokumentation):** A braucht je Plattform eine eigene `.node`-Datei (Windows x64/arm64, Linux x64/arm64, macOS x64 + arm64) → CI-Matrix mit nativen Runnern; für macOS Universal müssen die beiden Dateien per `lipo` zusammengeführt oder über `@electron/universal` (Optionen `x64ArchFiles`, `singleArchFiles`) eingebunden werden; Node-API ist ABI-stabil, also kein Neubau je Electron-Version. B braucht eine einzige `.wasm`-Datei für alle Plattformen.
6. **Fehlersuche und Robustheit:** A: Absturz des Kerns beendet nur den Utility-Prozess (Hauptprozess kann neu starten und den letzten Spielstand anbieten); native Werkzeuge (gdb/lldb, perf, Instruments) und volle Rust-Backtraces. B: Panic wird zum `unreachable`-Trap, Meldungen erst mit Zusatz-Crate; Profiling über Chrome-DevTools; Kern teilt sich Prozess und Speicher mit Oberfläche und Spielfeld.
7. **Plattform-Modul:** In A kann der Kern-Prozess Spielstände direkt schreiben; in B muss jeder Spielstand über die Preload-Schnittstelle zum Hauptprozess geschickt werden.
8. **Container-Hinweis:** Electron startet als root nur mit `--no-sandbox`; nur für die Messumgebung relevant.

## Empfehlung und Entscheidung

**Variante A (napi-rs im Electron-Utility-Prozess).** Am 2026-10-13 freigegeben, festgehalten in ADR-009 (`docs/decisions.md`). Variante B ist zurückgestellt; Neubewertung in Fahrplan-Schritt Q.2, sobald WebAssembly-Threads mit stabilem Rust baubar sind.
