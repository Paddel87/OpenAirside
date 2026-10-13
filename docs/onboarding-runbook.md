# Onboarding Runbook

<!-- Vollständige, getestete End-to-End-Anleitung vom Repo-Klon bis zum lauffähigen Spiel.
     README: Statusbild. Runbook: Bedienungs-Anleitung mit Plattform-Tiefe und Troubleshooting.
     Pflicht ab Klasse M; Klasse G mit Entwickler- und Reviewer-Variante.
     Inhalte stammen aus echter Durchführung, nicht aus dem Gedächtnis.
     Stand Initialisierung: Es gibt noch keinen Code. Nicht durchgeführte Schritte sind „TBD"
     und werden in Fahrplan-Schritt 2.1 erstbefüllt und in 4.2 gegen einen frischen Klon validiert. -->

<!-- ANCHOR:zweck -->
## 1. Zweck und Geltungsbereich

Dieses Runbook führt Mitwirkende vom frischen Klon bis zum lokal laufenden Spiel inklusive aller Prüfungen. Es ersetzt nicht die README (Statusbild) und nicht `docs/architecture.md`.

**Adressat:** Entwickler:innen und Reviewer:innen, die zum ersten Mal lokal arbeiten; der Projektinhaber für eigene Tests auf Windows, Linux und macOS.

**Voraussetzung an den Leser:** Umgang mit Terminal und Git; Lesezugriff auf das Repository.

**Geprüft am:** noch nicht geprüft (vor Schritt 2.1 gibt es nichts zu starten)

<!-- ANCHOR:voraussetzungen -->
## 2. Voraussetzungen pro Plattform

Plattform-Matrix: `docs/project-context.md` Abschnitt 3. Pflichtwerkzeuge: Git, Node.js 26 (LTS), Rust 1.98.1 über rustup (Version kommt aus `rust-toolchain.toml`), Python 3 für das Hook-Framework `pre-commit`.

### Linux (Ubuntu 24.04+ / Fedora 42+)

```bash
# TBD (Schritt 2.1): Installationsbefehle für Node 26, rustup, pre-commit,
# Systempakete für Electron (z. B. libnss3, libgtk-3) praktisch ermitteln
```

### macOS 13+ (Apple-Chips, ADR-012)

```bash
# TBD (Schritt 2.1): Xcode Command Line Tools, Node 26, rustup
# (Ziel aarch64-apple-darwin; kein Intel- und kein Universal-Build)
```

### Windows 11 (PowerShell oder Git Bash)

```bash
# TBD (Schritt 2.1): Node 26, rustup mit MSVC-Toolchain
# (Visual Studio Build Tools mit „Desktopentwicklung mit C++"), pre-commit
```

**Nicht unterstützte Plattformen:** 32-Bit-Windows und Linux armv7 (Electron 44 liefert keine Builds); macOS unter 13 (Electron 44 setzt macOS 13 voraus); Intel-Macs (ADR-012).

<!-- ANCHOR:setup -->
## 3. Setup (End-to-End)

### Schritt 1: Repository klonen

```bash
git clone https://github.com/Paddel87/OpenAirside.git
cd OpenAirside
```

**Erwartetes Ergebnis:** Verzeichnis enthält `CLAUDE.md`, `docs/`, `templates/`, `README.md`.

### Schritt 2: Hooks einrichten

```bash
pre-commit install
```

**Erwartetes Ergebnis:** `pre-commit installed at .git/hooks/pre-commit`. Erster Lauf mit `pre-commit run --all-files` prüft bis Schritt 2.1 nur Markdown und allgemeine Dateiregeln.

### Schritt 3: Abhängigkeiten installieren

```bash
# TBD (Schritt 2.1): npm ci; cargo build
```

### Schritt 4: Spiel im Entwicklungsmodus starten

```bash
# TBD (Schritt 2.1): z. B. npm run dev
```

### Schritt 5: Verifikation

```bash
# TBD (Schritt 2.1): npm run check (Lint, Format, Typen, Tests); cargo test
```

<!-- ANCHOR:troubleshooting -->
## 4. Troubleshooting

Noch keine Einträge. Wächst ab Schritt 2.1 mit jeder Mehrfach-Reibung.

<!-- ANCHOR:plattform-hinweise -->
## 5. Plattform-spezifische Hinweise

### Linux

- WebGL2 hängt am Grafiktreiber; bei schwarzem Spielfeld zuerst den Treiber prüfen (TBD, Schritt 1.5).

### macOS

- Builds sind unsigniert; der erste Start erfordert eine Freigabe in den Systemeinstellungen (genauer Ablauf TBD, Schritt 1.6).

### Windows

- Hilfsskripte sind Node-Skripte; es werden keine Shell-Skripte vorausgesetzt.

<!-- ANCHOR:rollen -->
## 6. Rollen-spezifische Varianten

### Variante: Entwickler:in (Default)

Setup nach Abschnitt 3 mit Entwicklungsmodus (Hot-Reload der Oberfläche, Debug-Build des Rust-Kerns). Details TBD in Schritt 2.1.

### Variante: Reviewer:in

Setup nach Abschnitt 3 Schritte 1–3, danach nur die Prüfungen aus Schritt 5; kein Start der Anwendung nötig. Details TBD in Schritt 2.1.

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
