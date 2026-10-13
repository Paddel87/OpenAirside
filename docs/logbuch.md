# Logbuch

<!-- Chronologischer Flugschreiber von OpenAirside. Ereignisbasierte Einträge, neueste oben.
     Zweck:
       1. Nahtlose Fortsetzung in neuer Session: was war zuletzt los, womit ging es zu Ende?
       2. Wiederfindbarkeit kleiner Lösungen
       3. Selbst-Beobachtung des Projekts

     Abgrenzung zu anderen Dokumenten:
       - fahrplan.md: Was tun wir? (Plan)
       - decisions.md: Warum so? (Begründung)
       - architecture.md: Wie ist es gebaut? (Zustand)
       - blockers.md: Was hindert uns aktuell? (offene Probleme)
       - CHANGELOG.md: Was hat sich für Nutzer geändert? (extern, versionsorientiert)
       - logbuch.md: Was ist während der Arbeit passiert? (intern, chronologisch) -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

Phase 1 abgeschlossen (ADR-016), Detail-Einträge im Archiv `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) läuft: 2.1 erledigt (ADR-018, ADR-019) mit Nachlauf; 2.8 erledigt (ADR-020); nächster Schritt 2.9 in neuer Session. Reflexion Reaktiv-Quote (Anlass ADR-020) zur Kenntnis genommen.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-13 19:20 (UTC) – [BEOBACHTUNG]

- **Kenntnisnahme:** Projektinhaber hat die Reflexion zur Reaktiv-Quote (Anlass ADR-020, 6 / 10) zur Kenntnis genommen; kein Umbau, Maßnahme (5) gilt. Damit keine offene STOPP-Situation.

### 2026-10-13 19:14 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 14 Minuten seit `[SESSIONSTART]` 19:00 UTC
- **Bearbeitet:** 2.8 – Weltmaßstab, Luftseite und Zeitmodell
- **Erreicht:** 2.8 `[ERLEDIGT]`. Messung `spikes/2.8-weltmassstab/` (Referenzterminal aus 1.5 in Welten 1536²–3072² eingebettet, dicht gegen 32 × 32-Blöcke, Simulationsschritt, Blocktabelle). ADR-020 freigegeben (A: 2 m je Zelle, Luftseite im Raster in echten Maßen, Karten 1536²/2048², Blöcke, Wegenetze; Spieltag 72 min bei 1×; Vorfeld nur auf Gehwegen). ADR-013 überholt. `architecture.md` 3/6/7/9, Fahrplan (2.2, 2.3, 2.9, 2.10, 3.4, 5.8, Abschlusskriterium Phase 2), Glossar in `project-context.md`, README nachgezogen.
- **Offen geblieben:** Kenntnisnahme der Reflexion „Anlass ADR-020" (Reaktiv-Quote 6 / 10) durch den Projektinhaber. Neuberechnung in Blöcken nur ersatzweise gemessen – Bestätigung durch Benchmark in 2.3. Kein PR geöffnet (nicht angefragt).
- **Nächster Schritt:** 2.9 (Schnittstellenverträge S-2 bis S-7) in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 19:13 UTC); 2.8 empfiehlt Entscheidung, 0 Schritte oberhalb der Empfehlung; keine Teilarbeit abgegeben (Probelauf offen).
- **Kontextgröße:** `get_session` meldet 102.635 Token, derselbe Wert wie zu Sessionbeginn – vermutlich nicht nachgeführt; unter der Grenze von 400.000.
- **Kontingent:** 5-Stunden-Fenster `allowed` (19:13 UTC).
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf; Runner-Umstellung auf Ubuntu 26 ab 2026-10-19 weiter in Q.1 zu beobachten.
- **Drift-Prüfung:** README synchron (Phase, Architektur-Reife, Nächste Schritte, Blocker 0); ADR-020 verweist auf vorhandene Schritte (2.2, 2.3, 2.9, 2.10, 3.4, 5.8); Reifegrade passend zu ADR-020; Reaktiv-Quote 6 / 10 stimmt mit den Tags von ADR-011 bis ADR-020; keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten; Größen: `project-context.md` 360 Zeilen, Logbuch unter 800.

### 2026-10-13 19:12 (UTC) – [ADR-ANGELEGT]

- **ADR-020** `[REAKTIV]` (Kategorie 4): Weltmaßstab 2 m je Zelle, Luftseite im Raster, Wegweiser in Blöcken, Zeitmodell. Freigabe des Projektinhabers per Auswahl: „A: 2 m, echte Maße", „72 Minuten", „Nur markierte Gehwege".
- **Reaktiv** allein wegen der 14-Tage-Regel (ändert ADR-013 vom selben Tag); Reflexion mit Maßnahme (5) in `fahrplan.md` ergänzt.
- **Reifegrad:** keine Beförderung; Datenmodell und Wegweiser-Prinzip bleiben `[BELASTBAR]`, offene Frage „Lage von Bahnen und Vorfeld" geschlossen.

### 2026-10-13 19:10 (UTC) – [BEOBACHTUNG]

- **Messmethode 2.8:** Die Rechenzeit einer Block-Speicherung wurde nicht mit einer eigenen Block-Implementierung gemessen, sondern auf dem Ausschnitt mit denselben begehbaren Zellen; Speicher aus der exakten Zahl belegter Blöcke. Mehraufwand der Block-Adressierung getrennt per Mikromessung (+3 ns je Zugriff). Der Benchmark in 2.3 bestätigt das am Produktionscode.
- **Mini-Reibung:** `/usr/bin/time` fehlt im Container; Prozess-Spitze stattdessen aus `VmHWM` im Programm. Zeitstempel des Eintrags `[ENTSCHEIDUNG-VORGELEGT]` zunächst mit 19:40 statt 19:08 geschrieben, vor dem Commit korrigiert – Uhrzeit immer mit `date -u` holen.
- **Original:** Spieltag-Länge nicht offiziell belegt; Forum-Angaben 1–4 s je Spielminute widersprechen sich (Versionen).

### 2026-10-13 19:08 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.8, Kategorie 4 (Datenmodell), ändert ADR-013.
- **Grundlage:** Messung `spikes/2.8-weltmassstab/` (Container, 4 vCPU): Luftseite im selben Raster mit dichter Speicherung 720 MB / 2,3 s (2048²) bis 1,6 GB / 5,2 s (3072²) für dasselbe Terminal; in 32×32-Blöcken über begehbaren Flächen 25,5 MB / 0,4 s unabhängig von der Weltgröße, Terminal 768² mit 64 Wegweisern 98 MB / 1,6 s. Schritt 0,23–0,28 ms, Blocktabelle +3 ns je Zugriff.
- **Vorgelegt:** Maßstab (2 m je Zelle, Luftseite in realen Abmessungen, Karten 1536² / 2048², Block-Speicherung, Graphen für Straßen, Rollwege, Bahnen), Zeitmodell (60 Ticks/s bei 1×, gleiche Tick-Länge bis 3×, Spieltag bei 1×), Begehbarkeit des Vorfelds.
- **Hinweis:** Der Folge-ADR ändert ADR-013 innerhalb von 14 Tagen und ist damit nach `CLAUDE.md` Abschnitt 6 `[REAKTIV]`; Reaktiv-Quote stiege auf 6 / 10.

### 2026-10-13 19:00 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 17:47 UTC plus `[BEOBACHTUNG]` 18:46 UTC (Nachlauf 2.1 erledigt); PR Paddel87/OpenAirside#14 gemergt (`0049866`), Branch `claude/gracious-galileo-2reksz` neu auf `main` gesetzt.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant:** 2.8 – Weltmaßstab, Luftseite und Zeitmodell (Entwurfsschritt, Vergleichsstudie mit Messung, Zeitbox eine Session).
- **Vorabprüfung:** Eingangskriterium Datenmodell `[BELASTBAR]` (ADR-010, ADR-014) erfüllt; Ergebnis ist freigabepflichtig (Kategorie 4).
- **Modus/Werkzeug:** Claude Code, Cloud-Session, semi-autonom.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: `get_session` 19:00 UTC) – Entscheidungs-Klasse; entspricht der Empfehlung für 2.8.

### 2026-10-13 18:46 (UTC) – [BEOBACHTUNG]

- **Nachlauf 2.1 abgeschlossen** (Rückmeldungen des Projektinhabers nach Sessionende, Branch neu vom `main` nach Merge von PR #13 aufgesetzt).
- **2.1-N1 Branch-Schutz:** Ruleset `main-schutz` laut Projektinhaber eingerichtet (PR-Pflicht ohne Freigabe-Zahl, 18 Pflicht-Checks, Force-Push und Löschen gesperrt). Von der KI nicht direkt prüfbar (kein Zugriff auf die Ruleset-Einstellungen); Nachweis beim nächsten PR.
- **2.1-N2 Startweg (Release-Lauf 38440601610):** Windows 11 Pro 25H2 (Build 26200.9457): SmartScreen-Warnung, Zulassen, Installer mit Benutzer- und Ordnerwahl, Fenster startet. macOS (Version nicht erfasst): DMG ohne Browser-Warnung, Gatekeeper „In den Papierkorb legen / Fertig", „Dennoch öffnen" in Datenschutz & Sicherheit mit Touch ID, Fenster startet. Ad-hoc-Signatur nach ADR-015 bestätigt.
- **Befunde mit Landeplatz:** Electron-Standardmenü (File/Edit/View/Window) mit „Reload" und „Toggle Developer Tools" im Release-Paket → 2.7 (Zu tun ergänzt); Standard-Symbol von Electron → 2.10 (Zu tun ergänzt).
- **Runbook** Abschnitt 3, Schritt 6 an die beobachteten Dialoge angepasst.

### 2026-10-13 17:47 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 67 Minuten seit `[SESSIONSTART]` 16:40 UTC
- **Bearbeitet:** 2.1 – Repository-Struktur, CI-Gates und Werkzeuge
- **Erreicht:** 2.1 `[ERLEDIGT]`. ADR-018 (Gerüst, CI, napi-derive 3.6.12) und ADR-019 (Lizenzliste erweitert, `[REAKTIV]`, Reflexion zur Kenntnis genommen). Rust-Workspace `core/` + `bridge/`, Electron-Gerüst mit Kern im Utility-Prozess, Lizenz-Skript, CI mit Rauchtest auf drei Plattformen, `release.yml` manuell startbar mit Ad-hoc-Signatur und Rauchtest der gepackten Programme. CI 38440436405 und 38440669665 grün, Release 38440601610 grün (macOS `Signature=adhoc`, natives Modul gültig signiert). Runbook erstbefüllt und unter Linux am frischen Worktree validiert.
- **Offen geblieben:** 2.1-N1 Branch-Schutz auf `main` und 2.1-N2 Startweg heruntergeladener Pakete (beide Projektinhaber, Fristen im Fahrplan). Kein PR geöffnet (nicht angefragt). „Build/CI" weiter als Modulname ohne eigenen Architektur-Eintrag (Beobachtung der Vorsession, unverändert).
- **Nächster Schritt:** 2.8 (Weltmaßstab, Luftseite, Zeitmodell) in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 17:46 UTC); 2.1 empfiehlt Entscheidung, 0 Schritte oberhalb der Empfehlung. Teilarbeiten der Routine-Klasse (Konfigurationsdateien, Runbook) nicht abgegeben: Probelauf offen. Worker-Neustart während der Session ohne Modellwechsel.
- **Kontextgröße:** 333.819 Token laut `get_session`, unter der Grenze von 400.000.
- **Kontingent:** 7-Tage-Fenster mit Status `allowed_warning`, Zurücksetzen 2026-10-18 08:00 UTC (in `project-context.md` Abschnitt 8 vermerkt).
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Electron 44: Vorlauf ab 2027-01-02); neu: abgekündigte Unterpakete von electron-builder (Q.1). Runner-Image `ubuntu-latest` → Ubuntu 26 ab 2026-10-19 (Q.1, ersten Lauf beobachten).
- **Drift-Prüfung:** README synchron (Phase 2 mit 2.1 erledigt, Quick Start, Nächste Schritte, Blocker 0, Architektur-Reife unverändert); ADR-018/019 verweisen auf vorhandene Schritte (2.1, 7.1, 7.2, Q.1); Reifegrade unverändert und passend; Reaktiv-Quote 5 / 10 in Teil A stimmt mit den Tags von ADR-010 bis ADR-019; FR-045 nennt 2.1; keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten nach ursprünglichem Plan (2.1-N1/N2 sind Nachlauf); Größen: `project-context.md` 356 Zeilen, Logbuch unter 800.

### 2026-10-13 17:38 (UTC) – [BEOBACHTUNG]

- **2.1 Validierung am frischen Worktree (Commit e1a42d2, Ubuntu 24.04.5, Node 26.11.1, Rust 1.98.1):** `npm ci` ok (audit: 8 moderat, 0 hoch); `pre-commit run --all-files` 15 / 15 grün; `npm run build` ok; Start des Fensters läuft (20 s Timeout ohne Absturz); fmt, clippy, `cargo test`, `cargo deny check`, ESLint, Prettier, tsc, Vitest (53 Tests, 87,2 % Lines; App-Dateien 100 %), Lizenz-Skript (535 Pakete), Rauchtest (1 passed) alle grün. Ohne `Cargo.lock` baut der Workspace mit napi-derive-backend 6.1.5.
- **Erzwungene Fehler (Gates schlagen an):** Rauchtest ohne natives Modul → rot (`state: failed`); `cargo llvm-cov report --package openairside-core --fail-under-lines 101` → Exit 1. Coverage Rust lokal: Kern 100 %, gesamt 100 % (Brücke `test = false`).
- **Paket Linux lokal:** AppImage (120 MB) und deb (95 MB) gebaut; gepacktes Programm besteht den Rauchtest (`OPENAIRSIDE_E2E_EXECUTABLE`). macOS und Windows nur in der CI.
- **Abkündigungen bei `npm install`:** inflight, rimraf 2, glob 7, boolean (alle über electron-builder 26.16.1) → Ablaufdaten-Register, Q.1.
- **Ohne Landeplatz bisher:** keine; CHANGELOG entsteht laut README ab erster Version.

### 2026-10-13 17:36 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** 2.1, atomare Commits.
- **Symptom:** Vier Commits scheiterten still in einer Schleife, alles landete im fünften (`ci:`) als Mix-Commit.
- **Ursache:** pre-commit verweigert Commits, solange `.pre-commit-config.yaml` geändert, aber nicht gestaged ist („Your pre-commit configuration is unstaged").
- **Lösung:** lokalen Mix-Commit per `git reset --soft` aufgelöst (nicht gepusht); ein weiterer `reset --soft` nach einem gescheiterten Probe-Commit setzte HEAD versehentlich einen Commit zu weit zurück und wurde sofort auf `89dfe86` (gepushter Stand) korrigiert – ohne Verlust, Index erhalten. Danach Hook-Konfiguration zuerst committet.
- **Rest-Abweichung:** Commit `c13ebd9` („chore: formatiere bestehende Konfiguration mit Prettier") enthält zusätzlich das Entfernen der `detect`-Gates in `security.yml` (eigentlich Typ `ci`, ADR-018). Historie bewusst nicht umgeschrieben (Stopp-Kriterium 6).
- **Lerneffekt:** Commit-Schleifen mit `set -e`-Abbruch und Fehlerausgabe bauen; nie `reset` auf ein ungeprüftes HEAD.

### 2026-10-13 17:30 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** 2.1, Electron-Rauchtest in der Cloud-Umgebung (Ubuntu 24.04, root).
- **Symptom:** Playwright wartet 30 s vergeblich auf das Fenster; Electron bricht mit `Running as root without --no-sandbox is not supported` ab. Mit `--no-sandbox` starten Kindprozesse (Netzwerkdienst, Utility-Prozess) trotzdem ohne den Schalter neu und brechen ebenso ab.
- **Ursache:** `app.enableSandbox()` erzwingt die Sandbox für alle Kindprozesse und hebt `--no-sandbox` dort auf.
- **Lösung:** `enableSandbox()` nur ohne `--no-sandbox`; der Rauchtest übergibt `--no-sandbox` nur unter Linux (Muster aus Spike 1.5, Linux-Sandbox in 7.2). Härtung im Normalbetrieb unverändert; `process` im Renderer weiter unsichtbar (im Test geprüft).
- **Aufwand:** ca. 10 Minuten. **Wiederkehrgefahr:** bei jedem Start als root oder in Containern; im Runbook unter Troubleshooting.

### 2026-10-13 17:24 (UTC) – [BEOBACHTUNG]

- **STOPP Reaktiv-Quote aufgehoben:** Projektinhaber: „Zur Kenntnis genommen" (Reflexion Anlass ADR-019). 2.1 wird fortgesetzt.
- **Worker-Neustart der Session:** Arbeitsbaum (unversionierte Gerüst-Dateien, `node_modules`, Node 26 unter `/opt/node26`, cargo-deny) blieb erhalten.

### 2026-10-13 16:59 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-019 – Erlaubte Lizenzen um BlueOak-1.0.0, 0BSD, Python-2.0 und WTFPL erweitert
- **Tag:** `[REAKTIV]` (unerwarteter Befund in der Umsetzung, Maßstab wie ADR-011/012/015)
- **Auslöser:** Projektinhaber wählte Option B (Empfehlung der KI war A: Werkzeuge nur gegen Ausschlussliste).
- **Folgen:** `project-context.md` Abschnitt 6 und `deny.toml` erweitert (`cargo deny check` weiter grün); 7.1 um Lizenzhinweise im Paket ergänzt. Nebenbefund: Windows- und Linux-Pakete für arm64 hatten keinen Fahrplan-Schritt, obwohl `project-context.md` Abschnitt 6 sie zusagt; Landeplatz in 7.2 ergänzt.
- **STOPP Reaktiv-Quote:** 5 / 10 (ADR-010 bis ADR-019), 50 %. Reflexion in `fahrplan.md` ergänzt (Maßnahme 4), Kenntnisnahme ausstehend; bis dahin keine weitere Umsetzung in 2.1.

### 2026-10-13 16:57 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.1, Kategorie 8 (Lizenz/Compliance), Stopp-Kriterium 3.
- **Befund:** Nach `npm install` (461 Pakete, Lockfile-Feld `license`) liegen 12 Pakete außerhalb der erlaubten Liste aus `project-context.md` Abschnitt 6: BlueOak-1.0.0 (8: chownr, isexe ×2, minimatch, minipass, sax, tar, yallist), Python-2.0 (argparse), WTFPL (truncate-utf8-bytes), 0BSD (tslib). Alle permissiv, keines Copyleft; alle nur über Werkzeuge (electron-builder, @napi-rs/cli), keines wird mit dem Spiel ausgeliefert (keine Laufzeit-Abhängigkeiten in `package.json`). WTFPL OR ISC, WTFPL OR MIT, MIT OR CC0-1.0 sind über die erlaubte Alternative gedeckt.
- **Rust:** `cargo deny check` grün (advisories, bans, licenses, sources ok).
- **Vorlage:** Optionen im Chat; Entscheidung steht aus. Bis dahin kein Lizenz-Skript.

### 2026-10-13 16:50 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-018 – Projektgerüst, CI-Gates und napi-Versionen für Phase 2
- **Tag:** `[OPERATIV]` (in 2.1 geplante Freigabe)
- **Auslöser:** Freigabe des Projektinhabers per Auswahl: 1A (napi-derive 3.6.12), 2A (Spike-Workflows entfernen), Rauchtest bei jedem Push, übrige CI-Erweiterungen und Begleitpakete wie vorgeschlagen.
- **Reaktiv-Quote:** 4 / 10 (ADR-009 bis ADR-018), unverändert; durch die bestätigte Reflexion vom 2026-10-13 abgedeckt.
- **Schritt 2.1:** `[IN ARBEIT]`.

### 2026-10-13 16:46 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.1 (Projektinhaber wählte 2.1 vor 2.8).
- **Kategorien:** 7 (Build- und Deploy-Pipeline), 3 (napi-Versionen, Begleitpakete).
- **Messung napi (Scratchpad, frischer Workspace ohne `Cargo.lock`, Rust 1.98.1):** napi 3.12.7 + napi-derive 3.6.8 ohne Festsetzung → Build bricht (`?` couldn't convert the error to `napi_derive_backend::Diagnostic`, backend 6.1.5 gezogen; Befund aus Spike 1.1 reproduziert). Mit `napi-derive-backend = "=6.1.4"` → baut. napi 3.12.7 + napi-derive 3.6.12 (zieht backend 6.1.5) → baut. napi 3.14.2 + napi-derive 3.6.12 + napi-build 2.6.0 → baut. Varianten 1 und 2 als `.node` in Node 22.22 geladen: Funktion, Klasse mit Konstruktor und `#[napi(object)]` liefern korrekte Werte.
- **Reife (crates.io, 2026-10-13):** napi-Linie 3.12 seit 2026-07-28 mit Patches; 3.13 (2026-09-22) ohne Patch, 3.14 seit 2026-10-01 (unter 1 Monat) → nicht reif. napi-derive-Linie 3.6 seit 2026-07-21, 3.6.12 vom 2026-10-06.
- **Vorlage:** siehe Antwort im Chat; Entscheidung steht aus.

### 2026-10-13 16:40 (UTC) – [SESSIONSTART]

- **Letzter Stand:** Phase 1 abgeschlossen (ADR-016), Phase 2 ausgeplant; STOPP Reaktiv-Quote durch Kenntnisnahme aufgehoben (19:05). PR #12 gemergt, Branch `claude/gracious-galileo-2reksz` steht auf `main` (24165c0), keine eigenen Commits darüber.
- **Geplant für diese Session:** Beginn von Phase 2 mit 2.1 (Repository-Struktur, CI-Gates) oder 2.8 (Weltmaßstab, Luftseite, Zeitmodell); Auswahl durch den Projektinhaber, da höchstens ein Schritt `[IN ARBEIT]`.
- **Vorabprüfung:** 2.1 – Eingangskriterium Stack und Betriebsart `[BELASTBAR]` erfüllt; 2.8 – Datenmodell im Kern `[BELASTBAR]` erfüllt. Keine aktiven Blocker, keine offenen STOPP-Situationen.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Auto-Modus), aus der Desktop-App gestartet.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` (16:40 UTC) → Entscheidungs-Klasse. Beide Kandidaten-Schritte empfehlen Entscheidung.
- **Hinweis Zeitstempel:** Die Systemuhr der Session zeigt 16:40 UTC; die Einträge der Vorsession (18:00–19:05) sind damit nicht in derselben Zeitbasis. Ab hier Zeitstempel in UTC mit Kennzeichnung, abgeglichen mit den Commit-Zeiten (erste Fassung enthielt geschätzte Zeiten, am 17:40 UTC korrigiert).

### 2026-10-13 19:05 – [BEOBACHTUNG]

- **STOPP Reaktiv-Quote aufgehoben:** Projektinhaber: „Zustimmung, Reflexion zur Kenntnis genommen". Kein Architektur-Umbau; Maßnahme (3) gilt (Beförderungen nennen den gemessenen Umfang). Phase 2 kann mit 2.1 und 2.8 beginnen.

### 2026-10-13 19:00 – [SESSIONENDE]

- **Session-Dauer:** ca. 60 Minuten seit `[SESSIONSTART]` 18:00
- **Bearbeitet:** 1.7 – Auswertung und Entscheidung (Phasenabschluss 1)
- **Erreicht:** 1.7 `[ERLEDIGT]`; Bewertung durch getrennte Instanz und Stellungnahme (`docs/research/phasenabschluss-1.md`); ADR-016 (gezielt umbauen, Regel-007), ADR-017 (NFR Performance auf das Gemessene begrenzt); Phase 2 mit 10 Schritten im vollen Format; Vision-Abgleich mit behobenen Lücken (FR-039, FR-017, 7.3); Phase 1 aus Fahrplan und Logbuch archiviert; README und `project-context.md` nachgezogen.
- **Offen geblieben:** Kenntnisnahme der Reaktiv-Reflexion (Quote 4 / 10 nach ADR-017) – bis dahin kein Umsetzungsschritt von Phase 2. CI des PR nicht abgewartet. „Build/CI" als Modulname ohne Architektur-Eintrag (Beobachtung 18:45).
- **Nächster Schritt:** nach Kenntnisnahme 2.1 (Repository-Struktur, CI-Gates; Freigabe Kategorie 7) und 2.8 (Weltmaßstab, Luftseite, Zeitmodell), in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 19:00); 1.7 empfiehlt Entscheidung, 0 Schritte oberhalb der Empfehlung. Abgegeben: Bewertung an einen Unteragenten als getrennte Instanz (Entscheidungs-Klasse, kein Sparzweck); keine Abgabe an Routine/Mechanik (Probelauf offen).
- **Kontextgröße:** 206.592 Token laut `get_session`, unter der Grenze von 400.000.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Electron 44: Vorlauf ab 2027-01-02). Hinweis: Runner-Image `ubuntu-latest` stellt ab 2026-10-19 auf Ubuntu 26 um (Q.1, ersten Lauf beobachten). **Drift-Prüfung:** README synchron (Phase 2, Architektur-Reife mit ADR-017, Nächste Schritte, Blocker 0); ADR-016/017 verweisen auf vorhandene Schritte (2.1–2.10, 3.4, 3.6, 4.1, 5.5, 5.6, 7.3); Reifegrade in `architecture.md` Abschnitt 9 passen zu ADR-017; Reaktiv-Quote 4 / 10 in Teil A stimmt mit Tags in ADR-008 bis ADR-017; jede Muss-Anforderung hat einen Schritt; keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 bei 10 Schritten nach ursprünglichem Plan; Größe `project-context.md` 354 Zeilen, Logbuch 110 Zeilen.

### 2026-10-13 18:50 – [PHASEN-WECHSEL]

- **Phase 1 → Phase 2:** Phase 1 (ERKUNDUNG) mit 1.7 abgeschlossen; Phase 2 (UMSETZUNG, Fundament) mit 10 Schritten ausgeplant (ADR-016). Detail-Einträge von Phase 1: [`docs/archiv/logbuch-phase-1.md`](archiv/logbuch-phase-1.md); Fahrplan: [`docs/archiv/fahrplan-phase-1.md`](archiv/fahrplan-phase-1.md).
- **Reflexion Phase 1:**
  - Gelernt: Kern mit großer Reserve (Schritt 0,3–0,8 ms bei 10.000 Personen, Budget 5,5 ms); Bild als Kopie per `MessagePort` (Rundlauf 0,6–2,8 ms); Wegweiser-Neuberechnung im Hintergrund statt inkrementell; Tabellen nur virtualisiert.
  - Gekippte Annahmen: macOS Universal (→ nur Apple-Chips, ADR-012); Linux-Messung in Phase 1 (→ 7.2, ADR-011); Bauaktion im nächsten Bild ohne Vorschau.
  - Reifegrade: Stack, Pattern, Betriebsart, Datenmodell, Wegweiser, S-1, große Tabellen `[BELASTBAR]` (ADR-014); NFR Performance geteilt (ADR-017).
  - ADRs der Phase: 009–017; reaktiv 011, 012, 015, 017 – alle in Verpackung, Testlogistik oder Reifegrad-Formulierung.
  - Neue Erkundungen: Weltmaßstab und Luftseite (2.8), Schnittstellenverträge S-2 bis S-7 (2.9), Darstellung mit echten Grafiken (2.10).
  - Mini-Reibungen, die wiederkehren können: Pin napi-derive-backend nur in `Cargo.lock`; Startweg unsignierter Mac-Pakete (Gatekeeper, ADR-015); Verwechslung „inkrementell/voll" zwischen Spikes.
- **Pflichtfrage:** „gezielt umbauen" (B), Bewertung durch getrennte Instanz, `docs/research/phasenabschluss-1.md`.
- **Vision-Abgleich:** FR-039 → 3.6, 5.6; FR-017 → 1.2, 2.3, 3.3; Erfolgskriterien „Gleichwertiges Spielerlebnis" und „Machbarkeitstest" → 7.3. Keine `[VERSCHOBEN]`-Zeilen. Keine verwaisten Elemente mehr.
- **Onboarding-Validierung:** nicht validierungsrelevant (kein lauffähiger Quick Start, Runbook bis 2.1 „TBD").

### 2026-10-13 18:45 – [ADR-ANGELEGT]

- **ADR-016** (`[ERKENNTNIS]`): Pflichtfrage „gezielt umbauen", Freigabe des Projektinhabers „B, freigegeben, Widerspruch und 7.3 wie vorgeschlagen". Regel-007 löst den Widerspruch `CLAUDE.md` Abschnitt 6 ↔ `architecture.md` Abschnitt 0: Grundsatzentscheidungen und genutzte Schnittstellen `[BELASTBAR]` vor einem Umsetzungsschritt, Module dürfen `[VORLÄUFIG]` starten.
- **ADR-017** (`[REAKTIV]`): Reifegrad „NFR Performance Gesamtsystem" auf das Gemessene begrenzt. Reaktiv, weil er die Reifegrad-Wirkung von ADR-014 innerhalb von 14 Tagen ändert. Reaktiv-Quote 4 / 10 (40 %) → STOPP, Reflexion im Fahrplan ergänzt, Kenntnisnahme steht aus.
- **Nachgezogen:** `architecture.md` (Regel-007 in Abschnitt 0, NFR geteilt, veraltete Stellen zu ECS, Änderungsbereichen, Übertragungsumfang und rayon bereinigt, Betriebsart und Datenmodell als `[BELASTBAR]` vermerkt); `requirements.md` (FR-017, FR-039, FR-050); Fahrplan (Phase 2, 3.6, 5.6, 7.3).
- **Beobachtung:** „Build/CI" steht in Fahrplan-Schritten als betroffenes Modul, ist aber kein Modul in `architecture.md` Abschnitt 2 (seit 1.6 so); gemeint ist das Tooling-Inventar (Abschnitt 10). Keine Änderung in dieser Session.

### 2026-10-13 18:25 – [ENTSCHEIDUNG-VORGELEGT]

- **Pflichtfrage Phasenende 1:** Bewertung der getrennten Instanz (Unteragent ohne Gesprächsverlauf, Entscheidungs-Klasse; Logbuch ausgeschlossen) empfiehlt „gezielt umbauen" (Plan, Verträge, Dokumente; nicht Stack oder Pattern), Konfidenz mittel. Stellungnahme: Zustimmung mit Ergänzungen und zwei Widersprüchen. Volltext: `docs/research/phasenabschluss-1.md`.
- **Nachgeprüft:** Befunde 5, 7, 8, 9 und Coverage-Gate am Repository bestätigt.
- **Widerspruch gefunden:** `CLAUDE.md` Abschnitt 6 verlangt `[BELASTBAR]` für berührte Bestandteile vor UMSETZUNG, `architecture.md` Abschnitt 0 lässt Module `[VORLÄUFIG]` bis zur eigenen Umsetzung. Auflösung per ADR mit der Entscheidung.
- **Vision-Abgleich:** FR-039 ohne Umsetzungsschritt; FR-017 Drift zwischen `requirements.md` und Fahrplan; Erfolgskriterien „Gleichwertiges Spielerlebnis" und „Machbarkeitstest" ohne Schritt-ID. Behebung mit der Ausplanung nach Freigabe, vor Phasenabschluss.
- **Onboarding-Validierung (Abschnitt 16, Trigger 3):** nicht validierungsrelevant – README-Quick-Start „noch nicht lauffähig", Runbook bis 2.1 „TBD"; kein Pfad zum Nachvollziehen.

### 2026-10-13 18:00 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 16:40; PR Paddel87/OpenAirside#11 gemergt (`621bb1b`), Branch `claude/gracious-galileo-2reksz` steht auf `main`.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant:** 1.7 – Auswertung und Entscheidung: Erkenntnisse zusammenfassen, Bewertung „Weiterbauen, umbauen oder neu aufsetzen" durch eine getrennte Instanz, Vision-Abgleich an der Phasengrenze, Vorlage an den Projektinhaber.
- **Vorabprüfung:** Eingangskriterium „alle Messprotokolle liegen vor" erfüllt (1.1–1.6 `[ERLEDIGT]`, 1.6-N1 erledigt). Keine aktiven Blocker. Reaktiv-Quote 3 / 10 über dem Schwellenwert, Reflexion bestätigt.
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Quelle: `get_session`) → Entscheidungs-Klasse, passend zur Empfehlung von 1.7; Kontext zu Beginn ca. 103.000 Token.

---

<!-- ANCHOR:eintragstypen -->
## Eintragstypen (Übersicht)

| Typ | Wann | Pflicht? |
|---|---|---|
| `[SESSIONSTART]` | Zu Beginn jeder Session | Ja |
| `[SESSIONENDE]` | Vor Sessionabschluss | Ja |
| `[PROBLEM-GELÖST]` | Nach Behebung eines Problems, das Reibung war | Empfohlen, alle Mini-Probleme erfassen |
| `[PROBLEM-OFFEN → BLOCKER]` | Wenn ein Problem zum Blocker eskaliert | Ja, mit Verweis auf `blockers.md` |
| `[BLOCKER-AUFGELÖST]` | Wenn ein Blocker gelöst wurde | Ja, mit Verweis auf den ursprünglichen Logbuch- und Blocker-Eintrag |
| `[REIFEGRAD-WECHSEL]` | Bei jeder Reifegrad-Änderung in `architecture.md` | Ja |
| `[ADR-ANGELEGT]` | Bei Anlage eines neuen ADR | Ja |
| `[BEOBACHTUNG]` | Wenn etwas auffällt, das später nützlich sein könnte | Optional, KI proaktiv |

### Format der Einträge

```text
### YYYY-MM-DD HH:MM – [TYP]

- **Feld:** Inhalt
```

Felder je Typ:

- `[SESSIONSTART]`: Letzter Stand, Geplant für diese Session, Vorabprüfung (Reifegrade, Eingangskriterien), Modus/Werkzeug, Modell (eingestellt und bedient) mit Modellklasse und Quelle (`CLAUDE.md` Abschnitt 0)
- `[SESSIONENDE]`: Session-Dauer, Bearbeitet, Erreicht, Offen geblieben, Nächster Schritt, Aktive Klasse mit Zahl der Schritte oberhalb ihrer Empfehlung und abgegebenen Teilarbeiten, Kontextgröße (mit Hinweis bei Überschreitung der Grenze aus `project-context.md` Abschnitt 6), Ablaufdaten-Prüfung, Stimmung/Beobachtung (optional)
- `[PROBLEM-GELÖST]`: Kontext, Symptom, Ursache, Lösung, Aufwand, Lerneffekt (optional), Wiederkehrgefahr (optional)
- `[PROBLEM-OFFEN → BLOCKER]`: Kontext, Symptom, Versuche bisher, Eskaliert nach (Blocker-Nr.)
- `[REIFEGRAD-WECHSEL]`: Bestandteil, Wechsel, Auslöser, Datum in `architecture.md` Abschnitt 9 nachgetragen
- `[ADR-ANGELEGT]`: ADR, Tag, Auslöser

<!-- ANCHOR:hinweise-zur-pflege -->
## Hinweise zur Pflege

- **Neueste Einträge oben.**
- **Zeitstempel ist Pflicht.** Format: `YYYY-MM-DD HH:MM` (24h, lokale Zeitzone).
- **Detailtiefe lieber zu hoch als zu niedrig.** Wenn unsicher, ob etwas eingetragen werden soll: eintragen.
- **Verweise statt Duplikate** zu ADRs, Blockern und Fahrplan-Schritten.
- **Keine sensiblen Daten.**
- **CI-Ergebnisse:** Bei Problemen aus der CI wird festgehalten, was im Log stand, nicht nur die Ampelfarbe.

<!-- ANCHOR:archivierung -->
## Archivierung

Bei mehr als 800 Zeilen: alte Einträge nach `docs/archiv/logbuch-YYYY-MM.md` auslagern; im aktiven Logbuch bleiben die letzten 4–8 Wochen plus alle Einträge zu offenen Blockern. Auslagerung ist Sessionende-Aktion.
