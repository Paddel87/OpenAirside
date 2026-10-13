# Onboarding Runbook

<!-- Vollständige, getestete End-to-End-Anleitung vom Repo-Klon bis zum lauffähigen Spiel.
     README: Statusbild. Runbook: Bedienungs-Anleitung mit Plattform-Tiefe und Troubleshooting.
     Pflicht ab Klasse M; Klasse G mit Entwickler- und Reviewer-Variante.
     Inhalte stammen aus echter Durchführung, nicht aus dem Gedächtnis.
     Erstbefüllt in Fahrplan-Schritt 2.1 (ADR-018). Linux-Befehle in einer Ubuntu-24.04-Umgebung
     ausgeführt; macOS- und Windows-Befehle sind aus den CI-Läufen abgeleitet und vom
     Projektinhaber auf eigener Hardware zu bestätigen (Abschnitt 3, Schritt 6). -->

<!-- ANCHOR:zweck -->
## 1. Zweck und Geltungsbereich

Dieses Runbook führt Mitwirkende vom frischen Klon bis zum lokal laufenden Spiel inklusive aller Prüfungen. Es ersetzt nicht die README (Statusbild) und nicht `docs/architecture.md`.

**Adressat:** Entwickler:innen und Reviewer:innen, die zum ersten Mal lokal arbeiten; der Projektinhaber für eigene Tests auf Windows, Linux und macOS.

**Voraussetzung an den Leser:** Umgang mit Terminal und Git; Lesezugriff auf das Repository.

**Geprüft am:** 2026-10-13 unter Linux (Ubuntu 24.04.5, frischer Klon, Schritte 1–5); macOS und Windows noch nicht von Hand geprüft (CI-Rauchtest auf allen drei Plattformen, ADR-018).

<!-- ANCHOR:voraussetzungen -->
## 2. Voraussetzungen pro Plattform

Plattform-Matrix: `docs/project-context.md` Abschnitt 3. Pflichtwerkzeuge: Git, Node.js 26.11.1, Rust 1.98.1 über rustup (Version kommt aus `rust-toolchain.toml`, rustup installiert sie beim ersten `cargo`-Aufruf), Python 3 für das Hook-Framework `pre-commit`.

### Linux (Ubuntu 24.04+ / Fedora 42+)

```bash
# Node.js 26.11.1, z. B. über das offizielle Archiv (oder einen Versionsmanager wie fnm/nvm)
curl -fsSLO https://nodejs.org/dist/v26.11.1/node-v26.11.1-linux-x64.tar.xz
sudo mkdir -p /opt/node26 && sudo tar -xJf node-v26.11.1-linux-x64.tar.xz -C /opt/node26 --strip-components=1
export PATH=/opt/node26/bin:$PATH
# Rust über rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
# pre-commit
pipx install pre-commit   # alternativ: pip install --user pre-commit
# Nur für den Rauchtest ohne Desktop (Server, Container): virtueller Bildschirm
sudo apt install xvfb
```

Electron braucht die üblichen Desktop-Bibliotheken (GTK 3, NSS, ALSA); auf Desktop-Installationen sind sie vorhanden.

### macOS 13+ (Apple-Chips, ADR-012)

```bash
xcode-select --install           # Compiler und Linker
# Node.js 26.11.1 als macOS-Installer von nodejs.org (arm64) oder über fnm/nvm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
pipx install pre-commit          # pipx z. B. über Homebrew
```

Gebaut wird nur für Apple-Chips (`aarch64-apple-darwin`); kein Intel- und kein Universal-Build.

### Windows 11 (PowerShell oder Git Bash)

```powershell
# Visual Studio Build Tools mit „Desktopentwicklung mit C++" (MSVC-Linker für Rust)
winget install Microsoft.VisualStudio.2022.BuildTools
winget install OpenJS.NodeJS --version 26.11.1
winget install Rustlang.Rustup
pip install pre-commit
```

**Nicht unterstützte Plattformen:** 32-Bit-Windows und Linux armv7 (Electron 44 liefert keine Builds); macOS unter 13 (Electron 44 setzt macOS 13 voraus); Intel-Macs (ADR-012). Pakete für Windows und Linux auf arm64 folgen in 7.2.

<!-- ANCHOR:setup -->
## 3. Setup (End-to-End)

### Schritt 1: Repository klonen

```bash
git clone https://github.com/Paddel87/OpenAirside.git
cd OpenAirside
```

**Erwartetes Ergebnis:** Verzeichnis enthält `CLAUDE.md`, `Cargo.toml`, `package.json`, `core/`, `bridge/`, `app/`, `docs/`.

### Schritt 2: Abhängigkeiten installieren

```bash
npm ci
```

**Erwartetes Ergebnis:** `added … packages`; `npm audit --audit-level=high` ohne Befund. Das Electron-Programm selbst lädt Electron beim ersten Start nach (`node_modules/electron/dist`).

### Schritt 3: Hooks einrichten

```bash
pre-commit install
pre-commit run --all-files
```

**Erwartetes Ergebnis:** `pre-commit installed at .git/hooks/pre-commit`; alle Hooks `Passed` (rustfmt, clippy, Prettier, ESLint, tsc, Vitest, Markdown).

### Schritt 4: Bauen und starten

```bash
npm start
```

**Erwartetes Ergebnis:** Der erste Lauf baut den Rust-Kern und die Brücke (`app/native/openairside-bridge.node`, einige Minuten), dann den Hauptprozess (`app/dist/main`) und die Oberfläche (`app/dist/renderer`). Es öffnet sich ein leeres, dunkles Fenster „OpenAirside". Unter Linux als root oder in Containern ist `--no-sandbox` nötig (`npx electron . --no-sandbox`, Abschnitt 4).

### Schritt 5: Verifikation (alle Pflicht-Gates wie in der CI)

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo deny check                      # einmalig: cargo install --locked cargo-deny@0.20.2
npm run lint
npm run format:check
npm run typecheck
npm test                              # Vitest mit Coverage ≥ 70 %
npm run check:licenses
npm run test:e2e                      # Linux ohne Desktop: xvfb-run -a npm run test:e2e
```

**Erwartetes Ergebnis:** alles grün; der Rauchtest meldet `1 passed` (Fenster offen, Kern-Prozess meldet `ready` mit `openairside-core`). Coverage des Rust-Kerns optional lokal: `cargo install --locked cargo-llvm-cov@0.9.1`, dann `cargo llvm-cov --workspace` (Kern ≥ 85 %, gesamt ≥ 70 %).

### Schritt 6: Startweg eines heruntergeladenen Pakets (Projektinhaber, je Plattform)

Maßnahme 1 der Reaktiv-Reflexion (Fahrplan, „Iterations-Reflexion"); in der CI nicht nachstellbar.

1. Auf GitHub unter „Actions → Release → Run workflow" einen Lauf auf dem gewünschten Branch starten.
2. Das Paket der Plattform aus den Artefakten des Laufs herunterladen (`openairside-macOS`, `-Windows`, `-Linux`).
3. **macOS:** dmg öffnen, App nach „Programme" ziehen, starten. Erwartet: Gatekeeper-Warnung zu einem nicht geprüften Entwickler; „Dennoch öffnen" unter Systemeinstellungen → Datenschutz & Sicherheit startet die App ohne Terminal (ADR-015). Meldung „beschädigt" ist ein Fehler.
4. **Windows:** Installer starten. Erwartet: SmartScreen „Der Computer wurde durch Windows geschützt"; „Weitere Informationen → Trotzdem ausführen" installiert und startet.
5. **Linux:** AppImage ausführbar machen (`chmod +x`) und starten, oder `sudo apt install ./openairside_*.deb`.
6. Ergebnis mit Datum im Logbuch festhalten.

<!-- ANCHOR:troubleshooting -->
## 4. Troubleshooting

### `Running as root without --no-sandbox is not supported`

- **Symptom:** Electron beendet sich sofort (SIGTRAP); ein Rauchtest wartet vergeblich auf das Fenster.
- **Ursache:** Die Chromium-Sandbox startet unter Linux nicht als root und nicht ohne SUID-Hilfsprogramm (Container, CI-Runner).
- **Lösung:** `--no-sandbox` übergeben. Der Rauchtest tut das unter Linux automatisch (`e2e/smoke.spec.ts`). Eine funktionierende Linux-Sandbox in den Paketen ist Schritt 7.2.
- **Gefunden:** 2026-10-13, Schritt 2.1.

### Build bricht mit `napi_derive_backend::Diagnostic`

- **Ursache:** napi-derive 3.6.8 mit napi-derive-backend 6.1.5.
- **Lösung:** napi-derive 3.6.12 (ADR-018); `Cargo.lock` nicht von Hand ändern.

<!-- ANCHOR:plattform-hinweise -->
## 5. Plattform-spezifische Hinweise

### Linux

- WebGL2 hängt am Grafiktreiber; bei schwarzem Spielfeld zuerst den Treiber prüfen. Leistungsmessung mit echten Treibern: Schritt 7.2 (ADR-011).
- Ohne Desktop (Server, Container): Rauchtest mit `xvfb-run -a`.

### macOS

- Pakete sind ad hoc signiert, nicht von Apple beglaubigt (ADR-015, Entscheidung zur Signatur in 7.1); erster Start über „Dennoch öffnen" (Abschnitt 3, Schritt 6).

### Windows

- Hilfsskripte sind Node-Skripte; es werden keine Shell-Skripte vorausgesetzt.
- Der Installer ist unsigniert; SmartScreen fragt beim ersten Start nach.

<!-- ANCHOR:rollen -->
## 6. Rollen-spezifische Varianten

### Variante: Entwickler:in (Default)

Setup nach Abschnitt 3, Schritte 1–5. `npm start` baut den Kern im Release-Modus und startet Electron. Ein Entwicklungsmodus mit Hot-Reload der Oberfläche fehlt noch; er folgt mit der Oberfläche in 2.7.

### Variante: Reviewer:in

Setup nach Abschnitt 3, Schritte 1–3, danach nur die Prüfungen aus Schritt 5 ohne `test:e2e`. Die Anwendung muss nicht gestartet werden; den Rauchtest übernimmt die CI auf drei Plattformen.

<!-- ANCHOR:notfall -->
## 7. Notfall

Pflicht ab dem ersten öffentlichen Release (Schritt 7.1). Das Spiel läuft lokal beim Spieler; es gibt keinen Server.

- **Zugang:** GitHub-Konto des Projektinhabers (Ort der Zugangsdaten: beim Projektinhaber, nicht im Repo)
- **Fehlerhaftes Release zurückziehen:** TBD (Schritt 7.1) – erprobt am –
- **Sicherung:** Das Repository auf GitHub ist die Sicherung; zusätzliche lokale Kopie TBD (Schritt 7.1)
- **Wen benachrichtigen:** Spieler über die Release-Seite und Issues

<!-- ANCHOR:pflegehinweise -->
## 8. Pflegehinweise

- **Validierungs-Pflicht:** vor jedem Phasen-Abschluss gegen einen frischen Klon (`CLAUDE.md` Abschnitt 17).
- **Drift-Verbot:** Abweichung zwischen Runbook und tatsächlichem Setup ist ein Bug und wird im selben Commit behoben.
- **„Geprüft am"** wird bei jeder Validierung aktualisiert.
- **Plattform-Hinweise** bleiben konsistent mit `docs/project-context.md` Abschnitt 3.
