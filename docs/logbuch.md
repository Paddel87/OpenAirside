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

Phase 1 abgeschlossen (ADR-016), Detail-Einträge in `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) abgeschlossen am 2026-10-19 mit „gezielt umbauen“ (ADR-036), Detail-Einträge in `docs/archiv/logbuch-phase-2.md`. Phase 3 (Vertikaler Schnitt „Kleiner Flughafen“) ausgeplant mit 13 Schritten, nächster Schritt 3.7.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-20 14:02 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-19 20:03 UTC, danach Merge PR #27 (`119d2b1`) mit Vermerk `d2e6c70` auf `docs/phasenabschluss-2`. Der Vermerk ist nach `project-context.md` Abschnitt 10 auf den Branch dieser Session vom aktuellen `main` übernommen (`0162671`); `docs/phasenabschluss-2` wird nicht weiterverwendet. Die Session „Phasenabschluss Phase 2“ erhielt am 2026-10-20 13:50 UTC „Weiter mit 3.7“ und wurde 11 Sekunden später versehentlich archiviert, ohne dass etwas ausgeführt wurde (Transkript über `list_events` geprüft); 3.7 beginnt hier.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 14:01 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Werkzeug:** Claude Code in der Cloud; vom Werkzeug vergebener Branch `scp/compassionate-shannon-wz4apd` (Typ und Schritt-ID im PR-Titel, `CLAUDE.md` Abschnitt 11).
- **Vorhaben:** Schritt 3.7 Personengruppen, Begehbarkeit und Querungen (Entwurf mit Messung, ADR in Ergänzung von ADR-020). Empfohlene Klasse: Entscheidung (Auslöser 1) – aktive Klasse passt.
- **Vorabprüfung:** Eingangskriterium „Datenmodell im Kern `[BELASTBAR]`“ erfüllt (`architecture.md` Abschnitt 9); Abhängigkeit 2.3 erledigt. Reaktiv-Quote 6 / 10 über dem Schwellenwert, Reflexion für Phase 2 liegt vor (Regel-009).

### 2026-10-19 20:10 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #27 (Phasenabschluss 2) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `119d2b1`. Vorher grün auf `62998bd`: Push-Lauf 39316204108 und die PR-Läufe CI 39316460887 und Security 39316460888 (39 Prüfungen, CodeQL ohne Meldung). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `docs/phasenabschluss-2` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 20:03 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 23 Minuten seit `[SESSIONSTART]` 19:40 UTC.
- **Bearbeitet:** Phasenabschluss 2: Vision-Abgleich, Pflichtfrage mit zwei getrennten Instanzen, Stellungnahme, Vorlage und Freigabe, ADR-036, Onboarding-Re-Validation, Archivierung und `[PHASEN-WECHSEL]`, Phase 3 im vollen Format.
- **Erreicht:** Phase 2 abgeschlossen. Phase 3 hat einen ursprünglichen Schrittplan von 13 Schritten; Regel-008 und Regel-009 gelten. Abschnitt 4 der Architektur ist ausgelagert, FR-052 neu, FR-016 ergänzt.
- **Offen geblieben:** PR für `docs/phasenabschluss-2` nach `main`, nur auf Anweisung. Rückmeldung an das Vorlagen-Repo Dev-Templates zur Zählregel der Reaktiv-Quote: im Fahrplan vermerkt, nicht gesendet, liegt außerhalb dieses Repos.
- **Nächster Schritt:** 3.7 Personengruppen, Begehbarkeit und Querungen, in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 20:03 UTC). 0 Schritte oberhalb der Empfehlung (Phasenabschluss: Entscheidung). Abgegeben: zwei Bewertungen an Unteragenten als getrennte Instanzen (Opus 5.5 und Sonnet 5.5). Das war kein Sparzweck; der Probelauf für die Abgabe an Routine/Mechanik bleibt offen.
- **Kontextgröße:** nicht feststellbar – `get_session` meldet `used_tokens: 0`. Kein Hinweis zur Sessiongröße.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 20:03 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung:** Fahrplan Phase 2 und Logbuch Phase 2 ausgelagert. Logbuch jetzt rund 130 Zeilen, Architektur 461 Zeilen (unter der Schwelle von 500), `project-context.md` 375 Zeilen.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Nächste Schritte, Verweise auf `architecture-schnittstellen.md`).
  - ADR-036 verweist auf vorhandene Schritte (3.1–3.13, 4.1, 4.3).
  - Reifegrade passen zu ADR-036 (keine Änderung).
  - Modulnamen der Phase-3-Schritte stimmen mit der Modul-Karte überein.
  - Jede Muss-Anforderung hat einen Schritt (FR-052 → 3.7, 3.4).
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036) stimmt mit den Tags.
  - Phase 3: 13 Schritte bei Plan 13.

### 2026-10-19 20:02 (UTC) – [PHASEN-WECHSEL]

- **Phase 2 → Phase 3:** Phase 2 (UMSETZUNG, Fundament) mit 11 von 11 Schritten abgeschlossen; Phase 3 (UMSETZUNG, vertikaler Schnitt) mit ursprünglichem Schrittplan von 13 Schritten ausgeplant (ADR-036). Detail-Einträge von Phase 2: [`docs/archiv/logbuch-phase-2.md`](archiv/logbuch-phase-2.md); Fahrplan: [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md). Die Einträge der Session 1.7 stehen jetzt in [`docs/archiv/logbuch-phase-1.md`](archiv/logbuch-phase-1.md).
- **Reflexion Phase 2:**
  - Gelernt: Fundament mit Reserve – Schritt mit 10.000 Personen 0,13–0,56 ms, Bild an der Grenze des Bildschirms auf Referenz-PC und Mac (2.7, 2.10), Spielstand bitgleich nach Laden auch während einer Neuberechnung (2.5). Getrennte Prüfungen fanden in jedem Schritt mit Kategorie 6 echte Mängel; die Sammelprüfung 2.11 fand 10 Vertragslücken auf einmal.
  - Gekippte Annahmen: Verträge aus 2.9 trugen das Gebaute nicht vollständig (ADR-026, ADR-028 bis ADR-032); „Kern-Module über Weltzustand und interne Ereignisse“ ist noch nicht gebaut (3.9); Neuberechnung im Hintergrund ohne Zusammenfassen der Anfragen trägt Dauerbau nicht (3.1); Linux bleibt ohne Messung auf echter Hardware (7.2).
  - Reifegrade: Takt & Weltzustand, Welt & Bau (Grundraster), Brücke, Plattform, Inhalte & Mods (Lader), Lokalisierung, Grafik-Pipeline `[BELASTBAR]` durch Umsetzung; S-2 bis S-7 bestätigt, S-8 neu; NFR Darstellung im gemessenen Umfang (ADR-034); Weltdarstellung `[OFFEN]` (3.8).
  - ADRs der Phase: ADR-018 bis ADR-034 (dazu ADR-035 im Querschnitt, ADR-036 Phasenabschluss); reaktiv ADR-019, ADR-020, ADR-025, ADR-026, ADR-028 bis ADR-032 – formal durch die 14-Tage-Regel oder additive Vertragsergänzungen, keiner änderte Code in Takt, Welt & Bau, Wegfindung oder Brücke. Maßnahmen: Regel-008, Regel-009.
  - Neue Erkundungen: 3.11 Suchen ohne Schilder und Feinwege, 3.12 Blockadevermeidung, 3.13 Schrittzeit-Spitzen auf Apple-Chips; Entwurf 3.9 Kernstruktur.
  - Mini-Reibungen, die wiederkehren können: Zeitangaben geschätzt statt mit `date -u` geholt (zweimal); Typ-Check des Pre-Commit-Hooks über den ganzen Arbeitsbaum; Prettier formatiert JSON in `docs/research/`; Suppression ohne Begründung (`scenario.rs:76`) blieb in Hooks und Prüfungen unbemerkt.
- **Pflichtfrage:** „gezielt umbauen“ (A), zwei getrennte Instanzen (Opus 5.5, Sonnet 5.5), `docs/research/phasenabschluss-2.md`.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ → FR-016 mit 3.2 und 4.1; MOD-13 Walk-out in den Kern → FR-052 mit 3.7 und 3.4; Erkundungen mit eigener ID 3.11, 3.12; keine `[VERSCHOBEN]`-Zeilen. Keine verwaisten Elemente mehr.
- **Onboarding-Validierung:** siehe `[ONBOARDING-VALIDATION]` 19:56.

### 2026-10-19 20:01 (UTC) – [ADR-ANGELEGT]

- **ADR-036** `[REAKTIV]` „Phasenabschluss 2 – gezielt umbauen vor dem vertikalen Schnitt“: freigegeben vom Projektinhaber per Auswahl (A; MOD-13 in den Kern; FR-016 ergänzen; 4.3 als 3.13 vorziehen; kein Linux-Rechner). Reaktiv, weil er den Landeplatz 4.3 aus ADR-034 (heute) nach 3.13 verlegt. Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036), Reflexion für Phase 2 im Fahrplan nach Regel-009.
- **Nachgezogen:** `decisions.md` (Teil A, Regel-008, Regel-009), Abschnitt 4 nach `docs/architecture-schnittstellen.md` mit Änderungsprotokoll und Ankern, `architecture.md` (Abschnitte 0, 2, 4, 9, offene Fragen), `requirements.md` (FR-016, FR-052, Schritt-IDs 3.11/3.12, Übersicht 52 Anforderungen), `modernisierung.md` (MOD-13 Kern), Fahrplan (Phase 3 im vollen Format, 4.1, 4.3, Reflexion, Replanning), README.

### 2026-10-19 19:56 (UTC) – [ONBOARDING-VALIDATION]

- **Phasen-Abschluss 2 (Trigger 3, Klasse G):** frischer Worktree von `origin/main` (`d4e457f`) im Container (Linux, 4 vCPU), Runbook Schritte 2 bis 5 wörtlich: `npm ci`, `pre-commit run --all-files` (alle Hooks grün), `npm run build`, Start `npx electron . --no-sandbox` unter `xvfb-run` (40 s ohne Absturz, nur GPU-Meldungen der Software-Grafik), dann alle Gates: rustfmt, clippy, `cargo test` (alle grün, u. a. 134 Kern-Unit-Tests, 26 Datenpaket-, 10 Spielstand-Fälle), cargo-deny, ESLint, Prettier, tsc, Vitest 356 Tests mit 86,6 % Lines, Lizenzen, `check:types`, `check:schema`, `check:graphics`, Benchmarks (Tick: Mittel 0,17 ms, p99 0,47 ms; Wegweiser: Neuberechnung Median 2,9 s bei Grenze 4 s, Schritt p99 1,8 ms), Rauchtest grün (17,1 s).
- **Fund und behoben:** Runbook Schritt 4 nannte für das Basispaket „3 entries“; seit 2.6 zählen die Texte der Sprachdateien mit. Text auf „N entries“ mit Erklärung umgestellt (`a0603e7`).
- **Einschränkungen:** Electron kam aus dem Cache der Container-Einrichtung (`~/.cache/electron`), der Download beim ersten Start ist damit nicht frisch belegt. Neuberechnung 2,9 s liegt über den früher lokal gemessenen 1,1–1,2 s (anderer Container, kein Gate verletzt; Beobachtung für 3.1). Zweite Plattform (Klasse G): Windows und macOS über die CI des Stands (PR #26 auf `0cf1724`, Lauf 39311921712 grün auf drei Plattformen), nicht von Hand.

### 2026-10-19 19:51 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Pflichtfrage Phasenende 2:** zwei getrennte Instanzen ohne Gesprächsverlauf, Logbuch ausgeschlossen: A Opus 5.5 (wie Phase 1), B Sonnet 5.5 (anderes Modell). Beide empfehlen „gezielt umbauen“, Konfidenz mittel, und schließen Neuaufsetzen mit Belegen aus. A: Kernstruktur für die Fachmodule vor 3.1, Dauerbau der Wegweiser, `main.ts` teilen, Architektur auslagern, Zählregel. B: Prozess (Zählregel, Reflexionen), `main.ts`, S-1 Layout 2 in 3.8. Volltext und Stellungnahme: `docs/research/phasenabschluss-2.md`.
- **Nachgeprüft:** `core/src/adoption.rs` (ältere Neuberechnung läuft weiter, wird verworfen), `vitest.config.ts` (Ausnahmen), `core/src/scenario.rs:76` (Suppression ohne Begründung, Verstoß seit 2.3; B hatte sie als begründet gezählt), keine internen Ereignisse im Kern, `architecture.md` über der eigenen Schwelle von 500 Zeilen.
- **Widerspruch der Stellungnahme:** Die Zählregel steht wörtlich in `CLAUDE.md` Abschnitt 6 und ist im Projekt nicht auslegbar. Stattdessen soll die Projektregel „jede Änderung an `[BELASTBAR]` ist freigabepflichtig“ an `CLAUDE.md` Abschnitt 4 Nr. 5 angeglichen werden (additiv ohne ADR) und es soll eine Reflexion je Phase geben.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ ohne Akzeptanzkriterium (FR-016 ergänzen, 3.2); MOD-13 Walk-out (Kandidat) steckt in 3.7 und über ADR-020 in 3.4 – Frage an den Projektinhaber; Erkundungen vor 3.3/3.4 ohne Schritt-ID (Regelverstoß, neue Schritte 3.11/3.12). Keine `[VERSCHOBEN]`-Zeilen.
- **Archivierung vorgezogen (unabhängig von der Antwort):** Fahrplan Phase 2 nach `docs/archiv/fahrplan-phase-2.md` mit Phasen-Bilanz; Logbuch-Einträge von Phase 2 nach `docs/archiv/logbuch-phase-2.md`, die Session 1.7 (2026-10-13 18:00–19:05) nach `docs/archiv/logbuch-phase-1.md`. `[PHASEN-WECHSEL]` mit Reflexion folgt nach der Antwort.
- **Drift behoben:** Fahrplan „Aktueller Stand“ nannte 7 / 10, `decisions.md` 6 / 10 (seit ADR-035); jetzt 6 / 10.

### 2026-10-19 19:40 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 18:53 UTC; danach Merge PR #25 (19:11), ADR-035 und Q.3 (19:31), Merge PR #26 (19:37, `d4e457f`). Der Abschluss-Commit `836ec95` nach `project-context.md` Abschnitt 10 auf den neuen Branch `docs/phasenabschluss-2` vom aktuellen `main` übernommen (`e1c2262`); `ci/q-3-rustup-toolchain-install` wird nicht weiterverwendet. Die vorige Session endete mit über 500.000 Token Kontext (Grenze 400.000).
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 19:40 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Vorhaben:** Phasenabschluss Phase 2 nach `CLAUDE.md` Abschnitt 12: Vision-Re-Derivations-Pass (mit `requirements.md`), Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“ mit Bewertung durch eine getrennte Instanz, Onboarding-Re-Validation (Trigger 3, Klasse G), Archivierung und `[PHASEN-WECHSEL]`. Empfohlene Klasse: Entscheidung (Auslöser 1 und 3) – aktive Klasse passt.

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
