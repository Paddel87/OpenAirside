# Project Context

<!-- Projektspezifischer Kontext von OpenAirside. Wird zu Sessionbeginn als erste Datei gelesen.
     Stand: Initialisierung (Modus 2) vom 2026-10-13.
     Klassifikation: Klasse G (Groß), siehe ADR-001. Stack und Versionen: ADR-002. -->

<!-- ANCHOR:kerndaten -->
## 1. Kerndaten

- **Projektname:** OpenAirside
- **Kurzbeschreibung:** Open-Source-Nachbau des eingestellten Flughafen-Tycoon-Spiels Airport CEO für Windows, Linux und macOS. Spielprinzip und Funktionsumfang wie das Original inklusive Add-ons, technisch ausgelegt auf sehr große Flughäfen und an den heutigen Flughafenbetrieb angepasst.
- **Status:** Konzeption
- **Version (SemVer):** v0.0.0 (vor der ersten Umsetzung)
- **Dokumentationssprache:** Deutsch
- **Codesprache (Kommentare, Variablennamen):** Englisch
- **Projekttyp:** Desktop-Spiel (Wirtschaftssimulation/Tycoon), Einzelspieler; Mixed (Rust-Kern, TypeScript-Präsentation)

<!-- ANCHOR:zielgruppe-und-nutzungskontext -->
## 2. Zielgruppe und Nutzungskontext

- **Primäre Nutzer:** Spieler von Aufbau- und Wirtschaftssimulationen, insbesondere die Community von Airport CEO; Maßstab für das Spielerlebnis ist der Projektinhaber. Kein technisches Vorwissen nötig. Beteiligte und Anwendungsfälle: `docs/requirements.md`.
- **Sekundäre Nutzer / Betreiber:** keine Betreiber. Spieler installieren das Spiel lokal aus GitHub Releases. Später Modder, Übersetzer und Grafiker als Mitwirkende.
- **Erwartete Last:** Einzelspieler, lokal. Simulationslast: Referenzszenario mit mindestens 10.000 gleichzeitig simulierten Personen (Abschnitt 6, Performance).
- **Nutzungsumgebung:** Desktop unter Windows, Linux und macOS 13+ auf Apple-Chips (ADR-012). Hardware-Untergrenze: integrierte Grafik. Zielhardware: Mittelklasse-PC (Abschnitt 6).

<!-- ANCHOR:technischer-stack -->
## 3. Technischer Stack

Entschieden in **ADR-002** (2026-10-13) unter der Bedingung des Leistungsnachweises in Phase 1; der Nachweis ist erbracht (ADR-014), die Stack-Bestandteile sind in `architecture.md` `[BELASTBAR]` (Grenzen nach ADR-017). Versionen nach der Regel „ausgereifte Linie" (`CLAUDE.md` Abschnitt 15), von der KI gegen offizielle Quellen belegt und vom Menschen am 2026-10-13 zeilenweise bestätigt.

- **Mindestreife neuer Linien:** mindestens ein Fehlerkorrektur-Release (x.y.1) **und** erste Version der Linie mindestens 1 Monat alt. Ausnahme: Projekte, die nie Patch-Releases veröffentlichen (rayon), gelten ab 3 Monaten Alter als reif.
- **Geplante Projektdauer:** 2 Jahre (bis Ende 2028). Linien mit rollierendem Unterstützungsmodell (Electron, Rust) erreichen das nie; sie werden über wiederkehrende Update-Schritte im Fahrplan nachgezogen.

### Fixiert

#### Sprachen und Laufzeiten

- Rust 1.98.1, festgeschrieben in `rust-toolchain.toml` — Verifiziert: 2026-10-13, Quelle: github.com/rust-lang/rust RELEASES.md (1.99.0 ohne Patch)
- TypeScript 6.0.3 — Verifiziert: 2026-10-13, Quelle: npm; TypeScript 7 nicht kompatibel mit typescript-eslint (Peer `<6.1.0`, Issue typescript-eslint#12518)
- Node.js 26.11.1 für Werkzeuge (LTS ab 2026-10-28) — Verifiziert: 2026-10-13, Quelle: github.com/nodejs/Release schedule.json
- Electron 44.7.0 (bringt Node 24.21.0 und Chromium 152 mit) — Verifiziert: 2026-10-13, Quelle: releases.electronjs.org/schedule

#### Frameworks und Bibliotheken (Präsentation)

- React 19.2.8 — Verifiziert: 2026-10-13, Quelle: npm (19.3 ohne Patch)
- Tailwind CSS 4.3.3 — Verifiziert: 2026-10-13, Quelle: npm
- TanStack Table 9.2.8 — Verifiziert: 2026-10-13, Quelle: npm (9.3 einen Tag alt)
- PixiJS 8.20.1 (WebGL2; WebGPU nur optional) — Verifiziert: 2026-10-13, Quelle: npm (8.21/8.22 ohne Patch)
- Vite 8.3.4 — Verifiziert: 2026-10-13, Quelle: vite.dev/releases
- electron-builder 26.16.1 (explizit pinnen, npm-Tag `latest` ist veraltet) — Verifiziert: 2026-10-13, Quelle: GitHub-Releases electron-userland/electron-builder

#### Rust-Kern und Anbindung

- Betriebsart des Kerns: natives Node-Modul (napi-rs) in einem Electron-Utility-Prozess (ADR-009, `[BELASTBAR]` seit ADR-014)
  - napi 3.12.7, napi-derive 3.6.12, @napi-rs/cli 3.9.1 (Node-API 10 in Electron 44) — Verifiziert: 2026-10-13, Quelle: crates.io, npm; napi-derive 3.6.12 zieht napi-derive-backend 6.1.5 und baut ohne Festsetzung (ADR-018, Messung im frischen Workspace)
  - napi-build 2.4.4 (Begleit-Crate, von napi 3.12.7 mit `^2.4.4` verlangt) — Verifiziert: 2026-10-13, Quelle: crates.io
  - zurückgestellt, nicht im Stack: wasm-bindgen 0.2.128 für WebAssembly mit Threads, Neubewertung in Fahrplan-Schritt Q.2
- serde 1.0.229, rayon 1.12.0, ts-rs 12.0.1 — Verifiziert: 2026-10-13, Quelle: crates.io
- serde_json 1.0.151 in der Brücken-Crate über napi `serde-json` (ADR-024; bringt itoa 1.0.18 und zmij 1.0.23 mit, beide MIT) und seit ADR-025 im Kern für den Datenpaket-Lader — Verifiziert: 2026-10-14, Quelle: crates.io
- serde_path_to_error 0.1.20 und semver 1.0.28 im Kern für den Datenpaket-Lader (ADR-025) — Verifiziert: 2026-10-18, Quelle: crates.io
- schemars 1.2.2 nur hinter dem Kern-Feature `schema` für die JSON-Schema-Dateien der Datenpakete (ADR-025; bringt schemars_derive, serde_derive_internals, dyn-clone, ref-cast mit, alle MIT bzw. MIT OR Apache-2.0) — Verifiziert: 2026-10-18, Quelle: crates.io (1.2.3 bei der Prüfung 6 Tage alt)
- postcard 1.1.3 (ohne Standard-Features, `use-std`; bringt cobs 0.3.0 mit) und miniz_oxide 0.9.1 (bringt adler2 2.0.1 mit) im Kern für den Spielstand (ADR-027; alle MIT bzw. MIT OR Apache-2.0, Zlib, 0BSD) — Verifiziert: 2026-10-19, Quelle: crates.io

#### Werkzeuge im Build (nicht ausgeliefert)

- resvg 0.48.1 ohne Standard-Features im Werkzeug-Crate `tools/assets` für die Rasterung der SVG-Grafiken zu Atlanten und Programmsymbol (ADR-033; bringt usvg 0.48.1, tiny-skia 0.12.0, png 0.18.1 und 31 weitere Crates mit, alle MIT, Apache-2.0, BSD-2/3-Clause, Zlib oder 0BSD) — Verifiziert: 2026-10-19, Quelle: crates.io

#### Persistenz und Paketverwaltung

- Persistenz: lokale Spielstand-Dateien, keine Datenbank
- Package Manager: npm (mit Node 26.11.1) und Cargo (mit Rust 1.98.1)

### Empfohlen (freigabefrei nutzbar)

- Werkzeuge mit Versionen siehe Abschnitt 7
- shadcn/ui – Oberflächen-Bausteine auf Basis von React und Tailwind (Version bei Einführung nach Abschnitt 15 belegen)
- Standard-Crates für Fehlerbehandlung und Logging im Kern (z. B. `thiserror`, `tracing`), Version bei Einführung belegen

### Explizit nicht erlaubt

- Material aus Airport CEO oder dessen Community-Mods (Code, Grafiken, Texte, Töne, Spielstände)
- Abhängigkeiten von Vertriebsplattformen im Spielkern (z. B. Steamworks)
- Abhängigkeiten mit Copyleft-Lizenz, die das Gesamtprojekt unter ihre Lizenz zwingen würden (GPL, AGPL)
- Tauri als Desktop-Hülle (unzuverlässige WebGL-Unterstützung unter Linux, ADR-002)
- Spiellogik in TypeScript: Regeln und Zustandsänderungen der Simulation gehören ausschließlich in den Rust-Kern (ADR-003)
- TypeScript 7.x als Hauptpaket, solange typescript-eslint es nicht unterstützt (Nachprüfung im Ablaufdaten-Register)

### Unterstützte Entwickler-Plattformen

| Aspekt | Linux (Ubuntu 24.04+ / Fedora 42+) | macOS 13+ (Apple-Chips, ADR-012) | Windows 11 (PowerShell oder Git Bash) |
|---|---|---|---|
| **Rust-Kern** | ✓ | ✓ | ✓ |
| **Präsentation (Electron/TypeScript)** | ✓ | ✓ | ✓ |
| **Hilfsskripte (`scripts/`)** | ✓ | ✓ | ✓ nur als Node-Skripte, keine Shell-Skripte |
| **Verpackung** | ✓ (AppImage, deb) | ✓ (dmg, arm64; unsigniert) | ✓ (NSIS) |
| **CI-Pipeline** | ✓ (GitHub-Hosted) | ✓ (GitHub-Hosted) | ✓ (GitHub-Hosted) |

Nicht unterstützt: 32-Bit-Windows und Linux armv7 (Electron 44 liefert keine Builds), macOS unter 13, Intel-Macs (ADR-012). Hilfsskripte sind Node-Skripte, damit alle drei Plattformen ohne Zusatzwerkzeuge laufen.

**Pflege-Regel:** Tabelle bei jedem Touch an `scripts/`, `package.json`, `Cargo.toml` (Top-Level-Dependencies) oder bei neuen Plattform-Blockern re-validieren; Verstöße im selben Commit korrigieren.

<!-- ANCHOR:architektur-grobstruktur -->
## 4. Architektur-Grobstruktur

Das Spiel ist aufgebaut wie eine Web-Anwendung mit Backend und Frontend, die beide lokal laufen. Der Rust-Simulationskern enthält die gesamte Spiellogik, läuft in festen Simulationsschritten in eigenen Threads und kennt keine Darstellung. Die TypeScript-Präsentation (Oberfläche, Spielfeld, Ton) erhält den sichtbaren Zustand je Bild aus dem Kern-Prozess (Electron-Utility-Prozess, ADR-009) und schickt Befehle des Spielers an den Kern. Inhalte (Flugzeugtypen, Airlines, Gebäude, Fahrzeuge) sind Daten, keine Programmlogik. Details in `architecture.md`.

**Module (Kurzübersicht):**

- Simulationskern (Rust) – Takt, Weltzustand, Befehlsverarbeitung, Ereignisverteilung
- Welt und Bau (Rust) – Raster, Etagen, Zonen, Räume, Bauaufträge, Bauarbeiter
- Wegfindung und Passagierführung (Rust) – Wegweiser-Prinzip, Beschilderung, Bodenmarkierungen
- Verkehr (Rust) – Straßenverkehr, Rollverkehr, Vorrangregeln, Verkehrssteuerung
- Flugbetrieb (Rust) – Flugpläne, Airlines, Verträge, Bahnen, Tower
- Abfertigung und Gepäck (Rust) – Turnaround-Services, Gepäckanlage, Kontrollen
- Personen (Rust) – Passagiere und Personal: Bedürfnisse, Verhalten, Schichten
- Wirtschaft und Management (Rust) – Finanzen, Führungskräfte, Forschung, Bewertungen
- Ereignisse (Rust) – Notfälle, Wetter, Tiere, Störungen
- Brücke (Rust/TypeScript) – Zustandspuffer und Befehlskanal zwischen Kern und Präsentation
- Spielfeld (TypeScript) – Draufsicht-Darstellung mit PixiJS, Kamera, Overlays
- Oberfläche (TypeScript) – Panels, Werkzeuge, Benachrichtigungen
- Audio (TypeScript) – Atmosphäre, Effekte, Durchsagen, Musik
- Plattform (TypeScript, Electron-Hauptprozess) – Fenster, Dateizugriff, Spielstände, Logs
- Inhalte und Mods (Rust/TypeScript) – Laden und Prüfen von Datenpaketen
- Lokalisierung (TypeScript) – Sprachpakete
- Benchmark und Leistungsüberwachung (Rust/TypeScript)

**Kommunikationsmuster:** Präsentation → Kern über einen Befehlskanal (Nachrichten); Kern → Präsentation über einen Zustandspuffer im Kern-Prozess, der je Bild per `MessagePort` an den Renderer übertragen wird (ADR-009), plus einen Ereignisstrom für Benachrichtigungen. Präsentation ↔ Plattform über die abgesicherte Electron-Schnittstelle (Preload, S-6); Plattform ↔ Kern-Prozess über die Nachrichten des Utility-Prozesses (S-8, ADR-032). Details und Reifegrade in `architecture.md`.

<!-- ANCHOR:externe-abhaengigkeiten -->
## 5. Externe Abhängigkeiten

### Services

Keine im Spiel. Das Spiel läuft lokal ohne Online-Zwang und ohne Konto.

| Service | Zweck | Authentifizierung | Ausfallverhalten |
|---|---|---|---|
| GitHub (Repository, Actions, Releases) | Code, CI, Vertrieb | Repository-Rechte | Kein Einfluss auf installierte Spiele; Entwicklung lokal weiter möglich |

### APIs

Keine externen APIs im Spiel.

<!-- ANCHOR:constraints -->
## 6. Constraints (operationalisierbar)

### Datenschutz

- Das Spiel erhebt und überträgt keine personenbezogenen Daten. → Regel: kein Netzwerkzugriff im Spiel außer einer ausdrücklich vom Spieler ausgelösten Aktion; keine Telemetrie.
- **Sicherheitsgrundriss (Modus 2 Schritt 4a):** entfällt – lokales Einzelspieler-Programm ohne Server, Konten oder personenbezogene Daten; Schutzbedarf „normal" ohne eigenen ADR.

### Sicherheit

- Mods werden nur als Datenpakete geladen. → Regel: Ausführbarer Mod-Code ist bis zur Entscheidung über Code-Mods (Abschnitt 11) nicht vorgesehen.
- Spielstände und Mod-Dateien sind nicht vertrauenswürdige Eingaben. → Regel: Laden mit Schema-Validierung; Fehler führen zu einer verständlichen Meldung statt zum Absturz.
- Electron-Härtung. → Regel: `contextIsolation` an, `nodeIntegration` im Renderer aus, Renderer in der Sandbox; Zugriff auf Dateisystem nur über eine schmale, typisierte Preload-Schnittstelle; keine entfernten Inhalte im Fenster.
- `unsafe` im Rust-Kern → Regel: nur mit Begründungskommentar `// SAFETY:` und Fahrplan-Referenz; clippy-Regel `undocumented_unsafe_blocks` aktiv.

### Performance

- **Referenzszenario:** 3 Terminalbereiche in U-Form, 2 Start- und Landebahnen, ca. 17 Gates, mehrere tausend Passagiere gleichzeitig, mehrere Gepäckanlagen. → Regel: Referenz-Spielstand im Repository, automatisierter Leistungstest.
- **Skalierungsziel:** mindestens 10.000 gleichzeitig simulierte Personen auf der Zielhardware. → Regel: Leistungstest mit 10.000 Personen.
- **Zielwerte:** Im Referenzszenario auf der Zielhardware mindestens 60 Bilder pro Sekunde; die Simulation hält auch bei höchster Spielgeschwindigkeit ohne Rückstand mit. → Regel: Leistungstest misst Bildrate und Rückstand der Simulationsschritte.
- **Bauen ohne Wartezeit:** → Regel: Eine Bauaktion ist im nächsten dargestellten Bild sichtbar; aufwendige Neuberechnungen laufen im Hintergrund weiter.
- **Langzeitstabilität:** → Regel: Dauertest über lange simulierte Spielzeit; Speicher und Rechenzeit pro Simulationsschritt dürfen nicht wachsen.
- **Oberfläche:** → Regel: Panels werden höchstens wenige Male pro Sekunde aktualisiert, lange Listen virtualisiert; die Oberfläche darf die Bildrate des Spielfelds nicht drücken.

### Plattform und Kompatibilität

- Windows 10/11 x64 und arm64, Linux x64 und arm64, macOS 13+ nur für Apple-Chips (arm64); kein Intel- und kein Universal-Build (ADR-012).
- Grafik über WebGL2. Leistungsnachweis in Phase 1 auf Windows und macOS; Linux bis Schritt 7.2 mit der Zusicherung „baut und startet im CI", Leistungsmessung mit gängigen Treibern (Intel, AMD, NVIDIA) in 7.2 oder früher durch Mitwirkende (ADR-011).
- Hardware-Untergrenze: integrierte Grafik, lauffähig, mit Benchmark-Empfehlung zur Kartengröße.
- Zielhardware (Referenz): Mittelklasse-PC mit 8 CPU-Kernen, 16 GB RAM und einer Mittelklasse-Grafikkarte der aktuellen Generation.
- Für Tests unter macOS steht ein Mac des Projektinhabers zur Verfügung.

### Compliance und Lizenz

- **Projektlizenz:** MIT für den Code; eigene Grafiken unter CC0.
- **Audio:** freie Bibliotheken und KI-generierte Sounds erlaubt. → Regel: Audio-Dateien mit abweichenden Bedingungen liegen in einem eigenen Verzeichnis mit Lizenzhinweis-Datei.
- **Erlaubte Abhängigkeitslizenzen:** MIT, BSD (2- und 3-Klausel), Apache-2.0, ISC, zlib, MPL-2.0, Unicode-3.0, BlueOak-1.0.0, 0BSD, Python-2.0, WTFPL (letzte vier seit ADR-019, gelten für ausgelieferte Teile und Werkzeuge gleichermaßen). → Regel: `cargo-deny` (Rust) und Lizenzprüfung für npm in der CI.
- **Ausgeschlossene Lizenzen:** GPL, AGPL und proprietäre Lizenzen, die Mitwirkende an ein Konto oder eine Lizenzvereinbarung binden.

### Methodik-Schwellenwerte

- **Reaktiv-ADR-Schwellenwert:** maximal 20 % `[REAKTIV]`-Anteil über die letzten 10 ADRs (Klasse G).
- **Wucherungs-Schwelle:** Faktor 2 gegenüber dem ursprünglichen Schrittplan **und** mindestens 5 zusätzliche Schritte.
- **Vorläufig-zu-Belastbar-Verhältnis:** nach jeder UMSETZUNG-Phase soll mindestens ein `[VORLÄUFIG]`-Bestandteil der berührten Module auf `[BELASTBAR]` befördert sein, sonst Reflexion.
- **Modellklassen-Zuordnung** (`CLAUDE.md` Abschnitt 0):
  - **Mechanik-Klasse:** Claude Haiku 5.5 – Probelauf: offen
  - **Routine-Klasse:** Claude Sonnet 5.5 – Probelauf: offen
  - **Entscheidungs-Klasse:** Claude Opus 5.5 – stärkstes regulär eingesetztes Modell
  - **Ausnahme-Klasse:** Claude Fable 5.1, falls im Abo verfügbar – nur auf Vorschlag mit Freigabe
  - **Bezugsmodell und knappe Ressource:** Claude-Abo des Projektinhabers; knapp ist das Nutzungskontingent (rollierendes 5-Stunden-Fenster laut Laufzeitumgebung, Stand 2026-10-13)
  - **Abgabe an Unteragenten:** Claude Code erlaubt Unteragenten mit fest eingestelltem Modell (`model` im Agenten-Aufruf bzw. in `.claude/agents/*.md`); Abgabe erst nach bestandenem Probelauf
  - **Meldet die Laufzeitumgebung das Modell / das Kontingent?** Modell ja, über den Systemkontext und die Abfrage `get_session` (eingestelltes und bedientes Modell); Kontingent ja, über `get_session` (`rate_limit_info`: Art „five_hour“, Status, Zurücksetz-Zeitpunkt) – geprüft 2026-10-13
  - **Preise je Klasse:** offen, beim Probelauf mit Quelle nachtragen
  - **Grenze der Sessiongröße:** 400.000 Token Kontext
  - **Kontingent-Warnung:** inaktiv, Begründung: erst nach Prüfung in der ersten Claude-Code-Session (`templates/werkzeuge/claude-code/`)

<!-- ANCHOR:code-standards-und-qualitaetsziele -->
## 7. Code-Standards und Qualitätsziele

Pflichtkategorien nach `CLAUDE.md` Abschnitt 15. Werkzeugversionen nach derselben Versionsregel wie Abschnitt 3 (Verifiziert: 2026-10-13).

### Tool-Festlegung pro Sprache

#### Rust (Simulationskern)

- **Linter:** `cargo clippy --all-targets -- -D warnings`
- **Formatter:** `rustfmt` (Prüfung mit `cargo fmt --check`)
- **Type-Checker:** Rust-Compiler; `RUSTFLAGS="-D warnings"` in CI
- **Security-Scanner:** `cargo-deny` 0.20.2 (Advisories, Lizenzen, Herkunft der Crates); `unsafe` nach Abschnitt 6
- **Dependency-Audit:** `cargo deny check advisories`, zusätzlich Dependabot
- **Test-Runner:** `cargo test`; Coverage mit `cargo-llvm-cov` 0.9.1; Benchmarks unter `core/benches/`
- **Naming-Konvention:** Rust-Standard (snake_case für Funktionen und Variablen, UpperCamelCase für Typen)

#### TypeScript (Präsentation und Plattform)

- **Linter:** ESLint 10.9.1 mit typescript-eslint 8.70.1 (typbasierte Regeln), `--max-warnings 0`
- **Formatter:** Prettier 3.9.10
- **Type-Checker:** `tsc --noEmit` (TypeScript 6.0.3) mit `strict` und `noUncheckedIndexedAccess`
- **Security-Scanner:** CodeQL über GitHub Actions; Electron-Härtung aus Abschnitt 6 als Lint- und Testprüfung
- **Dependency-Audit:** `npm audit --audit-level=high`, zusätzlich Dependabot
- **Begleitpakete (ADR-018):** `@types/node` 24.19.2 (Node 24 in Electron 44), `@eslint/js` 10.0.1, `@vitest/coverage-v8` 5.0.4, `@playwright/test` 1.62.1 — Verifiziert: 2026-10-13, Quelle: npm
- **Lizenzprüfung npm:** eigenes Skript `scripts/check-licenses.mjs` gegen die erlaubten Lizenzen aus Abschnitt 6 (ADR-018)
- **Test-Runner:** Vitest 5.0.4 mit Coverage (v8); Playwright 1.62.1 für Ende-zu-Ende-Tests der Electron-Anwendung (Electron-Unterstützung von Playwright ist „experimentell", Version exakt pinnen)
- **Naming-Konvention:** camelCase für Variablen und Funktionen, PascalCase für Typen und React-Komponenten

#### Markdown (Dokumentation)

- **Linter:** markdownlint-cli2 über pre-commit (wie im Template-Repo)

### Warnungs-Bestand

Default: Warnungen sind Fehler (clippy `-D warnings`, ESLint `--max-warnings 0`, `tsc` strict). Abweichungen nur als benannte Ausnahme in der Tabelle.

| Quelle | Mittel des Werkzeugs | Obergrenze / benannte Ausnahme | Stand vom | Fahrplan-Schritt |
|---|---|---|---|---|
| cargo-deny `bans`, doppelte Crate-Versionen | `skip` in `deny.toml` | benannte Ausnahme: `syn@2.0.119` (nur über ts-rs-macros 12.0.1 hinter dem Feature `ts`; serde nutzt syn 3) | 2026-10-13 | Q.1 (prüfen, ob ts-rs auf syn 3 umgestellt hat) |
| cargo-deny `bans`, doppelte Crate-Versionen | `skip` in `deny.toml` | benannte Ausnahme: `miniz_oxide@0.8.9` (nur über png 0.18.1 im Werkzeug-Crate `tools/assets`, ADR-033; der Spielstand nutzt 0.9.1) | 2026-10-19 | Q.1 (prüfen, ob png auf miniz_oxide 0.9 umgestellt hat) |

**Warnungsquellen ohne Schalter:** Hinweise der CI-Plattform zu Action-Versionen und Runner-Images (Annotationen im Lauf); Bündelgrößen-Warnungen von Vite; Abkündigungshinweise von electron-builder und npm (`npm warn deprecated`, Ablaufdaten-Register, Q.1). Bekannter Bestand (Stand 2026-10-13, Release-Lauf 38440601610): npm „install scripts not yet covered by allowScripts" für electron-winstaller und fsevents (Skripte laufen nicht; Bau und Tests grün; Prüfung in Q.1); electron-builder „asar usage is disabled" (Muster aus den Spikes, Entscheidung mit der Signatur in 7.1); „default Electron icon is used" behoben in 2.10 (eigenes Symbol `build/icon.png`, lokaler Paketbau ohne den Hinweis am 2026-10-19). Neu seit 2026-10-19 (CI-Lauf 39188204304): `npm audit` meldet 8 Schwachstellen der Stufe „moderate“ (sprintf-js, GHSA-hp3w-g68c-fv3c, über electron-builder → app-builder-lib → @electron/get → global-agent → roarr); nur Bauwerkzeug, nicht im Programmpaket; unter der Gate-Schwelle „high“; die von npm angebotene Behebung wäre eine Rückstufung von electron-builder auf 26.5.0 (Prüfung in Q.1).

### Durchsetzungsmechanismen

- **Pre-Commit-Hook-Framework:** `pre-commit` (Python-Framework, sprachübergreifend für Rust, TypeScript und Markdown). Abweichend vom ursprünglichen Plan husky + lint-staged, weil husky seit 2024 keine Version mehr veröffentlicht hat und `pre-commit` Rust-Hooks ohne Node einbindet (ADR-002).
- **Konfigurationsdatei:** `.pre-commit-config.yaml`
- **CI-Plattform:** GitHub Actions
- **Workflow-Dateien (Klasse G):** `.github/workflows/ci.yml` (Prüfungen auf Linux, Windows, macOS), `security.yml` (CodeQL, Audits, wöchentlich), `release.yml` (Builds bei Tag `v*`)
- **Trigger:** `push` auf alle Branches, `pull_request` auf `main`
- **Verpflichtende CI-Gates (Merge-Block bei Rot):**
  - Lint (clippy, ESLint, markdownlint)
  - Format-Check (rustfmt, Prettier; kein Auto-Fix in CI)
  - Type-Check (Compiler, tsc)
  - Security-Scan (cargo-deny, CodeQL)
  - Dependency-Audit (Schwellenwert: high)
  - Tests inklusive Coverage-Mindestwert
  - Kern-Benchmark „Tick mit 10.000 Personen" (seit Schritt 2.2, ADR-022: rot bei Mittelwert > 1 ms oder p99 > 4 ms)
  - Prüfung der erzeugten TypeScript-Typen gegen die Rust-Typen (seit Schritt 2.2, ADR-022, im Job „Rust Lint & Format")
  - Prüfung der erzeugten JSON-Schema-Dateien der Datenpakete gegen die Rust-Typen (seit Schritt 2.4, ADR-025, im Job „Rust Lint & Format")
  - Wegweiser-Benchmark im Referenz-Terminal (seit Schritt 2.3, ADR-023, im Job „Kern-Benchmark": rot bei Neuberechnung > 4 s oder Schritt p99 > 4 ms)
  - Leistungstest Referenzszenario (ab Phase 4, sobald das Referenzszenario baubar ist)
- **Branch-Protection auf `main`:** alle Pflicht-Gates grün; Force-Push gesperrt (einzurichten in Schritt 2.1).
- **Seit Schritt 2.1** laufen alle Code-Jobs bei jedem Push (ADR-018): Rust (fmt, clippy, cargo-deny, Tests auf drei Plattformen, Coverage Kern ≥ 85 % und gesamt ≥ 70 %), TypeScript (ESLint, Prettier, tsc, npm audit, Lizenz-Skript, Vitest mit Coverage auf drei Plattformen) und der Electron-Rauchtest auf drei Plattformen; die Brücken-Crate hat keine cargo-Tests und wird über den Rauchtest geprüft.

### Coverage-Mindestwerte

- **Globaler Mindestwert:** 70 % Lines
- **Kritische Pfade:** Rust-Simulationskern (alle Kern-Module): 85 % Lines
- **Ausnahmen:** Spielfeld-Darstellung und Audio (schwer automatisiert testbar); dafür Playwright-Prüfungen und manuelle Prüfschritte im Fahrplan

### Commit-Lint

- **Tool:** nicht aktiv, Begründung: Commit-Format nach `CLAUDE.md` Abschnitt 11 wird über die DoD geprüft.
- **Erlaubte Typen:** Bereichs-Präfixe nach `CLAUDE.md` Abschnitt 11
- **Branch-Präfixe:** wie `CLAUDE.md` Abschnitt 11; zusätzlich: keine
- **Vom Werkzeug vergebene Branch-Namen:** Claude-Code-Cloud-Sessions können `claude/<zufallsname>` vergeben (so PR #1 bis #3, `claude/gracious-galileo-2reksz`); Typ und Schritt-ID stehen dann im PR-Titel

### Editor-Integration (empfohlen, nicht erzwungen)

- **EditorConfig:** `.editorconfig` im Repo-Root (ab Schritt 2.1)

<!-- ANCHOR:betrieb-und-deployment -->
## 8. Betrieb und Deployment

- **Deployment-Ziel:** GitHub Releases mit fertigen Builds für Windows, Linux und macOS (electron-builder)
- **CI/CD:** GitHub Actions; `release.yml` baut bei Tag `v*` alle drei Plattformen und legt einen Release-Entwurf an
- **Umgebungen:** lokal → Release-Kandidat (Vorabversion auf GitHub) → Release
- **Monitoring:** keines im Spiel; Leistungsdaten nur lokal (Benchmark, Leistungsanzeige)
- **Logging-Level Default:** `INFO` in Releases, `DEBUG` in Entwicklungs-Builds; Logdatei lokal beim Spieler
- **macOS-Signatur:** vorerst unsigniert; Entscheidung über Signatur (Kosten im Rahmen unten) vor Phase 7
- **Vertretung:** offen bis zum Deployment-Gate vor dem ersten öffentlichen Release (Phase 7)
- **Notfall-Handbuch:** `docs/onboarding-runbook.md` Abschnitt „Notfall"; noch nicht erprobt
- **KI im Betrieb:** Claude-Abo des Projektinhabers; Kontingent: rollierendes 5-Stunden-Fenster, Zurücksetz-Zeitpunkt je Session über `get_session` (geprüft 2026-10-13); Rückfallweg ohne KI: Entwicklung pausiert, das veröffentlichte Spiel ist unabhängig
- **Zugriff der KI auf die Produktion:** kein Zugriff auf Spielerrechner; im Repository Schreibrechte über Claude Code; Releases nur nach Freigabe
- **Unbeaufsichtigtes Handeln der KI:** nein

### Ablaufdaten-Register

| Was | Ablauf / Lebensende | Vorlauf | Quelle | Fahrplan-Schritt |
|---|---|---|---|---|
| Electron 44 | 2027-03-02 | 2 Monate | releases.electronjs.org/schedule | Wiederkehrender Schritt „Electron-Update" (Fahrplan, Querschnitt Q.1) |
| Node.js 26 (Werkzeuge) | 2029-04-30 | 6 Monate | nodejs/Release schedule.json | – |
| Node.js 24 (in Electron 44 enthalten) | 2028-04-30 | – | nodejs/Release schedule.json | folgt mit Electron-Update |
| Rust 1.98.1 | rollierend, 6-Wochen-Takt | bei jedem Electron-Update mitprüfen | RELEASES.md | Q.1 |
| TypeScript 7 / typescript-eslint | Nachprüfung 2027-01-13 (Unterstützung durch typescript-eslint?) | – | typescript-eslint#10940 | Q.1 |
| ESLint 10 | Lebensende „TBD" | Nachprüfung bei jedem Q.1 | eslint.org/version-support | Q.1 |
| Playwright Electron-Unterstützung | experimentell, keine Frist | – | playwright.dev/docs/api/class-electron | Q.1 |
| Übrige npm-Werkzeuge und Crates (ohne formale Richtlinie) | Nachprüfung 2027-04-13 | – | npm, crates.io | Q.1 |
| Runner-Image `ubuntu-latest` → Ubuntu 26 | ab 2026-10-19 (Umstellung durch GitHub, schrittweise); Lauf 39188204304 am 2026-10-19 02:04 UTC noch auf Ubuntu 24.04.5 (Image `ubuntu-24.04` 20261012) | – | CI-Annotation, actions/runner-images#14748 | Q.1 (ersten Lauf auf Ubuntu 26 beobachten) |
| Node 20 in GitHub Actions (abgekündigt) | behoben am 2026-10-13: pre-commit/action ersetzt | – | CI-Protokoll, GitHub-Changelog 2025-09-19 | – |
| WebAssembly-Threads mit stabilem Rust (`atomics`, threadfähige Standardbibliothek ohne `build-std`) | Nachprüfung 2027-01-02, danach bei jedem Q.1 | – | rust-lang/rust RELEASES.md, wasm-bindgen-Doku | Q.2 |
| Abgekündigte Unterpakete von electron-builder 26.16.1 (inflight 1.0.6, rimraf 2.6.3, glob 7.2.3, boolean 3.2.0; Hinweise bei `npm install`) | keine Frist, Nachprüfung bei jedem Q.1 | – | npm-Installationsprotokoll 2026-10-13 | Q.1 (electron-builder-Version prüfen, ob die Unterpakete ersetzt sind) |
| Automatische Toolchain-Installation durch `rustup show` (abgekündigt; Schritt `run: rustup show` in `ci.yml` und `release.yml`) | keine Frist angekündigt; Umstellung spätestens mit Q.1 (Frist 2027-01-02) | – | CI-Protokoll Lauf 39290398392 („auto-installation is deprecated“), rust-lang/rustup#4836 | Q.1 |
| Claude-Kontingent | rollierendes 5-Stunden-Fenster („five_hour“) und 7-Tage-Fenster („seven_day“), Zurücksetz-Zeitpunkt je Session über `get_session`; am 2026-10-13 17:46 UTC 7-Tage-Fenster mit Status `allowed_warning`, Zurücksetzen 2026-10-18 08:00 UTC | – | Laufzeitumgebung, geprüft 2026-10-13 | – (kein Fristablauf) |

### Kosten

- **Kostenrahmen:** bis 50 € im Monat für Betrieb, Werkzeuge und Dienste, ohne das bestehende KI-Abo (Antwort vom 2026-10-13). Kosten werden nur nach Freigabe eingegangen.
- **Kostenregister:** nicht geführt; Posten stehen in der Tabelle unten.

| Posten | Art (laufend / einmalig / KI-Verbrauch) | Betrag je Monat | Stand vom | Entscheidung nötig ab |
|---|---|---|---|---|
| GitHub (öffentliches Repo, Actions) | laufend | 0 € | 2026-10-13 | private Nutzung oder Actions-Limits |
| Claude-Abo | KI-Verbrauch | außerhalb des Rahmens | 2026-10-13 | – |

<!-- ANCHOR:entscheidungsbefugnisse -->
## 9. Entscheidungsbefugnisse

- **Freigabe-Entscheidungen trifft:** der Projektinhaber (GitHub: Paddel87)
- **Rollen:** Der Projektinhaber ist Auftraggeber, Visionär und Qualitätsprüfer; der Coding-Assistent (Claude Code) trägt die technische Umsetzung und Bewertung und legt freigabepflichtige Entscheidungen vor
- **Kommunikationskanal für Freigaben:** direkt im Chat bzw. in der Claude-Code-Session, bevorzugt mit anklickbaren Antwortoptionen (Spracheingabe)
- **Reaktionszeit-Erwartung:** asynchron, keine harte Antwortzeit

<!-- ANCHOR:repository-regeln -->
## 10. Repository-Regeln

- **Repository:** github.com/Paddel87/OpenAirside (öffentlich)
- **Hauptbranch:** `main`
- **Push-Regel:** Initialisierung direkt auf `main`; ab Schritt 2.1 Arbeit auf Feature-Branches, Merge nach grüner CI
- **Sessionende-Commits (Festlegung des Projektinhabers vom 2026-10-13):** Commits zum Sessionende, die nach dem Merge des Schritt-PR entstehen, bekommen keinen eigenen PR. Sie bleiben auf dem Arbeits-Branch und gehen mit dem PR des nächsten Schritts nach `main`. Die nächste Session übernimmt diese Commits auf den aktuellen `main`, statt den Branch ohne sie neu aufzusetzen.
- **Schutzregeln:** keine Force-Pushes auf `main`; Branches nach Merge löschen
- **Methodik-Vorlagen:** `templates/` stammt aus dem Template-Repo Dev-Templates und bleibt unverändert im Repo

<!-- ANCHOR:offene-grundsatzfragen -->
## 11. Offene Grundsatzfragen

- **Betriebsart des Rust-Kerns** – entschieden in ADR-009 (napi-rs im Utility-Prozess); WebAssembly mit Threads wird in Q.2 neu bewertet
- **Umfang der Mod-Unterstützung** (nur Datenpakete oder auch ausführbarer Code) – Entscheidung in Schritt 6.1 vor der Umsetzung des Mod-Systems

<!-- ANCHOR:glossar -->
## 12. Glossar (projektspezifische Begriffe)

- **Original:** Airport CEO von Apog Labs, Stand Version 1.1-3, einschließlich aller DLCs.
- **Funktionsgleichheit:** Alle Funktionen aus `funktionsinventar.md` sind gleichwertig abgebildet, in der modernisierten Form laut `modernisierung.md`.
- **Kern / Kandidat:** Kern = Teil der Funktionsgleichheit; Kandidat = unverbindliche Erweiterung nach der Funktionsgleichheit.
- **Referenzszenario:** der in Abschnitt 6 beschriebene große Flughafen als Maßstab für Leistung.
- **Zielhardware / Untergrenze:** Rechner, für den optimiert wird / Rechner, auf dem das Spiel mindestens lauffähig sein muss.
- **Simulationskern:** der Rust-Teil mit der gesamten Spiellogik.
- **Präsentation:** der TypeScript-Teil mit Oberfläche, Spielfeld und Ton.
- **Brücke:** Zustandspuffer und Befehlskanal zwischen Simulationskern und Präsentation.
- **Wegweiser-Prinzip:** Wege werden je Ziel einmal für die ganze Karte berechnet; Personen folgen den Wegweisern, statt einzeln zu suchen.
- **Passagierführung:** Schilder und Bodenmarkierungen, mit denen der Spieler die Wegweiser beeinflusst.
- **Rollweg-Knoten:** vom Spieler gezogene Mittellinien auf Rollwegen, entlang derer Flugzeuge rollen.
- **Verkehrssteuerung:** automatisches System in der Rolle von Tower und Rollkontrolle, das Rollrouten zuweist und Konflikte nach der Vorrang-Rangfolge löst.
- **Simulationsschritt (Tick):** fester Zeitschritt, in dem die Simulation ihren Zustand fortschreibt, unabhängig von der Bildrate.
- **Zelle:** kleinste Einheit des Weltrasters, 2 m × 2 m (ADR-020); Gebäude, Straßen, Vorfeld, Rollwege und Bahnen liegen im selben Raster.
- **Block:** 32 × 32 Zellen einer Etage; Speichereinheit für Raster und Wegweiser, existiert nur, wo gebaut bzw. begehbar ist (ADR-020).
- **Wegenetz:** aus den Bauten abgeleitetes Netz aus Fahrspuren, Rollweg-Knoten und Bahnen, auf dem Fahrzeuge und Flugzeuge fahren.
- **Spieltag:** 24 Spielstunden; dauert bei 1× 72 Minuten, bei 3× 24 Minuten (Tick = 1/3 Spielsekunde, ADR-020).
- **PAX:** Passagiere.

---

**Pflegehinweis:** Änderungen an Status, Stack oder Constraints sind freigabepflichtig (`CLAUDE.md` Abschnitt 4) und erzeugen einen ADR-Eintrag. Statuswechsel ziehen README-Badge- und CHANGELOG-Updates nach sich.
