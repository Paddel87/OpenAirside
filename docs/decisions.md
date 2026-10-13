# Decisions

<!-- Begründete Entscheidungen und daraus abgeleitete Regeln für OpenAirside.
     Teil A: ADR-Übersicht (Pflichtlektüre bei Sessionstart)
     Teil B: ADRs (Detailteil, nur bei Bedarf lesen)
     Teil C: Entscheidungsregeln (Mindest-Lektüre wie Teil A)
     Teil D: Geschäftsentscheidungen (BDR), aktiv ab Klasse G
     Einträge werden nicht gelöscht oder verändert; überholte ADRs werden durch neue ersetzt.
     Klasse G: Einzeldatei, Auslagerung in decisions/ADR-NNN.md sobald die ADR-Anzahl zweistellig wird. -->

<!-- ANCHOR:teil-a-adr-uebersicht -->
## Teil A: ADR-Übersicht

| ADR | Datum | Status | Klassifikation | Themen | Kategorie | Kurztitel |
|---|---|---|---|---|---|---|
| 001 | 2026-10-13 | Aktiv | STRATEGISCH | METHODIK | Methodik | Klasse G und Anpassung des Vorlagen-Sets |
| 002 | 2026-10-13 | Aktiv (bedingt) | STRATEGISCH | STACK | Externe Abhängigkeiten | Stack: Web-Präsentation + Rust-Simulationskern |
| 003 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL, SCHNITTSTELLE | Architektur | Kern/Präsentation-Trennung, feste Simulationsschritte, Inhalte als Daten |
| 004 | 2026-10-13 | Aktiv | STRATEGISCH | PERFORMANCE, MODUL | Architektur | Wegfindung nach dem Wegweiser-Prinzip |
| 005 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Verkehrsregeln, Vorrang-Rangfolge und automatische Verkehrssteuerung |
| 006 | 2026-10-13 | Aktiv | STRATEGISCH | METHODIK, DATENMODELL | Methodik | Funktionsumfang: Inventar, Modernisierung, Cargo vorbereitet |
| 007 | 2026-10-13 | Aktiv | STRATEGISCH | DEPLOYMENT | Lizenz/Compliance, Build/Deploy | Vertrieb über GitHub, Lizenzen MIT und CC0 |
| 008 | 2026-10-13 | Aktiv | STRATEGISCH | MODUL | Architektur | Eigene SVG-Grafiken, Rasterung im Build |
| 009 | 2026-10-13 | Aktiv | ERKENNTNIS | STACK, SCHNITTSTELLE, PERFORMANCE | Architektur | Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess |

### Reaktiv-Quote

- **Aktueller Wert:** 0 / 9
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

#### ADR-001: Klasse G und Anpassung des Vorlagen-Sets

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[METHODIK]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** keine
- **Kategorie:** Methodik
- **Kontext:** Erste Initialisierung nach Abschluss der Vision (Modus 1, 2026-10-12). Die Projektgröße bestimmt die Struktur des Vorlagen-Sets (CLAUDE.md Abschnitt 1B).
- **Optionen:**
  - **A: Klasse M** – Konsequenzen: schlanke Dokumente; passt nicht zu über 15 Modulen und hoher Leistungsanforderung.
  - **B: Klasse G** – Konsequenzen: 5–7 Phasen, strengerer Reaktiv-Schwellenwert, Auslagerung von Architektur und ADRs bei Wachstum.
  - **C: Klasse V** – Konsequenzen: Pflicht-Splitting für Services; passt nicht, da nur eine Auslieferungseinheit ohne verteilte Dienste.
- **Entscheidung:** B, Klasse G. Stufe 1 (Vision): über 15 Module, viele ineinandergreifende Systeme, hohe NFR-Komplexität (10.000 Personen, 60 Bilder pro Sekunde), mehrere Plattformen, wenige externe Abhängigkeiten, eine Nutzergruppe. Stufe 2 (Architektur-Grobschnitt): eine Desktop-Anwendung je Plattform, zwei Sprachen (Rust, TypeScript), Kern und Präsentation in getrennten Prozessen bzw. Threads, eine Persistenzform (Spielstand-Dateien), keine verteilten Dienste. Stufe 2 bestätigt die Hypothese.
- **Konsequenzen:**
  - `architecture.md` als ein Dokument; Auslagerung in `architecture-<modul>.md`, sobald > 500 Zeilen oder > 5 Module eigene detaillierte Schnittstellen haben.
  - `decisions.md` als Einzeldatei; Auslagerung in `decisions/ADR-NNN.md` sobald die ADR-Anzahl zweistellig wird.
  - `fahrplan.md` mit 7 Phasen, Teil-Fahrpläne pro Modul erst bei Bedarf.
  - Reaktiv-Schwellenwert 20 %.
  - **Abweichung vom Standard-Set:** zwei zusätzliche verbindliche Dokumente aus der Konzeptphase: `funktionsinventar.md` (Maßstab für Funktionsgleichheit) und `modernisierung.md` (entschiedene Abweichungen vom Original). Beide liegen in `docs/` und werden wie Pflicht-Dokumente gepflegt, gehören aber nicht zur Mindest-Lektüre; sie werden bei Arbeit an einer Funktion gezielt gelesen.
  - README in voller Form, maximal 10 Badges.
  - `requirements.md` vollständig (Klasse G), befüllt in Modus 1.5 am 2026-10-13; Anwendungsfälle und Muss-Anforderungen vom Projektinhaber ausdrücklich bestätigt.
  - `onboarding-runbook.md` angelegt (Pflicht ab Klasse M).
  - Teil D (Geschäftsentscheidungen) aktiv.
  - Nachzug am 2026-10-13: Die erste Initialisierung folgte einer älteren Fassung des Regelwerks; Modus 1.5, Versions-Verifikation, Anker und neue Vorlagen wurden vor dem Init-Commit nach `templates/projektstart.md` ergänzt.
- **Vision-Frage, die entschied:** n/a
- **Konfidenz zum Zeitpunkt:** n/a
- **Abgeleitete Regel:** keine (Einzelfall-Entscheidung)

#### ADR-002: Stack: Web-Präsentation + Rust-Simulationskern

- **Datum:** 2026-10-13
- **Status:** Aktiv, unter der Bedingung des Leistungsnachweises in Phase 1
- **Tags:** `[STRATEGISCH]` `[STACK]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** alle Stack-Bestandteile `[VORLÄUFIG]` bis Abschluss von Phase 1, Schritt 1.5
- **Kategorie:** Externe Abhängigkeiten (CLAUDE.md 4.3)
- **Kontext:** Die Vision lässt die Technologie offen. Randbedingungen: Windows, Linux, macOS; 10.000+ Personen bei 60 Bildern pro Sekunde auf einem Mittelklasse-PC; sehr oberflächenlastiges Spiel (Flugplaner, Tabellen, Panels); offizielle Mod-Unterstützung; MIT-Lizenz; kostenlose Werkzeuge; Umsetzung überwiegend durch einen KI-Coding-Assistenten; Grundsätze „Stabilität vor Neuheit" und „Wiederverwenden vor Eigenbau". Alle Versionen und Eigenschaften am 2026-10-12/13 live recherchiert. Der Projektverantwortliche hat eine erste Empfehlung (A) hinterfragt, weil Teile davon auf Vorwissen statt frischer Recherche beruhten; die Neubewertung mit frischer Recherche ergab die Entscheidung unten.
- **Optionen:**
  - **A: Godot 4 + C#** (Simulation als C#-Bibliothek) – Konsequenzen: echte Spiele-Engine mit Editor und Export, eine Sprache, C#-Mods nachgewiesen (Slay the Spire 2); .NET 10 erst ab Godot 4.8 (Q4 2026); kein fertiges Tabellen-Element, große Tabellen bleiben Eigenarbeit; KI-Werkzeuge der Community zielen auf GDScript.
  - **B: Bevy (Rust, ECS)** – Konsequenzen: beste eingebaute Mehrkern-Verarbeitung; Release alle 3–5 Monate mit 75–100+ Migrationsabschnitten, Zusatzpakete hinken hinterher; kein Editor, kein Tabellen-Element, Mod-Skripte experimentell; ein Kolonie-Spiel-Team (Architect of Ruin) wechselte 2025 wegen Umbauten, Oberfläche, Mods und KI-Rückstand weg.
  - **C: Eigenbau .NET 10 + MonoGame/FNA** – Konsequenzen: stabil und kontrollierbar, aber Oberfläche, Werkzeuge, Asset-Pipeline und Mod-Lader komplett selbst zu bauen.
  - **D: Electron + TypeScript + React + PixiJS, Simulationskern in Rust** – Konsequenzen: stärkste Oberflächen-Technik für dutzende Tabellen und Panels; höchste KI-Eignung; für den Projektverantwortlichen nachvollziehbar (Web-Erfahrung, bestehender Stack mit TypeScript und Tailwind); Mods als JSON-Daten, später ggf. Plugins; Zeichnen unkritisch (PixiJS laut eigenen Messungen ~200.000 Sprites bei 60 Bildern pro Sekunde); Risiken: kein bekanntes großes Management-Spiel dieser Bauart, Datenübergabe zwischen Kern und Präsentation muss ohne Serialisierung je Bild gelöst werden, Linux-Grafiktreiber im Browser-Kern (deshalb WebGL2), größere Downloads, zwei Sprachen.
  - Außerdem geprüft und verworfen: Unity (proprietärer Editor), Tauri (WebGL unter Linux unzuverlässig), Stride (Windows-Editor), Fyrox (junge 1.0, ein Hauptentwickler), Raylib/macroquad (Oberfläche und Mods Eigenbau), libGDX/Ebitengine (kein Vorteil gegenüber den Kandidaten).
- **Entscheidung:** D, freigegeben vom Projektverantwortlichen am 2026-10-13. Begründung: Das Spiel besteht zu einem großen Teil aus Oberfläche, wo Web-Technik am stärksten ist; KI-Eignung und Nachvollziehbarkeit für den Projektverantwortlichen sind am höchsten; die Leistungslast liegt im Rust-Kern mit nativer Geschwindigkeit. Das Risiko ist begrenzt: Scheitert die Web-Präsentation, bleibt der Rust-Kern erhalten und kann über godot-rust mit Godot als Präsentation weiterverwendet werden.
- **Konsequenzen:**
  - **Bedingung:** Phase 1 muss den Leistungsnachweis erbringen (60 Bilder pro Sekunde, 10.000 Personen, kein Rückstand bei höchster Geschwindigkeit, auf Referenzhardware; lauffähig auf integrierter Grafik; Linux mit WebGL2). Scheitert er, wird per ADR neu entschieden; Rückfalloption ist Godot als Präsentation mit dem Rust-Kern.
  - Betriebsart des Kerns (natives Node-Modul über napi-rs oder WebAssembly) ist `[OFFEN]` bis Phase 1, Schritt 1.1.
  - Zwei Werkzeugketten (Cargo, npm), zwei Testaufbauten, gemeinsame CI; TypeScript-Typen der Schnittstellen werden aus Rust erzeugt.
  - Nach Modus 1.5 erneut geprüft (2026-10-13): Die bestätigten Anforderungen (insbesondere FR-017, FR-039, FR-040, FR-045) ändern die Stack-Wahl nicht.
  - **Versions-Verifikation (Modus 2 Schritt 2a, 2026-10-13):** Mindestreife „ein Patch-Release und Linie mindestens 1 Monat alt", Projektdauer 2 Jahre. Vom Projektinhaber zeilenweise bestätigt:

    | Komponente | Fixierte Version | Abweichung vom ersten Entwurf |
    |---|---|---|
    | Electron | 44.7.0 | – |
    | Node.js (Werkzeuge) | 26.11.1 | statt 24.21.0: einzige Linie mit Unterstützung bis Ende 2028 |
    | TypeScript | 6.0.3 | statt 7.0.2: typescript-eslint unterstützt TS 7 nicht (Peer `<6.1.0`) |
    | React | 19.2.8 | statt 19.3.0: 19.3 ohne Patch |
    | Vite / Tailwind CSS | 8.3.4 / 4.3.3 | – |
    | TanStack Table | 9.2.8 | statt 9.3.1: 9.3 zu jung |
    | PixiJS | 8.20.1 | statt 8.22.0: 8.21/8.22 ohne Patch |
    | electron-builder | 26.16.1 | statt 26.15.3 |
    | Rust | 1.98.1 | statt 1.99.0: 1.99 ohne Patch |
    | napi / napi-derive / @napi-rs/cli | 3.12.7 / 3.6.8 / 3.9.1 | statt 3.14.2 / – / 3.10.8 |
    | wasm-bindgen | 0.2.128 | statt 0.2.129 |
    | serde / rayon / ts-rs | 1.0.229 / 1.12.0 / 12.0.1 | rayon: Ausnahme, Projekt veröffentlicht keine Patches |
    | cargo-deny / cargo-llvm-cov | 0.20.2 / 0.9.1 | – |
    | Vitest / Playwright | 5.0.4 / 1.62.1 | Playwright statt 1.64.0 |
    | ESLint / typescript-eslint | 10.9.1 / 8.70.1 | statt 10.12.0 / 8.71.2 |
    | Prettier / lint-staged | 3.9.10 / 17.5.1 | – |

    Quellen und Lebensende: `project-context.md` Abschnitte 3 und 8.
  - **Hook-Framework:** `pre-commit` statt husky + lint-staged (Vorlage `templates/pre-commit/typescript.yaml`; husky ohne Release seit 2024; ein Framework für Rust, TypeScript und Markdown).
  - Electron und Rust haben rollierende Unterstützung; ein wiederkehrender Update-Schritt im Fahrplan (Q.1) hält sie aktuell.
- **Vision-Frage, die entschied:** (sinngemäß) „Was ist dir wichtiger: dass das Spiel auf einer echten Spiele-Engine steht, oder dass die vielen Tabellen und Panels so gut werden wie in modernen Web-Anwendungen und du den Aufbau selbst nachvollziehen kannst?" – Antwort: Option D (Web + Rust), nachdem die Empfehlung mit frischer Recherche neu bewertet wurde.
- **Konfidenz zum Zeitpunkt:** mittel; Umkehrbarkeit teuer für die Präsentation, billig für den Kern (Rückfall über godot-rust). Validierung in Phase 1 (ERKUNDUNG), Schritt 1.5.
- **Abgeleitete Regel:** Regel-001, Regel-002

#### ADR-003: Kern/Präsentation-Trennung, feste Simulationsschritte, Inhalte als Daten

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[MODUL]` `[SCHNITTSTELLE]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** Architektur-Pattern `[VORLÄUFIG]` bis Phase 1
- **Kategorie:** Architekturänderungen (CLAUDE.md 4.1)
- **Kontext:** Die Leistungsprobleme des Originals lagen in einer gewachsenen Architektur, in der Simulation, Darstellung und später ergänzte Systeme verwoben waren. Die Vision verlangt eine spätere andere Ansicht (isometrisch/3D), einen Benchmark und Testbarkeit; Inhalte sollen per Mod erweiterbar sein; Cargo muss später ohne Umbau ergänzbar sein.
- **Optionen:**
  - **A: Klassische Spielschleife** (Logik und Darstellung im selben Durchlauf) – Konsequenzen: einfacher Einstieg, aber Logik hängt an der Bildrate, schwer ohne Grafik testbar, Wiederholung der Fehler des Originals.
  - **B: Simulationskern getrennt von der Präsentation, feste Simulationsschritte, Inhalte als Daten** – Konsequenzen: klare Grenzen, Testbarkeit ohne Grafik, austauschbare Darstellung; dafür definierte Schnittstellen (Zustandspuffer, Befehlskanal, Ereignisstrom) nötig.
- **Entscheidung:** B.
- **Konsequenzen:**
  - Der Kern kennt keine Darstellung; die Präsentation enthält keine Spiellogik.
  - Simulationsschritte laufen in festen Zeitschritten, unabhängig von der Bildrate.
  - Flugzeugtypen, Airlines, Objekte, Fahrzeuge, Forschungsbaum sind Datenpakete mit Schema.
  - Determinismus des Kerns wird angestrebt (Prüfung in Phase 1).
- **Vision-Frage, die entschied:** (sinngemäß) „Soll eine spätere andere Ansicht (isometrisch) ohne Neubau der Spiellogik möglich sein, und soll das Spiel bei großen Flughäfen nicht mehr an der Bildrate hängen?" – Antwort: ja, beides.
- **Konfidenz zum Zeitpunkt:** hoch; Umkehrbarkeit teuer.
- **Abgeleitete Regel:** Regel-002, Regel-003

#### ADR-004: Wegfindung nach dem Wegweiser-Prinzip

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[PERFORMANCE]` `[MODUL]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** Modul Wegfindung `[VORLÄUFIG]` bis Phase 1, Schritt 1.2
- **Kategorie:** Architekturänderungen (CLAUDE.md 4.1)
- **Kontext:** Das Original berechnete Wege pro Person mit A*; die Wegfindung wurde laut eigenen Release Notes mit steigender Personenzahl deutlich langsamer, Bauänderungen lösten teure Neuberechnungen aus, und Passagiere blieben stecken. In der Konzeptphase wurde festgelegt, dass Passagiere der Beschilderung folgen statt Wege zu erkunden, und dass Schilder und Bodenmarkierungen die sichtbare Spielebene dieses Prinzips sind; ohne Schilder finden Passagiere ihr Ziel durch Suchen, nur langsamer.
- **Optionen:**
  - **A: A* pro Person** – Konsequenzen: einfach, aber Aufwand wächst mit der Personenzahl; Ursache der Probleme des Originals.
  - **B: Wegweiser je Ziel für die ganze Karte** (Flussfeld-artig), mit inkrementeller Aktualisierung und Feinwegen für die letzten Meter – Konsequenzen: Aufwand abhängig von der Zahl der Ziele; Unerreichbarkeit direkt erkennbar; Speicherbedarf bei vielen Zielen zu prüfen.
- **Entscheidung:** B.
- **Vision-Frage, die entschied:** (sinngemäß) „Sollen Passagiere ihren Weg wie im echten Flughafen über Schilder finden, statt jeden Weg zu kennen?" – Antwort: „Schilder und Markierungen ermöglichen das technische Prinzip"; ohne Schilder finden Passagiere ihr Ziel durch Suchen, nur langsamer.
- **Konfidenz zum Zeitpunkt:** mittel (Speicherbedarf bei vielen Zielen offen); Umkehrbarkeit teuer. Validierung in Schritt 1.2.
- **Konsequenzen:** Passagierführung (Schilder, Bodenmarkierungen) wirkt über die Bewertung von Kacheln in den Wegweisern; Unerreichbarkeit wird mit Ursache an den Spieler gemeldet; Leistungsnachweis in Phase 1.
- **Abgeleitete Regel:** keine

#### ADR-005: Verkehrsregeln, Vorrang-Rangfolge und automatische Verkehrssteuerung

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[MODUL]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** Modul Verkehr `[VORLÄUFIG]`
- **Kategorie:** Architekturänderungen (CLAUDE.md 4.1)
- **Kontext:** Im Original blieben Fahrzeuge an Kurven und Kreuzungen stecken, und die Flugzeug-Wegfindung konnte den halben Flughafen für ein Flugzeug anhalten. In der Konzeptphase festgelegt (Vorrangregeln geprüft gegen SERA.3210).
- **Optionen:**
  - **A: Freie Bewegung mit lokaler Kollisionsvermeidung** – Konsequenzen: Blockaden wie im Original.
  - **B: Regelbasierter Verkehr mit zentraler Verkehrssteuerung** – Konsequenzen: Fahrspuren, Vorfahrt, Reservierung, Rangfolge; aufwendiger, aber blockadefrei planbar.
- **Entscheidung:** B, mit folgenden Festlegungen:
  - Straßenverkehr mit Fahrspuren je Richtung; kein Einfahren in Kreuzungen, die nicht geräumt werden können.
  - Vorfahrt standardmäßig automatisch, je Kreuzung auf manuell umschaltbar (Vorfahrtsschilder, Stoppstellen, Ampeln).
  - Rollwege mit vom Spieler gezogenen Rollweg-Knoten wie im Original, je Abschnitt einstellbar.
  - Vorrang: 1. Einsatzfahrzeuge zu einem Flugzeug in Not, 2. Flugzeuge beim Landen, Starten und Rollen (auch geschleppt), 3. übrige Einsatzfahrzeuge im Einsatz, 4. Pushback und Schlepp, 5. Abfertigung, 6. sonstige Betriebsfahrzeuge, 7. öffentlicher Verkehr.
  - Automatische Verkehrssteuerung in der Rolle von Tower und Rollkontrolle: weist Rollrouten zu, vergibt Startfreigaben am Gate, „Aufrollen und Warten", löst Konflikte nach der Rangfolge, greift bei drohender Blockade ein; Eingriffe sind für den Spieler nachvollziehbar.
- **Vision-Frage, die entschied:** (sinngemäß) „Soll die Vorfahrt automatisch geregelt werden oder manuell?" – Antwort: wählbar, automatisch als Standard; dazu die Rangfolge mit Einsatzfahrzeugen zu einem Flugzeug in Not an erster Stelle und die „eingreifende automatische Verkehrssteuerung".
- **Konfidenz zum Zeitpunkt:** mittel (Verfahren zur Blockadevermeidung offen); Umkehrbarkeit mittel. Validierung in Schritt 3.4.
- **Konsequenzen:** Ein Stillstand, der sich nicht selbst auflöst, gilt als Fehler. Verfahren zur Blockadevermeidung wird vor der Umsetzung in einer Erkundung geklärt.
- **Abgeleitete Regel:** keine

#### ADR-006: Funktionsumfang: Inventar, Modernisierung, Cargo vorbereitet

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[METHODIK]` `[DATENMODELL]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** keine direkte
- **Kategorie:** Methodik
- **Kontext:** Erfolgskriterium ist die Funktionsgleichheit mit dem Original. In der Konzeptphase wurden ein Funktionsinventar (rund 150 Funktionen inkl. DLCs) und eine Modernisierungsliste (55 Punkte, je Kategorie entschieden) erstellt.
- **Optionen:**
  - **A: Funktionsumfang offen lassen und schrittweise entscheiden** – Konsequenzen: kein Maßstab, Gefahr von Drift.
  - **B: Inventar und Modernisierungsliste als verbindliche Referenz** – Konsequenzen: prüfbare Funktionsgleichheit, Fahrplan-Schritte verweisen auf Inventar-IDs.
- **Entscheidung:** B.
  - `funktionsinventar.md` ist der Maßstab für Funktionsgleichheit.
  - `modernisierung.md`: 19 Punkte „Kern" ersetzen die jeweilige Original-Funktion, 33 „Kandidat", 3 verworfen.
  - Cargo ist feste Erweiterung nach Funktionsgleichheit; Datenmodell und Flugbetrieb werden von Beginn an dafür vorbereitet.
  - Kurven beim Bauen bleiben ausgeschlossen; isometrische/3D-Ansicht ist unverbindlicher Kandidat.
- **Vision-Frage, die entschied:** n/a
- **Konfidenz zum Zeitpunkt:** n/a
- **Konsequenzen:** Jeder Umsetzungsschritt nennt die betroffenen Inventar- und Modernisierungs-IDs; der Abgleich der Funktionsgleichheit ist ein eigener Fahrplan-Schritt.
- **Abgeleitete Regel:** Regel-004

#### ADR-007: Vertrieb über GitHub, Lizenzen MIT und CC0

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[DEPLOYMENT]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** keine
- **Kategorie:** Lizenz- und Compliance-relevante Änderungen (4.8), Build- und Deploy-Pipeline (4.7)
- **Kontext:** Festlegungen der Konzeptphase: kostenlos, kein Gewinnziel, keine Store-Abhängigkeit, sauberer Abstand zum Original.
- **Optionen:**
  - **A: Steam mit Workshop** – Konsequenzen: Reichweite, aber Gebühr, proprietäres SDK, Plattformabhängigkeit; vom Projektverantwortlichen abgelehnt.
  - **B: GitHub Releases** – Konsequenzen: betreiberneutral, geringere Reichweite.
- **Entscheidung:** B. Code unter MIT; eigene Grafiken unter CC0; Audio aus freien Bibliotheken und KI-generiert erlaubt, abweichende Bedingungen in einem eigenen Verzeichnis mit Lizenzhinweis. Kein Material und keine Kompatibilität zum Original oder dessen Mods. Stores und Apple-Signatur werden später gesondert entschieden.
- **Vision-Frage, die entschied:** n/a
- **Konfidenz zum Zeitpunkt:** n/a
- **Konsequenzen:** Release-Builds für drei Plattformen über GitHub Actions; Lizenzprüfung der Abhängigkeiten in der CI.
- **Abgeleitete Regel:** Regel-005

#### ADR-008: Eigene SVG-Grafiken, Rasterung im Build

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[MODUL]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** Grafik-Pipeline `[VORLÄUFIG]`
- **Kategorie:** Architekturänderungen (CLAUDE.md 4.1)
- **Kontext:** Freie Grafiken sind uneinheitlich und lizenzrechtlich gemischt; flughafenspezifische Objekte gibt es frei praktisch nicht. In der Konzeptphase festgelegt: Grafiken aus einer Hand, als Vektoren erstellt, im Spiel als Bilddateien.
- **Optionen:**
  - **A: Freie Asset-Pakete** – Konsequenzen: Stilbruch, Lizenzmischung, Kernobjekte fehlen.
  - **B: Eigene SVG-Grafiken mit Stilleitfaden, Rasterung im Build** – Konsequenzen: einheitlich, CC0, anfangs schlicht.
- **Entscheidung:** B. Stilleitfaden (Palette, Strichstärken, Maßstab, Detailgrad) wird vor den ersten Grafiken festgelegt; SVG-Quellen liegen im Repository; der Build erzeugt Textur-Atlanten in mehreren Auflösungen.
- **Vision-Frage, die entschied:** (sinngemäß) „Sollen die Grafiken aus einer Hand kommen und frei weitergegeben werden dürfen?" – Antwort: eigene Vektorgrafiken, einmal gezeichnet, danach Bilddateien; frei lizenziert.
- **Konfidenz zum Zeitpunkt:** hoch; Umkehrbarkeit billig.
- **Konsequenzen:** Grafik-Pipeline als Teil des Builds; Freiwillige arbeiten später auf denselben Quellen.
- **Abgeleitete Regel:** keine

#### ADR-009: Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess

- **Datum:** 2026-10-13
- **Status:** Aktiv
- **Tags:** `[ERKENNTNIS]` `[STACK]` `[SCHNITTSTELLE]` `[PERFORMANCE]`
- **Phasentyp-Kontext:** ERKUNDUNG (Phase 1, Schritt 1.1)
- **Reifegrad-Wirkung:** Betriebsart des Rust-Kerns `[OFFEN]` → `[VORLÄUFIG]` (Bestätigung durch den Leistungsnachweis in 1.5); S-1 bleibt `[VORLÄUFIG]` mit geändertem Übertragungsweg
- **Kategorie:** Architekturänderungen (CLAUDE.md 4.1), mit Folgen für externe Abhängigkeiten (4.3)
- **Kontext:** ADR-002 ließ offen, ob der Rust-Kern als natives Node-Modul (napi-rs) oder als WebAssembly läuft. Spike 1.1 (`spikes/1.1-betriebsart/`) hat beide Varianten im gehärteten Electron 44.7.0 mit 10.000 Personen gemessen (Messprotokoll `spikes/1.1-betriebsart/README.md`, Rohdaten `results/matrix.jsonl`). Messumgebung: 4-vCPU-Container ohne GPU, nicht Referenzhardware; Windows und macOS nicht gemessen. Kein Rücknahme- oder Änderungs-ADR zu ADR-002, sondern Klärung eines dort ausdrücklich offen gelassenen Punkts; daher nicht `[REAKTIV]`.
- **Optionen:**
  - **A: napi-rs-Modul im Electron-Utility-Prozess, Bild je Frame per MessagePort kopiert** – Konsequenzen: 1,27–1,50 × schneller als WebAssembly bei gleicher Threadzahl (Arbeit 8, 4 Threads: 401 µs gegenüber 602 µs je Schritt); alle Kerne mit stabilem Rust 1.98.1; Absturz des Kerns lässt Fenster und Hauptprozess stehen; native Werkzeuge zur Fehlersuche. Dafür Kopie über die Prozessgrenze (160 KB bei 10.000 Personen: 1,1 ms Mittel, p99 2–3,4 ms; 800 KB bei 50.000: 3,4 ms) und je Plattform ein eigener Build (6 Ziele, macOS Universal per `lipo` bzw. `@electron/universal`).
  - **B: WebAssembly mit mehreren Threads im Web Worker, Dreifachpuffer im SharedArrayBuffer** – Konsequenzen: keine Kopie zur Anzeige, eine `.wasm` für alle Plattformen; aber 1,37–1,50 × langsamer als A und nur mit Nightly-Rust plus `-Z build-std` baubar (stabiles 1.98.1 meldet `atomics` als „unstable feature"; wasm-bindgen-Doku „Parallel Raytracing").
  - **C: WebAssembly mit einem Thread** – Konsequenzen: einfachster, stabiler Build, aber nur ein Kern; bei schwerer Last (Arbeit 32) 3,9 × langsamer als A mit 4 Threads.
  - Alle drei hielten 60 Bilder pro Sekunde im Renderer; Prüfsummen nach 1000 Ticks sind nativ und in WebAssembly bitgleich.
- **Entscheidung:** A, freigegeben vom Projektinhaber am 2026-10-13 („A, freigegeben, B perspektivisch"). B bleibt als spätere Option erhalten: Neubewertung in Fahrplan-Schritt Q.2, sobald WebAssembly-Threads mit stabilem Rust baubar sind. C ist verworfen (`architecture.md` Abschnitt 8).
- **Vision-Frage, die entschied:** „Ist dir Rechenreserve für sehr große Flughäfen wichtiger als ein einfacher Build- und Vertriebsweg?" – Antwort: Rechenreserve (A); der einfachere Weg B soll perspektivisch wieder geprüft werden.
- **Konfidenz zum Zeitpunkt:** mittel (Vergleich eindeutig, aber nicht auf Referenzhardware und nicht unter Windows/macOS gemessen); Umkehrbarkeit billig, solange der Kern frei von Anbindungscode bleibt (Regel-006).
- **Konsequenzen:**
  - Der Kern läuft in einem Electron-Utility-Prozess, gestartet und überwacht vom Modul Plattform; die Simulation rechnet auf eigenen Rust-Threads (rayon), der JavaScript-Thread des Utility-Prozesses beantwortet nur Anfragen.
  - S-1: Doppel-/Dreifachpuffer im Kern-Prozess, Übertragung je Bild als Kopie per `MessagePort` an den Renderer. Ein `SharedArrayBuffer` zwischen Prozessen ist nicht möglich, ein natives Modul im Sandbox-Renderer nicht ladbar. Schritt 1.3 klärt, ob bei großen Datenmengen nur sichtbare oder geänderte Daten übertragen werden.
  - S-2 (Befehle) und S-3 (Ereignisse) laufen über denselben `MessagePort`-Weg; Port-Übergabe als Handshake mit Bestätigung (Lehre aus dem Spike, Logbuch 2026-10-13).
  - Stack (Fortschreibung der Versionstabelle aus ADR-002): napi 3.12.7, napi-derive 3.6.8, @napi-rs/cli 3.9.1 wie fixiert; zusätzlich Begleit-Crate **napi-build 2.4.4** (napi 3.12.7 verlangt `^2.4.4`; Linie 2.4 seit 2026-07-28, mit Patches; Verifiziert: 2026-10-13, Quelle: crates.io) und Lockfile-Festsetzung **napi-derive-backend 6.1.4** (6.1.5 vom 2026-10-05 bricht den Build mit napi-derive 3.6.8). `Cargo.lock` wird eingecheckt; Aktualisierungen nur bewusst in Q.1.
  - wasm-bindgen 0.2.128 wird nicht Teil des Stacks; die Version bleibt nur als Ausgangspunkt für Q.2 vermerkt.
  - Build: je Plattform eine `.node`-Datei (Windows x64/arm64, Linux x64/arm64, macOS x64 + arm64 zu Universal); Prüfung in Schritt 1.6.
  - Der Leistungsnachweis 1.5 misst mit dieser Betriebsart; scheitert er am Übertragungsweg, wird B bzw. ein Hybrid über Q.2 neu bewertet.
- **Abgeleitete Regel:** Regel-006 (Kern frei von Anbindungscode)

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
