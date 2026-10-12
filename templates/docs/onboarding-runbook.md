# Onboarding Runbook

<!-- Vollständige, getestete End-to-End-Anleitung vom Repo-Klon bis zum lauffähigen System.
     Ergänzt die README:
       - README: Statusbild – knapp, status-orientiert, ohne Plattform-Tiefe.
       - Runbook: Bedienungs-Anleitung – ausführlich, plattform-differenziert,
         mit Troubleshooting und ggf. Rollen-Aufteilung.

     Pflicht ab Klasse M. Optional Klasse K. In Klasse G/V nach Rolle aufteilbar.
     Pflege-Trigger: CLAUDE.md Abschnitt 17 und Abschnitt 3.

     Inhalte stammen aus echter Klon-/Worktree-frischer Durchführung,
     nicht aus dem Gedächtnis. Drift zwischen Runbook und tatsächlichem
     Setup-Pfad ist ein Bug. -->

## 1. Zweck und Geltungsbereich

[1–2 Sätze, was dieses Runbook leistet und was es nicht leistet.
Beispiel: „Dieses Runbook führt einen neuen Anwender oder Reviewer
vom frischen Klon bis zum hochgefahrenen lokalen Stack inklusive
Smoke-Test. Es ersetzt nicht die README (Statusbild) und nicht die
Architektur-Dokumentation."]

**Adressat:** [z. B. „Entwickler:innen, die zum ersten Mal lokal arbeiten" / „Reviewer:innen für Code-Audit-Aufgaben" / „Operations für Produktiv-Deploys"]

**Voraussetzung an den Leser:** [z. B. „Grundkenntnisse in Bash und Docker; Lese-Zugriff auf das Repository"]

**Geprüft am:** [YYYY-MM-DD, auf welcher Plattform validiert]

## 2. Voraussetzungen pro Plattform

[Verweis auf `docs/project-context.md` Abschnitt 3 „Unterstützte Entwickler-Plattformen" und Pflicht-Voraussetzungen aus dem README.
Hier nur die konkreten Installations-Befehle pro Plattform.]

### Linux (Ubuntu 22.04+ / Debian 12+ / Fedora 40+)

```bash
# z. B. Paketliste, Versionen, Installations-Befehle
[konkrete Befehle]
```

### macOS 14+ (Apple Silicon und Intel)

```bash
# z. B. Homebrew-Befehle, Sonderfälle Apple Silicon
[konkrete Befehle]
```

### Windows 11 mit Git Bash

```bash
# z. B. zusätzliche Tools (jq, openssl), die nicht im Standard-Git-Bash sind
[konkrete Befehle]
```

### Windows 11 mit WSL2

```bash
# Identisch zu Linux, plus WSL2-spezifische Hinweise (Pfad-Mapping, Docker-Backend)
[konkrete Befehle]
```

**Nicht unterstützte Plattformen:** [explizit benennen, mit Begründung. Stille Nicht-Unterstützung ist unzulässig.]

## 3. Setup (End-to-End)

[Sequenz der Befehle, exakt so, wie sie ein neuer Anwender ausführt.
Jeder Schritt mit erwarteter Ausgabe.
Bei Bruch: konkret beschreiben, wie der Bruch sich zeigt und welche Aktion ihn löst.]

### Schritt 1: Repository klonen

```bash
git clone [URL] [zielverzeichnis]
cd [zielverzeichnis]
```

**Erwartetes Ergebnis:** Verzeichnis enthält [...]. Falls nicht: [Fehleranalyse-Hinweis].

### Schritt 2: Tooling installieren

```bash
[konkreter Befehl, z. B. uv sync && pnpm install]
```

**Erwartetes Ergebnis:** Beide Lock-Files unverändert; Abhängigkeiten in der erwarteten Version installiert. **Bei Versions-Konflikten:** [Hinweis].

### Schritt 3: Konfiguration

```bash
cp .env.example .env
# Folgende Werte MÜSSEN ersetzt werden, sonst startet das Backend nicht:
#   SECRET_KEY=...     → mit `openssl rand -hex 32` generieren
#   [WEITERE_VARIABLE]=... → siehe [Quelle / Person]
```

**Fallstrick:** Vergessene Ersetzung des `SECRET_KEY` führt zu [konkreter Fehlermeldung]. Lösung: Wert ersetzen, Backend neu starten.

### Schritt 4: Stack hochfahren

```bash
docker compose up -d
# Wartezeit für Initialisierung: ca. [N] Sekunden
```

**Erwartetes Ergebnis:** Alle Container im Status `running` und `healthy`. Prüfen mit `docker compose ps`.

### Schritt 5: Verifikation

```bash
# Smoke-Test gegen die laufende Instanz
[konkreter Befehl, z. B. ./scripts/dev-smoke.sh oder curl http://localhost:8000/api/health]
```

**Erwartetes Ergebnis:** [konkrete Erfolgs-Antwort, z. B. `{"status":"ok"}`].

## 4. Troubleshooting

[Häufig auftretende Brüche und ihre Behebung. Wächst mit dem Projekt mit.
Bei jeder neuen Mehrfach-Reibung: hier ergänzen.]

### Symptom: [konkrete Fehlermeldung oder Verhalten]

- **Wahrscheinliche Ursache:** [...]
- **Behebung:** [konkrete Schritte]
- **Vorbeugung:** [optional: was zukünftig vermieden werden sollte]

### Symptom: [...]

[...]

## 5. Plattform-spezifische Hinweise

[Brüche oder Sonderfälle, die nur auf bestimmten Plattformen auftreten.
Verweisen auf die Plattform-Matrix in `docs/project-context.md` Abschnitt 3.]

### Linux

- [z. B. „SELinux-Kontext für Compose-Volumes setzen"]

### macOS

- [z. B. „Docker Desktop muss FileSharing für das Repo-Verzeichnis aktivieren"]

### Windows (Git Bash)

- [z. B. „Pfade mit Leerzeichen müssen quotiert werden"]

### Windows (WSL2)

- [z. B. „Compose-Volumes im WSL2-Filesystem, nicht im Windows-Filesystem, sonst signifikanter I/O-Overhead"]

## 6. Rollen-spezifische Varianten

[Nur Pflicht in Klasse G/V; in Klasse M optional.]

### Variante: Entwickler:in (Default)

[Standardpfad – Setup für interaktive Entwicklung. Inkludiert Hot-Reload, Debug-Modus, lokale Test-Suite.]

### Variante: Reviewer:in

[Setup für Code-/Security-Review – kein Hot-Reload nötig, dafür alle Lint-/Type-/Test-Gates lokal lauffähig. Optional: read-only Klon-Strategie.]

### Variante: Operations

[Setup für Produktiv-nahe Verifikation – Compose-Profile für Production, ohne Dev-Tooling, mit echten Secrets aus Vault statt `.env`.]

## 7. Notfall

[Pflicht ab dem ersten öffentlichen Deployment (CLAUDE.md Abschnitt 12, Prüfpunkt 7). Geschrieben für einen Menschen **ohne KI** und ohne Vorwissen über das Projekt. Jeder Ablauf ist mindestens einmal praktisch durchgespielt; Datum am Ablauf.]

- **Zugang:** [wo liegen die Zugangsdaten für Server, Domain, Backups – Ort, nicht Wert]
- **System anhalten:** [konkrete Befehle oder Klickpfad] – erprobt am [YYYY-MM-DD]
- **Sicherung ziehen:** [konkrete Befehle] – erprobt am [YYYY-MM-DD]
- **Wiederherstellen:** [konkrete Befehle, erwartete Dauer, woran man den Erfolg erkennt] – erprobt am [YYYY-MM-DD]
- **Wen benachrichtigen:** [Nutzer, Vertretung, ggf. Datenschutz-Meldepflicht mit Frist]

## 8. Pflegehinweise

- **Validierungs-Pflicht:** Vor jedem Phasen-Abschluss wird dieses Runbook gegen einen frischen Worktree validiert (CLAUDE.md Abschnitt 17, Trigger 3 in Abschnitt 16).
- **Drift-Verbot:** Wenn das Runbook nicht mehr zur Realität passt (Skript umbenannt, ENV-Variable neu, Plattform-Sonderfall geändert), ist das ein Bug und wird im selben Commit gefixt.
- **„Geprüft am"-Datum** (Abschnitt 1) wird bei jeder Validierung aktualisiert.
- **Troubleshooting-Sektion wächst mit:** Bei jeder Mehrfach-Reibung wird ein Eintrag ergänzt. Einträge, die seit 12 Monaten nicht mehr aufgetreten sind, dürfen ins Archiv (`docs/archiv/onboarding-troubleshooting-YYYY.md`).
- **Plattform-spezifische Hinweise** müssen mit der Plattform-Matrix in `docs/project-context.md` Abschnitt 3 konsistent bleiben (siehe Inter-Pflicht-Drift-Prüfung in CLAUDE.md Abschnitt 16).

---

**Initialisierungshinweis (erste Session nach Projektanlage):**

- **Klasse K (Klein):** Runbook optional. Wenn vorhanden, reduzierte Form – Abschnitte 5 (Plattform-spezifisch) und 6 (Rollen-Varianten) entfallen meist. Ohne Runbook steht der Notfall-Abschnitt (7) in `docs/notfall-handbuch.md`.
- **Klasse M (Mittel):** Runbook Pflicht. Volle Form, Abschnitt 6 (Rollen-Varianten) optional.
- **Klasse G (Groß):** Runbook Pflicht. Volle Form, Abschnitt 6 mit mindestens Dev- und Reviewer-Variante.
- **Klasse V (Verteilt-Groß):** Runbook Pflicht, ggf. in mehrere Dateien aufgeteilt:
  - `docs/onboarding-runbook-dev.md`
  - `docs/onboarding-runbook-reviewer.md`
  - `docs/onboarding-runbook-operations.md`
  - `docs/onboarding-runbook.md` als Index mit Verweisen.
- **Erstbefüllung erfolgt NICHT aus dem Gedächtnis**, sondern aus einer echten Klon-/Worktree-frischen Durchführung. Schritte, die nicht praktisch durchgeführt wurden, werden mit `TBD` markiert und im Fahrplan als Stabilisierungs-Schritt geführt.
- **Erster realer Eintrag in „Geprüft am"** erfolgt mit der ersten erfolgreichen End-to-End-Durchführung nach der Erstbefüllung.
