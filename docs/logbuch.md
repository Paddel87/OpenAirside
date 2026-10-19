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

Phase 1 abgeschlossen (ADR-016), Detail-Einträge im Archiv `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) läuft: 2.1 erledigt (ADR-018, ADR-019) mit Nachlauf; 2.8 erledigt (ADR-020); 2.9 erledigt (ADR-021); 2.2 erledigt (ADR-022); 2.3 erledigt (ADR-023); 2.7 erledigt (ADR-024, Messung auf Mac und Referenz-PC 2026-10-15); 2.4 erledigt (ADR-025, ADR-026, 2026-10-18); 2.5 erledigt (ADR-027 bis ADR-030, 2026-10-19). Neuer Entwurfsschritt 3.7 vor 3.1. Reflexion Reaktiv-Quote (Anlass ADR-020) zur Kenntnis genommen.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-19 18:37 (UTC) – [BEOBACHTUNG]

- **Messung 2.10, Mac-Lauf 1 (Standardaufteilung 2 + 8) eingegangen:** abgelegt als `docs/research/2.10-ergebnisse/mac-m3pro-standard.json`. 119,1–120,1 Bilder/s bei 120 Hz mit Szene (20.000 Objekte, 64 Zonen, 10.000 Personen); neu zwei lange Aufgaben im Renderer (131 und 83 ms); Schrittzeit-Ausreißer wie in 2.7 (bis 75 ms, 11 von 20 Fenstern über 4 ms), Minimum 180 Ticks/s in allen Fenstern. Auswertung in `docs/research/2.10-messung.md`. Ausstehend: Mac-Lauf 2 (`--background-threads=4`) und Referenz-PC.

### 2026-10-19 17:10 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 52 Minuten seit `[SESSIONSTART]` 16:18 UTC.
- **Bearbeitet:** Übernahme des Merge-Vermerks auf `feat/2-10-stilleitfaden-svg-pipeline`; Schritt 2.10: Vorlage des Rasterers, Freigabe A, ADR-033, Werkzeug-Crate `tools/assets`, Stilleitfaden mit Palette, SVG-Quellen (Bank, Betonboden, Betonwand, Ersatzgrafik, Programmsymbol), Prüfer `scripts/assets.mjs`, Atlanten im Spielfeld, Szene im Messwerkzeug, Fenstersymbol unter Linux, Rauchtest, Messanleitung, Dokumentation.
- **Erreicht:** Pipeline umgesetzt und auf drei Plattformen grün (CI-Lauf 39290398392, 17 Jobs; Release-Lauf 39291102397 mit Paketen); Grafik-Pipeline `[BELASTBAR]`; Coverage: Vitest gesamt 86,5 % Lines (`atlas.ts` 100 %, `measure.ts` 98,0 %, `assets.mjs` 93,6 %), Rust gesamt und Kern über den Schwellen (Kern 98,4 %), `tools/assets/src/lib.rs` 96,0 % Lines; Onboarding im frischen Worktree validiert. Commits `f975893` … `62f3ac0` plus dieser Sessionende-Commit.
- **Offen geblieben:** 2.10 bleibt `[IN ARBEIT]`: Messung auf Referenz-PC und Mac (je ein Lauf, Mac zusätzlich mit `--background-threads=4`) durch den Projektinhaber nach `docs/research/2.10-messung.md`, danach Auswertung und ADR zur NFR Darstellung. Kein PR vor dem Abschluss des Schritts. Abkündigung `rustup show` wartet auf Q.1 (freigabepflichtig).
- **Nächster Schritt:** Ergebnisdateien der Messung auswerten (gegen 2.7), ADR zur NFR Darstellung, 2.10 abschließen, PR.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 16:18 und 17:10 UTC); 2.10 empfiehlt die Entscheidungs-Klasse, 0 Schritte oberhalb der Empfehlung; Routine-Teilarbeiten (Logbuch, README) nicht abgegeben, Probelauf offen.
- **Kontextgröße:** über die Laufzeitumgebung nicht feststellbar (`get_session` meldet `used_tokens` 0); kein Hinweis möglich.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 17:10 UTC.
- **Ablaufdaten-Prüfung:** neuer Eintrag `rustup show` (Q.1); kein Eintrag mit erreichtem Vorlauf ohne Schritt.
- **Archivierung:** Logbuch über 800 Zeilen, alle Einträge aus Oktober 2026 – nichts auszulagern; Verdichtung mit dem `[PHASEN-WECHSEL]` nach Phase 2.
- **Drift-Prüfung:** README nachgezogen (Verwendung: Grafiken, `measure` mit Szene; Architektur-Reife: Grafik-Pipeline); Nächste Schritte unverändert (2.10 läuft). ADR-033 verweist auf vorhandene Schritte (2.10, Q.1). Reifegrad Grafik-Pipeline passt zu ADR-008/ADR-033 und Umsetzung. Modul-Liste unverändert (`tools/assets` ist Werkzeug des Moduls Inhalte & Mods, `atlas.ts` gehört zum Spielfeld). FR-050 verweist auf 2.10. Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker. Reaktiv-Quote 7 / 10 (ADR-024 bis ADR-033) stimmt mit den Tags. Phase 2: 11 Schritte bei Plan 10. Größen: `project-context.md` unter 400 Zeilen.

### 2026-10-19 17:04 (UTC) – [REIFEGRAD-WECHSEL]

- **Grafik-Pipeline (SVG → Atlanten):** `[VORLÄUFIG]` (ADR-008) → `[BELASTBAR]` durch funktionierende Umsetzung in 2.10 (Beförderungsregel): Werkzeug und Prüfer mit Tests, CI-Lauf 39290398392 auf `5862b4e` grün auf drei Plattformen (17 Jobs), Rauchtest lädt die Atlanten unter der CSP. NFR Darstellung mit echten Grafiken bleibt `[VORLÄUFIG]` bis zur Messung.
- **CI-Protokoll gelesen (Warnungsquellen ohne Schalter):** neu `rustup show` „auto-installation is deprecated“ (rustup#4836) – Ablaufdaten-Register und Q.1 ergänzt (Umstellung freigabepflichtig, Kategorie 7); bekannt: Hinweis Ubuntu 26, `npm warn deprecated` (electron-builder-Unterpakete), `install-scripts` electron-winstaller, 8 × moderate; neu, nur Hinweis der Plattform: Wartezeiten der macOS-arm64-Runner.

### 2026-10-19 17:02 (UTC) – [ONBOARDING-VALIDATION]

- **Anlass:** `package.json` (`build` mit neuem Schritt `build:assets`) und Runbook geändert – Quick-Start-relevant. Frischer Worktree von `5862b4e` im Scratchpad: `npm ci`, `npm run build` (Kern, Brücke, Atlanten, Hauptprozess, Renderer) und Rauchtest `xvfb-run -a npx playwright test` grün (1 passed); Atlanten und `build/icon.png` entstehen ohne weitere Voraussetzung. Worktree danach entfernt.

### 2026-10-19 17:00 (UTC) – [PROBLEM-GELÖST]

- **CI rot auf macOS und Windows (Lauf 39289876778, Job „Rust Tests"):** Der Test `reports_bad_inputs_with_their_file` legte `Bench.svg` neben `bench.svg` an; auf Dateisystemen ohne Unterscheidung von Groß- und Kleinschreibung ist das dieselbe Datei, das Werkzeug fand also keinen ungültigen Namen. Fehler im Test, nicht im Werkzeug. Behoben mit `bench.old.svg` (Commit `5862b4e`).
- **Mini-Reibung, zwei Entwurfsfehler vor dem Commit bemerkt:** (1) Die SVG-Quellen lagen zuerst in `content/base/graphics/` – das hätte das Paketformat S-4 um einen Ordner erweitert und die Quellen ausgeliefert; verlegt nach `assets/graphics/base/`. (2) Die Bank war zuerst 4 × 2 Zellen groß gezeichnet; `width`/`depth` im Paket sind Meter (ADR-020), richtig sind 2 × 1 Zellen. Der Prüfer vergleicht seitdem die Größe mit der Grundfläche.
- **Laden ohne CSP-Änderung:** PixiJS `Assets` lädt über `fetch`, die CSP hat `connect-src 'none'` (Kategorie 6). Stattdessen Bildliste per Vite-Import im Bündel, Seiten als `<img>` unter `img-src 'self'`.

### 2026-10-19 16:59 (UTC) – [BEOBACHTUNG]

- **Probelauf der Messung im Container:** `measure 1` mit Szene (20.000 Objekte, 200 mit Ersatzgrafik, 64 Zonen) läuft durch, Ergebnisdatei mit `series.scene` und `tierPx`; einmal über Canvas (ohne WebGL), einmal über WebGL2/SwiftShader. Bildraten ohne Aussage (Software-Grafik auf 4 vCPU), Tabelle in `docs/research/2.10-messung.md`.
- **Uhrzeit wieder geschätzt:** Die Vorlage stand zunächst mit 16:31 UTC in Fahrplan und Logbuch, `date -u` meldete 16:22; vor dem Commit korrigiert. Die Lehre aus 15:28 wirkt noch nicht zuverlässig: Uhrzeit jetzt immer im selben Befehl wie die Änderung holen.

### 2026-10-19 16:29 (UTC) – [ADR-ANGELEGT]

- **ADR-033** `[OPERATIV]` „SVG-Rasterer resvg im Werkzeug-Crate `tools/assets`": freigegeben vom Projektinhaber per Auswahl A (resvg 0.48.1, Rust). Benannte Ausnahme `miniz_oxide@0.8.9` im Warnungs-Bestand. Reaktiv-Quote 7 / 10 über ADR-024 bis ADR-033, unverändert über dem Schwellenwert (kein neuer `[REAKTIV]`-ADR, Reflexion zu ADR-032 gilt).

### 2026-10-19 16:22 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 2.10, SVG-Rasterer (Kategorie 3, mit Kategorie 7):** Optionen A resvg 0.48.1 als Rust-Werkzeug-Crate im Workspace, B @resvg/resvg-js 2.6.2 (npm), C Rasterung in Electron/Chromium; Empfehlung A. Belege aus crates.io und npm (Abruf 16:19–16:21 UTC) und einem Probe-Projekt im Scratchpad: 37 Crates ohne Standard-Features, alle Lizenzen erlaubt, `miniz_oxide` doppelt (0.8.9 über png 0.18.1, 0.9.1 über flate2) – benannte Ausnahme nötig. sharp verworfen (libvips-Binärdateien unter LGPL-3.0).
- **Mini-Reibung:** Die Einrichtung im Hintergrund brach bei `rustup component add` ab, weil ein gleichzeitiges `cargo --version` die Toolchain aus `rust-toolchain.toml` installierte (rustup-Sperre); einzeln wiederholt, grün. Lehre: Toolchain-Aufrufe während der Einrichtung nicht parallel starten.

### 2026-10-19 16:18 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 15:30 UTC plus `[BEOBACHTUNG]` 16:13 (Merge PR #24, `2a464bf`). Der Merge-Vermerk `de7eabb` nach `project-context.md` Abschnitt 10 auf den neuen Branch `feat/2-10-stilleitfaden-svg-pipeline` vom aktuellen `main` übernommen; `erkundung/2-11-sammelpruefung-vertragssaetze` wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht). Vertiefung für 2.10: ADR-008.
- **Geplant für diese Session:** Schritt 2.10 (Stilleitfaden, SVG-Pipeline, Messung): Versionen des SVG-Rasterers nach Regel-001 belegen und als `ENTSCHEIDUNG ERFORDERLICH` (Kategorie 3) vorlegen; erst nach Freigabe ADR und Code; Messwerkzeug und Ablauf so vorbereiten, dass der Projektinhaber auf Referenz-PC und Mac nur starten und die JSON-Datei zurückgeben muss.
- **Vorabprüfung:** S-1 `[BELASTBAR]`, Spielfeld-Grundgerüst aus 2.7 vorhanden; Weltdarstellung `[OFFEN]` (3.8) – das Messwerkzeug platziert die Sprites selbst (ADR-032, L-4), 2.10 hängt also nicht vom offenen Bestandteil ab (Regel-007). Reaktiv-Quote 7 / 10, Maßnahmen (7)–(9) gelten.
- **Modus/Werkzeug:** semi-autonom, Claude Code in einer Cloud-Session.
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Entscheidungs-Klasse), Quelle `get_session` 16:18 UTC (`configured_model`, `session_context.model`, `last_served_model`); Kontingent 5-Stunden-Fenster `allowed`.

### 2026-10-19 16:13 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #24 (Schritt 2.11) auf Anweisung des Projektinhabers („Kenntnis + PR bei grün“) nach `main` gemergt, Merge-Commit `2a464bf`; vorher grün auf `a96a040`: Push-Lauf 39278499215, PR-Läufe CI 39278544213 und Security 39278543616. Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf dem Arbeits-Branch und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 15:30 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 16 Minuten seit `[SESSIONSTART]` 15:14 UTC.
- **Bearbeitet:** Übernahme des Sessionende-Commits `1816efb` auf `erkundung/2-11-sammelpruefung-vertragssaetze`; Schritt 2.11 vollständig: Prüfliste (81 Sätze), Vorlage, Freigabe „Alle Empfehlungen“, ADR-032, Fortschreibung von `architecture.md` (Modul-Karte, Grundlagen, S-1 bis S-7, neu S-8), Landeplätze im Fahrplan, Reflexion Reaktiv-Quote mit Kenntnisnahme.
- **Erreicht:** 2.11 `[ERLEDIGT]`; S-8 `[BELASTBAR]`; Weltdarstellung `[OFFEN]` mit Entwurfsschritt 3.8. Commits `aa0a7da`, `ae2f751`, `aca6ac5` plus dieser Sessionende-Commit; CI auf `ae2f751` grün (Lauf 39276970557), auf `aca6ac5` gestartet (Lauf 39278162416); PR nach Anweisung „Merge bei grün“.
- **Offen geblieben:** ungemergte fremde Branches und die alten Branches `feat/2-5-spielstand-mit-autosave` und `feat/2-6-lokalisierung` (gemergt, nicht weiterverwendet; Löschen nur auf Anweisung). Die Mermaid-Modul-Karte wurde nur per markdownlint geprüft, nicht gerendert; Sichtprüfung auf GitHub empfohlen.
- **Nächster Schritt:** 2.10 (Stilleitfaden, SVG-Pipeline und Messung der Darstellung; Bildname = Objekt-ID nach ADR-032) in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 15:13 und 15:29 UTC); 2.11 empfiehlt die Entscheidungs-Klasse, 0 Schritte oberhalb der Empfehlung (Logbuch- und README-Pflege als Routine-Teilarbeit nicht abgegeben, Probelauf offen).
- **Kontextgröße:** über die Laufzeitumgebung nicht feststellbar (`get_session` meldet `used_tokens` 0); kein Hinweis möglich.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 15:29 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Archivierung:** Logbuch über 800 Zeilen, alle Einträge aus Oktober 2026 – nichts auszulagern (wie 2026-10-19 12:48); Verdichtung mit dem `[PHASEN-WECHSEL]` nach Phase 2.
- **Drift-Prüfung:** README nachgezogen (Phase, Architektur-Reife, Skizze mit Plattform ↔ Kern, Nächste Schritte). ADR-032 verweist auf vorhandene Schritte (2.10, 2.11, 3.1, 3.6, 3.8, 5.1, 5.6, 6.1). Reifegrade S-8 und Weltdarstellung passen zu ADR-032. Modul-Liste unverändert (Benchmark & Leistung mit TypeScript-Teil ist dasselbe Modul). FR-010 (bei 3.8 vorbereitend) existiert. Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker. Reaktiv-Quote 7 / 10 (ADR-023 bis ADR-032) stimmt mit den Tags. Phase 2: 11 Schritte bei Plan 10 (unter der Schwelle); Phase 3 noch ohne festgehaltenen Schrittplan (Verfeinerung vor Phasenbeginn). Größen: `project-context.md` 369 Zeilen.

### 2026-10-19 15:28 (UTC) – [BEOBACHTUNG]

- **Uhrzeit geschätzt statt gemessen:** Die Vorlage von 2.11 stand zunächst mit 15:50 UTC in Logbuch, Fahrplan und ADR-032; tatsächlich 15:19 UTC (Commit `ae2f751`). Beim Fortschreiben bemerkt und korrigiert. Lehre: Zeitangaben in Einträgen immer mit `date -u` holen.
- **Hooks beim ersten Commit nicht installiert:** `ae2f751` entstand vor `pre-commit install` (Container ohne Einrichtung); die Hooks liefen danach über die drei Dateien grün (markdownlint und allgemeine Prüfungen). Seitdem aktiv.

### 2026-10-19 15:27 (UTC) – [REIFEGRAD-WECHSEL]

- **Neu `[BELASTBAR]`:** S-8 Kern-Prozess-Steuerung (Plattform ↔ Kern-Prozess) per ADR-032; beschreibt den in 2.4–2.6 umgesetzten, mit Tests und Rauchtest auf drei Plattformen geprüften Kanal, keine Verhaltensänderung.
- **Neu `[OFFEN]`:** Weltdarstellung im Spielfeld (Raster, Bauten, Zonen) – Entwurfsschritt 3.8 vor 3.1.
- S-1 bis S-7 bleiben `[BELASTBAR]`, fortgeschrieben durch ADR-032.

### 2026-10-19 15:24 (UTC) – [ADR-ANGELEGT]

- **ADR-032** `[REAKTIV]` „Funde der Sammelprüfung der Vertragssätze S-1 bis S-7": freigegeben vom Projektinhaber per Auswahl „Alle Empfehlungen" (L-1 Entwurf 3.8, L-2 S-7 `catalog`, L-3 S-6 `game.new`, L-4 Bildname = Objekt-ID, L-5 `ParamValue { key }`, L-6 Wörterbuch `{ key }`/`{ text }` und `tick` im Detail, L-7 Debug-Vermerk in 5.6, L-8 S-8, L-9 `sessionEnded` gestrichen, L-10 Texte der laufenden Sitzung in 6.1; Modul-Karte K-1 … K-5; R-1 … R-8). Reaktiv-Quote 7 / 10, Reflexion im Fahrplan, Kenntnisnahme ausstehend. Schritt 2.11 damit erledigt.

### 2026-10-19 15:19 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 2.11, Sammelprüfung:** Prüfliste `docs/research/2.11-sammelpruefung.md` (81 Sätze, 41 ohne Fund). Vorgelegt: 10 Lücken mit Optionen (Kategorie 5, für L-8 und die Modul-Karte Kategorie 1), 5 Kartenfunde, 8 redaktionelle Funde; Empfehlungen und Landeplätze im Fahrplan bei 2.11. Auffälligster Kartenfund K-1: die Mermaid-Kennung `ABF` steht doppelt, Abfrage und Abfertigung & Gepäck sind ein Knoten. Ein gemeinsamer ADR wäre `[REAKTIV]` (14-Tage-Regel gegenüber ADR-021), Quote 7 / 10.

### 2026-10-19 15:14 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 14:44 UTC plus `[BEOBACHTUNG]` 14:51 (Merge PR #23, `987c2ee`). Der Sessionende-Commit `1816efb` nach `project-context.md` Abschnitt 10 auf den neuen Branch `erkundung/2-11-sammelpruefung-vertragssaetze` vom aktuellen `main` übernommen (`aa0a7da`); `feat/2-6-lokalisierung` wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht). Für 2.11 zusätzlich Architektur Abschnitte 3 und 4 vollständig (Vertiefung: Gegenstand der Prüfung).
- **Geplant für diese Session:** Schritt 2.11 (Sammelprüfung der Vertragssätze S-1 bis S-7, Maßnahme 10): Prüfliste in `docs/research/2.11-sammelpruefung.md`, Lücken gesammelt als `ENTSCHEIDUNG ERFORDERLICH`; ADR und Doku erst nach Freigabe.
- **Vorabprüfung:** S-1 bis S-7 `[BELASTBAR]`; 2.11 ist Entwurfsschritt (ERKUNDUNG-Kontext) ohne Code; Eingangskriterium 2.6 erledigt. Reaktiv-Quote 6 / 10, Maßnahmen (7)–(10) gelten. Befund 2 aus 2.6 (Gesamtgrenze über alle Pakete) hat den Landeplatz 6.1 und wird hier nicht gelöst.
- **Modus/Werkzeug:** semi-autonom, Claude Code in einer Cloud-Session (Container ohne eingerichtete Werkzeuge; Einrichtung nur, falls Funde Code berühren).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 15:13 UTC → **Entscheidungs-Klasse**; entspricht der Empfehlung für 2.11.

### 2026-10-19 14:51 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #23 (Schritt 2.6) auf Anweisung des Projektinhabers („Erstelle den PR für 2.6, merge bei grün“) nach `main` gemergt, Merge-Commit `987c2ee`; vorher grün: Push-Lauf 39271631746, PR-Läufe CI 39272068681 und Security 39272068679 auf `781b50d`. Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf dem Arbeits-Branch und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 14:44 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 1 Stunde 16 Minuten seit `[SESSIONSTART]` 13:28 UTC; Neustart des Arbeitsprozesses gegen 14:05 UTC (zwischen Frage und Antwort), ohne Verlust.
- **Bearbeitet:** Übernahme der Sessionende-Commits auf `feat/2-6-lokalisierung`; Abgleich vor 2.6 (Maßnahmen 7–9) mit Vorlage; ADR-031; Reflexion Reaktiv-Quote, Schritt 2.11 (Maßnahme 10); Stufe 2.6a; Vorlage Ort von `Settings` (Option A); Stufe 2.6b; getrennte Prüfung (Kategorie 6) und Erledigung; Entscheidung zu Befund 2 (Landeplatz 6.1); Abschluss von 2.6.
- **Erreicht:** 2.6 `[ERLEDIGT]` (CI-Lauf 39270974482 auf `ff53c01`); FR-006 erledigt; Modul Lokalisierung `[BELASTBAR]`. Commits `10d1965` … `ff53c01` plus dieser Sessionende-Commit; Branch gepusht, kein PR.
- **Offen geblieben:** PR für 2.6 nach `main` (nur auf Anweisung); ungemergte fremde Branches wie im `[SESSIONENDE]` 2026-10-18 23:27 (Löschen nur auf Anweisung); der alte Branch `feat/2-5-spielstand-mit-autosave` (gemergt, nicht weiterverwendet; Löschen nur auf Anweisung).
- **Nächster Schritt:** PR für 2.6, danach 2.11 (Sammelprüfung der Vertragssätze) in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 13:27 und 14:42 UTC). Freigaben (Auslöser 1) und Beförderung (Auslöser 4) auf der Pflicht-Klasse; 2.6a und 2.6b als Routine-Arbeit: 2 Stufen oberhalb der Empfehlung (Warnung zu Beginn). Abgegeben: keine Teilarbeit (Probelauf offen); die getrennte Prüfung lief bewusst auf Sonnet 5.5 als Prüfinstanz, nicht als Abgabe.
- **Kontextgröße:** 472.458 Token laut `get_session` 14:42 UTC – **über der Grenze von 400.000**: die nächste Arbeit (PR, 2.11) in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`, Zurücksetzen 2026-10-19 15:50 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Archivierung:** Logbuch 880+ Zeilen (Trigger > 800); alle Einträge aus Oktober 2026, keine vorletzte Monats-Scheibe – nichts auszulagern (Lücke der Methodik wie am 2026-10-19 12:48 festgehalten). Verdichtung mit dem `[PHASEN-WECHSEL]` nach Phase 2.
- **Drift-Prüfung:** README nachgezogen (Phase, Architektur-Reife, Verwendung, Nächste Schritte). ADR-031 verweist auf vorhandene Schritte (2.6, 5.1, 6.1). Reifegrad Lokalisierung `[BELASTBAR]` passt zu ADR-031 und Umsetzung. Modul-Liste: 2.6 berührte Lokalisierung, Oberfläche, Inhalte & Mods, Brücke, Plattform, Spielfeld (Konsole) und mit `core/src/settings.rs` den Kern-Crate – alle vor dem Code vorgelegt und freigegeben. FR-006 nennt 2.6, Status erledigt. Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker. Reaktiv-Quote 6 / 10 (ADR-022 bis ADR-031) stimmt mit den Tags. Phase 2: 11 Schritte bei Plan 10 (unter der Schwelle). Größen: `project-context.md` 369 Zeilen.

### 2026-10-19 14:43 (UTC) – [REIFEGRAD-WECHSEL]

- **Modul Lokalisierung:** `[VORLÄUFIG]` → `[BELASTBAR]` durch Umsetzung in 2.6 (Regel-007, ADR-031); Beleg: `core/tests/lang.rs`, `app/renderer/i18n.test.ts`, Rauchtest mit Sprachwechsel ohne Neustart und Sprachpaket aus dem Ordner `packs`, getrennte Prüfung.

### 2026-10-19 14:41 (UTC) – [BEOBACHTUNG]

- **Getrennte Prüfung 2.6** (Sonnet 5.5, nur Diff und Bedrohungsbild): 7 Befunde, keiner hoch. Behoben 1 (quadratischer Aufwand, Sprach-Einträge zählen), 3 (Wettlauf `setSettings`), 4 (`settings.json` über Handle), 5 (Temp-Datei), 6 (`__proto__` als Schlüssel, Texte als Paare); 2 teilweise behoben, Gesamtgrenze per Entscheidung des Projektinhabers („Landeplatz 6.1“) in 6.1; 7 begründet nicht geändert. Liste: `docs/research/2.6-getrennte-pruefung.md`.

### 2026-10-19 14:20 (UTC) – [PROBLEM-GELÖST]

- **Problem:** Der erste Commit von 2.6b scheiterte am Pre-Commit-Hook „TypeScript type-check“: noch nicht vorgemerkte Renderer-Dateien nutzten Typen aus späteren Commits.
- **Ursache:** Der Hook prüft den ganzen Arbeitsbaum; neue, nicht verfolgte Dateien stellt `pre-commit` nicht beiseite.
- **Lösung:** Renderer und Rauchtest kurz mit `git stash push --include-untracked` beiseitegelegt, Kern und Plattform committet, zurückgeholt, Rest committet. Kein `--no-verify`.

### 2026-10-19 14:05 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Vor 2.6b (Maßnahme 7, Stopp-Kriterium 5):** Ort des erzeugten Typs `Settings` (S-6). Projektinhaber per Auswahl: „A: Rust im Kern-Crate“ – `core/src/settings.rs` nur mit Typ und Grenzen für ts-rs; berührt formal den Kern-Crate (Modul Takt & Weltzustand), entgegen der Ankündigung „unberührt“; Vertrag unverändert, kein ADR. Arbeitsprozess vor der Antwort neu gestartet, ohne Verlust (Stand `8753163` gepusht).

### 2026-10-19 14:01 (UTC) – [BEOBACHTUNG]

- **Abweichung von der Commit-Konvention:** `fbf72e3` („core: …“) enthält zusätzlich die Brücke (`bridge/src/lib.rs`) und die Korrektur des Schlüsselmusters in `architecture.md`, `ADR-031.md` und `fahrplan.md`; Ursache: ein `git add -A` vor `check:types`, das alle Änderungen vormerkte. Nicht nachträglich aufgeteilt (Änderung der Historie, Stopp-Kriterium 6). Lehre: vor dem Prüflauf nur die erzeugten Dateien vormerken.

### 2026-10-19 13:58 (UTC) – [PROBLEM-GELÖST]

- **Problem:** Nach dem Einbau von `lang/` scheiterten `core/tests/datapacks.rs::base_pack_of_the_repository_loads` und der Rauchtest an „3 Einträge“ des Basispakets (jetzt 111).
- **Ursache:** gewollte Folge der Festlegung „jeder Text zählt als Eintrag“ (ADR-031); die Tests kannten nur die Objekte.
- **Lösung:** Die Erwartung wird aus den Daten abgeleitet (Kern-Test aus `outcome.languages`, Rauchtest aus `content/base/lang/*.json`), nicht als neue feste Zahl eingetragen. Rauchtest danach grün im Container.

### 2026-10-19 13:57 (UTC) – [BEOBACHTUNG]

- **Stufe 2.6a umgesetzt** (Routine-Arbeit oberhalb der Empfehlung; Warnung zu Beginn): Format `lang/`, Basis-Sprachdateien `de`/`en` (54 Texte je Sprache), Schema, Brücke; 9 neue Integrationstests, Coverage Kern 98,33 % Lines. Fehler der eigenen Vorlage vor dem Code korrigiert: Schlüsselmuster `[A-Za-z0-9_.-]` statt `[a-z0-9_.-]` (vorhandene Schlüssel enthalten Großbuchstaben), in ADR-031 vermerkt, dem Projektinhaber zu melden.
- **Für 2.6b festgestellt (Maßnahme 7, vor dem Code vorzulegen):** S-6 verlangt `Settings` als erzeugten Typ (ts-rs aus Rust). Der Kern liest keine Einstellungen; ein Rust-Typ dafür läge im Kern-Crate außerhalb der zugesagten Module (Takt & Weltzustand sollte unberührt bleiben).

### 2026-10-19 13:37 (UTC) – [BEOBACHTUNG]

- **Kenntnisnahme:** Projektinhaber zur Reflexion ADR-031 per Auswahl „Kenntnis + Maßnahme (10)“. Schritt 2.11 (Sammelprüfung der Vertragssätze) vor 2.10 angelegt; Phase 2 damit 11 Schritte bei Plan 10 (Wucherungs-Schwelle: 20 und +5, nicht erreicht).

### 2026-10-19 13:35 (UTC) – [ADR-ANGELEGT]

- **ADR-031** `[REAKTIV]`: Sprachpakete erreichen die Oberfläche über `lang.list`/`lang.texts` in S-6; Basis-Sprachdateien zusätzlich beim Build eingebunden; neue Beziehung Inhalte & Mods → Lokalisierung. Freigabe des Projektinhabers per Auswahl: „A: lang.* in S-6“ und „Ja, alle“ (Festlegungen). Architektur Abschnitt 2, 3 (Lokalisierung), 4 (S-4, S-6) nachgezogen; Landeplätze 5.1 (Einstellungs-Menü, Prüfung der 60 Minuten) und 6.1 (Überschreiben fremder Texte). Reaktiv-Quote 6 / 10, Reflexion mit Vorschlag Maßnahme (10) im Fahrplan.

### 2026-10-19 13:31 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 2.6, Abgleich vor dem Code (Maßnahmen 7–9):** keine Bibliothek (Kategorie 3 nicht berührt). Zwei Lücken: S-3/S-4 setzen voraus, dass Texte aus `lang/` die Oberfläche erreichen, S-6 und der Port bieten keinen Weg (Empfehlung A: `lang.list`/`lang.texts` in S-6, `[REAKTIV]` nach der 14-Tage-Regel, Quote danach 6 / 10); Texte ohne laufenden Kern (Basis-Sprachdateien zusätzlich beim Build eingebunden). Abweichung „Betroffene Module“: zusätzlich Inhalte & Mods, Brücke, Plattform, Spielfeld. Einstellungen mit Grenzen und Herkunft (Maßnahme 9), Aufteilung 2.6a/2.6b. Volltext im Fahrplan bei 2.6.

### 2026-10-19 13:28 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 12:48 UTC plus `[BEOBACHTUNG]` 13:24 (Merge PR #22, `12a2c77`) und 13:25 (Kenntnisnahme ADR-029/ADR-030). Die beiden Sessionende-Commits `3323131` und `5d83103` nach `project-context.md` Abschnitt 10 auf den neuen Branch `feat/2-6-lokalisierung` vom aktuellen `main` übernommen (`10d1965`, `b37249d`); `feat/2-5-spielstand-mit-autosave` wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 13:27 UTC → Entscheidungs-Klasse. Schritt 2.6 empfiehlt Routine; Teile oberhalb der Empfehlung werden zu Beginn gewarnt.
- **Vorhaben:** Schritt 2.6 (Lokalisierung): Abgleich der Vertragssätze S-6 `settings.*` und S-4 `lang/` sowie der „Betroffenen Module“ gegen den Plan (Maßnahmen 7–9), dann Vorlage bzw. Umsetzung.

### 2026-10-19 13:25 (UTC) – [BEOBACHTUNG]

- **Kenntnisnahme:** Projektinhaber: „Zur Kenntnis genommen, Reflexionen ADR-029 und ADR-030“. Im Fahrplan vermerkt; kein Umbau, Maßnahme (9) gilt.

### 2026-10-19 13:24 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #22 (Schritt 2.5) auf Anweisung des Projektinhabers („merge bei grün“) nach `main` gemergt, Merge-Commit `12a2c77`; vorher grün: Push-Lauf 39254862979, PR-Läufe CI 39259088549 und Security 39259088469 auf `a773a44`. Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf dem Arbeits-Branch und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 12:48 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 1 Stunde 46 Minuten seit `[SESSIONSTART]` 11:02 UTC; Neustart des Arbeitsprozesses zwischen Frage und Antwort (11:06–11:37), ohne Verlust.
- **Bearbeitet:** Abgleich vor 2.5b (Maßnahmen 7, 8) mit Vorlage; ADR-029; Stufe 2.5b umgesetzt; getrennte Prüfung (Kategorie 6); Befunde behoben; ADR-030; Abschluss von 2.5.
- **Erreicht:** 2.5 `[ERLEDIGT]` (CI-Lauf 39253155836 auf `a4fff12` grün, 17 Jobs, drei Plattformen); FR-004 erledigt; S-5 durch Umsetzung bestätigt. Commits `4813e7e` … `a4fff12` plus dieser Sessionende-Commit; Branch gepusht, kein PR.
- **Offen geblieben:** PR für 2.5 nach `main` (nur auf Anweisung); Kenntnisnahme der Reflexionen zu ADR-029 und ADR-030; ungemergte fremde Branches wie im `[SESSIONENDE]` 2026-10-18 23:27 (Löschen nur auf Anweisung).
- **Nächster Schritt:** PR für 2.5, danach 2.6 (Lokalisierung) in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 11:01 und 12:46 UTC). Freigaben (Auslöser 1) auf der Pflicht-Klasse; 2.5b als Routine-Arbeit: 1 Stufe oberhalb der Empfehlung (Warnung zu Beginn). Abgegeben: keine Teilarbeit (Probelauf offen); die getrennte Prüfung lief bewusst auf einem anderen Modell (Sonnet 5.5) als Prüfinstanz, nicht als Abgabe.
- **Kontextgröße:** 408.755 Token laut `get_session` 12:46 UTC – **über der Grenze von 400.000**: die nächste Arbeit (PR, 2.6) in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`, Zurücksetzen 2026-10-19 15:50 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf; Runner weiter auf Ubuntu 24.04 (Hinweis zur Umstellung in jedem Lauf). Neuer CI-Hinweis „Due to capacity constraints, jobs targeting macOS arm64 runners may experience longer queue times“ (3 je Lauf), ohne Wirkung auf das Ergebnis, keine Frist.
- **Archivierung:** Logbuch erreicht 800 Zeilen (Trigger > 800 mit diesem Eintrag überschritten). Alle Einträge stammen aus dem laufenden Monat Oktober 2026; eine vorletzte Monats-Scheibe nach `CLAUDE.md` Abschnitt 14 gibt es nicht, auszulagern ist nichts. Die Verdichtung folgt mit dem `[PHASEN-WECHSEL]` nach Phase 2 (2.6 und 2.10 offen). Lücke der Methodik festgehalten: Der Zeilen-Trigger kann innerhalb eines Monats nicht greifen.
- **Drift-Prüfung:** README nachgezogen (Phase, Architektur-Reife, Verwendung, Nächste Schritte). ADR-029 und ADR-030 verweisen auf vorhandene Schritte (2.5, 4.1). Reifegrade S-5, S-6 passen zu ADR-027 bis ADR-030. Modul-Liste: 2.5b berührte Takt & Weltzustand, Brücke, Plattform, Spielfeld (Konsole) und Oberfläche (Liste) – wie in „Betroffene Module“ geplant, keine Abweichung. FR-004 nennt 2.5, Status erledigt. Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker. Reaktiv-Quote 5 / 10 (ADR-021 bis ADR-030) stimmt mit den Tags. Phase 2 weiter 10 Schritte (Plan 10). Größen: `project-context.md` 369 Zeilen.

### 2026-10-19 12:40 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-030 `[REAKTIV]` – Grenzen des Spielstands 512 MB (Option A, Freigabe per Auswahl); Anlass Befund 5 der getrennten Prüfung. Reaktiv-Quote 5 / 10 (ADR-021 bis ADR-030), Reflexion mit Maßnahme (9) im Fahrplan, Kenntnisnahme offen.

### 2026-10-19 12:30 (UTC) – [BEOBACHTUNG]

- **Getrennte Prüfung 2.5b (Kategorie 6):** 2026-10-19 ab etwa 12:16 UTC (nach dem Push um 12:15), Dauer 9,5 Minuten laut Laufzeitumgebung; Prüfinstanz Sonnet 5.5 (Unteragent ohne Gesprächsverlauf, nur Diff `c43998f..cb6e554`, S-5, S-6, Sicherheitsregeln, Bedrohungsmodell). Ergebnis „Freigabe mit Auflagen“, 12 Befunde (5 mittel, 7 niedrig, keiner hoch); Fuzzing ohne Panik. Erledigung in `docs/research/2.5-getrennte-pruefung.md`: 1–4, 6–8, 10, 12 behoben (`a6763e9`), 5 per ADR-030 (`ad16a93`), 9 und 11 teilweise mit begründetem Restrisiko. Befunde 2 und 4 vor der Korrektur selbst nachgestellt (`Array.from` über 2·10⁷ Zeichen 309 ms; Link auf `/dev/zero`: Größe 0, keine reguläre Datei).

### 2026-10-19 11:58 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** neuer Test `runner::tests::a_failed_load_keeps_the_session`.
- **Symptom:** rot in 3 von 15 Läufen: Status nach fehlgeschlagenem Laden noch `loading`.
- **Ursache:** Der Takt-Thread schickte die Antwort vor dem Veröffentlichen des neuen Status; der Aufrufer las den alten.
- **Lösung:** erst veröffentlichen, dann antworten (`core/src/runner.rs`, `load_game`); danach 25 von 25 Läufen grün.
- **Lerneffekt:** Tests mit Thread-Übergaben mehrfach hintereinander laufen lassen, bevor sie als grün gelten.

### 2026-10-19 11:50 (UTC) – [BEOBACHTUNG]

- **Rauchtest:** F5 wirkte nicht, solange der Fokus im Konsolenfeld lag (der Tastatur-Handler übergeht Eingaben im Feld). F5 gilt jetzt überall. Der erzwungene Absturz brauchte eine eigene Wartebedingung: Der Status blieb kurz `ready`, nachdem der Prozess schon beendet war, und die Messwerte nannten noch die alte PID.

### 2026-10-19 11:40 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-029 `[REAKTIV]` – `onCoreExit` meldet `restarting` (Option A, Freigabe per Auswahl); Festlegungen (a)–(l) zu 2.5b freigegeben.
- **Korrektur:** Punkt (e) des Abgleichs widersprach ADR-028 (`unknownSave` für unbekannte IDs); es gilt ADR-028. Dem Projektinhaber gemeldet.
- **Reaktiv-Quote:** 5 / 10 (ADR-020 bis ADR-029), Reflexion im Fahrplan, Kenntnisnahme offen.
- **Mini-Reibung:** Der Arbeitsprozess der Session wurde zwischen Frage und Antwort neu gestartet; Arbeitsstand und Container blieben erhalten.

### 2026-10-19 11:06 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Anlass:** Abgleich vor Stufe 2.5b nach Maßnahmen (7) und (8), Wortlaut im Fahrplan bei 2.5.
- **Befund:** eine Abweichung – das Angebot nach Absturz des Kerns braucht die Angabe, ob die Plattform neu startet; `onCoreExit` liefert nur den Code. Optionen A (zweites Argument, ADR `[REAKTIV]`) und B (Oberfläche zählt selbst). Übrige Punkte (a)–(l) ohne Vertragsänderung; „Betroffene Module“ deckt den geplanten Code.
- **Umgebung:** Container frisch: Node 26.11.1 über `/opt/nvm`, `npm ci`, Electron-Binärdatei per `install.js`, `pre-commit install`, `cargo fetch`.

### 2026-10-19 11:02 (UTC) – [SESSIONSTART]

- **Session:** `session_01WZthnhceTiMMFDi2Ways2p`, Fortsetzung von `session_01P9QoM7dmXTH6S8x8Uamvco` (dort Kontext über 400.000 Token).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (`get_session` 11:01 UTC) – Entscheidungs-Klasse.
- **Wiedereinstieg:** Branch `feat/2-5-spielstand-mit-autosave` auf `c43998f`, gleich `origin`. Aktiver Schritt 2.5 `[IN ARBEIT]`, Stufe 2.5b (Plattform). Mindest-Lektüre nach `CLAUDE.md` Abschnitt 2 durchlaufen; keine aktiven Blocker.
- **Warnung (Abschnitt 0):** 2.5b ist Routine-Arbeit (Umsetzung von S-5/S-6 nach Freigabe) und läuft oberhalb ihrer Empfehlung; Abgabe an Unteragenten nicht möglich (Probelauf offen).
- **Vorgehen:** zuerst Maßnahmen (7) und (8): Vertragssätze S-5/S-6 und „Betroffene Module“ gegen den Plan von 2.5b prüfen, Abweichungen vor dem Code vorlegen.

### 2026-10-19 10:59 (UTC) – [BEOBACHTUNG]

- **Nachtrag nach dem Sessionende:** Projektinhaber: „Zur Kenntnis genommen, Reflexion und Abweichung“. Kenntnisnahme der Reflexion zu ADR-028 (Maßnahme 8 gilt) und der Abweichung nach Stopp-Kriterium 5 in 2.5a im Fahrplan vermerkt; kein Umbau.

### 2026-10-19 02:45 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 51 Minuten seit `[SESSIONSTART]` 01:54 UTC. Mini-Reibung: Die Uhrzeit des Eintrags war zuerst geschätzt (02:48) statt mit `date -u` abgefragt (02:42) – wie am 2026-10-18; vor dem Commit korrigiert.
- **Bearbeitet:** Sessionstart mit Übernahme des Merge-Vermerks `b59a95f`; Vorlage 2.5 (Kategorie 3, Probeaufbau, Messung, Abgleich mit S-5/S-6 nach Maßnahme 7); Freigabe A und Plan durch den Projektinhaber; ADR-027, ADR-028 mit Reflexion; Stufe 2.5a (Kern) umgesetzt und getestet.
- **Erreicht:** 2.5 `[IN ARBEIT]`, Stufe 2.5a fertig (`07188d5`); Tests und Gates lokal grün, Coverage Kern 98,49 % Lines. CI-Lauf 39188204304 (Vorlage) grün auf 17 Jobs; CI-Lauf 39191152964 auf `50176b1` (mit dem Kern-Commit `07188d5`) grün. Branch `feat/2-5-spielstand-mit-autosave` gepusht, kein PR (2.5 ist nicht abgeschlossen; PR nur auf Anweisung).
- **Abweichung (Stopp-Kriterium 5):** 2.5a hat Module berührt, die nicht in „Betroffene Module“ von 2.5 standen: Welt & Bau (`Block::from_fields`, `GridSnapshot::entries`/`from_entries`, `Grid::from_snapshot`, `Network::nodes`/`edges`/`from_parts`), Wegfindung & Passagierführung (`FieldSet::source`) und Inhalte & Mods (`ActivePack` im Katalog). Es sind interne Hilfen ohne neue Verantwortung und ohne Änderung öffentlicher Verträge, aber nach `CLAUDE.md` Abschnitt 8 hätten sie vor dem Code vorgelegt werden müssen – derselbe Fehler wie bei 2.4 (dort Brücke, Plattform, Spielfeld). Nachgetragen im Fahrplan; Kenntnisnahme des Projektinhabers offen. Lehre: Beim Abgleich nach Maßnahme (7) auch die Liste „Betroffene Module“ gegen den geplanten Code prüfen.
- **Offen geblieben:** 2.5b (Plattform); Kenntnisnahme der Reflexion zu ADR-028 und der Abweichung oben; ungemergte fremde Branches wie im `[SESSIONENDE]` 2026-10-18 23:27 (Löschen nur auf Anweisung).
- **Nächster Schritt:** 2.5b in einer neuen Session (Fahrplan „Aktueller Stand“).
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 01:54 und 02:41 UTC). Freigabe (Auslöser 1), ADR-028 `[REAKTIV]` mit Reflexion auf der Pflicht-Klasse; 2.5a ist Routine-Arbeit: 1 Stufe oberhalb der Empfehlung (Warnung zu Beginn ausgesprochen). Abgegeben: keine Teilarbeit (Probelauf offen).
- **Kontextgröße:** `get_session` meldet 0 (nicht aussagekräftig); nach dem Token-Zähler der Laufzeitumgebung rund 450.000 Token – **über der Grenze von 400.000**: 2.5b in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`, Zurücksetzen 2026-10-19 07:10 UTC (`get_session` 02:41 UTC).
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. Runner-Umstellung auf Ubuntu 26 noch nicht erfolgt (Register ergänzt). Neue Crates `postcard`, `cobs`, `miniz_oxide`, `adler2` fallen unter die Sammelzeile „Übrige npm-Werkzeuge und Crates“ (Nachprüfung 2027-04-13).
- **Drift-Prüfung:** README nachgezogen (Letzte Änderung, Nächste Schritte; 2.5a ohne Wirkung für Spieler, Verwendung unverändert). ADR-027 und ADR-028 verweisen auf vorhandene Schritte (2.5, 2.6, 3.1, 3.4, 5.1). Reifegrade S-5, S-6 passen zu ADR-027/028 (unverändert `[BELASTBAR]`). Modul-Liste: „Betroffene Module“ von 2.5 ergänzt (siehe Abweichung). FR-004 nennt 2.5. Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker. Reaktiv-Quote 5 / 10 (ADR-019 bis ADR-028) stimmt mit den Tags. Phase 2 weiter 10 Schritte (Plan 10; die Stufen 2.5a/2.5b zählen als ein Schritt). Größen: `project-context.md` 369 Zeilen, Logbuch unter 800 (vor diesem Eintrag 761 – Trigger bald erreicht).

### 2026-10-19 02:41 (UTC) – [BEOBACHTUNG]

- **Stufe 2.5a umgesetzt** (`07188d5`): Spielstand-Container im Kern, Tests `core/tests/savegame_roundtrip.rs` (10) und Unit-Tests der Invarianten (4) grün, Coverage Kern 98,49 % Lines. Alle Tests liefen beim ersten Versuch grün bis auf eine eigene Annahme im Unit-Test (geteilte Leer-Blöcke im offenen Saal – dort ist jeder Block bebaut; Prüfung auf eine leere Welt umgestellt) und einen clippy-Befund (`err().expect` → `expect_err`).
- **Entwurfsentscheidungen im Rahmen des freigegebenen Plans:** Die „Bauänderungen seit der Revision“ (S-5) sind die Blöcke, die sich zwischen Eingabe der aktiven Wegweiser und aktuellem Raster unterscheiden; dafür behält `FieldSet` ihre Eingabe (geteilte Blöcke, zusätzlich nur die Tabelle der Zeiger). Das Tempo wird mitgespeichert, weil es Teil der Zustands-Prüfsumme ist. Befehle prüft der Kern jetzt gegen die Karte der Welt statt gegen die Einstellung, damit ein Spielstand mit anderer Kartengröße richtig geprüft wird.
- **CI-Lauf 39188204304** (Vorlage, `a4ace0c`): 17 von 17 Jobs grün. Ubuntu-Jobs noch auf Ubuntu 24.04.5 (Image `ubuntu-24.04` 20261012): Umstellung auf Ubuntu 26 noch nicht erfolgt, Register-Eintrag ergänzt. Neu im Protokoll: `npm audit` mit 8 Befunden „moderate“ (sprintf-js über electron-builder, nur Bauwerkzeug) – in den Warnungs-Bestand (`project-context.md` Abschnitt 7) und nach Q.1 aufgenommen. Der Hinweis „install scripts not yet covered“ nennt nur noch electron-winstaller.
- **Dauer der Tests:** `savegame_roundtrip` braucht im Debug-Build rund 22 s (300 zufällige Änderungen am Referenz-Terminal 256²); vertretbar, aber im Blick behalten.

### 2026-10-19 02:10 (UTC) – [ADR-ANGELEGT]

- **ADR-027 `[OPERATIV]`:** Spielstand mit `postcard` 1.1.3, Deflate über `miniz_oxide` 0.9.1, Prüfsumme FNV-1a 64 aus dem Kern (Option A), freigegeben vom Projektinhaber per Auswahl.
- **ADR-028 `[REAKTIV]`:** `saves.preview(id)` als additive S-6-Funktion; freigegeben mit dem Abgleich (1)–(9) und der Aufteilung in 2.5a/2.5b. Reaktiv-Quote 5 / 10 (ADR-019 bis ADR-028), Reflexion mit Maßnahme (8) im Fahrplan, Kenntnisnahme offen.
- **Landeplätze nachgetragen:** Autosave-Einstellungen 2.6, `MAX_OBJECTS` 3.1, `MAX_VEHICLES` 3.4, Flughafenname und Spielmenü 5.1.
- **Modell:** Entscheidungs-Klasse (`claude-opus-5-5`, `get_session` 01:54 UTC; Auslöser 1). 2.5a ist nach der Freigabe Routine-Arbeit: läuft oberhalb ihrer Empfehlung, Abgabe an einen Unteragenten nicht möglich (Probelauf offen).

### 2026-10-19 02:05 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 2.5, Kategorie 3:** Serialisierungsformat, Kompression und Prüfsumme des Spielstands. Optionen A `postcard` + `miniz_oxide` + FNV-1a 64 aus dem Kern (Empfehlung), B wie A mit `zstd` (C), C nur `postcard` ohne Kompression. Belege, Messung und Abgleich mit S-5/S-6 (Maßnahme 7) im Fahrplan bei 2.5.
- **Umgebung:** Container frisch: Node 26.11.1 über `/opt/nvm`, `npm ci`, `pre-commit install`, `cargo-deny` 0.20.2 aus dem GitHub-Release. Probeaufbau unter dem Scratchpad, nicht im Repository.
- **Beobachtung:** `bincode` trägt seit 2025-12-16 das Advisory RUSTSEC-2025-0141 (nicht mehr gepflegt; 3.0.0 ist eine Abschluss-Version); `ruzstd` hatte vor 8 Tagen ein Advisory zur Dekompressionsbombe. Beides schließt sie für nicht vertrauenswürdige Spielstände aus.

### 2026-10-19 01:54 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.4 erledigt (2026-10-18), PR Paddel87/OpenAirside#21 gemergt (`eead91b`); kein `[IN ARBEIT]`-Schritt; nächster Schritt 2.5. Letzter `[SESSIONENDE]`: 2026-10-18 23:27 UTC, danach Beobachtungen 23:30 und 23:38 UTC.
- **Branches:** Neuer Branch `feat/2-5-spielstand-mit-autosave` von `main` `eead91b` (Schema `CLAUDE.md` Abschnitt 11; das Werkzeug hat keinen Namen vergeben). Merge-Vermerk `b59a95f` von `scp/zealous-ride-mwvv5m` als `dae5b43` übernommen (`project-context.md` Abschnitt 10). `docs/branch-konvention` ist vollständig in `main`. Übrige ungemergte Branches unverändert wie im `[SESSIONENDE]` 23:27 genannt; Löschen nur auf Anweisung.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant für diese Session:** kein ausdrücklicher Auftrag („Neue Session“); nach Fahrplan 2.5 beginnen: Vertrag S-5 und S-6 vertiefen, Maßnahme (7) anwenden (Vertragssätze vor dem Code gegen den Plan prüfen), Freigabe von Serialisierungsformat, Prüfsumme und ggf. Kompression mit belegten Versionen vorlegen.
- **Vorabprüfung:** keine aktiven Blocker; Eingangskriterien 2.5 (S-5, S-6 `[BELASTBAR]`) erfüllt; Reaktiv-Quote 5 / 10 über dem Schwellenwert, Reflexionen zu ADR-025 und ADR-026 zur Kenntnis genommen, kein neuer reaktiver ADR seitdem; Phase 2 bei 10 Schritten (Plan 10). Ablaufdaten: heute stellt GitHub `ubuntu-latest` auf Ubuntu 26 um – ersten CI-Lauf dieser Session darauf prüfen (Q.1); sonst kein Vorlauf erreicht.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Start über die iOS-App).
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 01:54 UTC – Entscheidungs-Klasse (Pflicht für die Freigabe in 2.5, Auslöser 1). Kontingent: 5-Stunden-Fenster `allowed`, Zurücksetzen 2026-10-19 02:10 UTC.

### 2026-10-18 23:38 (UTC) – [BEOBACHTUNG]

- **PR Paddel87/OpenAirside#21 gemergt** (`eead91b`, Merge-Commit) auf Anweisung des Projektinhabers („merge wenn grün“); vorher alle 39 Prüfläufe auf `f52f469` grün (CI für Push und PR, CodeQL, wöchentliche Audits), PR konfliktfrei, keine Reviews oder Kommentare. Branch `scp/zealous-ride-mwvv5m` per Fast-Forward auf den neuen `main` gesetzt. Dieser Eintrag liegt nach `project-context.md` Abschnitt 10 nur auf dem Branch und geht mit dem PR des nächsten Schritts (2.5) nach `main`.

### 2026-10-18 23:30 (UTC) – [BEOBACHTUNG]

- **Nachtrag nach dem Sessionende:** Projektinhaber: „PR öffnen und Reflexionen zur Kenntnis genommen". Kenntnisnahme der Reflexionen zu ADR-025 (Maßnahme 6) und ADR-026 (Maßnahme 7) im Fahrplan vermerkt; kein Umbau. PR für `scp/zealous-ride-mwvv5m` nach `main` wird geöffnet.
- **Mini-Reibung:** Der `[SESSIONENDE]`-Eintrag trug 23:30 UTC, geschrieben wurde er um 23:27 UTC – trotz der Regel vom Sessionstart, die Uhrzeit aus `date -u` zu nehmen (geschätzt statt abgefragt). Auf 23:27 korrigiert, Dauer 78 statt 81 Minuten.

### 2026-10-18 23:27 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 78 Minuten seit `[SESSIONSTART]` 22:09 UTC.
- **Bearbeitet:** Sessionstart mit Übernahme des belegten Merge-Vermerks zu PR Paddel87/OpenAirside#20; Schritt 2.4 vollständig (Freigabe ADR-025, Umsetzung, erzwungener Fehlerfall der Schema-Prüfung, getrennte Prüfung mit Erledigung, Freigabe ADR-026, Abschluss).
- **Erreicht:** 2.4 `[ERLEDIGT]`; Modul Inhalte & Mods (Lader) `[BELASTBAR]`; ADR-025 und ADR-026 (beide `[REAKTIV]`), Reflexionen mit Maßnahmen (6) und (7). CI-Lauf 39176235417 auf `89fa783` grün auf allen 17 Jobs. 22 Commits auf `scp/zealous-ride-mwvv5m` seit `main` `f652370` (einschließlich des übernommenen Merge-Vermerks und dieses Abschlusses), gepusht; kein PR (nicht angefragt).
- **Offen geblieben:** Kenntnisnahme der Reflexionen zu ADR-025 und ADR-026 durch den Projektinhaber (nachgeholt 23:30, siehe Nachtrag); Merge des Branches (PR nur auf Anweisung). Ungemergte fremde Branches (`claude/messwerkzeug-2-7-rb2hq7`, `claude/gracious-galileo-2reksz`, `scp/friendly-einstein-rggagz`, `claude/naechster-schritt-2-8-736kf4`) – Löschen nur auf Anweisung. Kein CHANGELOG im Repository (Einträge ab dem ersten Release).
- **Nächster Schritt:** 2.5 Spielstand mit Autosave – zuerst Freigabe von Serialisierungsformat, Prüfsumme und ggf. Kompression (Kategorie 3, Entscheidungs-Klasse); Maßnahme (7) anwenden; dazu die Fehlerform von S-6 aus der getrennten Prüfung 2.4.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 22:06 und 23:26 UTC). 2.4 empfiehlt die Routine-Klasse: 1 Schritt oberhalb der Empfehlung (Warnung zu Beginn ausgesprochen); die Freigaben ADR-025 und ADR-026, die Reflexionen und die Beförderung lagen auf der Pflicht-Klasse Entscheidung. Abgegeben: getrennte Prüfung an einen Unteragenten Sonnet 5.5 (Prüfauftrag, keine Abgabe von Arbeit); keine Abgabe an Routine/Mechanik (Probelauf offen).
- **Kontextgröße:** 569.408 Token laut `get_session` 23:26 UTC – **über der Grenze von 400.000**: die nächste Arbeit in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`, Zurücksetzen 2026-10-19 02:10 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Electron 44: Vorlauf ab 2027-01-02). Runner-Umstellung `ubuntu-latest` → Ubuntu 26 ab morgen (2026-10-19): ersten Lauf danach in Q.1 beobachten. Neue Crates fallen unter die Sammelzeile „Übrige npm-Werkzeuge und Crates" (Nachprüfung 2027-04-13); `schemars` 1.2.3 dort nachziehen.
- **Drift-Prüfung:** README nachgezogen (Phase, Reife, Verwendung, Nächste Schritte, Letzte Änderung); ADR-025 und ADR-026 verweisen auf vorhandene Schritte (2.4, 2.5, 2.6, 3.4, 3.6, 5.6, 6.1); Reifegrad Inhalte & Mods passt zu ADR-025/026; Modul-Liste: „Betroffene Module" von 2.4 um Brücke, Plattform und Spielfeld ergänzt (bei der Planung fehlten sie, obwohl `packs.*` im „Zu tun" stand); FR-044 nennt 2.4, 3.6, 6.1 und steht in 3.6; keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Reaktiv-Quote 5 / 10 (ADR-017 bis ADR-026) stimmt mit den Tags; Phase 2 weiter 10 Schritte (Plan 10). Größen: `project-context.md` 368 Zeilen, Logbuch unter 800.

### 2026-10-18 23:28 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Modul Inhalte & Mods (Lader) `[VORLÄUFIG]` → `[BELASTBAR]`; S-4 durch die Umsetzung bestätigt.
- **Grundlage:** Umsetzung 2.4 nach Regel-007 (ADR-025, ADR-026), 26 Integrationstests mit fehlerhaften Paketen, Rauchtest mit Mod-Paket auf drei Plattformen, getrennte Prüfung mit Erledigung aller Befunde; Schutzmechanismus Schema-Prüfung durch erzwungenen Fehler belegt (Lauf 39173571403), Junction-Abwehr unter Windows durch Test in der CI belegt (Lauf 39176235417). Eskalations-Auslöser 4; aktive Klasse Entscheidung (`get_session` 23:26 UTC).

### 2026-10-18 23:24 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** CI-Läufe 39175650047 und 39175755891, Job „Rust Tests (windows-latest)".
- **Symptom:** Neuer Test `junctions_are_refused_on_windows` scheiterte mit „mklink /J failed".
- **Ursache:** `user.path().join("p/objects")` ergab unter Windows einen Pfad mit `/`; `mklink` las `/objects` als Schalter.
- **Lösung:** Pfade aus einzelnen Teilen (`join("p").join("objects")`), Ausgabe von `mklink` im Fehlertext (`2f5c65a`); vorher lokal für `x86_64-pc-windows-gnu` mit Clippy geprüft. Lauf 39176235417 grün.
- **Aufwand:** etwa 5 Minuten.

### 2026-10-18 23:20 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-026 – Pakete in S-6 über ihre Manifest-ID (Kategorie 5).
- **Tag:** `[REAKTIV]` (ändert S-6 aus ADR-021 nach fünf Tagen).
- **Auslöser:** Freigabe des Projektinhabers „A (Empfohlen)" auf der Entscheidungs-Klasse. Reaktiv-Quote 5 / 10, Reflexion mit Maßnahme (7) in `fahrplan.md` (Eskalations-Auslöser 5, aktive Klasse Entscheidung); Kenntnisnahme offen.

### 2026-10-18 23:15 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.4, Kategorie 5 (API-Vertrag S-6); Eskalations-Auslöser 1 – aktive Klasse bereits Entscheidung.
- **Frage:** S-6 verlangt für Pakete vom Hauptprozess vergebene undurchsichtige IDs; umgesetzt sind die Manifest-IDs (Befund 8 der getrennten Prüfung). A: Vertrag auf Manifest-IDs anpassen (Empfehlung); B: Hauptprozess vergibt eigene IDs nur für den Renderer, intern weiter Manifest-IDs. Bei A wird der ADR nach der 14-Tage-Regel `[REAKTIV]` (ändert ADR-021), Reaktiv-Quote dann 5 / 10.
- **Stand:** 2.4 bleibt `[IN ARBEIT]`, bis Vertragstext und Umsetzung übereinstimmen.

### 2026-10-18 23:14 (UTC) – [BEOBACHTUNG]

- **Getrennte Prüfung 2.4 abgeschlossen** (Unteragent Sonnet 5.5, nur Diff, Verträge S-4/S-6, Sicherheitsregeln und ADR-025; Dauer rund 18 Minuten): 14 Befunde, keiner „hoch", 2 „mittel" (Dateizahl erst nach dem Aufzählen; neueres Format mit neuen Feldern meldet `invalidStructure`). Kein Absturz, keine Pfadumgehung durch ein Paket allein; 300.000 Zufallsdokumente gegen `serde_json::Value`, 500.000 Zufallsfolgen, 4.000 Mutationsläufe ohne Panic.
- **Erledigung** in `docs/research/2.4-getrennte-pruefung.md`: behoben in `bae5bcd` und `191142f` (Befunde 1–6, 10, 11, große Teile von 7, 9, 13, 14; 8 neue Integrationstests, darunter Junctions unter Windows); Landeplätze 2.5 (Fehlerform von S-6) und 6.1 (Laden im Hintergrund, Neu-Einlesen zur Laufzeit); Restrisiken harte Links unter Windows, FIFO-Race unter Unix, Cloud-Platzhalter; Befund 8 zur Entscheidung.
- **Mini-Reibung:** Zwei erwartete Spalten in neuen Tests waren von Hand falsch gezählt (81 statt 66, 26 statt 24); mit einem Python-Einzeiler nachgerechnet, die Umsetzung stimmte. Erwartete Positionen künftig gleich ausrechnen statt zählen.

### 2026-10-18 22:52 (UTC) – [BEOBACHTUNG]

- **Erzwungener Fehlerfall der Schema-Prüfung (Kategorie 7, ADR-025):** `49d45bd` ändert eine eingecheckte Schema-Datei; CI-Lauf 39173571403: nur der Schritt „Erzeugte Schemas (content/schema) aktuell" im Job „Rust Lint & Format" rot, alle übrigen 16 Jobs grün (darunter Rauchtest mit Mod-Paket auf Windows, macOS und Linux, Coverage, Kern-Benchmark). Zurückgenommen in `f59fe24`. Lokal nachvollzogen: `npm run check:schema` meldet mit dem committeten Stand die Datei.
- **Datenordner im Entwicklungs-Build** geprüft: `app.getName()` = `openairside`, `userData` = `~/.config/openairside`; der Hauptprozess legt dort `packs/` an. Der Rauchtest nutzt `--user-data-dir` mit einem frischen Ordner, damit gespeicherte Paketwahl eines Entwicklers nicht stört; Electron übernimmt den Schalter (Test wäre sonst bei `packs set kiosks` gescheitert).

### 2026-10-18 22:37 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** CI-Lauf 39172857755 auf `ac90598` (Zwischenstand der Kern-Commits von 2.4).
- **Symptom:** Job „Rust Lint & Format" rot: `gen-types` meldet `PackError.ts`, `PackReport.ts`, `PackSource.ts`, `PackState.ts` als abweichend.
- **Ursache:** Die Rust-Typen mit ts-rs-Export kamen in `b36844f`, die erzeugten TypeScript-Dateien erst mit der Brücke in `79a6d00`. Zwischen beiden Commits war `check:types` rot; gepusht wurde dieser Zwischenstand.
- **Lösung:** Mit `79a6d00` behoben; CI-Lauf 39173547010 auf `b2b7d7d` grün auf allen Jobs. Für künftige Schritte: erzeugte Typen im selben Commit wie die exportierten Rust-Typen.
- **Aufwand:** etwa 2 Minuten.

### 2026-10-18 22:18 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-025 – Bibliotheken für den Datenpaket-Lader und die Schema-Erzeugung (Kategorie 3 und 7).
- **Tag:** `[REAKTIV]` (hebt „`serde_json` nur in der Brücken-Crate“ aus ADR-024 auf, 14-Tage-Regel).
- **Auslöser:** Freigabe des Projektinhabers „A (Empfohlen)“ per Auswahl auf der Entscheidungs-Klasse (`claude-opus-5-5`, `get_session` 22:06 UTC, seitdem keine Wechselmeldung). Reaktiv-Quote 4 / 10 (ADR-016 bis ADR-025), Reflexion mit Maßnahme (6) in `fahrplan.md`, Kenntnisnahme offen (Eskalations-Auslöser 5 – aktive Klasse bereits Entscheidung).
- **Folgen:** 2.4 `[IN ARBEIT]`; Landeplätze für die Schemas der übrigen Inhaltsarten in 2.6, 3.4, 3.6 und 5.6 eingetragen; `project-context.md` Abschnitt 3 nachgezogen.

### 2026-10-18 22:15 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.4, Kategorie 3 (Externe Abhängigkeiten) und 7 (CI-Schritt für Schema-Dateien, Basispaket im Programmpaket); Eskalations-Auslöser 1 – aktive Klasse bereits Entscheidung, kein Stopp nötig.
- **Frage:** A `serde_json` auch im Kern, `serde_path_to_error`, `semver`, `schemars` hinter Feature `schema` (Empfehlung); B ohne `schemars`, Schema-Dateien erst in 6.1; C ohne `serde_path_to_error` und `semver`, eigener Code. Versionen auf crates.io belegt, Probeaufbau mit `cargo deny check` grün (Fahrplan 2.4, „Vorlage zur Freigabe“).
- **Klassifikation:** ADR-025 wird `[REAKTIV]` (hebt „nur in der Brücken-Crate“ aus ADR-024 auf, 14-Tage-Regel).
- **Stand:** 2.4 `[WARTET-AUF-FREIGABE]`.

### 2026-10-18 22:09 (UTC) – [BEOBACHTUNG]

- **Zwei Merge-Vermerke zu PR Paddel87/OpenAirside#20:** Auf `scp/friendly-einstein-rggagz` (`45f544c`, Session ab 23:40) und auf `claude/naechster-schritt-2-8-736kf4` (`274e662`, Session ab 06:55, die den PR beobachtete) liegt je ein Eintrag „00:30 – [BEOBACHTUNG]“, nicht auf `main`. Sie widersprechen sich: `45f544c` nennt den Projektinhaber als Ausführenden, `274e662` den Merge durch die KI „auf Anweisung des Projektinhabers (‚merge wenn grün‘)“. Geprüft im Verlauf der Session `013Yz3…` (`list_events`): Anweisung „merge wenn grün“ vom Projektinhaber, Aufruf `merge_pull_request` um 00:29:11 UTC, Merge laut GitHub 00:29:18 UTC (`merged_by` zeigt das Konto des Projektinhabers, weil die KI über dessen Zugang arbeitet). Übernommen ist daher nur `274e662` (als `4e0b0f3`); `45f544c` bleibt unübernommen. Die übrigen Angaben aus `45f544c` stehen im `[SESSIONSTART]` unten.
- **Zeitstempel im Logbuch vorgegangen:** Die Einträge „00:25“, „00:33“, „00:35“ und „00:45“ vom 2026-10-15 stehen in Commits von 00:12, 00:15, 00:15 und 00:24 UTC, liegen also 13 bis 21 Minuten nach ihrer Entstehung. Die Reihenfolge stimmt, die Uhrzeiten nicht. Die Einträge bleiben unverändert; maßgeblich sind die Commit-Zeiten. Ab jetzt nimmt jeder Eintrag seine Uhrzeit aus `date -u`.
- **Wieder kein `[SESSIONENDE]`:** Die Session ab 2026-10-14 23:40 UTC endete ohne Eintrag (letzter Commit `1f98838` 00:24 UTC, Merge 00:29), wie schon die Session ab 06:55. Fahrplan „Aktueller Stand“ und README waren nachgezogen (geprüft: Phase, Reife, Blocker, Nächste Schritte stimmen); Modell-Bilanz, Kontextgröße und Drift-Prüfung fehlen für beide Sessions.

### 2026-10-18 22:09 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.7 erledigt (2026-10-15), PR Paddel87/OpenAirside#20 gemergt (`f652370`); kein `[IN ARBEIT]`-Schritt; nächster Schritt 2.4 mit vorheriger Freigabe der neuen Abhängigkeiten (Kategorie 3). Letzter `[SESSIONENDE]`: 2026-10-14 05:32 UTC.
- **Branches:** Session-Branch `scp/zealous-ride-mwvv5m` (vom Werkzeug vergeben) stand auf `main` `f652370`; Merge-Vermerk `274e662` als `4e0b0f3` übernommen (Abschnitt 10 `project-context.md`). `claude/naechster-schritt-2-8-736kf4` ist gemergt bis auf diesen Vermerk; `scp/friendly-einstein-rggagz` trägt nur `45f544c` (nicht übernommen, siehe Beobachtung oben); `claude/messwerkzeug-2-7-rb2hq7` (abgebrochene Parallel-Session) und `claude/gracious-galileo-2reksz` (nur `fbf2507`, als `a4d3521` übernommen) liegen ungemergt vor. Löschen nur auf Anweisung des Projektinhabers.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C/D, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant für diese Session:** kein ausdrücklicher Auftrag („Neue Session.“); nach Fahrplan 2.4 beginnen: Vertrag S-4 und Modul Inhalte & Mods vertiefen, Freigabe der Abhängigkeiten (JSON-Lader, Schema-Erzeugung) mit belegten Versionen vorlegen.
- **Vorabprüfung:** keine aktiven Blocker; Eingangskriterium 2.4 (S-4 `[BELASTBAR]`) erfüllt; Reaktiv-Quote 4 / 10 über dem Schwellenwert, Reflexionen liegen vor, kein neuer reaktiver ADR seitdem; Phase 2 bei 10 Schritten (ursprünglicher Plan 10). Ablaufdaten: Runner-Umstellung `ubuntu-latest` → Ubuntu 26 ab morgen (2026-10-19), Q.1 beobachtet den ersten Lauf danach; sonst kein Vorlauf erreicht.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Start über Desktop-App). Container frisch: Node 26.11.1 über `/opt/nvm`, `npm ci`, Electron-Binärdatei per `install.js`, `pre-commit install`, `cargo-deny` 0.20.2 aus dem GitHub-Release.
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 22:06 UTC – Entscheidungs-Klasse. Kontextgröße meldet `get_session` mit 0 (nicht aussagekräftig); Kontingent-Angaben (`rate_limit_info`) fehlten in der Antwort.

### 2026-10-15 00:30 (UTC) – [BEOBACHTUNG]

- **PR Paddel87/OpenAirside#20 gemergt** (`f652370`, Merge-Commit) auf Anweisung des Projektinhabers („merge wenn grün“); vorher alle 56 Prüfläufe auf `1f98838` grün, PR konfliktfrei, Entwurfsstatus aufgehoben. Beobachtung des PR beendet. Dieser Eintrag liegt nach `project-context.md` Abschnitt 10 nur auf dem Branch und geht mit dem PR des nächsten Schritts (2.4) nach `main`.

### 2026-10-15 00:45 (UTC) – [BEOBACHTUNG]

- **Zweiter PC-Lauf mit 2 + 4 Threads** (fertig 00:22 UTC, `--threads=2 --background-threads=4`, in `coreOptions` bestätigt): abgelegt unter `docs/research/2.7-ergebnisse/referenz-pc-windows-2-4-threads.json`, ausgewertet im Messprotokoll. Simulation ohne Rückstand, Einzelschritt höchstens 0,94 ms (Standardlauf 3,8 ms); Neuberechnung bis 1,59 s (Standard 0,37 s); Bauaktion weiter 1–2 Bilder; Speicher ohne Anstieg. Standardaufteilung bleibt; Vergleichslauf mit weniger Hintergrund-Threads auf dem Mac in 2.10 ergänzt.
- **Mini-Reibung:** Die erste hochgeladene Datei war eine Kopie des ersten Laufs (gleiche Prüfsumme); erkannt am Zeitstempel im Namen und an `coreOptions`. Hinweis für künftige Messanleitungen: Zeitstempel im Dateinamen und `coreOptions` als Prüfmerkmal nennen.

### 2026-10-15 00:35 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteile:** Modul Brücke und Modul Plattform `[VORLÄUFIG]` → `[BELASTBAR]`; Kommunikations-Grundmodus (`architecture.md` Abschnitt 1) ebenso.
- **Grundlage:** Umsetzung 2.7 nach Regel-007 (Tests, Rauchtest auf drei Plattformen, CI-Lauf 38526061047 grün) und Messung auf Mac M3 Pro und Referenz-PC (gepackte Programme aus Release-Lauf 38526063928). Eskalations-Auslöser 4; aktive Klasse Entscheidung (`claude-opus-5-5` laut `get_session` 23:38 UTC, seitdem keine Wechselmeldung).
- **Schritt:** 2.7 `[ERLEDIGT]`, FR-009 erledigt.

### 2026-10-15 00:33 (UTC) – [BEOBACHTUNG]

- **Störung der PC-Messung geklärt:** Der Bildschirm war aus (Angabe des Projektinhabers); Monitor 60 Hz; am Mac wurde während der Messung nichts bedient. Fenster 9 bis 12 der PC-Messung gehen nicht in die Bewertung ein.
- **Nebenbefund:** Auch bei ausgeschaltetem Bildschirm hielt der Kern 180 Ticks/s im Mittel, mit Schritten bis 75 ms und einer Rückstandsmeldung. Nach dem Wiedereinschalten lag das Bildabstand-p99 in den Kreisen-Fenstern bei 33 statt 17 ms (Bildrate weiter 60). Ursache beider Effekte nicht untersucht; ohne Bedeutung für das Abschlusskriterium.
- **Aussetzer auf dem Mac** (zwei Schritte bis 110 ms) damit ohne Einwirkung von außen und weiter ungeklärt; Landeplatz Schritt 2.10 (Messung auf dem Mac dort ohnehin vorgesehen).

### 2026-10-15 00:25 (UTC) – [BEOBACHTUNG]

- **Messung 2.7 auf dem Referenz-PC** (Windows, Datei des Projektinhabers, fertig 00:08 UTC, gleichzeitig mit dem Mac gestartet): ausgewertet in `docs/research/2.7-messung.md`, Datei unter `docs/research/2.7-ergebnisse/referenz-pc-windows.json`. In 16 ungestörten Fenstern gleichwertig oder besser als Spike 1.5: 60 Bilder/s, 180 Ticks/s ohne Rückstand, Schritt Mittel 0,13–0,14 ms (Spike 0,33–0,42), Neuberechnung höchstens 366 ms (Spike 388, Welt jetzt 2048²), Bauaktion 1–2 Bilder (Spike 2–3), Kern-Prozess ohne Anstieg.
- **Störung in Fenster 9 bis 12** (302–441 s): kein gezeichnetes Bild in Fenster 10 und 11, lange Aufgabe 162 ms, GPU-Prozess +40–50 MB, Kern-Schritte bis 75 ms, eine Rückstandsmeldung (14 Ticks). Muster eines nicht gezeichneten Fensters bei Last von außen; Ursache offen, Rückfrage gestellt. Zeitlich fast gleichzeitig mit den Aussetzern auf dem Mac (00:01:50–00:02:50 UTC).
- Zweiter Lauf mit festen Threads liegt nicht vor.

### 2026-10-15 00:10 (UTC) – [BEOBACHTUNG]

- **Messung 2.7 auf dem Mac M3 Pro** (Datei des Projektinhabers, fertig 00:07 UTC, Paket aus Release-Lauf 38526063928 auf `bbdd5c8`): ausgewertet in `docs/research/2.7-messung.md`, Datei abgelegt unter `docs/research/2.7-ergebnisse/mac-m3pro.json`. Gleichwertig oder besser als Spike 1.5: 120 Bilder/s, Schritt Mittel 0,26–0,56 ms (Spike 0,52–0,84), Neuberechnung höchstens 422 ms (Spike 724), Bauaktion 1–3 Bilder (Spike 3–6), Speicher ab 200 s ohne Anstieg.
- **Auffällig:** zwei Aussetzer bis 110 ms in Fenster 8 und 9 (Rückstand bis 19 Ticks, 3 Rückstandsmeldungen, aufgeholt); Sekunden-p99 über 4 ms in 10 von 20 Fenstern. Ursache nicht belegt, Vermutung Thread-Verteilung auf Leistungs- und Effizienzkerne. Die Kennzahl „Sekunden-p99" des Messwerkzeugs ist strenger als das Fenster-p99 des Spikes und nicht direkt vergleichbar.
- **Speicher höher als im Spike** (bis 989 statt 715 MB gesamt, Kern bis 393 statt 236 MB), Anstieg nur in den ersten 200 s; Ursache nicht untersucht.
- Offen für 2.7: Messung auf dem Referenz-PC.

### 2026-10-14 23:40 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.7 `[IN ARBEIT]`, umgesetzt einschließlich Messwerkzeug; offen ist nur die Messung durch den Projektinhaber auf Referenz-PC und Mac nach `docs/research/2.7-messung.md`. PR Paddel87/OpenAirside#20 als Entwurf offen (Kopf `bbdd5c8`). Die Session ab 06:55 UTC endete ohne `[SESSIONENDE]`-Eintrag (letzter Eintrag 07:00 UTC); Fahrplan „Aktueller Stand" ist dort aber nachgezogen.
- **Branches:** Session-Branch `scp/friendly-einstein-rggagz` (vom Werkzeug vergeben, stand auf `main` `70d29e5`) per Fast-Forward auf `claude/naechster-schritt-2-8-736kf4` (`bbdd5c8`) gesetzt, keine Historie geändert. Daneben liegt `claude/messwerkzeug-2-7-rb2hq7` mit einer abgebrochenen Parallel-Session (06:29–06:45 UTC, nur Logbuch, Freigabe zu 2.7 mit „Abbruch" beantwortet); ihr Inhalt ist durch ADR-024 überholt und wird nicht übernommen.
- **Geplant für diese Session:** offen – Auftrag des Projektinhabers steht aus („Neue Session." ohne Arbeitsauftrag). Naheliegend: Messergebnisse 2.7 auswerten, falls vorhanden; sonst 2.4 vorbereiten.
- **Vorabprüfung:** keine aktiven Blocker; Reaktiv-Quote 4 / 10 über dem Schwellenwert, Reflexionen liegen vor; Phase 2 bei 10 Schritten (ursprünglicher Plan 10).
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Start über Desktop-App). Pre-Commit-Hook im frischen Container nachinstalliert.
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 23:38 UTC – Entscheidungs-Klasse. Kontextgröße meldet `get_session` mit 0 (nicht aussagekräftig); Kontingent-Angaben (`rate_limit_info`) fehlten in der Antwort.

### 2026-10-14 07:00 (UTC) – [BEOBACHTUNG]

- **Messwerkzeug 2.7 umgesetzt** (`dd762ea` Kern, `ebe0342` Renderer und Plattform): `TickStats` im Status des Takts und Port-Nachricht `coreStats` je Sekunde (additiv, `protocol` bleibt 1); Konsolenbefehl `measure [Minuten]`; S-6 `measure.start`/`measure.report` (additiv, `apiVersion` bleibt 1) mit Speicher aller Prozesse und JSON-Datei im Download-Ordner. Einordnung: im Fahrplan-Schritt 2.7 vorgesehen („Messwerkzeug … mit Ausgabe als JSON-Datei; dazu Kennzahlen des Takts im Status“), additive Erweiterungen bestehender Verträge, keine neue Abhängigkeit – daher ohne Freigabe nach `CLAUDE.md` Abschnitt 4. Der neue IPC-Aufruf schreibt eine Datei; Ordner und Name wählt der Hauptprozess, ein Bericht je gestarteter Messung, höchstens 1 MB, nie überschreiben, Absenderprüfung wie alle Aufrufe.
- **Probelauf im Container** (`measure 1`, 4 vCPU, SwiftShader): läuft Ende-zu-Ende, Datei vollständig (Systemdaten, 2 Fenster, 16 Speicherproben). Werte im Messprotokoll `docs/research/2.7-messung.md`; ohne Aussage für die Zielhardware.
- **Download-Ordner im Container:** `app.getPath("downloads")` liefert ohne `~/Downloads` den Home-Ordner (`/root`); unter Windows und macOS gibt es den Ordner immer. Im Runbook vermerkt.

### 2026-10-14 06:58 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** erster Probelauf des Messwerkzeugs.
- **Symptom:** Die dichteste Stelle für die Kamera kam als Kartenmitte (1024, 1024) zurück – Rückfallwert für „keine Personen“.
- **Ursache:** Die Suche lief auf Etage 1 (wie Spike 1.5: Abflugebene). Nach 10 s Aufwärmen bei 3× stehen aber alle 10.000 Personen noch im Erdgeschoss (geprüft: Etage 0 10.000, Etage 1 0, Etage 2 0); Spike 1.5 spulte vorher ungebremst vor, das gibt es im Produktionskern nicht.
- **Lösung:** Übersicht und Kreisen im Erdgeschoss; Test prüft die Etage und den Mittelpunkt des Kreisens. Zweiter Probelauf: dichteste Stelle (1224, 1256) in der Halle.
- **Aufwand:** etwa 5 Minuten.

### 2026-10-14 06:55 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.7 `[IN ARBEIT]`, umgesetzt bis auf Messwerkzeug und Messung (Sessionende 05:32 UTC); PR #20 als Entwurf offen auf diesem Branch.
- **Geplant für diese Session:** fehlenden Logbuch-Vermerk zum Merge von PR #19 übernehmen (Commit `fbf2507` von `claude/gracious-galileo-2reksz`, Regel `project-context.md` Abschnitt 10); Messwerkzeug für 2.7 (Messreihe mit JSON-Ausgabe, Kennzahlen des Takts im Status); danach Messanleitung für den Projektinhaber.
- **Erledigt vor diesem Eintrag:** Cherry-Pick von `fbf2507` als `a4d3521` (Konflikt im Logbuch gelöst, Eintrag chronologisch zwischen 00:05 und 04:47 eingeordnet).
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Start über iOS), Branch `claude/naechster-schritt-2-8-736kf4`. Container frisch: Node 26.11.1 über `/opt/nvm` installiert, `npm ci`, Electron-Binärdatei per `install.js`, `pre-commit install`.
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 06:40 UTC – Entscheidungs-Klasse. 5-Stunden-Fenster `allowed`. Kontextgröße meldet `get_session` mit 0 (nicht aussagekräftig).
- **Warnung:** 2.7 empfiehlt die Routine-Klasse; der Schritt läuft oberhalb der Empfehlung. Keine Abgabe an Unteragenten, weil der Probelauf offen ist.

### 2026-10-14 05:32 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 65 Minuten seit Sessionstart 04:28 UTC (Eintrag `[SESSIONSTART]` 04:47).
- **Bearbeitet:** Fragen zu 2.8 (Zellgröße im Original, Personen auf dem Vorfeld); neuer Entwurfsschritt 3.7; 2.7 – Brücke und Plattformschicht.
- **Erreicht:** 3.7 im Fahrplan (vor 3.1). ADR-024 freigegeben (napi `serde-json`, `serde_json =1.0.151`). 2.7 umgesetzt bis auf Messwerkzeug und Messung: Bildformat S-1 mit Dreifachpuffer, Port-Protokoll mit Handshake und Sitzungen, Debug-Befehle, S-7 mit Test-Tabelle, napi-Klasse `Core`, Port-Vermittlung und Neustart im Hauptprozess, Preload S-6 mit Absenderprüfung, PixiJS-Spielfeld, Konsole und Leistungsanzeige. Kern 96 Unit-Tests, TypeScript 120 Tests (Coverage 94 % Lines), Rauchtest lokal grün. CI-Lauf 38516083109 (`4edcdab`) grün auf allen Jobs.
- **Offen geblieben:** 2.7 bleibt `[IN ARBEIT]` – Messwerkzeug (Messreihe mit JSON-Ausgabe, Kennzahlen des Takts im Status) und Messung auf Referenz-PC und Mac. CI-Lauf 38516637456 für `86bb713` (S-7) lief bei Sessionende noch; die Doku-Commits dieser Session lösen einen weiteren Lauf aus. Kein PR geöffnet (nicht angefragt). Kein CHANGELOG im Repository; Einträge ab dem ersten Release.
- **Nächster Schritt:** 2.7 abschließen: Messwerkzeug bauen, dem Projektinhaber die Messanleitung geben, Messwerte eintragen, Brücke und Plattform befördern (Auslöser 4 → Entscheidungs-Klasse).
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, laut `get_session` 04:47 und 05:28 UTC eingestellt und bedient) für die ganze Session. Freigabe ADR-024 auf der Entscheidungs-Klasse (Pflicht). Die Umsetzung von 2.7 (Empfehlung Routine) lief oberhalb der Empfehlung: 1 Schritt; keine Abgabe an Unteragenten, weil der Probelauf offen ist (`templates/wirtschaftlichkeit.md`). Eine Warnung zu Beginn des Schritts fehlte – Abweichung von `CLAUDE.md` Abschnitt 0, hier nachgetragen.
- **Kontextgröße:** nicht verlässlich feststellbar: `get_session` meldete 169.106 Token zu Beginn und am Ende unverändert (Wert offenbar nicht aktualisiert). Eigene Schätzung aus dem verbrauchten Kontingent der Session: rund 340.000 Token, unter der Grenze von 400.000, aber nahe daran – die nächste Arbeit in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf (Electron 44: Vorlauf ab 2027-01-02); Runner-Umstellung auf Ubuntu 26 ab 2026-10-19 in Q.1 zu beobachten.
- **Drift-Prüfung:** README nachgezogen (Phase, Quick Start, Verwendung, Nächste Schritte); ADR-024 verweist auf den vorhandenen Schritt 2.7; Reifegrade unverändert (Brücke und Plattform bleiben `[VORLÄUFIG]` bis zum Abschluss); Reaktiv-Quote 4 / 10 (ADR-015 bis ADR-024) stimmt mit den Tags; Modul-Liste unverändert (neue Kern-Dateien `frame`, `port`, `host`, `query` gehören zu Brücke bzw. Takt & Weltzustand, `architecture.md` Abschnitt 3 und 4 nachgezogen); keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten, Phase 3 ohne festgehaltenen Schrittplan; Größen: `project-context.md` 366 Zeilen, Logbuch unter 800.

### 2026-10-14 05:28 (UTC) – [BEOBACHTUNG]

- **Abfrage S-7 im Container:** Rundlauf im Fenster 136–187 ms für 10.000 Zeilen (Fenster 1.000), kernseitig aber nur 1,7–1,9 ms (direkt über das native Modul gemessen); ein Bild kodieren 0,1 ms. Der Rest ist der Renderer, der ohne Grafikkarte auf der CPU zeichnet (PixiJS fällt ohne WebGL auf Canvas zurück). Aussagekräftig nur auf echter Grafik (Messung durch den Projektinhaber).
- **Brücken-Logik im Kern-Crate:** Port-Zustand, Ereignisauslieferung und Abfragen liegen in `core/src/port.rs`, `host.rs`, `query.rs` statt in der Brücken-Crate, weil diese keine cargo-Tests hat (`test = false`). Kein napi im Kern, keine Spiellogik in der Brücke – Regel-006 gewahrt; in `architecture.md` (Modul Brücke) beschrieben.

### 2026-10-14 05:22 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** PixiJS im Container, Sichtprüfung mit 10.000 Personen.
- **Symptome:** (1) Personen in der Übersicht unsichtbar (0,8 Zellen bei 0,38 Pixel je Zelle); (2) Leistungsanzeige nannte keine GPU.
- **Ursachen:** (1) feste Größe in Zellen; (2) ohne Grafikkarte nutzt PixiJS den Canvas-Renderer, es gibt keinen WebGL-Kontext.
- **Lösung:** (1) Mindestgröße 2 Pixel, Skalierung bei Zoom; (2) Anzeige von WebGL-Version und GPU aus dem Canvas-Kontext, sonst „renderer canvas". Mit `--enable-unsafe-swiftshader` läuft WebGL2 samt `ParticleContainer` unter der strengen CSP.
- **Aufwand:** etwa 10 Minuten.

### 2026-10-14 05:15 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** Rauchtest, Neuladen der Seite.
- **Symptom:** Nach dem Neuladen kam keine Sitzung 2 an.
- **Ursache:** Der Renderer registrierte den Empfang des Ports erst nach dem asynchronen Start von PixiJS; der Port kam vorher und ging verloren.
- **Lösung:** Empfang sofort beim Laden registrieren, Ports bis zum Start puffern. Der Rauchtest prüft das Neuladen jetzt mit.
- **Aufwand:** etwa 5 Minuten.

### 2026-10-14 05:10 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** Werkzeuge im frischen Container.
- **Symptome:** (1) `node` war 22.22, die Shell vergisst `nvm use` zwischen Aufrufen; ein `npm install` mit npm 10 entfernte die `libc`-Felder aus `package-lock.json`. (2) Electron-Binärdatei fehlte nach `npm ci` (Installationsskript nicht ausgeführt). (3) `cargo-deny` nicht installiert.
- **Lösung:** (1) Sperrdatei zurückgesetzt und mit Node 26.11.1 neu erzeugt; jeder Befehl lädt nvm. (2) `node node_modules/electron/install.js`. (3) Binärdatei 0.20.2 aus dem GitHub-Release.
- **Aufwand:** etwa 10 Minuten.

### 2026-10-14 04:53 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-024 – Umwandlung JS ↔ Rust in der Brücke mit napi `serde-json`.
- **Tag:** `[OPERATIV]` (in ADR-021 für 2.7 vorgesehen).
- **Auslöser:** Freigabe des Projektinhabers („A, serde_json 1.0.151“) auf der Entscheidungs-Klasse (`claude-opus-5-5`, `get_session` 04:47 UTC). Reaktiv-Quote 4 / 10 (ADR-015 bis ADR-024).

### 2026-10-14 04:51 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.7, Kategorie 3 (Externe Abhängigkeiten); Eskalations-Auslöser 1 – aktive Klasse bereits Entscheidung, kein Stopp nötig.
- **Frage:** Umwandlung JS ↔ Rust: A napi `serde-json` (Empfehlung), B JSON-Text über die Grenze, C von Hand. Version: `serde_json` 1.0.151 statt der drei Tage alten 1.0.154 (crates.io, geprüft 2026-10-14).
- **Stand:** 2.7 `[IN ARBEIT]`.

### 2026-10-14 04:49 (UTC) – [BEOBACHTUNG]

- **Pre-Commit-Hook fehlte im frischen Container:** `.git/hooks/pre-commit` war nicht installiert; der Commit `d0c41e2` (nur Doku) lief ohne Hook. Nachgeholt mit `pip install pre-commit && pre-commit install` und `pre-commit run --files docs/fahrplan.md docs/logbuch.md`: alle Prüfungen grün. Runbook Schritt „pre-commit install“ gilt auch für Cloud-Sessions.

### 2026-10-14 04:48 (UTC) – [BEOBACHTUNG]

- **Personengruppen auf dem Vorfeld nicht geregelt:** ADR-020 („Personen betreten Vorfeld, Rollwege und Bahnen nicht; Ausnahme Vorfeld-Gehwege“) ist mit Blick auf Fluggäste formuliert. Der Kern kennt bisher nur eine Personenart mit den Zonenregeln der Fluggäste (`core/src/grid.rs`, `Zone`). Nicht betrachtet: Vorfeldagenten (TURN-03, GEP-06, GATE-06), Servicetechniker, übriges Personal mit Personalzonen (ZON-02), Franchise-Personal, Bauarbeiter (BAU-03); im Inventar fehlen Crew und Einweiser.
- **Erfahrung des Projektinhabers mit dem Original:** Vorfeldagenten kamen nur per Servicewagen über eine Straße vom Terminal zum Stellplatz; ohne diese Straße fand die Service-Runde nie statt, ohne dass das Spiel die Ursache nannte. Bauarbeiter liefen ohne Kollision über laufende Bahnen. Wunsch: Fluggäste nur auf dem Weg vom Gate zur Parkposition auf dem Vorfeld, und dann kein Fahrzeug über diese Querung.
- **Folge:** Entwurfsschritt 3.7 vor 3.1 angelegt (Fahrplan, Replanning-Historie). 2.7 bis 2.10 sind nicht betroffen.

### 2026-10-14 04:47 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.3 erledigt (ADR-023), PR #19 nach `main` gemergt (`70d29e5`), Sessionende-Commit `998e941` darin enthalten; nächster Schritt 2.7.
- **Geplant für diese Session:** Fragen des Projektinhabers zu 2.8 (Zellgröße im Original, Personen auf dem Vorfeld); Schritt 3.7 anlegen; 2.7 beginnen, zuerst Freigabe der Abhängigkeit für JS ↔ Rust (Kategorie 3).
- **Vorabprüfung:** 2.7 setzt S-1, S-2, S-3, S-6, S-7 `[BELASTBAR]` voraus – erfüllt (`architecture.md` Abschnitt 9); Module Brücke und Plattform gehen nach Regel-007 `[VORLÄUFIG]` in ihre erste Umsetzung. Keine aktiven Blocker. Reaktiv-Quote 4 / 10 über dem Schwellenwert, Reflexion liegt vor (Fahrplan „Iterations-Reflexion“).
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Start über iOS), Branch `claude/naechster-schritt-2-8-736kf4` (vom Werkzeug vergeben) auf Stand `main`.
- **Modell:** eingestellt und bedient `claude-opus-5-5` laut `get_session` 04:47 UTC – Entscheidungs-Klasse. Kontext 169.106 Token; 5-Stunden-Fenster `allowed`.
- **Hinweis:** Der Eintrag entstand erst nach drei reinen Auskunftsfragen ohne Dateiänderung (2.8, Zellgröße im Original mit Websuche, Personengruppen), nicht direkt nach der Mindest-Lektüre.

### 2026-10-14 00:11 (UTC) – [BEOBACHTUNG]

- **PR Paddel87/OpenAirside#19 gemergt** (`70d29e5`, Merge-Commit) auf Anweisung des Projektinhabers („merge wenn grün“); vorher alle 39 Prüfläufe auf `a10dc6f` grün, PR konfliktfrei. Beobachtung des PR beendet. Dieser Eintrag liegt nach `project-context.md` Abschnitt 10 nur auf dem Branch und geht mit dem PR des nächsten Schritts (2.7) nach `main`.

### 2026-10-14 00:05 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 50 Minuten seit `[SESSIONSTART]` 2026-10-13 23:16 UTC.
- **Bearbeitet:** 2.3 – Weltzustand, Datenmodell und Wegweiser.
- **Erreicht:** 2.3 `[ERLEDIGT]`; Freigabe Kategorie 7 (ADR-023, Option A); CI auf `01f9779` grün auf allen 17 Jobs; erzwungener Fehlerfall des neuen Benchmark-Schritts belegt (Lauf 38486734288); Modul Welt & Bau (Grundraster) `[BELASTBAR]`; FR-046 erledigt; Block-Speicherung aus ADR-020 bestätigt.
- **Offen geblieben:** kein PR geöffnet (nicht angefragt); Branch `claude/gracious-galileo-2reksz` liegt mit 2.3 und dem Logbuch-Commit `998e941` vor `main`. Kein CHANGELOG: 2.3 ändert nichts für Spieler. Der Trend-Vergleich des Wegweiser-Benchmarks zeigt erst nach dem ersten Lauf auf `main` Werte.
- **Nächster Schritt:** 2.7 – Brücke und Plattformschicht (Routine-Klasse empfohlen; Freigabe der Abhängigkeit für JS ↔ Rust ist Entscheidung). Dort festlegen: Standardaufteilung `threads`/`background_threads` nach Kernzahl (Summe höchstens Kernzahl), Prüfung der Schwelle der Rückstandsmeldung.
- **Modell-Bilanz:** Routine-Klasse (`claude-sonnet-5-5`) von 23:16 bis 23:50 UTC für die Umsetzung (empfohlen: Routine, 0 Schritte oberhalb); Entscheidungs-Klasse (`claude-opus-5-5`, laut `get_session` 23:50 UTC eingestellt und bedient) ab 23:50 UTC für Freigabe und Beförderung (Pflicht-Eskalation, Auslöser 1 und 4). Oberhalb der Empfehlung lief dabei Routine-Arbeit auf der Entscheidungs-Klasse: Optimierung der Nachbarzugriffe, CI-Einbau und Dokumentation (1 Schritt teilweise oberhalb; keine Abgabe an Unteragenten, weil der Probelauf offen ist; Warnung zu Beginn entfiel, weil der Schritt auf der Routine-Klasse begann).
- **Kontextgröße:** 529.449 Token laut `get_session` 23:50 UTC, über der Grenze von 400.000 – die nächste Arbeit in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed`.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf; Runner-Umstellung auf Ubuntu 26 ab 2026-10-19 weiter in Q.1 zu beobachten.
- **Drift-Prüfung:** README synchron (Phase, Architektur-Reife, Nächste Schritte 2.7/2.4, Blocker 0); ADR-023 verweist auf den vorhandenen Schritt 2.3; Reifegrade passend (Welt & Bau durch Umsetzung, Regel-007); Reaktiv-Quote 4 / 10 stimmt mit den Tags von ADR-014 bis ADR-023; Modul-Liste unverändert (neue Dateien gehören zu bestehenden Modulen); keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten; Größen: `project-context.md` 364 Zeilen, Logbuch unter 800.

### 2026-10-14 00:04 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Modul Welt & Bau (Grundraster).
- **Wechsel:** `[VORLÄUFIG]` → `[BELASTBAR]`.
- **Auslöser:** Umsetzung in 2.3 mit grüner CI (Regel-007), auf der Entscheidungs-Klasse vorgenommen (Auslöser 4); Bauen mit Wirkung folgt in 3.1, Verkehrslogik der Wegenetze in 3.4.
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja.

### 2026-10-14 00:02 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** erster CI-Lauf mit dem Wegweiser-Benchmark (Lauf 38486734288, erzwungener Fehlerfall).
- **Symptom:** Neuberechnung auf dem GitHub-Runner 3.589 ms gegen 1.646 ms lokal; bis zur Grenze von 4 s nur 10 % Abstand – Gefahr eines unzuverlässigen Gates.
- **Ursache:** langsamerer Runner; in der Berechnung wurden die Nachbarn einer Zelle für die Eckregel zweimal zusätzlich gesucht, jede Suche mit Division, und die Warteschlange forderte bei jedem Schritt neuen Speicher an.
- **Lösung:** Nachbarn innerhalb eines Blocks ohne Division, acht Nachbarn einmal je Zelle, Puffer wiederverwendet (`af84dfb`); Ergebnis unverändert (Tests gegen die dichte Referenz und bitgleich über Threadzahlen grün). Runner 2.484 ms (38 % Abstand), lokal 1,13–1,23 s. Die Grenze aus ADR-023 blieb unverändert – eine Änderung binnen 14 Tagen wäre ein reaktiver ADR gewesen.
- **Aufwand:** etwa 10 Minuten.

### 2026-10-14 00:00 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-023 – Wegweiser-Benchmark als Schritt im CI-Job „Kern-Benchmark“.
- **Tag:** `[OPERATIV]` (in ADR-020 und im Fahrplan vorgesehen).
- **Auslöser:** Freigabe des Projektinhabers („Option A, Wechsel auf Opus gemacht“) nach dem Stopp 23:50 UTC; Wechsel auf die Entscheidungs-Klasse per `get_session` bestätigt (eingestellt und bedient `claude-opus-5-5`). Reaktiv-Quote 4 / 10 (ADR-014 bis ADR-023).

### 2026-10-13 23:50 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.3, Kategorie 7 (Build- und Deploy-Pipeline) und Beförderung Welt & Bau; Eskalations-Auslöser 1 und 4 aus `CLAUDE.md` Abschnitt 0 – aktive Klasse Routine, Stopp-Block `STOPP – MODELLWECHSEL ERFORDERLICH` ausgegeben.
- **Frage:** Soll der Wegweiser-Benchmark (`core/benches/flowfield.rs`) als zusätzlicher Schritt im CI-Job „Kern-Benchmark“ laufen und bei Neuberechnung > 4 s oder Schrittzeit p99 > 4 ms den Merge sperren? A (Empfehlung): zusätzlicher Schritt im bestehenden Job, kein neuer Pflicht-Check; B: eigener Job (neuer Pflicht-Check im Ruleset); C: nur lokal.
- **Stand:** 2.3 `[IN ARBEIT]`, Arbeit angehalten bis zur Antwort auf der Entscheidungs-Klasse. Code, Tests und Doku sind committet; CI-Änderung und Beförderung stehen aus.

### 2026-10-13 23:48 (UTC) – [BEOBACHTUNG]

- **Threadaufteilung:** Der Wegweiser-Benchmark zeigte mit 4 Simulations- und 3 Hintergrund-Threads auf 4 Kernen einen schwankenden p99-Wert des Schritts (2,8 bis 4,6 ms, Höchstwert bis 28 ms), weil mehr Threads als Kerne um die CPU konkurrieren. Mit 2 + 2 Threads: p99 0,74–0,92 ms, Höchstwert 4,3–4,9 ms. Der Schritt selbst braucht nur etwa 0,1 ms Rechenzeit; ein großer Tick-Pool bringt nichts. Folgerung: `background_threads` ist eine eigene Einstellung; die Standardaufteilung legt die Brücke (2.7) nach Kernzahl fest, in der Summe höchstens so viele Threads wie Kerne.
- **Übernahme-Schritt:** Das Freigeben der alten Wegweiser (100 MB) im Tick-Thread kostete bis zu 5 ms; seit der Freigabe im Hintergrund unter 1 ms.
- **Messung ohne Echtzeit-Takt:** Eine Schleife ohne Takt lief in 0,25 s durch und sah nie eine Übernahme; der Benchmark läuft deshalb im Takt von 3× (180 Schritte/s).
- **Rückstandsmeldung:** Höchstwerte von 4 bis 5 ms eines Schritts (bei 3× sind 5,5 ms Budget) liegen an der Grenze, ab der `simulationLag` (3 Ticks) melden würde; Prüfung in 2.7 bleibt vorgemerkt.

### 2026-10-13 23:35 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** Tests des Rasters und des Referenz-Terminals (2.3).
- **Symptome:** (1) Zwei Raster-Tests schlugen fehl; (2) im Terminal wurde eine Person der Startreihe nicht erzeugt; (3) ein Test des Weltzustands erwartete die falsche Laufrichtung.
- **Ursachen:** (1) Der Tabellenindex 0 gehört zur Etage −2, nicht zum Erdgeschoss; (2) einzelne Zellen der Eingangsreihe sind Innenwände (alle 64 Zellen); (3) das Ziel 0 liegt hinter der neu gebauten Wand, erreichbar ist Ziel 1.
- **Lösung:** (1) und (3) im Test korrigiert, (2) die Startzellen werden nur aus begehbaren Zellen gewählt. Kein Fehler in der Berechnung der Wegweiser: Die Distanzen stimmten von Anfang an mit der dichten Referenz überein.
- **Aufwand:** etwa 10 Minuten.
- **Lerneffekt:** Tabellen- und Etagenindex trennen (`GROUND_INDEX`).

### 2026-10-13 23:20 (UTC) – [BEOBACHTUNG]

- **Determinismus-Test:** Der Test mit Terminal und drei Neuberechnungen brauchte im Debug-Build 63 s (4 Läufe × 4 Berechnungen); verkleinert auf 3 Läufe × 3 Berechnungen (34 s Debug, 4 s Release).
- **Abhängigkeiten:** Kein `serde_json` für den Schema-Test von FR-046 verwendet (neue Abhängigkeit, erst in 2.7 zur Freigabe); der Test prüft das Schema über Typen, die TypeScript-Prüfung der CI sichert den erzeugten Typ `LoadKind`.
- **Tick-Benchmark (ADR-022):** misst seit 2.3 im Referenz-Terminal mit Wegweisern statt der künstlichen Bewegung; Werte unverändert niedrig (Mittel 0,13 ms, p99 0,43 ms), Grenzen unverändert.

### 2026-10-13 23:16 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 22:52 UTC plus Nachträge bis 23:08 UTC (2.2 erledigt, 2.2-N1 erledigt, PR #18 gemergt, `be53a27`). Branch `claude/gracious-galileo-2reksz` auf `main` übernommen (Merge `5b5b217`, kein Force-Push); der Logbuch-Commit `998e941` bleibt nach `project-context.md` Abschnitt 10 auf dem Branch und geht mit dem PR von 2.3 nach `main`.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht); die Dokumente stammen aus derselben laufenden Claude-Code-Sitzung und wurden vor Beginn gegen den Stand von `main` abgeglichen.
- **Geplant für diese Session:** 2.3 – Weltzustand, Datenmodell und Wegweiser. Vertiefung: Modul-Einträge Welt & Bau und Wegfindung, ADR-010, ADR-020, `spikes/1.2-wegweiser/src/field.rs`, `core/src/world.rs`, `core/src/adoption.rs`.
- **Vorabprüfung:** Eingangskriterien erfüllt (Datenmodell `[BELASTBAR]`, Weltmaßstab ADR-020, 2.2 erledigt). **Eskalations-Auslöser vorab erkannt:** Die Beförderung des Moduls Welt & Bau von `[VORLÄUFIG]` auf `[BELASTBAR]` (Reifegrad-Wirkung von 2.3) ist Auslöser 4 aus `CLAUDE.md` Abschnitt 0 und braucht die Entscheidungs-Klasse. Die Umsetzung läuft auf der Routine-Klasse; vor der Beförderung gibt es einen Stopp-Block.
- **Modus/Werkzeug:** Claude Code, Cloud-Session, Berechtigungsmodus `auto`; Modellwechsel durch den Projektinhaber per `/model` von `claude-opus-5-5` auf `claude-sonnet-5-5`.
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` (Quelle: `get_session` 23:14 UTC) → Routine-Klasse. Kontingent: 5-Stunden-Fenster `allowed`. Kontextgröße zum Start 415.860 Token, über der Grenze von 400.000: Empfehlung, die Arbeit in einer frischen Session fortzusetzen (kein Stopp).

### 2026-10-13 23:08 (UTC) – [BEOBACHTUNG]

- **PR Paddel87/OpenAirside#18 gemergt** (`be53a27`, Merge-Commit) durch den Projektinhaber; vorher alle 39 Prüfläufe auf `3aec3dc` grün, PR konfliktfrei. Beobachtung des PR beendet, Sicherheits-Check-in gelöscht. Dieser Eintrag liegt nach `project-context.md` Abschnitt 10 nur auf dem Branch und geht mit dem PR des nächsten Schritts (2.3) nach `main`.

### 2026-10-13 23:02 (UTC) – [BEOBACHTUNG]

- **Ruleset `main-schutz`:** Der Projektinhaber hat „Require branches to be up to date before merging" ausgeschaltet. Geprüft über `rules/branches/main`: `strict_required_status_checks_policy: false`, 19 Pflicht-Checks inklusive „Kern-Benchmark". Fahrplan 2.1-N1 („Branch nicht zwingend aktuell") entspricht damit wieder dem Ist-Zustand.

### 2026-10-13 22:58 (UTC) – [BEOBACHTUNG]

- **2.2-N1 erledigt:** Der Projektinhaber hat „Kern-Benchmark" als Pflicht-Check im Ruleset `main-schutz` eingetragen. Geprüft über die öffentliche Schnittstelle `rules/branches/main`: 19 Pflicht-Checks, „Kern-Benchmark" mit Quelle GitHub Actions. Mini-Reibung: Die Liste der Checks war in der Ruleset-Ansicht eingeklappt („Show additional settings").
- **Abweichung:** Im Ruleset ist „Require branches to be up to date" aktiv, bei 2.1-N1 steht „Branch nicht zwingend aktuell". Rückfrage an den Projektinhaber gestellt; der Fahrplan bleibt bis zur Antwort unverändert.

### 2026-10-13 22:52 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 35 Minuten seit `[SESSIONSTART]` 22:17 UTC.
- **Bearbeitet:** 2.2 – Simulationskern (Takt, Zeitsteuerung, Befehle, Ereignisse, Prüfmodus).
- **Erreicht:** Freigabe Kategorie 7 (ADR-022, A/A); 2.2 `[ERLEDIGT]`, CI-Lauf 38480239819 auf allen 17 Jobs grün; Modul Takt & Weltzustand `[BELASTBAR]`; erzwungener Fehlerfall des Benchmark-Gates belegt (Lauf 38479926291).
- **Offen geblieben:** Nachlauf 2.2-N1 (Projektinhaber trägt „Kern-Benchmark" im Ruleset `main-schutz` ein); kein PR geöffnet (nicht angefragt), Branch liegt mit den Sessionende-Commits der Vorsession und 2.2 vor `main`. Der Trend-Vergleich mit `main` zeigt erst nach dem ersten Lauf auf `main` Werte. CHANGELOG nicht angelegt: 2.2 ändert nichts für Spieler.
- **Nächster Schritt:** 2.3 – Weltzustand, Datenmodell und Wegweiser (Routine-Klasse empfohlen). Vertiefung: ADR-010, ADR-020, `spikes/1.2-wegweiser/src/field.rs`, `core/src/world.rs` und `core/src/adoption.rs` aus 2.2.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 22:50 UTC); 2.2 empfiehlt Entscheidung (Freigabe), 0 Schritte oberhalb der Empfehlung; Umsetzungsteil wäre abgebbar gewesen, aber keine Abgabe, weil der Probelauf für Unteragenten noch offen ist.
- **Kontextgröße:** nicht feststellbar (`get_session` meldet `used_tokens` 0); Hinweis entfällt.
- **Kontingent:** 5-Stunden-Fenster `allowed`.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf; Runner-Umstellung auf Ubuntu 26 ab 2026-10-19 weiter in Q.1 zu beobachten; neue benannte Ausnahme `syn@2.0.119` in Q.1 aufgenommen.
- **Drift-Prüfung:** README synchron (Phase, Architektur-Reife, Nächste Schritte, Blocker 0); ADR-022 verweist auf vorhandene Schritte (2.2, 2.3); Reifegrad Takt & Weltzustand passend zu Regel-007 und 2.2; Reaktiv-Quote 4 / 10 stimmt mit den Tags von ADR-013 bis ADR-022; Modul-Liste unverändert; keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten; Größen: `project-context.md` 363 Zeilen, Logbuch unter 800.

### 2026-10-13 22:51 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Modul Takt & Weltzustand.
- **Wechsel:** `[VORLÄUFIG]` → `[BELASTBAR]`.
- **Auslöser:** Umsetzung in 2.2 mit grüner CI (Regel-007); Weltzustand mit Raster und Wegweisern wird in 2.3 bestätigt.
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja.

### 2026-10-13 22:50 (UTC) – [ONBOARDING-VALIDATION]

- **Anlass:** Runbook Schritt 5 um `npm run check:types` und den Kern-Benchmark ergänzt (Quick-Start-relevant).
- **Durchführung:** frischer Worktree von `ea32484`, `npm ci`, `npm run check:types` („matches"), `cargo bench -p openairside-core --bench tick` (Mittel 0,115 ms, p99 0,248 ms).
- **Ergebnis:** grün, keine Befunde.

### 2026-10-13 22:49 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** erster CI-Lauf mit dem Job „Kern-Benchmark" (38479926291, erzwungener Fehlerfall).
- **Symptom:** „No files were found with the provided path: bench-tick.json"; Zusammenfassung ohne Ergebnis.
- **Ursache:** `cargo bench` startet das Messprogramm im Paketordner `core/`; der relative Pfad zeigte dorthin.
- **Lösung:** absoluter Pfad `$GITHUB_WORKSPACE/bench-tick.json` in `ci.yml` (`7547424`), Hinweis im Kopf von `core/benches/tick.rs`. Folgelauf: Artefakt `bench-tick` abgelegt.
- **Mini-Reibung dabei:** `git revert` des Fehlerfall-Commits hat keinen Commit erzeugt, das anschließende `git commit --amend` hat den schon gepushten Commit `ea32484` lokal umgeschrieben. Der Push wurde als Nicht-Fast-Forward abgelehnt, auf dem Remote hat sich nichts geändert. Lokal per `git reset --soft` auf den Remote-Stand zurückgesetzt, Revert neu erzeugt (`9b3d4f7`), kein Force-Push. Lehre: nach `git revert` erst `git log` prüfen, dann ändern.

### 2026-10-13 22:47 (UTC) – [BEOBACHTUNG]

- **Rückstandsmeldung S-3:** Der Vertrag meldet `simulationLag` ab 1 Spielsekunde Rückstand, das sind nur 3 Ticks (bei 3× rund 17 ms Echtzeit). Spike 1.5 zeigte ohne wachsenden Trend Rückstände bis 8 Ticks (Referenz-PC) bzw. 33 Ticks (Mac). Umgesetzt ist der Vertrag wie festgelegt; ob die Schwelle im Spiel Fehlalarme auslöst, prüft 2.7 bei der Messung auf Referenz-PC und Mac (im Fahrplan vermerkt). Bei Bedarf wäre die Änderung eine Vertragsänderung (Kategorie 5).
- **Festlegungen der Umsetzung (additiv, `architecture.md` Abschnitt 4):** Kalender (Tick 0 = Tag 1 des Jahres 1, 00:00, Start-Tick je Sitzung), Codes `notNight`, `nightSkipBlocked`, `tickLimit`, Textschlüssel-Schema, Ende des Rückstaus bei höchstens 4.096 wartenden Ereignissen, Rückstand nach Arbeitsscheiben von 16 ms und Obergrenze von einer Sekunde.
- **Befehl `debug`** ist nicht in 2.2 umgesetzt, weil seine Aktionen erst mit dem Debug-Panel feststehen; Landeplatz 2.7.
- **npm audit:** 8 Funde der Stufe „moderate" (unverändertes Lockfile, Schwelle „high" grün); keine Maßnahme nötig, Prüfung mit Q.1.

### 2026-10-13 22:31 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-022 – Kern-Benchmark als CI-Gate und Prüfung der erzeugten TypeScript-Typen.
- **Tag:** `[OPERATIV]` (in 2.2 als Freigabe geplant).
- **Auslöser:** Freigabe des Projektinhabers per Auswahldialog: Frage 1 „A: feste Grenze", Frage 2 „A: einchecken + prüfen". Reaktiv-Quote jetzt 4 / 10 (ADR-013 bis ADR-022), weiter über dem Schwellenwert, sinkend.
- **Mini-Reibung:** Der Commit `abfafd5` entstand ohne Pre-Commit-Hook, weil der frische Container ihn nicht installiert hatte; Hooks danach auf beiden Dateien nachgeholt (grün) und `pre-commit install` ausgeführt. Für neue Cloud-Sessions: Hook-Installation gehört zum Sessionstart.

### 2026-10-13 22:25 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 2.2, Kategorie 7 (Build- und Deploy-Pipeline), Eskalations-Auslöser 1 – aktive Klasse Entscheidung, kein Modellwechsel nötig.
- **Frage 1, Kern-Benchmark:** A absolute Grenze aus dem Echtzeit-Budget als Pflicht-Check auf Linux, Trend nur als Anzeige (Empfehlung); B zusätzlich rot bei relativer Verschlechterung gegenüber `main`; C nur nächtlich bzw. von Hand, kein Merge-Gate; D externer Messdienst (Zählung von Instruktionen).
- **Frage 2, ts-rs-Typen:** A Typen erzeugen, mit Prettier formatieren, Abweichung vom eingecheckten Stand färbt den bestehenden Job rot (Empfehlung); B Typen nur im Build erzeugen, nicht einchecken (weicht von ADR-021 ab).
- **Vertiefung:** `architecture.md` Abschnitt 4 (Grundlagen, S-2), ADR-020, ADR-021, `ci.yml`, Spike 1.5 (Schritt 0,25–0,29 ms bei 10.000 Personen im Container mit 4 Kernen).
- **Stand:** 2.2 `[WARTET-AUF-FREIGABE]`, Arbeit angehalten bis zur Antwort.

### 2026-10-13 22:17 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 21:03 UTC plus `[BEOBACHTUNG]` 21:56 UTC (PR Paddel87/OpenAirside#17 gemergt, `118fe75`). Branch `claude/gracious-galileo-2reksz` steht auf `main` plus zwei Sessionende-Commits (`54b2e13`, `3f46cf6`), die nach `project-context.md` Abschnitt 10 mit dem PR von 2.2 nach `main` gehen.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]` 21:03, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant für diese Session:** 2.2 – Simulationskern; zuerst Freigabe Kategorie 7 (Benchmark-Job mit Leistungsschwelle, CI-Prüfung der ts-rs-Typen).
- **Vorabprüfung:** Eingangskriterien von 2.2 erfüllt (S-2, S-3 `[BELASTBAR]` nach ADR-021; Zeitmodell ADR-020); Modul Takt & Weltzustand darf `[VORLÄUFIG]` in die erste Umsetzung gehen (Regel-007). Reaktiv-Quote 5 / 10 weiter über dem Schwellenwert, Reflexion liegt vor.
- **Modus/Werkzeug:** Claude Code, Cloud-Session, Berechtigungsmodus `auto`; Auftrag aus der Vorsession („neue Session für 2.2 starten").
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: `get_session` 22:17 UTC) → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.

### 2026-10-13 21:56 (UTC) – [BEOBACHTUNG]

- **PR Paddel87/OpenAirside#17 gemergt** (`118fe75`, Merge-Commit) auf Anweisung des Projektinhabers („merge"); vorher CI auf dem PR-Stand grün und ohne Konflikt. Rückfrage des Projektinhabers: Antworten vom iPhone zum ersten Auswahldialog kamen in der Session nur als „Denied by user" an; inhaltlich gleich mit dem zweiten Durchgang, keine Änderung an ADR-021. Dieser Eintrag liegt nach der Festlegung zu Sessionende-Commits (`project-context.md` Abschnitt 10) nur auf dem Branch und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-13 21:03 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 80 Minuten seit `[SESSIONSTART]` 19:44 UTC.
- **Bearbeitet:** 2.9 – Schnittstellenverträge S-2 bis S-7.
- **Erreicht:** 2.9 `[ERLEDIGT]` mit ADR-021; Verträge in `architecture.md` Abschnitt 4, S-2 bis S-7 `[BELASTBAR]`; getrennte Prüfung erfolgt (13 von 15 Befunden eingearbeitet); Vorbilder per Websuche belegt; Landeplätze in 2.2, 2.4, 2.5, 2.7, 6.1, 7.2.
- **Offen geblieben:** Kein PR geöffnet (nicht angefragt); Branch `claude/gracious-galileo-2reksz` liegt mit 2.9 vor `main`, Merge nur per PR (Ruleset `main-schutz`). Nur Dokumentation geändert, daher keine Code-Gates außer markdownlint (lokal grün).
- **Nächster Schritt:** 2.2 – Simulationskern (Takt, Zeitsteuerung, Befehle und Ereignisse nach ADR-021); zu Beginn Freigabe Kategorie 7 (Benchmark-Job, Prüfung der ts-rs-Typen). Vertiefung: `architecture.md` Abschnitt 4 (Grundlagen, S-2, S-3), ADR-020 (Zeitmodell), ADR-010 (Prüfmodus).
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 21:03 UTC); 2.9 empfiehlt Entscheidung, 0 Schritte oberhalb der Empfehlung; keine Teilarbeit abgegeben (Prüfung durch Unteragent ist keine Abgabe).
- **Kontextgröße:** 314.544 Token laut `get_session`, unter der Grenze von 400.000.
- **Kontingent:** 5-Stunden-Fenster `allowed`.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf; Runner-Umstellung auf Ubuntu 26 ab 2026-10-19 weiter in Q.1 zu beobachten.
- **Drift-Prüfung:** README synchron (Phase, Architektur-Reife, Nächste Schritte, Blocker 0); ADR-021 verweist auf vorhandene Schritte (2.2, 2.4, 2.5, 2.7, 3.1, 6.1, 7.2); Reifegrade passend zu ADR-021; Reaktiv-Quote 5 / 10 stimmt mit den Tags von ADR-012 bis ADR-021; Modul-Liste unverändert (S-7 ist Schnittstelle, kein Modul); keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker; Phase 2 weiter bei 10 Schritten; Größen: `project-context.md` 361 Zeilen, Logbuch unter 800.

### 2026-10-13 21:03 (UTC) – [REIFEGRAD-WECHSEL]

- S-2 Befehlskanal, S-3 Ereignisstrom, S-6 Plattform-API `[VORLÄUFIG]` → `[BELASTBAR]`; S-4 Datenpaket-Schema, S-5 Spielstand-Format `[OFFEN]` → `[BELASTBAR]`; S-7 Abfrage neu `[BELASTBAR]` (ADR-021, Regel-007; Bestätigung durch Umsetzung in 2.2, 2.4, 2.5, 2.7).

### 2026-10-13 21:03 (UTC) – [ADR-ANGELEGT]

- **ADR-021** `[ERKENNTNIS]` (Kategorie 5, 1, 6): Schnittstellenverträge S-2 bis S-7. Freigabe des Projektinhabers per Auswahl: „A freigeben", Pause „Ja", Mod-Fehler „Ganzes Paket aus", alte Spielstände „Ab Version 1.0", fehlendes Paket „Erst verweigern, später Factorio-Weg". Erster Auswahldialog war abgelehnt worden; auf Wunsch des Projektinhabers folgten Anzeige des Entwurfs, belegte Vorbilder und Websuche, dann erneute Frage.
- **Übernahme:** Vertragstext nach `architecture.md` Abschnitt 4 (gemeinsame Grundlagen vor S-1), Modul-Karte um „Abfrage" ergänzt, Reifegrade Abschnitt 9; zwei Unstimmigkeiten des Entwurfs dabei behoben (Rückstau-Regel S-3, fehlende `panel`-Kennung S-7). Landeplätze in 2.2, 2.4, 2.5, 2.7, 6.1, 7.2. Reaktiv-Quote 5 / 10.

### 2026-10-13 20:53 (UTC) – [BEOBACHTUNG]

- **Websuche „weitere hilfreiche Quellen"** (Frage des Projektinhabers): Status des Originals bestätigt – Apog Labs: „development of Airport CEO has concluded", 1.1-3 vom 2026-01-24 nur Wartung (Unity-Update wegen CVE-2025-59489; forum.apoglabs.com, Thema 22277). Name „OpenAirside" im Web nicht belegt (keine Namenskollision gefunden). Kein quelloffenes Flughafen-Tycoon-Spiel gefunden; kommerziell: SimAirport, JetSet. Datenquellen: OurAirports (Public Domain), ICAO-Flugplatz-Referenzcode A–F nach Spannweite (Skybrary). Wegweiser: Game AI Pro (Kapitel kostenlos auf gameaipro.com; Kapitel „Flow Field Tiles" dort nicht verifiziert). napi-rs mit Electron: Beispiel von Dave Ceddia, kein Beispiel mit `utilityProcess` gefunden. Keine Änderung an Plan oder Entwurf.

### 2026-10-13 20:48 (UTC) – [BEOBACHTUNG]

- **Vorbilder 2.9 belegt** (Websuche auf Wunsch des Projektinhabers): Factorio, OpenTTD, OpenRCT2, Minecraft, Cities: Skylines, RimWorld, Electron-Checkliste; Tabelle mit Güte der Quelle im Entwurf. Zwei Behauptungen aus dem Gedächtnis nicht belegbar und gestrichen (Early-Access-Spielstände, Bauen in der Pause bei Cities: Skylines/OpenRCT2).
- **Korrektur:** Electron empfiehlt URL-Prüfung per Parser statt Zeichenkettenvergleich; S-6 im Entwurf ergänzt. `app/src/main/security.ts` (`isAllowedNavigation`) parst zwar, vergleicht dann aber per `startsWith` auf `href` – in 2.7 bei der Absenderprüfung mit ansehen (Fahrplan-Schritt 2.7, Härtungstests).
- **Frage 4** um Option (c) ergänzt: in Phase 2 verweigern, Laden mit Entfernen nach Factorio-Vorbild in 6.1.

### 2026-10-13 19:50 (UTC) – [BEOBACHTUNG]

- **Getrennte Prüfung 2.9 erfolgt** (Unteragent, Sonnet 5.5, nur Entwurf, Bedrohungsmodell, Ist-Code): 15 Befunde (3 hoch, 8 mittel, 3 niedrig, 1 optional). Eingearbeitet: 1–12 und 14 (u. a. Grenzen für Spielstände und Pakete, Pfadprüfung, Absenderprüfung per `senderFrame`, Sitzungen in jeder Nachricht, atomares Laden, Flusssteuerung S-3, Ablehnungs- und Fehlercodes). Nicht übernommen: 13 (Prüfer rechnete mit 9 statt 180 Ticks/s bei 3×) und 15 (Signatur, über dem Schutzbedarf). Befund im Entwurf vermerkt.

### 2026-10-13 19:48 (UTC) – [BEOBACHTUNG]

- **Entwurf 2.9:** `docs/research/2.9-schnittstellenvertraege.md` angelegt (S-2, S-3, S-7, S-4, S-5, S-6 mit gemeinsamen Grundlagen: ein `MessagePort` mit Handshake, Typen aus Rust per ts-rs, Weltkoordinaten nach ADR-020). Nach Freigabe wandern die Verträge nach `architecture.md` Abschnitt 4.
- **Getrennte Prüfung:** Der Entwurf berührt Kategorie 6 (nicht vertrauenswürdige Spielstände und Pakete, IPC). Prüfung durch einen Unteragenten mit anderem Modell (Sonnet 5.5), der nur Entwurf, Bedrohungsmodell und Ist-Code erhält, nicht den Gesprächsverlauf – keine Abgabe von Arbeit, daher kein Probelauf nötig.
- **Mini-Reibung:** Tick-Reichweite von `u32` im ersten Entwurf falsch überschlagen („45 Spieljahre"); nachgerechnet: 2^32 Ticks zu 1/3 s ≈ 16.500 Spieltage.

### 2026-10-13 19:44 (UTC) – [SESSIONSTART]

- **Letzter Stand:** 2.8 erledigt (ADR-020); PR Paddel87/OpenAirside#16 hat die Sessionende-Commits der Vorsession nach `main` gebracht (`b045477`). Branch `claude/gracious-galileo-2reksz` enthielt keine weiteren Commits und wurde auf `origin/main` neu aufgesetzt.
- **Geplant für diese Session:** 2.9 – Schnittstellenverträge S-2 bis S-7 (Entwurf, Vorlage zur Freigabe Kategorie 5 und 1, ADR).
- **Vorabprüfung:** Eingangskriterium „Weltmaßstab und Zeitmodell aus 2.8 entschieden" erfüllt (ADR-020). Reifegrade vor 2.9: S-1 `[BELASTBAR]`, S-2/S-3/S-6 `[VORLÄUFIG]`, S-4/S-5 `[OFFEN]`, S-7 nicht vorhanden – als Entwurfsschritt (Phasentyp-Kontext ERKUNDUNG) zulässig. Keine aktiven Blocker, keine offene STOPP-Situation.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]` 19:34, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modus/Werkzeug:** Claude Code (Cloud-Session), semi-autonom.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: `get_session` 19:44 UTC) – Entscheidungs-Klasse; 2.9 empfiehlt Entscheidung (Eskalations-Auslöser 1 erfüllt ohne Wechsel).
- **Kontingent:** 5-Stunden-Fenster `allowed`.

### 2026-10-13 19:34 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 34 Minuten seit `[SESSIONSTART]` 19:00 UTC; ersetzt als Wiedereinstiegspunkt den Eintrag 19:22.
- **Bearbeitet:** 2.8 – Weltmaßstab, Luftseite und Zeitmodell; PR Paddel87/OpenAirside#15
- **Erreicht:** 2.8 `[ERLEDIGT]` (ADR-020), PR #15 vom Projektinhaber um 19:30 UTC gemergt (`a3a8b65`). CI auf `main` nach dem Merge (Lauf 38455440587) und Security (38455439937) grün.
- **Nachweis 2.1-N1:** Beim Öffnen des PR zeigte GitHub kurz „Merge nicht möglich" mit einer roten Prüfung, nach Aktualisierung alles grün und Merge möglich – die Regel `main-schutz` sperrt rote Prüfungen. Einzelliste der 18 Pflicht-Checks nicht geprüft.
- **Mini-Reibung:** Rauchtest Electron (macOS) im Push-Lauf 38454141711 rot, weil der Runner das Electron-Binary nicht laden konnte (`TypeError: fetch failed`, vor dem Test). Kein Bezug zum PR; einmal neu gestartet, danach grün (2:15 min). Bei Wiederholung: Electron-Download im Workflow zwischenspeichern (Vorschlag, kein Schritt).
- **Offen geblieben:** Dieser Eintrag liegt nur auf dem Branch `claude/gracious-galileo-2reksz` (neu auf `main` aufgesetzt) und kommt mit dem PR der nächsten Session nach `main`. Neuberechnung in Blöcken nur ersatzweise gemessen, Bestätigung in 2.3.
- **Nächster Schritt:** 2.9 – Schnittstellenverträge S-2 bis S-7 (Entwurfsschritt, Entscheidungs-Klasse, Freigabe Kategorie 5 und 1). Vertiefung: `architecture.md` Abschnitt 4, ADR-020, ADR-009/ADR-010, Spike 1.4 (`readTable`) für S-7, `project-context.md` Abschnitt 6 für S-4/S-5/S-6.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 19:20 UTC); 0 Schritte oberhalb der Empfehlung; keine Teilarbeit abgegeben (Probelauf offen).
- **Kontextgröße:** 285.309 Token laut `get_session` 19:20 UTC, unter der Grenze von 400.000.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Drift-Prüfung:** unverändert gegenüber 19:14; README synchron; keine offene STOPP-Situation.

### 2026-10-13 19:22 (UTC) – [SESSIONENDE]

- **Session-Dauer:** ca. 22 Minuten seit `[SESSIONSTART]` 19:00 UTC; ersetzt als Wiedereinstiegspunkt den Eintrag 19:14 (danach nur die Kenntnisnahme 19:20).
- **Bearbeitet:** 2.8 – Weltmaßstab, Luftseite und Zeitmodell
- **Erreicht:** 2.8 `[ERLEDIGT]` mit ADR-020 (Details im Eintrag 19:14); Reflexion zur Reaktiv-Quote vom Projektinhaber zur Kenntnis genommen, keine offene STOPP-Situation.
- **Offen geblieben:** Branch `claude/gracious-galileo-2reksz` liegt 6 Commits vor `main` (`d9cbf8a` bis `9f2456d`), kein PR geöffnet (nicht angefragt). Merge auf `main` nur per PR (Ruleset `main-schutz`); dabei Nachweis 2.1-N1 (Pflicht-Checks als „Required") prüfen. Neuberechnung in Blöcken nur ersatzweise gemessen, Bestätigung in 2.3.
- **Nächster Schritt:** 2.9 – Schnittstellenverträge S-2 bis S-7 (Entwurfsschritt, Entscheidungs-Klasse, Freigabe Kategorie 5 und 1). Einstieg: zuerst klären, ob der Branch gemergt ist (sonst auf ihm weiterarbeiten bzw. nach Merge neu von `main`); Vertiefung: `architecture.md` Abschnitt 4 (Schnittstellenverträge), ADR-020 (Adressierung in Weltkoordinaten, Zeitmodell), ADR-009/ADR-010 (Übernahme der Wegweiser, Prüfmodus), Spike 1.4 (`readTable`) für S-7, `project-context.md` Abschnitt 6 (Sicherheit: Validierung von Spielständen und Mods) für S-4/S-5/S-6.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 19:20 UTC); 0 Schritte oberhalb der Empfehlung; keine Teilarbeit abgegeben (Probelauf offen).
- **Kontextgröße:** 285.309 Token laut `get_session` 19:20 UTC, unter der Grenze von 400.000. Korrektur zum Eintrag 19:14: der dort genannte Wert war veraltet.
- **Kontingent:** 5-Stunden-Fenster `allowed`.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Drift-Prüfung:** unverändert gegenüber 19:14 (README, ADR-Verweise, Reifegrade, Reaktiv-Quote 6 / 10, Blocker 0, Phase 2 bei 10 Schritten); Fahrplan „Aktueller Stand" ohne offene STOPP-Situation.

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
