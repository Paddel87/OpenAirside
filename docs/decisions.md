# Decisions

<!-- Begründete Entscheidungen und daraus abgeleitete Regeln für OpenAirside.
     Teil A: ADR-Übersicht (Pflichtlektüre bei Sessionstart)
     Teil B: ADRs (Detailteil, nur bei Bedarf lesen)
     Teil C: Entscheidungsregeln (Mindest-Lektüre wie Teil A)
     Teil D: Geschäftsentscheidungen (BDR), aktiv ab Klasse G
     Einträge werden nicht gelöscht oder verändert; überholte ADRs werden durch neue ersetzt.
     Klasse G: Teil B seit 2026-10-13 ausgelagert in docs/decisions/ADR-NNN.md (ADR-Anzahl zweistellig, ADR-001). -->

<!-- ANCHOR:teil-a-adr-uebersicht -->
## Teil A: ADR-Übersicht

| ADR | Datum | Status | Klassifikation | Themen | Kategorie | Kurztitel |
|---|---|---|---|---|---|---|
| 001 | 2026-10-13 | Aktiv | STRATEGISCH | METHODIK | Methodik | Klasse G und Anpassung des Vorlagen-Sets |
| 002 | 2026-10-13 | Aktiv (bestätigt durch ADR-014) | STRATEGISCH | STACK | Externe Abhängigkeiten | Stack: Web-Präsentation + Rust-Simulationskern |
| 003 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL, SCHNITTSTELLE | Architektur | Kern/Präsentation-Trennung, feste Simulationsschritte, Inhalte als Daten |
| 004 | 2026-10-13 | Aktiv | STRATEGISCH | PERFORMANCE, MODUL | Architektur | Wegfindung nach dem Wegweiser-Prinzip |
| 005 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Verkehrsregeln, Vorrang-Rangfolge und automatische Verkehrssteuerung |
| 006 | 2026-10-13 | Aktiv | STRATEGISCH | METHODIK, DATENMODELL | Methodik | Funktionsumfang: Inventar, Modernisierung, Cargo vorbereitet |
| 007 | 2026-10-13 | Aktiv | STRATEGISCH | DEPLOYMENT | Lizenz/Compliance, Build/Deploy | Vertrieb über GitHub, Lizenzen MIT und CC0 |
| 008 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Eigene SVG-Grafiken, Rasterung im Build |
| 009 | 2026-10-13 | Aktiv | ERKENNTNIS | STACK, SCHNITTSTELLE, PERFORMANCE | Architektur | Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess |
| 010 | 2026-10-13 | Aktiv | ERKENNTNIS | DATENMODELL, PERFORMANCE, MODUL | Datenmodell, Architektur | Datenmodell und Parallelisierung im Kern, Übernahme der Wegweiser |
| 011 | 2026-10-13 | Aktiv | REAKTIV | METHODIK, PERFORMANCE | Projekt-Vorgabe | Linux-Leistungsmessung auf echter Hardware verschoben auf Schritt 7.2 |
| 012 | 2026-10-13 | Aktiv | REAKTIV | DEPLOYMENT | Build/Deploy, Projekt-Vorgabe | macOS nur für Apple-Chips, kein Intel- und kein Universal-Build |
| 013 | 2026-10-13 | Überholt durch ADR-020 | ERKENNTNIS | DATENMODELL, PERFORMANCE | Datenmodell | Größte Karte 768 × 768 Zellen je Etage, fünf Etagen |
| 014 | 2026-10-13 | Aktiv | ERKENNTNIS | STACK, PERFORMANCE | Architektur | Stack aus ADR-002 bestätigt, Leistungsnachweis mit Auflagen erbracht |
| 015 | 2026-10-13 | Aktiv | REAKTIV | DEPLOYMENT | Build/Deploy | Ad-hoc-Signatur für macOS-Pakete der Spike-Builds |
| 016 | 2026-10-13 | Aktiv | ERKENNTNIS | METHODIK | Architektur, Methodik | Phasenabschluss 1 – gezielt umbauen, Reifegrad-Regel für Umsetzungsschritte |
| 017 | 2026-10-13 | Aktiv | REAKTIV | PERFORMANCE | Architektur | Reifegrad „NFR Performance Gesamtsystem" auf das Gemessene begrenzt |
| 018 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, STACK | Build/Deploy, Externe Abhängigkeiten | Projektgerüst, CI-Gates und napi-Versionen für Phase 2 |
| 019 | 2026-10-13 | Aktiv | REAKTIV | STACK | Lizenz/Compliance | Erlaubte Lizenzen um BlueOak-1.0.0, 0BSD, Python-2.0 und WTFPL erweitert |
| 020 | 2026-10-13 | Aktiv | REAKTIV | DATENMODELL, PERFORMANCE | Datenmodell | Weltmaßstab 2 m je Zelle, Luftseite im Raster, Wegweiser in Blöcken, Zeitmodell |
| 021 | 2026-10-13 | Aktiv | ERKENNTNIS | SCHNITTSTELLE, SECURITY | API-Verträge, Architektur, Sicherheit | Schnittstellenverträge S-2 bis S-7 |
| 022 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, PERFORMANCE | Build/Deploy | Kern-Benchmark als CI-Gate und Prüfung der erzeugten TypeScript-Typen |
| 023 | 2026-10-13 | Aktiv | OPERATIV | DEPLOYMENT, PERFORMANCE | Build/Deploy | Wegweiser-Benchmark als Schritt im CI-Job „Kern-Benchmark“ |
| 024 | 2026-10-14 | Aktiv (Einschränkung auf die Brücke aufgehoben durch ADR-025) | OPERATIV | STACK, SCHNITTSTELLE | Externe Abhängigkeiten | Umwandlung JS ↔ Rust in der Brücke mit napi `serde-json` |
| 025 | 2026-10-18 | Aktiv | REAKTIV | STACK, SCHNITTSTELLE, DEPLOYMENT | Externe Abhängigkeiten, Build/Deploy | Bibliotheken für den Datenpaket-Lader und die Schema-Erzeugung |

### Reaktiv-Quote

- **Aktueller Wert:** 4 / 10 über die letzten 10 ADRs (ADR-016 bis ADR-025: ADR-017, ADR-019, ADR-020, ADR-025), 40 % – **über dem Schwellenwert**; gesamt 7 / 25. Reflexionen in `fahrplan.md` Abschnitt „Iterations-Reflexion" (letzte: Anlass ADR-025, 2026-10-18)
- **Schwellenwert (in `project-context.md` festgelegt):** maximal 20 % über die letzten 10 ADRs
- **Bei Überschreitung:** STOPP, Reflexion in `fahrplan.md` ergänzen, prüfen ob Architektur-Refactoring nötig ist.

---

<!-- ANCHOR:teil-b-architecture-decision-records -->
## Teil B: Architecture Decision Records

### Format

```text
### ADR-NNN: [Kurztitel]

- **Datum:** YYYY-MM-DD
- **Status:** Aktiv | Überholt durch ADR-M | Verworfen
- **Tags:** [aus Tag-Liste unten]
- **Phasentyp-Kontext:** [ERKUNDUNG | UMSETZUNG | STABILISIERUNG | INITIALISIERUNG]
- **Reifegrad-Wirkung:** [...]
- **Kategorie:** [aus CLAUDE.md Abschnitt 4 oder "Methodik"]
- **Kontext:** [...]
- **Optionen:** [...]
- **Entscheidung:** [...]
- **Vision-Frage, die entschied:** [bei Architektur-Entscheidungen Pflicht, sonst „n/a"]
- **Konfidenz zum Zeitpunkt:** [bei Architektur-Entscheidungen Pflicht, sonst „n/a": hoch/mittel/niedrig + Umkehrbarkeit billig/teuer]
- **Konsequenzen:** [...]
- **Abgeleitete Regel:** [...]
```

### Tags

#### Klassifikations-Tags (genau einer pflichtig)

- `[STRATEGISCH]` – Grundsatzentscheidung in Konzeptphase oder Initialisierung
- `[OPERATIV]` – Entscheidung während der Umsetzung im Rahmen der geplanten Architektur
- `[REAKTIV]` – Entscheidung wegen etwas Unerwartetem in der Umsetzung (Indikator für schwache Architektur)
- `[ERKENNTNIS]` – Ergebnis einer Erkundungsphase oder eines Spikes

#### Themen-Tags (optional, mehrere möglich)

- `[STACK]`, `[MODUL]`, `[SCHNITTSTELLE]`, `[DATENMODELL]`, `[SECURITY]`, `[PERFORMANCE]`, `[DEPLOYMENT]`, `[OBSERVABILITY]`, `[METHODIK]`

### Nummerierung

Durchgehend, keine Lücken. Auch verworfene oder überholte Einträge behalten ihre Nummer.

### Einträge

Seit 2026-10-13 liegt jeder ADR als eigene Datei in `docs/decisions/` (ADR-001: Auslagerung ab zweistelliger ADR-Anzahl). Neue ADRs werden dort angelegt und in Teil A eingetragen.

- [ADR-001: Klasse G und Anpassung des Vorlagen-Sets](decisions/ADR-001.md)
- [ADR-002: Stack: Web-Präsentation + Rust-Simulationskern](decisions/ADR-002.md)
- [ADR-003: Kern/Präsentation-Trennung, feste Simulationsschritte, Inhalte als Daten](decisions/ADR-003.md)
- [ADR-004: Wegfindung nach dem Wegweiser-Prinzip](decisions/ADR-004.md)
- [ADR-005: Verkehrsregeln, Vorrang-Rangfolge und automatische Verkehrssteuerung](decisions/ADR-005.md)
- [ADR-006: Funktionsumfang: Inventar, Modernisierung, Cargo vorbereitet](decisions/ADR-006.md)
- [ADR-007: Vertrieb über GitHub, Lizenzen MIT und CC0](decisions/ADR-007.md)
- [ADR-008: Eigene SVG-Grafiken, Rasterung im Build](decisions/ADR-008.md)
- [ADR-009: Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess](decisions/ADR-009.md)
- [ADR-010: Datenmodell und Parallelisierung im Kern, Übernahme der Wegweiser](decisions/ADR-010.md)
- [ADR-011: Linux-Leistungsmessung auf echter Hardware verschoben auf Schritt 7.2](decisions/ADR-011.md)
- [ADR-012: macOS nur für Apple-Chips, kein Intel- und kein Universal-Build](decisions/ADR-012.md)
- [ADR-013: Größte Karte 768 × 768 Zellen je Etage, fünf Etagen](decisions/ADR-013.md)
- [ADR-014: Stack aus ADR-002 bestätigt, Leistungsnachweis mit Auflagen erbracht](decisions/ADR-014.md)
- [ADR-015: Ad-hoc-Signatur für macOS-Pakete der Spike-Builds](decisions/ADR-015.md)
- [ADR-016: Phasenabschluss 1 – gezielt umbauen, Reifegrad-Regel für Umsetzungsschritte](decisions/ADR-016.md)
- [ADR-017: Reifegrad „NFR Performance Gesamtsystem" auf das Gemessene begrenzt](decisions/ADR-017.md)
- [ADR-018: Projektgerüst, CI-Gates und napi-Versionen für Phase 2](decisions/ADR-018.md)
- [ADR-019: Erlaubte Lizenzen um BlueOak-1.0.0, 0BSD, Python-2.0 und WTFPL erweitert](decisions/ADR-019.md)
- [ADR-020: Weltmaßstab 2 m je Zelle, Luftseite im Raster, Wegweiser in Blöcken, Zeitmodell](decisions/ADR-020.md)
- [ADR-021: Schnittstellenverträge S-2 bis S-7](decisions/ADR-021.md)
- [ADR-022: Kern-Benchmark als CI-Gate und Prüfung der erzeugten TypeScript-Typen](decisions/ADR-022.md)
- [ADR-023: Wegweiser-Benchmark als Schritt im CI-Job „Kern-Benchmark“](decisions/ADR-023.md)
- [ADR-024: Umwandlung JS ↔ Rust in der Brücke mit napi `serde-json`](decisions/ADR-024.md)
- [ADR-025: Bibliotheken für den Datenpaket-Lader und die Schema-Erzeugung](decisions/ADR-025.md)

---

<!-- ANCHOR:teil-c-entscheidungsregeln -->
## Teil C: Entscheidungsregeln

### Format (Regel)

```text
### Regel-NNN: [Kurztitel]

- **Herkunft:** ADR-[Nr.]
- **Gilt für:** [...]
- **Regel:** [...]
- **Ausnahmen:** [...]
- **Gegenbeispiel:** [...]
```

### Regeln

#### Regel-001: Versionen belegen, nie aus dem Gedächtnis

- **Herkunft:** ADR-002
- **Gilt für:** jede neue Abhängigkeit, jedes Setup, jede Versionsangabe in Dokumenten oder Konfiguration
- **Regel:** Version gegen offizielle Quellen belegen (Paketregister, Release-Seite, Lebensende), nach „ausgereifte Linie" und Mindestreife aus `project-context.md` Abschnitt 3 wählen (`CLAUDE.md` Abschnitt 15), Kompatibilität mit den tragenden Abhängigkeiten prüfen, `Verifiziert: YYYY-MM-DD` mit Quelle notieren. Neue Linien für Sprache, Framework oder Laufzeit legt die KI als Tabelle zur Bestätigung vor.
- **Ausnahmen:** keine
- **Gegenbeispiel:** eine Version aus dem Gedächtnis oder aus einem älteren Beispiel übernehmen.

#### Regel-002: Spiellogik nur im Rust-Kern

- **Herkunft:** ADR-002, ADR-003
- **Gilt für:** jede Regel, Berechnung oder Zustandsänderung des Spiels
- **Regel:** Spiellogik gehört in den Rust-Simulationskern. Die Präsentation zeigt an, nimmt Eingaben entgegen und schickt Befehle; sie entscheidet nichts über den Spielzustand.
- **Ausnahmen:** reine Darstellungslogik (Animation, Interpolation, Formatierung, Sortierung von Anzeigen)
- **Gegenbeispiel:** die Oberfläche berechnet, ob ein Bau zulässig ist, oder ändert Geldbeträge direkt.

#### Regel-003: Inhalte sind Daten

- **Herkunft:** ADR-003
- **Gilt für:** Flugzeugtypen, Airlines, Objekte, Fahrzeuge, Forschung, Preise, Texte
- **Regel:** Neue Inhalte werden als Datensätze in Datenpaketen mit Schema angelegt, nicht als Programmcode.
- **Ausnahmen:** grundlegend neues Verhalten, das neue Logik im Kern erfordert (dann Fahrplan-Schritt)
- **Gegenbeispiel:** ein Flugzeugtyp wird als eigene Rust-Struktur mit fest eingetragenen Werten angelegt.

#### Regel-004: Funktionen gegen Inventar prüfen

- **Herkunft:** ADR-006
- **Gilt für:** jeden Umsetzungsschritt, der Spielfunktionen betrifft
- **Regel:** Der Schritt nennt die betroffenen IDs aus `funktionsinventar.md` und `modernisierung.md`; die Akzeptanzkriterien decken das dort beschriebene Spielprinzip ab.
- **Ausnahmen:** reine Infrastruktur-Schritte
- **Gegenbeispiel:** eine Funktion wird „nach Gefühl" umgesetzt, ohne Abgleich mit dem Inventar.

#### Regel-005: Kein Material des Originals

- **Herkunft:** ADR-007
- **Gilt für:** Code, Grafiken, Texte, Töne, Daten, Beiträge von Mitwirkenden
- **Regel:** Orientierung ausschließlich an Funktionen und Spielprinzip; nichts aus Airport CEO oder dessen Mods übernehmen, nichts nachzeichnen, keine verwechselbaren Namen.
- **Ausnahmen:** keine
- **Gegenbeispiel:** Texte wörtlich aus dem Spiel oder Wiki des Originals übernehmen; eine Grafik des Originals als Vorlage abpausen. (Zahlenwerte aus dem Inventar dienen nur als Orientierung für eigenes Balancing.)

#### Regel-006: Kern frei von Anbindungscode

- **Herkunft:** ADR-009
- **Gilt für:** alle Rust-Crates des Simulationskerns
- **Regel:** Die Kern-Crates kennen weder napi-rs noch wasm-bindgen noch Electron. Die Anbindung an JavaScript liegt ausschließlich in einer eigenen Brücken-Crate, die den Kern über seine öffentliche Rust-Schnittstelle nutzt. Der Kern bleibt ohne Anbindung baubar und testbar (auch für `wasm32-unknown-unknown`, damit Q.2 jederzeit möglich bleibt).
- **Ausnahmen:** keine
- **Gegenbeispiel:** `#[napi]`-Attribute oder `napi`-Typen in einer Kern-Crate; Spiellogik in der Brücken-Crate.

#### Regel-007: Reifegrad vor Umsetzungsschritten

- **Herkunft:** ADR-016
- **Gilt für:** jeden Schritt einer UMSETZUNG-Phase (Auslegung von `CLAUDE.md` Abschnitt 6, „Architektur-Reifegrad respektieren", im Verhältnis zu `architecture.md` Abschnitt 0)
- **Regel:** Vor dem Schritt müssen die **Grundsatzentscheidungen**, auf denen er aufbaut, `[BELASTBAR]` sein: Stack, Pattern, Datenmodell und jede Schnittstelle (S-n), die der Schritt nutzt oder umsetzt. **Module** dürfen `[VORLÄUFIG]` in ihre erste Umsetzung gehen; sie werden durch diese Umsetzung belastbar (Beförderungsregel). Ein `[OFFEN]`-Bestandteil, von dem der Schritt abhängt, ist immer ein Erkundungsgrund.
- **Ausnahmen:** keine
- **Gegenbeispiel:** Schritt 2.5 (Spielstand) beginnt, während S-5 noch `[OFFEN]` oder `[VORLÄUFIG]` ist; oder ein Modul wird zurückgestellt, weil es vor seiner ersten Umsetzung nicht `[BELASTBAR]` ist.

<!-- ANCHOR:teil-d-geschaeftsentscheidungen -->
## Teil D: Geschäftsentscheidungen (BDR)

### BDR-001: Kostenloses Hobbyprojekt ohne Gewinnabsicht

- **Datum:** 2026-10-13
- **Entschieden von:** Projektinhaber
- **Frage:** Wie wird das Spiel angeboten?
- **Optionen:** A: kostenlos über GitHub, ohne Gewinnabsicht / B: Verkauf über einen Store / C: Spenden oder Unterstützermodell
- **Entscheidung:** A. Das Projekt ist ein Hobbyprojekt und Machbarkeitstest ohne Zeitrahmen. Kommerzielle Nutzung durch Dritte regelt die MIT-Lizenz; daraus entstehende Fragen sind Sache der Nutzer.
- **Folgen für Anforderungen und Fahrplan:** FR-045; ADR-007; kein Schritt für Store-Anbindung.

### BDR-002: Kostenrahmen 50 € im Monat

- **Datum:** 2026-10-13
- **Entschieden von:** Projektinhaber
- **Frage:** Was darf das Projekt monatlich kosten (ohne das bestehende KI-Abo)?
- **Optionen:** A: 0 € / B: bis 10 € / C: bis 50 €
- **Entscheidung:** C, bis 50 € im Monat. Jeder Posten wird vor Einsatz freigegeben und in `project-context.md` Abschnitt 8 eingetragen.
- **Folgen für Anforderungen und Fahrplan:** macht eine spätere Codesignatur oder KI-Audio möglich (Entscheidung vor Phase 7 bzw. Schritt 6.2); FR-047.
