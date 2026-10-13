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

Phase 1 abgeschlossen (ADR-016), Detail-Einträge im Archiv `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) ausgeplant; Reaktiv-Reflexion zu ADR-017 bestätigt; nächste Schritte 2.1 und 2.8 in neuer Session.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

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
