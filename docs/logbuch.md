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

<!-- PHASEN-WECHSEL 2 → 3 folgt nach der Antwort auf die Pflichtfrage. Detail-Einträge: docs/archiv/logbuch-phase-2.md, Session 1.7: docs/archiv/logbuch-phase-1.md -->

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
