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

Phase 1 läuft. Schritte 1.1 (ADR-009), 1.2 (ADR-010), 1.3, 1.4 und 1.6 erledigt; Linux-Messung nach ADR-011 in 7.2. Nächster Schritt: 1.5.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-13 14:38 – [ADR-ANGELEGT]

- **ADR-013:** Größte Karte 768 × 768 Zellen je Etage, fünf Etagen (`[ERKENNTNIS]`, füllt die an 1.5 übergebene Festlegung aus ADR-010 aus, ändert ADR-010 nicht). Freigabe des Projektinhabers: „b".
- **Nachgezogen:** `decisions.md` (Teil A, Index, Reaktiv-Quote 2/10 über ADR-004 bis ADR-013), `fahrplan.md` 1.5, `architecture.md` Modul Wegfindung und Reifegrad-Übersicht (bleibt `[VORLÄUFIG]`).
- **Modellklasse:** Entscheidungs-Block nach Abschnitt 4 → Eskalations-Auslöser 1; aktiv `claude-opus-5-5` (Entscheidungs-Klasse), kein Wechsel nötig.

### 2026-10-13 14:28 – [BEOBACHTUNG]

- **1.3/1.4 maßen Bauaktionen mit inkrementeller Reparatur:** `Sim::new` aus Spike 1.2 setzt `incremental = true`; die Brücke aus 1.3 änderte das nicht. Die in 1.3/1.4 gemessenen 39–54 ms bis zur Übernahme neuer Wegweiser gelten damit für die Reparatur, nicht für die in ADR-010 gewählte volle Neuberechnung (bei 512² × 2 laut 1.2: 0,2–0,4 s auf 4 Threads). Kein Einfluss auf die Akzeptanzkriterien von 1.3/1.4 (Bauaktion sichtbar, Wände wirken sofort); Testprogramm 1.5 misst die volle Neuberechnung. Festgehalten im Messprotokoll 1.5, Befund 4.
- **Pre-commit-Hook war in diesem Container nicht installiert;** nach `pre-commit install` scheitern `eslint`, `cargo-fmt` und `cargo-clippy` mangels Projektdateien am Repo-Stamm (erst ab 2.1, wie in 1.1–1.4 dokumentiert). Commit des Spike-Codes mit `SKIP=eslint,cargo-fmt,cargo-clippy`; `cargo fmt --check` und `cargo clippy -D warnings` liefen direkt in den drei Spike-Workspaces, übrige Hooks (Prettier, YAML, JSON, Markdown) grün.
- **Rastergröße nicht aus Dokumenten ableitbar:** keine Zellgröße in Metern festgelegt; Vorlage im Fahrplan 1.5 relativ zum Layout aus 1.2.

### 2026-10-13 14:11 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 14:15; PR Paddel87/OpenAirside#9 gemergt (`1b8e76e`), Branch `claude/gracious-galileo-2reksz` neu auf `main` gesetzt.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht); vertieft: ADR-010, Messprotokolle 1.2 und 1.3.
- **Geplant für diese Session:** 1.5 – Leistungsnachweis auf den Zielsystemen: Messung im Container (Rastergrößen, fünf Etagen), Testprogramm für Referenz-PC und Mac.
- **Vorabprüfung:** ERKUNDUNG, keine BELASTBAR-Pflicht; Abhängigkeiten 1.2, 1.3, 1.4 erledigt; Testsysteme laut Fahrplan verfügbar. Ergebnis ist freigabepflichtig (Bestätigung oder Verwerfung von ADR-002).
- **Modus/Werkzeug:** Claude Code (Cloud-Session, Desktop-App).
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Quelle: `get_session`, `configured_model` und `last_served_model`) → Entscheidungs-Klasse; entspricht der Empfehlung von 1.5. Kontext zu Sessionbeginn laut `get_session`: 102.629 Token (Zählung nach `/clear`). Zeitstempel ab hier nach Systemuhr (UTC); die Stempel der Vorsession waren geschätzt.

### 2026-10-13 14:15 – [SESSIONENDE]

- **Session-Dauer:** ca. 65 Minuten seit `[SESSIONSTART]` 13:10 (mit Neustart der Laufzeitumgebung dazwischen)
- **Bearbeitet:** 1.4 (Spike, Testprogramm, Auswertung Referenz-PC und Mac), ADR-012 (macOS nur Apple-Chips)
- **Erreicht:** 1.4 `[ERLEDIGT]`; „Oberfläche mit großen Tabellen" `[BELASTBAR]` mit Bedingungen; ADR-012 mit Nachzug; Messprotokoll, Fahrplan, Architektur, README nachgezogen.
- **Offen geblieben:** Startablauf des neuen macOS-Pakets (nur Apple-Chips) vom Projektinhaber nicht zurückgemeldet; CI des letzten Dokumentations-Commits nicht abgewartet. Folgepunkte mit Landeplatz: Sortierung großer Tabellen und Bauaktion (1.7), Linux (7.2), Signatur (7.1).
- **Nächster Schritt:** 1.5 – Leistungsnachweis auf den Zielsystemen, in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 1 Schritt oberhalb der Empfehlung (1.4 empfiehlt Routine, Warnung gegeben); keine Abgabe (Probelauf offen).
- **Kontextgröße:** 750.317 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** Umstellung `ubuntu-latest` auf Ubuntu 26 ab 2026-10-19 steht bei Q.1; kein weiterer Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** README synchron (Phase, Architektur-Reife, Nächste Schritte); Reifegrade in Abschnitt 3 und 9 gleich; Reaktiv-Quote 2/10 laut Teil A; Phase 1 weiter 7 Schritte.

### 2026-10-13 14:10 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Oberfläche mit großen Tabellen
- **Wechsel:** `[VORLÄUFIG]` → `[BELASTBAR]`, mit Bedingungen (Virtualisierung Pflicht; Sortierung sehr großer Tabellen nach Entscheidung 1.7)
- **Auslöser:** Spike 1.4 auf Referenz-PC (Windows, 60 Hz) und Mac M3 Pro (120 Hz); Linux nach ADR-011 in 7.2. Entscheidungs-Klasse aktiv (`claude-opus-5-5`), Eskalations-Auslöser 4.
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja

### 2026-10-13 14:08 – [BEOBACHTUNG]

- **Spike 1.4 auf dem Mac M3 Pro** (120 Hz, Ergebnisdatei vom Projektinhaber): bis 5.000 sortierte Zeilen 120 Bilder/s bei jeder Rate; 20.000 sortierte Zeilen 110–120 (Aktualisierung 11–19 ms, über dem 120-Hz-Budget von 8,3 ms); ohne Virtualisierung 59–94 mit Hängern bis 274 ms. **1.4 erledigt.**

### 2026-10-13 14:00 – [BEOBACHTUNG]

- **Spike 1.4 auf dem Referenz-PC unter Windows** (Ergebnisdatei vom Projektinhaber, `spikes/1.4-tabellen/results/referenz-pc-windows-rx7700xt.json`): virtualisiertes Panel ohne Einfluss auf die Bildrate bis 5.000 sortierte Zeilen bei jeder Rate und bis 20.000 sortierte Zeilen bei bis zu 30 Aktualisierungen/s; 20.000 sortierte Zeilen kosten rund 20 ms je Aktualisierung (TanStack-Sortierung 17–19 ms). Ohne Virtualisierung 17–46 Bilder/s bei 2.000 Zeilen. Offen: Mac.
- **Wiedereinstieg nach Neustart der Laufzeitumgebung:** Spike 1.4 und ADR-012 entstanden in einer Session, deren Verlauf hier nicht vorliegt; Stand aus Logbuch 13:10–13:45, Fahrplan 1.4, ADR-012 und Messprotokoll 1.4 nachgelesen, bevor ausgewertet wurde.

### 2026-10-13 13:45 – [ADR-ANGELEGT]

- **ADR-012:** macOS nur für Apple-Chips, kein Intel- und kein Universal-Build (`[REAKTIV]`, ändert eine Konsequenz von ADR-009 innerhalb von 14 Tagen). Freigabe des Projektinhabers: „A, nur Apple-Chips". Anlass: kleinere Testpakete; Apple beendet die Intel-Unterstützung mit macOS 26; Intel nie testbar.
- **Nachgezogen:** `project-context.md` (Abschnitte 2, 3, 6), `architecture.md` Abschnitt 1, `onboarding-runbook.md` Abschnitt 2, `release.yml`, Teil A und Reaktiv-Quote in `decisions.md` (2/10 = 20 %, genau am Schwellenwert, kein STOPP).
- **Vorher, ebenfalls freigegeben:** `spike-build-1-4.yml` baut je System nur ein Format (Windows ZIP, macOS DMG arm64, Linux AppImage).
- **Modellklasse:** Entscheidungs-Block nach Abschnitt 4 → Eskalations-Auslöser 1; aktiv `claude-opus-5-5` (Entscheidungs-Klasse, `get_session`), kein Wechsel nötig.

### 2026-10-13 13:25 – [BEOBACHTUNG]

- **Spike 1.4 im Container** (`spikes/1.4-tabellen`, Messprotokoll im README): Das Panel drückt die Bildrate bis 5.000 nach einer Live-Spalte sortierte Zeilen bei keiner Rate bis 60/s; 20.000 sortierte Zeilen ab 4/s schon (TanStack sortiert 24–30 ms je Aktualisierung, reines `Array.sort` der Zeilennummern 4,8 ms). Ohne Virtualisierung kosten 2.000 Zeilen 120–180 ms je Aktualisierung.
- **Reibung TanStack Table 9:** API gegenüber Version 8 umgebaut (`useTable` mit `tableFeatures`, `rowSortingFeature` + `createSortedRowModel`, `sortFn_*`); gelöst über die mitgelieferten Anleitungen in `node_modules/@tanstack/*/skills/`. Zeilen müssen Objekte sein (`RowData`), deshalb `{ i }` je Zeile.
- **Reibung Vite 8 im Bibliotheksmodus:** ersetzt `process.env.NODE_ENV` nicht (React sonst in Entwicklungsfassung) → `define`; vier Warnungen `MODULE_LEVEL_DIRECTIVE` (`"use client"` in TanStack) – für Phase 2 im Messprotokoll vermerkt.
- **Hooks:** Rust-Hooks am Repo-Stamm greifen erst ab 2.1; `cargo fmt --check` und `cargo clippy -D warnings` im Spike direkt, übrige Hooks über `pre-commit run` grün (Prettier als Container-Version 3.8.1, wie in 1.1–1.3).

### 2026-10-13 13:10 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 13:05; PR Paddel87/OpenAirside#8 gemergt (`2c2f123`), Branch `claude/gracious-galileo-2reksz` neu auf `main` gesetzt.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant für diese Session:** 1.4 – Großes Tabellen-Panel neben dem Spielfeld (Prototyp auf Basis von Spike 1.3).
- **Vorabprüfung:** ERKUNDUNG, keine BELASTBAR-Pflicht; Eingangskriterium „Spielfeld aus 1.3 lauffähig" erfüllt. React 19.2.8, TanStack Table 9.2.8 und Vite 8.3.4 stehen im Stack (ADR-002); Virtualisierung ohne neue Abhängigkeit (eigene Fensterung).
- **Modus/Werkzeug:** Claude Code (Cloud-Session, Desktop-App).
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Quelle: `get_session`, `configured_model` und `last_served_model`) → Entscheidungs-Klasse. Schritt 1.4 empfiehlt Routine → läuft oberhalb der Empfehlung; Abgabe nicht möglich (Probelauf offen), Warnung gegeben.

### 2026-10-13 13:05 – [SESSIONENDE]

- **Session-Dauer:** ca. 15 Minuten Arbeitszeit nach dem `[SESSIONENDE]` von 12:50 (dieselbe Sitzung, Mac-Test begleitet)
- **Bearbeitet:** 1.3 und 1.6 (Mac-Auswertung, macOS-Startablauf), Beförderung S-1
- **Erreicht:** 1.3 und 1.6 `[ERLEDIGT]`; S-1 `[BELASTBAR]`; Messprotokolle, Fahrplan, Architektur, README nachgezogen.
- **Offen geblieben:** CI des Dokumentations-Commits beim Schreiben noch nicht abgewartet. Folgepunkte mit Landeplatz: Bauaktion im übernächsten Bild (1.7), Linux (7.2), Signatur (7.1).
- **Nächster Schritt:** 1.4 – Großes Tabellen-Panel neben dem Spielfeld, in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 0 Schritte oberhalb der Empfehlung; keine Abgabe.
- **Kontextgröße:** 680.642 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** README (Phase, Architektur-Reife, Nächste Schritte) synchron; Reifegrad S-1 im Abschnitt 4 und 9 gleich; keine neuen ADRs; Phase 1 weiter 7 Schritte.

### 2026-10-13 13:00 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** S-1 Zustandspuffer
- **Wechsel:** `[VORLÄUFIG]` → `[BELASTBAR]`
- **Auslöser:** Spike 1.3 auf Referenz-PC (Windows, 60 Bilder/s, Rundlauf 0,6–2,8 ms) und Mac M3 Pro (120 Bilder/s, Rundlauf 0,7–2,1 ms p50); Linux nach ADR-011 in 7.2. Validierung durch Messung, kein Schutzmechanismus im Sinne von `CLAUDE.md` Abschnitt 6. Entscheidungs-Klasse aktiv (`claude-opus-5-5`, `get_session`), Eskalations-Auslöser 4 erfüllt.
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja
- **Nicht befördert:** Verpackung bleibt `[VORLÄUFIG]` (unsigniert, Linux-Sandbox).

### 2026-10-13 12:58 – [BEOBACHTUNG]

- **Spike 1.3 auf dem Mac M3 Pro** (eingebautes Display 120 Hz, Pixeldichte 2): 10.000 und 50.000 Personen mit `particles` 120 Bilder/s, Bild p99 ≤ 9,4 ms, kein Bild über 20 ms; Sprites bei 50.000 beim Schwenken 80–109 Bilder/s. Simulationsschritt 0,5–1,5 ms (länger als auf dem Referenz-PC), Baubefehl im Simulations-Thread bis 17,8 ms. Bauaktion p50 27 ms (4 Bilder bei 120 Hz).
- **macOS-Start ohne Signatur:** Freigabe über Systemeinstellungen → Datenschutz & Sicherheit → „Dennoch öffnen" funktioniert; Rechtsklick → „Öffnen" wirkt seit macOS 15 nicht mehr.
- **1.3 und 1.6 erledigt.** Offene Folgepunkte mit Landeplatz: Bauaktion im übernächsten Bild (1.7), Linux auf echter Hardware und Linux-Sandbox (7.2), Signatur (7.1).

### 2026-10-13 12:50 – [SESSIONENDE]

- **Session-Dauer:** ca. 40 Minuten seit `[SESSIONSTART]` 12:10
- **Bearbeitet:** 1.3 (Auswertung Windows-Ergebnis), 1.6 (Windows-Rückmeldung), ADR-011 (Linux-Messung verschoben auf 7.2)
- **Erreicht:** Windows-Werte von Spike 1.3 im Messprotokoll (60 Bilder/s bei 10.000 und 50.000 Personen mit `particles`); SmartScreen-Verhalten unter Windows belegt; ADR-011 mit Nachzug in `project-context.md` Abschnitt 6 und Fahrplan 1.3/1.5/1.6/7.2; Reifegrad-Vermerk S-1 aktualisiert (bleibt `[VORLÄUFIG]`).
- **Offen geblieben:** 1.3 und 1.6 `[IN ARBEIT]`, warten nur noch auf die Mac-Messung (Projektinhaber ab 14:45 Uhr).
- **Nächster Schritt:** Ergebnisdatei und Startverhalten vom Mac auswerten, 1.3 und 1.6 abschließen, S-1 ggf. auf `[BELASTBAR]` (Eskalations-Auslöser 4); danach 1.4.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 0 Schritte oberhalb der Empfehlung; keine Abgabe.
- **Kontextgröße:** 625.658 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Umstellung `ubuntu-latest` ab 2026-10-19 bleibt bei Q.1). **Drift-Prüfung:** README unverändert gültig; ADR-011 verweist auf 1.3, 1.5, 1.6, 7.2 (existieren); Reaktiv-Quote 1/10 stimmt mit Teil B überein; Phase 1 weiter 7 Schritte.

### 2026-10-13 12:40 – [ADR-ANGELEGT]

- **ADR:** ADR-011 „Linux-Leistungsmessung auf echter Hardware verschoben auf Schritt 7.2" (`docs/decisions/ADR-011.md`)
- **Tag:** `[REAKTIV]` – ändert die Bedingung „Linux mit WebGL2" aus ADR-002 innerhalb von 14 Tagen; Reaktiv-Quote 1/10 (10 %, Schwelle 20 %)
- **Auslöser:** Projektinhaber möchte keine Neustarts seiner Arbeitsrechner für Linux; Freigabe „A und B" (Messung in 7.2, früher durch Mitwirkende)

### 2026-10-13 12:25 – [BEOBACHTUNG]

- **Rückmeldung Projektinhaber, Windows:** SmartScreen-Warnung beim Start, nach Zulassen lauffähig (1.6, Teil Windows). Bildschirm auf 60 Hz begrenzt – die gemessenen 60 Bilder/s sind damit die Obergrenze des Bildschirms, nicht der Grafik.

### 2026-10-13 12:15 – [BEOBACHTUNG]

- **Spike 1.3 auf dem Referenz-PC unter Windows** (Ryzen 9 3900X, RX 7700 XT, Ergebnisdatei vom Projektinhaber): 10.000 Personen 60 Bilder/s ohne ein Bild über 20 ms (beide Zeichenarten), 50.000 mit `particles` 60 Bilder/s, mit `sprites` bis 46 Bilder/s. Bauaktion im übernächsten Bild (p50 33,5 ms). Ein Baubefehl kostet im Simulations-Thread bis 17 ms (Raster-Schnappschuss) – Ansatzpunkt für Phase 2, kein Rückstand gemessen. Details im Messprotokoll.
- **Nebenbefund 1.6:** Das Windows-Paket von Spike 1.3 lief auf dem Referenz-PC; die Rückmeldung zum Startablauf (SmartScreen) fehlt noch.

### 2026-10-13 12:10 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 11:45 (Spike 1.3 in einer anderen Session gebaut, PR Paddel87/OpenAirside#6 gemergt als `2436e99`); Branch neu auf `main`.
- **Pflichtlektüre:** Änderungen seit dem letzten Lesen in dieser Sitzung gezielt gelesen (`git diff 1c5c9fb 2436e99` für `docs/` und README; Logbuch 11:10–11:45; Messprotokoll `spikes/1.3-spielfeld/README.md`).
- **Geplant:** Ergebnisdatei des Testprogramms 1.3 vom Referenz-PC (Windows) auswerten.
- **Modell:** eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse (`get_session`); 1.3 empfiehlt Entscheidung.
- **Kontextgröße:** 528.832 Token laut `get_session` – über der Grenze von 400.000 (Fortsetzung dieser Sitzung auf Wunsch des Projektinhabers).

### 2026-10-13 11:45 – [SESSIONENDE]

- **Session-Dauer:** ca. 35 Minuten seit `[SESSIONSTART]` 11:10
- **Bearbeitet:** 1.3 – Zustandsübergabe und Spielfeld-Darstellung
- **Erreicht:** Spike `spikes/1.3-spielfeld/` (Brücke mit Dreifachpuffer, Bildformat, Filter, Bauaktion; PixiJS-Spielfeld; eingebaute Messreihe); Kern aus 1.2 additiv ergänzt (Tests grün); Messreihe im Container mit und ohne Zeichnen; Messprotokoll; Workflow `spike-build-1-3.yml`, erster Lauf auf allen drei Plattformen grün. Prüfungen: `cargo test` (bridge-core 4, wegweiser 3), `cargo clippy -D warnings`, `cargo fmt --check`, Prettier (Container-Version 3.8.1, wie in 1.1), pre-commit-Dokumentations-Hooks grün; ESLint ohne Projektkonfiguration bis 2.1 nicht anwendbar.
- **Offen geblieben:** 1.3 bleibt `[IN ARBEIT]`: Bildrate auf echter Grafik nur mit dem Testprogramm auf Referenz-PC und Mac messbar. 1.6 unverändert (Rückmeldung zum Start). Abweichung „Bauaktion im übernächsten statt nächsten Bild" zur Entscheidung in 1.7.
- **Nächster Schritt:** Ergebnisdateien der Testprogramme 1.3 (Ordner „Downloads") und Rückmeldung 1.6 auswerten; S-1 dann ggf. auf `[BELASTBAR]` (Eskalations-Auslöser 4, Entscheidungs-Klasse); danach 1.4.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 0 Schritte oberhalb der Empfehlung (1.3 empfiehlt Entscheidung); keine Abgabe (Probelauf offen).
- **Kontextgröße:** 286.870 Token laut `get_session`, unter der Grenze von 400.000.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Umstellung `ubuntu-latest` erst ab 2026-10-19). **Drift-Prüfung:** README „Nächste Schritte" nachgezogen; keine ADR- oder Reifegrad-Änderung (S-1 bleibt `[VORLÄUFIG]`, Vermerk in `architecture.md` Abschnitt 9); Modulnamen unverändert; keine Blocker; Phase 1 weiter 7 Schritte.

### 2026-10-13 11:35 – [BEOBACHTUNG]

- **Spike 1.3 gemessen** (Container, 4 vCPU, ohne Grafikkarte): Übergabe 0,5–2,6 ms je Bild; Partikel statt Sprites nötig; Bauaktion erst im übernächsten Bild (27–37 ms) – Abweichung von `project-context.md` Abschnitt 6, Entscheidung in 1.7. Details im Messprotokoll `spikes/1.3-spielfeld/README.md`.
- **SwiftShader verfälscht alles, nicht nur die Bildrate:** Mit Software-Grafik stiegen Simulationsschritt (0,7 → 3,5 ms) und Rundlauf der Übergabe (0,7 → 20–90 ms), weil die Grafik dieselben 4 Kerne belegt. Gelöst durch eine Kontrollmessung ohne Zeichnen (`--draw=0`); erst sie zeigt die Übergabezeiten.
- **Gelöste Reibungen:** (1) Messreihe lieferte zunächst nur Nullen – Fehler im Renderer waren unsichtbar; Fehlerausgabe ins Protokoll eingebaut, Ursache CSP: der globale `unsafe-eval`-Build von PixiJS 8.20.1 patcht nur seine eigene Kopie von `ParticleBuffer` (Umgehung: eine Zeile). (2) Anfragen vor dem Anlegen der Simulation gingen verloren und blockierten die Schleife; Anfragen erst nach „ready". (3) Nahansicht zeigte 0 Personen, weil zu Spielbeginn alle am Eingang stehen; 3 s Zeitraffer und Kamera an der dichtesten Stelle. (4) Testkarte aus 1.2 stürzt bei 96 × 96 ab; Test auf 128 × 128.
- **Spike-Build 1.3** (Lauf 38389783470): Pakete und Rauchtest für Windows, macOS Universal und Linux beim ersten Lauf grün.

### 2026-10-13 11:10 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 10:40; PR Paddel87/OpenAirside#5 gemergt (`1c5c9fb`), Branch `claude/gracious-galileo-2reksz` neu auf `main` gesetzt.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant:** Schritt 1.3 – Zustandsübergabe und Spielfeld-Darstellung (Anweisung des Projektinhabers „weiter mit 1.3"). 1.6 bleibt `[IN ARBEIT]` aus der Vorsession und wartet nur auf die Rückmeldung des Projektinhabers vom Start auf seinen Rechnern; in dieser Session wird 1.6 nicht bearbeitet.
- **Vorabprüfung:** Eingangskriterium „Kern aus 1.2 lauffähig" erfüllt; S-1 ist `[VORLÄUFIG]`. Keine Kategorie aus `CLAUDE.md` Abschnitt 4 vorab berührt; PixiJS 8.20.1 ist fixierter Stack (ADR-002). Bildraten im Container (ohne Grafikkarte) nicht aussagekräftig – deshalb wird der Spike wie 1.1 als Testprogramm über die CI gebaut (Freigabe „CI baut fertige Testprogramme" aus 1.6).
- **Modus/Werkzeug:** Claude Code, Cloud-Session.
- **Modell:** eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse (Quelle `get_session`); 1.3 empfiehlt Entscheidung.
- **Kontextgröße zu Beginn:** rund 100.000 Token laut `get_session`.

### 2026-10-13 10:40 – [SESSIONENDE]

- **Session-Dauer:** ca. 25 Minuten Arbeitszeit seit `[SESSIONSTART]` 02:40 (mit Unterbrechung)
- **Bearbeitet:** 1.6 – Verpackung für drei Plattformen (vor 1.3 auf Wunsch des Projektinhabers)
- **Erreicht:** electron-builder 26.16.1 eingerichtet; Workflow `spike-build.yml` baut und startet Pakete für Windows, macOS Universal und Linux (Lauf 38383032090 grün); Testanleitung im README von Spike 1.1; Befund Linux-Sandbox mit Landeplatz 7.2.
- **Offen geblieben:** 1.6 bleibt `[IN ARBEIT]` bis zur Rückmeldung des Projektinhabers vom Start auf Windows-PC, Mac M3 Pro und Linux-Live-USB (insbesondere Verhalten von macOS ohne Signatur). CI-Lauf des letzten Dokumentations-Commits nicht abgewartet.
- **Nächster Schritt:** Rückmeldung zu 1.6 auswerten und 1.6 abschließen; danach 1.3 in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 1 Schritt oberhalb der Empfehlung (1.6 empfiehlt Routine); keine Abgabe (Probelauf offen).
- **Kontextgröße:** 520.846 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** README unverändert gültig (Phase 1, nächste Schritte); Fahrplan 1.6 und 7.2 nachgezogen; keine ADR- oder Reifegrad-Änderung (Verpackung bleibt `[VORLÄUFIG]` bis zum Abschluss von 1.6).

### 2026-10-13 10:35 – [BEOBACHTUNG]

- **Spike-Build, erster Lauf grün** (38383032090): Pakete für Windows, macOS Universal und Linux gebaut und im CI gestartet.
- **Linux-Sandbox:** Entpacktes Programm bricht mit Chromium-Sandbox ab (Exit 133, SUID-Hilfsprogramm nicht root-eigen); ohne Sandbox lauffähig. Landeplatz 7.2.
- **macOS:** Ad-hoc-Signatur vorhanden, obwohl electron-builder nicht signiert; Verhalten nach dem Herunterladen (Gatekeeper) nur auf echtem Mac prüfbar.
- **Xvfb:** Mit der Voreinstellung (8 Bit Farbtiefe) fiel die Bildrate lokal auf 0,3; mit `-screen 0 1280x800x24` normal. Im Workflow gesetzt.
- **npm audit** nach Einführung von electron-builder: 8 × moderate (`sprintf-js`), 0 × high – innerhalb der Schwelle aus `project-context.md` Abschnitt 7.

### 2026-10-13 02:40 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 02:35 einer anderen Session (Branch-Konvention, PR Paddel87/OpenAirside#4, gemergt als `04c7c5b`); Branch neu auf `main`.
- **Pflichtlektüre:** Änderungen seit dem letzten Lesen gezielt gelesen (`git diff 1353542 04c7c5b`): `CLAUDE.md` Abschnitt 11 (Branch-Schema, ein Branch je PR, vom Werkzeug vergebene Namen), `project-context.md` Abschnitt 7, README „Mitwirken", Logbuch 02:28/02:35. Übrige Pflicht-Dokumente unverändert seit dem Lesen in dieser Sitzung. Folge: Branch `claude/gracious-galileo-2reksz` ist vom Werkzeug vorgegeben, Typ und Schritt-ID stehen im PR-Titel.
- **Geplant:** Schritt 1.6 (Verpackung für drei Plattformen), damit die GPU-Messungen von 1.3 und 1.5 als fertige Programme auf den Rechnern des Projektinhabers laufen können. Reihenfolge vom Projektinhaber gewählt (Rückfrage zu 1.3: Container ohne Grafikkarte, Bildrate hier nicht aussagekräftig).
- **Freigabe CI-Änderung:** Die gewählte Option nennt ausdrücklich, dass die CI fertige Testprogramme baut (Kategorie 7, Abschnitt 4).
- **Modell:** `claude-opus-5-5` (Entscheidungs-Klasse, `get_session` 02:2x). 1.6 empfiehlt Routine → läuft oberhalb der Empfehlung; keine Abgabe möglich (Probelauf offen).
- **Kontextgröße zu Beginn:** rund 460.000 Token, über der Grenze von 400.000; Fortsetzung auf Wunsch des Projektinhabers.

### 2026-10-13 02:35 – [SESSIONENDE]

- **Session-Dauer:** ca. 10 Minuten
- **Bearbeitet:** Übernahme der Branch-Konvention aus Dev-Templates (Commit `046eecd`, PR Paddel87/Dev-Templates#50) auf Anweisung des Projektinhabers; kein Fahrplan-Schritt (Methodik-Pflege).
- **Erreicht:** `CLAUDE.md` Abschnitt 11 und `templates/docs/project-context.md` wieder identisch mit Dev-Templates; `project-context.md` Abschnitt 7 um Branch-Präfixe und vom Werkzeug vergebene Namen ergänzt; README „Mitwirken" nachgezogen. Branch `docs/branch-konvention` nach dem neuen Schema.
- **Offen geblieben:** PR wartet auf Review und Merge.
- **Nächster Schritt:** unverändert 1.3 (parallel 1.6), in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); Arbeit wäre Routine-Klasse (Dokumentationspflege) – 1 Schritt oberhalb der Empfehlung, keine Abgabe (Probelauf für Unteragenten offen).
- **Kontextgröße:** von `get_session` zu Sessionbeginn als 0 gemeldet, nicht verlässlich feststellbar; deutlich unter der Grenze von 400.000.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** README „Mitwirken" an `CLAUDE.md` §11 angeglichen; sonst keine Pflicht-Dokumente berührt.

### 2026-10-13 02:28 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 02:25; PR #3 gemergt (`1353542`).
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker, Requirements-Übersicht).
- **Geplant für diese Session:** Branch-Konvention aus Dev-Templates übernehmen (Anweisung des Projektinhabers).
- **Vorabprüfung:** reine Dokumentations- und Methodik-Pflege, keine Kategorie aus `CLAUDE.md` Abschnitt 4 berührt; keine aktiven Blocker.
- **Modus/Werkzeug:** Claude Code, Cloud-Session mit Dev-Templates und OpenAirside.
- **Modell:** eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse (Quelle `get_session`).

### 2026-10-13 02:25 – [SESSIONENDE]

- **Session-Dauer:** ca. 20 Minuten nach dem `[SESSIONENDE]` von 02:05 (dieselbe Sitzung)
- **Bearbeitet:** Merge von PR Paddel87/OpenAirside#2 auf Anweisung; Einschätzung zur Umsetzbarkeit (keine Dokumentänderung); Testsysteme für 1.5 im Fahrplan vermerkt.
- **Erreicht:** Testsysteme für 1.5 geklärt: Referenz-PC mit AMD Radeon RX 7700 XT (Windows, Linux per Live-USB), Mac M3 Pro mit 18 GB.
- **Offen geblieben:** Untergrenze „integrierte Grafik" ohne schwaches Testgerät nur hochrechenbar; Intel-Teil des macOS-Universal-Builds ungetestet; Treiberunterstützung RX 7700 XT unter Linux in 1.5 gegen Quellen belegen.
- **Nächster Schritt:** 1.3 (parallel 1.6) in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 0 Schritte oberhalb der Empfehlung; keine Abgaben.
- **Kontextgröße:** 456.458 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** nur Fahrplan-Notizen zu 1.5 geändert, keine Auswirkung auf README oder andere Pflicht-Dokumente.

### 2026-10-13 02:05 – [SESSIONENDE]

- **Session-Dauer:** ca. 60 Minuten seit `[SESSIONSTART]` 01:10
- **Bearbeitet:** 1.2 (Prototyp, Messreihe, Entscheidungsvorlage, ADR-010); Auslagerung der ADRs nach `docs/decisions/` (ADR-001: ab zweistelliger Anzahl)
- **Erreicht:** 1.2 `[ERLEDIGT]` nach Freigabe „A, freigegeben, Referenzszenario mit Reserve"; Datenmodell `[OFFEN]` → `[VORLÄUFIG]`; Wegfindung bleibt `[VORLÄUFIG]` bis 1.5; 1.5 um Rastergröße und fünf Etagen ergänzt; Landeplatz für „Suchen ohne Schilder"/Feinwege: 3.3.
- **Offen geblieben:** PR Paddel87/OpenAirside#2 wartet auf Review und Merge (CI auf `2befa46` grün, Code-Jobs bis 2.1 übersprungen); `shellcheck` im Container nicht installiert, `run-all.sh` des Spikes daher ungeprüft (Spike-Skript, nicht in `scripts/`).
- **Nächster Schritt:** 1.3 – Zustandsübergabe und Spielfeld-Darstellung (parallel möglich 1.6), in neuer Session.
- **Modell-Bilanz:** aktive Klasse Entscheidung (`claude-opus-5-5`, eingestellt und bedient laut `get_session` am Sessionende); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben.
- **Kontextgröße:** 440.467 Token laut `get_session` – **über der Grenze von 400.000** (`project-context.md` Abschnitt 6). Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Drift-Prüfung:** README (Phase, Architektur-Reife, Nächste Schritte) nachgezogen; ADR-010 verweist auf 1.2, 1.5 (existieren); Reifegrade passen zu ADR-009/010; Reaktiv-Quote 0/10; Phase 1 weiter 7 Schritte.
- **Archivierung:** Teil B von `decisions.md` in Einzeldateien `docs/decisions/ADR-001.md` bis `ADR-010.md` ausgelagert; Logbuch unter 800 Zeilen.

### 2026-10-13 02:00 – [ADR-ANGELEGT]

- **ADR:** ADR-010 „Datenmodell und Parallelisierung im Kern, Übernahme der Wegweiser" (`docs/decisions/ADR-010.md`)
- **Tag:** `[ERKENNTNIS]`; Reaktiv-Quote 0/10
- **Auslöser:** Freigabe „A, freigegeben, Referenzszenario mit Reserve"

### 2026-10-13 02:00 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Datenmodell und Parallelisierung im Kern
- **Wechsel:** `[OFFEN]` → `[VORLÄUFIG]` (Wegfindung bleibt `[VORLÄUFIG]`: fünf Etagen und Rastergröße erst in 1.5)
- **Auslöser:** Spike 1.2, ADR-010
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja

### 2026-10-13 01:50 – [BEOBACHTUNG]

- **Spike 1.2 abgeschlossen bis zur Entscheidung:** Messreihe in `spikes/1.2-wegweiser/results/`, Auswertung in dessen README. Vorlage im Fahrplan 1.2.
- **Inkrementelle Reparatur der Wegweiser** liefert exakt dieselben Distanzen wie die Vollberechnung (Test), ist aber beim Schließen und Öffnen von Türen langsamer, weil Türen Engstellen sind. Erster Versuch scheiterte am Test: Neue Wände verbieten auch diagonale Schritte der Nachbarzellen (Eckregel); Lösung: Nachbarn geänderter Zellen als zusätzliche Wurzeln.
- **Informationslücke:** Rastergröße des Referenzszenarios und der größten Karte ist in keinem Dokument festgelegt; als Frage in die Entscheidungsvorlage aufgenommen. Spike lief mit 2 statt 5 Etagen (BAU-07).
- **Widerspruch erkannt, nicht aufgelöst:** Determinismus (Architektur-Ziel) gegen „Bauen ohne Wartezeit" (Vision) bei langen Neuberechnungen – Teil der Entscheidungsvorlage.

### 2026-10-13 01:10 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 01:05; PR Paddel87/OpenAirside#1 gemergt (`5813931`), Branch auf `main` vorgespult.
- **Pflichtlektüre:** Die Dokumente auf `main` unterscheiden sich nicht vom Stand des letzten Commits dieser Sitzung (`git diff 1c9da21 HEAD` leer); die in derselben Sitzung gelesenen und selbst geschriebenen Abschnitte sind damit aktuell. Kein neues Lesen nötig.
- **Geplant für diese Session:** Schritt 1.2 – Wegweiser-Prinzip mit 10.000 Personen (Prototyp, Zeitbox zwei Sessions).
- **Vorabprüfung:** 1.2 ist ERKUNDUNG; Eingangskriterium „Betriebsart aus 1.1 entschieden" erfüllt (ADR-009). Wegfindung `[VORLÄUFIG]`, Datenmodell `[OFFEN]` – Gegenstand des Schritts. Keine aktiven Blocker.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (4 vCPU, keine GPU), Branch `claude/gracious-galileo-2reksz`; Fortsetzung derselben Sitzung auf Wunsch des Projektinhabers.
- **Modell:** eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse (Quelle `get_session`, 00:3x). Empfohlene Klasse 1.2: Entscheidung – passt.
- **Kontextgröße zu Beginn:** rund 310.000 Token; die Grenze von 400.000 wird in diesem Schritt voraussichtlich überschritten.

### 2026-10-13 01:05 – [SESSIONENDE]

- **Session-Dauer:** ca. 65 Minuten (ersetzt den vorläufigen Eintrag von 00:45, nach dem die Session für die Freigabe offen blieb)
- **Bearbeitet:** 1.1 (Spike, Messreihe, Entscheidungsvorlage, ADR-009), Q.2 neu angelegt
- **Erreicht:** 1.1 `[ERLEDIGT]`: Kern als napi-rs-Modul im Electron-Utility-Prozess (ADR-009, Regel-006); Betriebsart `[OFFEN]` → `[VORLÄUFIG]`; WebAssembly mit Threads zurückgestellt nach Q.2 (Auslegung vom Projektinhaber bestätigt). CI-Läufe 4 und 5 grün.
- **Offen geblieben:** nichts aus 1.1. Spike-Code wird von der CI bis 2.1 nicht gebaut (nur Dokumentations-Prüfung).
- **Nächster Schritt:** 1.2 – Wegweiser-Prinzip mit 10.000 Personen (Entscheidungs-Klasse, bis zu zwei Sessions); parallel möglich 1.6. Empfehlung: in neuer Session beginnen.
- **Modell-Bilanz:** aktive Klasse Entscheidung (eingestellt und bedient `claude-opus-5-5`, Quelle `get_session` am Sessionende); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben (Probelauf für Unteragenten offen).
- **Kontextgröße:** 307.567 Token laut `get_session` (`context_usage`), unter der Grenze von 400.000; kein Hinweis nötig.
- **Ablaufdaten-Prüfung:** kein Eintrag hat seinen Vorlauf erreicht; neu: Nachprüfung WebAssembly-Threads (2027-01-02, Q.2).
- **Drift-Prüfung:** README (Phase, Architektur-Reife, Nächste Schritte, Blocker 0) synchron; ADR-009 verweist auf existierende Schritte 1.3, 1.5, 1.6, Q.2; Reifegrad Betriebsart passt zu ADR-009; Modulnamen in 1.1/1.3/Q.2 wie in der Modul-Karte; Reaktiv-Quote 0/9; Phase 1 weiter 7 Schritte (Q.2 ist Querschnitt).
- **Archivierung/Budgets:** Logbuch 146 Zeilen, `project-context.md` 352 Zeilen – keine Trigger.
- **Uncommitted:** nichts.

### 2026-10-13 01:00 – [BEOBACHTUNG]

- **CI für Schritt 1.1:** Läufe 4 (`865b25f`) und 5 (`d912570`) grün. Aktiv sind bis Schritt 2.1 nur „Dokumentation (pre-commit)" und „Projektteile erkennen"; die Code-Jobs werden mangels `Cargo.toml`/`package.json` am Repo-Stamm übersprungen, der Spike-Code wird in der CI also nicht gebaut. Einzige Annotation: der bekannte Hinweis zur Umstellung von `ubuntu-latest` auf Ubuntu 26 ab 2026-10-19 (Ablaufdaten-Register, Q.1).

### 2026-10-13 00:55 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Betriebsart des Rust-Kerns
- **Wechsel:** `[OFFEN]` → `[VORLÄUFIG]`
- **Auslöser:** Spike 1.1 und ADR-009; `[BELASTBAR]` erst nach dem Leistungsnachweis 1.5
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja

### 2026-10-13 00:55 – [ADR-ANGELEGT]

- **ADR:** ADR-009 „Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess“, dazu Regel-006 „Kern frei von Anbindungscode“
- **Tag:** `[ERKENNTNIS]` – klärt einen in ADR-002 offen gelassenen Punkt, ändert ADR-002 nicht; Reaktiv-Quote 0/9
- **Auslöser:** Freigabe des Projektinhabers „A, freigegeben, B perspektivisch“. „Perspektivisch“ ausgelegt als: B wird wieder geprüft, sobald WebAssembly-Threads mit stabilem Rust baubar sind → Landeplatz Q.2, Nachprüfung im Ablaufdaten-Register (erstmals 2027-01-02). Auslegung vom Projektinhaber am 2026-10-13 bestätigt („genau das meinte ich“).
- **Modell:** weiterhin `claude-opus-5-5` (Entscheidungs-Klasse), erneut über `get_session` abgefragt; Kontext 267.791 Token laut Laufzeitumgebung (unter der Grenze von 400.000).

### 2026-10-13 00:45 – [SESSIONENDE] (vorläufig, Session blieb für die Freigabe offen)

- **Session-Dauer:** ca. 45 Minuten
- **Bearbeitet:** Schritt 1.1 (Vergleichsstudie napi-rs gegen WebAssembly)
- **Erreicht:** Spike `spikes/1.1-betriebsart/` mit gemeinsamem Kern, napi-Anbindung, WebAssembly ein- und mehrthreadig und gehärteter Electron-Messumgebung; Messreihe mit 17 Konfigurationen × 2 Wiederholungen plus native Grundlinie; Messprotokoll in `spikes/1.1-betriebsart/README.md`. Entscheidungsvorlage im Fahrplan, Schritt 1.1 auf `WARTET-AUF-FREIGABE`.
- **Offen geblieben:** Freigabe der Betriebsart; danach ADR (ERKENNTNIS), `architecture.md` (Betriebsart `[OFFEN]` → `[VORLÄUFIG]`, S-1 je Betriebsart) und Abschluss von 1.1.
- **Nächster Schritt:** Antwort auf die Entscheidungsvorlage 1.1, danach 1.2.
- **Modell-Bilanz:** aktive Klasse Entscheidung (eingestellt und bedient `claude-opus-5-5`, Quelle `get_session`); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben (Probelauf für Unteragenten offen).
- **Kontextgröße:** nicht feststellbar – `get_session` meldet `context_usage.used_tokens: 0`, offensichtlich nicht aktuell; Hinweis entfällt.
- **Ablaufdaten-Prüfung:** Kein Eintrag hat seinen Vorlauf erreicht (Electron 44: Vorlauf ab 2027-01-02). Zeile „Claude-Kontingent“ in `project-context.md` Abschnitt 8 mit den Angaben der Laufzeitumgebung gefüllt.
- **Drift-Prüfung:** README (Phase, Nächste Schritte) nachgezogen; Modul-Liste, Blocker (0), Reaktiv-Quote (0/8) unverändert; kein neuer ADR; Phasenumfang 7 Schritte unverändert.
- **Hinweis:** Die Session bleibt für die Antwort offen; wird sie fortgesetzt, folgt kein neuer `[SESSIONSTART]`.

### 2026-10-13 00:35 – [BEOBACHTUNG]

- **WebAssembly mit Threads braucht Nightly-Rust.** Stabiles Rust 1.98.1 akzeptiert `-C target-feature=+atomics` nur mit der Warnung „unstable feature“; die mitgelieferte Standardbibliothek ist nicht threadfähig. Die wasm-bindgen-Doku („Parallel Raytracing“) nennt Nightly und `-Z build-std`. Im Spike mit `nightly-2026-10-12` gebaut, nur für die Messung.
- **Determinismus nativ = WebAssembly:** bitgleiche Prüfsummen über nativ (1/2/4 Threads), wasm ein- und mehrthreadig, solange nur IEEE-exakte Operationen vorkommen. Relevant für 1.2 und spätere Spielstände.
- **Abweichung Werkzeugversion:** Der Container hat Node 22.22.0 statt 26.11.1 (`project-context.md` Abschnitt 3); betrifft nur npm und Hilfsskripte des Spikes, Electron bringt eigenes Node 24 mit.
- **Code-Hooks:** Die lokalen pre-commit-Code-Hooks (cargo fmt/clippy am Repo-Wurzel, Prettier/ESLint über die Projekt-`package.json`) setzen Schritt 2.1 voraus. Im Spike liefen `cargo fmt` und `cargo clippy -D warnings` (für `sim-wasm` zusätzlich `undocumented_unsafe_blocks`) direkt im Spike-Verzeichnis, die übrigen Hooks über `pre-commit run`: Dokumentations-Hooks und Prettier grün (Prettier lief als im Container vorhandene 3.8.1, nicht als Projekt-Version 3.9.10, die erst mit 2.1 installiert wird); ESLint ohne Projektkonfiguration nicht anwendbar (wie in der CI bis 2.1 übersprungen).

### 2026-10-13 00:25 – [PROBLEM-GELÖST]

- **Kontext:** napi-Variante in Electron
- **Symptom:** Lauf endete im Timeout ohne Fehlermeldung.
- **Ursache:** Der Renderer registrierte den Listener für den MessagePort erst nach dem Warten auf die Konfiguration; der Hauptprozess schickt den Port aber zuerst, die Nachricht ging verloren.
- **Lösung:** Listener vor dem ersten `await` registrieren (`app/renderer/main.js`).
- **Aufwand:** 5 Minuten
- **Lerneffekt:** In Phase 2 die Port-Übergabe als Handshake mit Bestätigung bauen, nicht auf die Reihenfolge verlassen.

### 2026-10-13 00:15 – [PROBLEM-GELÖST]

- **Kontext:** Build von `sim-napi` mit den in ADR-002 fixierten Versionen (napi 3.12.7, napi-derive 3.6.8)
- **Symptom:** Kompilierfehler in `napi-derive` („`?` couldn't convert the error to `Diagnostic`“).
- **Ursache:** Cargo wählte die neueste Unterabhängigkeit `napi-derive-backend` 6.1.5 (2026-10-05), die mit napi-derive 3.6.8 nicht kompatibel ist (Bruch in einer Patch-Version).
- **Lösung:** `cargo update -p napi-derive-backend --precise 6.1.4` (erschienen am selben Tag wie napi-derive 3.6.8). Begleit-Crate `napi-build` 2.4.4 gewählt (napi 3.12.7 verlangt `^2.4.4`; Linie 2.4 seit 2026-07-28, mit Patches).
- **Aufwand:** 5 Minuten
- **Wiederkehrgefahr:** hoch – napi-rs veröffentlicht mehrmals pro Woche; `Cargo.lock` einchecken und Aktualisierungen bewusst in Q.1 durchführen.

### 2026-10-13 00:07 – [PROBLEM-GELÖST]

- **Kontext:** Electron-Start im Cloud-Container
- **Symptom:** `Running as root without --no-sandbox is not supported`, Abbruch mit SIGTRAP.
- **Ursache:** Der Container läuft als root; Chromium verweigert die OS-Sandbox als root.
- **Lösung:** Messläufe mit `--no-sandbox` (nur Messumgebung; `sandbox: true` des Renderers bleibt gesetzt).
- **Aufwand:** 2 Minuten

### 2026-10-13 00:05 – [SESSIONSTART]

- **Letzter Stand:** Initialisierung abgeschlossen (Commit `e130cd4`), danach CI-Anpassung `b6fb933`. Kein `[SESSIONENDE]`-Eintrag vorhanden (erste reguläre Session).
- **Geplant für diese Session:** Schritt 1.1 – Betriebsart des Rust-Kerns (Vergleichsstudie napi-rs gegen WebAssembly), Ergebnis als Entscheidungsvorlage.
- **Vorabprüfung:** 1.1 ist ERKUNDUNG, keine Eingangskriterien auf `[BELASTBAR]` nötig. Betriebsart `[OFFEN]` – genau das ist Gegenstand des Schritts. Keine aktiven Blocker.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Linux-Container, 4 CPU-Kerne, 15 GB RAM, keine GPU), Branch `claude/gracious-galileo-2reksz`.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` → Entscheidungs-Klasse. Quelle: Laufzeitabfrage `get_session` (`configured_model`, `last_served_model`). Empfohlene Klasse für 1.1: Entscheidung – passt.
- **Kontingent:** Laufzeitumgebung meldet Art „five_hour" (rollierendes 5-Stunden-Fenster), Status „allowed", nächstes Zurücksetzen 2026-10-13 03:00 UTC. Quelle: `get_session` (`rate_limit_info`).

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
