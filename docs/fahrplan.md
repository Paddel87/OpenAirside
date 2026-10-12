# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Phase 1 im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-13
- **Laufende Phase:** keine (Initialisierung abgeschlossen, Phase 1 bereit)
- **Phasentyp:** –
- **Aktiver Schritt:** keiner
- **Nächster Schritt:** 1.1 – Betriebsart des Rust-Kerns
- **Offene STOPP-Situationen:** keine

<!-- ANCHOR:phasen-typen -->
## Phasen-Typen

Jede Phase ist genau einem Typ zugeordnet. Der Typ bestimmt das Akzeptanzformat der Schritte.

### ERKUNDUNG

**Zweck:** Erkenntnis gewinnen, Annahmen validieren, Risiken vor der Umsetzung reduzieren.

- Akzeptanzkriterien sind **wissensbasiert**.
- Code ist Wegwerf-Code oder Spike, sofern nicht anders gekennzeichnet.
- Reduzierte Definition of Done; Erkenntnisse müssen in `decisions.md` oder `architecture.md` dokumentiert sein.
- Spike-Code, der weiterverwendet werden soll, durchläuft eine Stabilisierungsphase.

**Typische Schritt-Arten:** Spike, Prototyp, Vergleichsstudie, Lasttest / Messung.

### UMSETZUNG

**Zweck:** Geplante Funktionalität auf Basis belastbarer Architektur produktiv bauen.

- Akzeptanzkriterien sind **funktionsbasiert**.
- Berührte Architektur-Bestandteile müssen vor Schrittbeginn `[BELASTBAR]` sein – sonst Stopp.
- Volle Definition of Done (CLAUDE.md Abschnitt 9).
- Architektur-Lücke während der Umsetzung: Schritt stoppen, Lücke als `[OFFEN]` markieren, ERKUNDUNG-Schritt anlegen.

### STABILISIERUNG

**Zweck:** Härten, was in vorherigen Phasen entstanden ist.

- Akzeptanzkriterien sind **qualitätsbasiert**.
- Volle Definition of Done.
- Nach jeder Erkundungsphase, deren Ergebnisse weiterverwendet werden, Pflicht.

<!-- ANCHOR:schritt-format -->
## Schritt-Format

Jeder Schritt folgt diesem Schema. Abweichungen nur nach Freigabe.

```text
### [Phase].[Nummer]: Kurztitel

- **Status:** [OFFEN | VERSCHOBEN | IN ARBEIT | WARTET-AUF-FREIGABE | BLOCKIERT | ERLEDIGT | CI-AUSSTEHEND (mit Nachprüf-Schritt und Frist) | VERWORFEN]
- **Landeplatz (nur VERSCHOBEN):** [Ziel-Schritt-ID, z. B. „6.4" – ein Phasen-Verweis ohne Schritt-ID ist unzulässig, siehe CLAUDE.md Abschnitt 6]
- **Phasentyp-Kontext:** [ERKUNDUNG | UMSETZUNG | STABILISIERUNG] – ergibt sich aus der Phase
- **Schritt-Art (nur ERKUNDUNG):** [Spike | Prototyp | Vergleichsstudie | Lasttest | sonstiges]
- **Zeitbox (nur ERKUNDUNG):** [z. B. „maximal 4h Arbeit, dann Zwischenstand"]
- **Abhängigkeiten:** [Schritt-IDs, die vorher abgeschlossen sein müssen, oder "keine"]
- **Frist (Pflicht bei Abkündigungen, Ablaufdaten, Secret-Rotation):** [YYYY-MM-DD]
- **Freigabepflichtig:** [ja/nein – siehe CLAUDE.md Abschnitt 4]
- **Empfohlene Klasse:** [Mechanik | Routine | Entscheidung | Ausnahme] – [ein Satz Begründung aus CLAUDE.md Abschnitt 0]
- **Eingangskriterien:** [was muss gegeben sein, bevor der Schritt begonnen werden kann; bei UMSETZUNG: alle berührten Architektur-Bestandteile auf [BELASTBAR]]
- **Anforderungen (ab Klasse M):** [FR-IDs aus `docs/requirements.md`, die dieser Schritt umsetzt, oder „keine"]
- **Zu tun:** [konkrete Arbeitsanweisung; bei UMSETZUNG implementierungsnah, bei ERKUNDUNG: zu klärende Fragen]
- **Akzeptanzkriterien:** [phasentypabhängig – wissensbasiert / funktionsbasiert / qualitätsbasiert]
- **Betroffene Module:** [Modulnamen – wenn >1, ggf. aufsplitten]
- **Reifegrad-Wirkung:** [welche Architektur-Bestandteile werden durch diesen Schritt befördert oder zurückgestuft]
- **Artefakte:** [erwartete Dateien/Änderungen; bei ERKUNDUNG: ADRs, Architektur-Updates, Erkenntnisdokumente]
- **Notizen:** [optional: Hinweise, bekannte Fallstricke]
```

<!-- ANCHOR:aktuelle-phasen -->
## Aktuelle Phasen

### Phase 1: Technischer Durchstich und Leistungsnachweis – Typ: ERKUNDUNG

**Ziel:** Nachweis, dass der gewählte Stack (ADR-002) und das Architektur-Pattern (ADR-003) mit dem Wegweiser-Prinzip (ADR-004) das Leistungsziel tragen: 10.000 Personen, 60 Bilder pro Sekunde, kein Rückstand bei höchster Spielgeschwindigkeit auf der Referenzhardware, lauffähig auf integrierter Grafik, auf Windows, Linux und macOS.

**Abschlusskriterium:** Messergebnisse liegen vor und sind ausgewertet; ADR-002 ist per ERKENNTNIS-ADR bestätigt oder verworfen; die offenen Architekturfragen aus Schritt 1.1 und 1.2 sind entschieden.

**Reifegrad-Erwartung am Phasenende:** Stack, Architektur-Pattern, Wegfindung, S-1 Zustandspuffer und NFR Performance auf `[BELASTBAR]` (bei Bestehen); Betriebsart des Kerns und Datenmodell im Kern von `[OFFEN]` auf `[VORLÄUFIG]` oder `[BELASTBAR]`.

**Ursprünglicher Schrittplan:** 7 Schritte, festgehalten am 2026-10-13.

**Pflichtfrage am Phasenende:** ADR „Weiterbauen, umbauen oder neu aufsetzen" in Schritt 1.7 (CLAUDE.md Abschnitt 12).

**Hinweis:** Gesamter Code dieser Phase ist Spike-Code in einem eigenen Verzeichnis (`spikes/`). Was weiterverwendet wird, wird in Phase 2 sauber neu aufgebaut oder durchläuft eine Stabilisierung.

#### 1.1: Betriebsart des Rust-Kerns

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session, dann Zwischenstand
- **Abhängigkeiten:** keine
- **Freigabepflichtig:** ja, für das Ergebnis (Architekturentscheidung)
- **Empfohlene Klasse:** Entscheidung – Ergebnis ist eine Architekturentscheidung (Eskalations-Auslöser 1 und 3).
- **Eingangskriterien:** Repository angelegt; Versionen nach ADR-002 (Versions-Verifikation vom 2026-10-13)
- **Anforderungen (ab Klasse M):** FR-017, FR-040 (Vorarbeit)
- **Zu tun:**
  - Minimalen Rust-Kern bauen, der einen Zustand mit 10.000 Einträgen je Simulationsschritt fortschreibt und mehrere Kerne nutzt.
  - Variante A: als natives Node-Modul (napi-rs) im Electron-Hauptprozess oder einem Utility-Prozess, Übergabe des Zustands über gemeinsamen Speicher.
  - Variante B: als WebAssembly in einem Web Worker mit gemeinsamem Speicher und Threads.
  - Vergleichen: Rechenleistung, Mehrkern-Nutzung, Übergabe an den Renderer ohne Serialisierung, Build-Aufwand für drei Plattformen (inkl. macOS Universal), Debugbarkeit, Komplexität für spätere Arbeit.
- **Akzeptanzkriterien:** Beide Varianten laufen; Messwerte liegen vor; eine Variante ist begründet empfohlen und dem Projektverantwortlichen vorgelegt.
- **Betroffene Module:** Takt & Weltzustand, Brücke, Plattform
- **Reifegrad-Wirkung:** Betriebsart des Kerns `[OFFEN]` → `[VORLÄUFIG]`
- **Artefakte:** Spike-Code, ADR (ERKENNTNIS), Update `architecture.md`
- **Notizen:** Electron-Härtung (contextIsolation, Sandbox) von Anfang an beachten, sonst sind Messungen nicht repräsentativ.

#### 1.2: Wegweiser-Prinzip mit 10.000 Personen

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal zwei Sessions, dann Zwischenstand
- **Abhängigkeiten:** 1.1
- **Freigabepflichtig:** ja, für das Ergebnis (Datenmodell und Parallelisierung)
- **Empfohlene Klasse:** Entscheidung – Datenmodell und Parallelisierung sind Architekturentscheidungen; Beförderung auf BELASTBAR möglich.
- **Eingangskriterien:** Betriebsart aus 1.1 entschieden
- **Anforderungen (ab Klasse M):** FR-017
- **Zu tun:**
  - Testraster in Größe des Referenzszenarios mit zwei Etagen, Wänden, einer Sicherheitszone mit Kontrolle und mehreren Zieltypen (Check-in, Sicherheit, Gates, Toiletten, Ausgang).
  - Wegweiser je Ziel berechnen, 10.000 Personen mit wechselnden Zielen folgen lassen.
  - Bauänderung während der Simulation: inkrementelle Aktualisierung, Messung der Verzögerung.
  - Unerreichbarkeit erkennen und Ursache ermitteln.
  - Datenhaltung im Kern (kompakte Datenblöcke, Parallelisierung) festlegen und Determinismus prüfen.
  - Bodenmarkierung als Kachelbewertung in einer einfachen Form erproben.
- **Akzeptanzkriterien:** Wir wissen, wie viel Rechenzeit ein Simulationsschritt mit 10.000 Personen braucht, wie viel Speicher die Wegweiser belegen und wie schnell eine Bauänderung wirkt; Datenmodell und Parallelisierungsstrategie sind entschieden; Determinismus ist bestätigt oder die Abweichung begründet.
- **Betroffene Module:** Takt & Weltzustand, Welt & Bau, Wegfindung, Personen
- **Reifegrad-Wirkung:** Wegfindung `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen); Datenmodell im Kern `[OFFEN]` → `[VORLÄUFIG]`
- **Artefakte:** Spike-Code, Messprotokoll, ADR (ERKENNTNIS), Update `architecture.md`
- **Notizen:** Lehre aus dem Original: keine Prozesse, die sich über die Spielzeit ansammeln; Langzeitlauf mitmessen.

#### 1.3: Zustandsübergabe und Spielfeld-Darstellung

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session, dann Zwischenstand
- **Abhängigkeiten:** 1.1, 1.2
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Entscheidung – Beförderung von S-1 auf BELASTBAR (Eskalations-Auslöser 4).
- **Eingangskriterien:** Kern aus 1.2 lauffähig
- **Anforderungen (ab Klasse M):** FR-011, FR-040 (Vorarbeit)
- **Zu tun:**
  - Zustandspuffer (S-1) mit Doppel- oder Dreifachpuffer umsetzen.
  - PixiJS-Spielfeld in Electron: Raster, 10.000 Personen als Sprites, Zoom und Verschieben, Etagenwechsel, einfache Detailstufen.
  - Befehlskanal (S-2) in Minimalform: Bauaktion aus dem Spielfeld.
- **Akzeptanzkriterien:** Wir wissen, wie viele Bilder pro Sekunde das Spielfeld mit 10.000 Personen erreicht, wie lange die Übergabe je Bild dauert und ob eine Bauaktion im nächsten Bild sichtbar ist.
- **Betroffene Module:** Brücke, Spielfeld, Plattform
- **Reifegrad-Wirkung:** S-1 `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Spike-Code, Messprotokoll, Update `architecture.md`
- **Notizen:** WebGL2 als Grundlage, WebGPU nicht voraussetzen.

#### 1.4: Großes Tabellen-Panel neben dem Spielfeld

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.3
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Routine – durchspezifizierter Prototyp ohne Architekturentscheidung.
- **Eingangskriterien:** Spielfeld aus 1.3 lauffähig
- **Anforderungen (ab Klasse M):** FR-039
- **Zu tun:** Ein Panel im Stil des Flugmonitors mit mehreren tausend Zeilen, Sortierung und Live-Werten (React, TanStack Table, virtualisiert, gedrosselte Aktualisierung) neben dem laufenden Spielfeld.
- **Akzeptanzkriterien:** Wir wissen, ob das Panel die Bildrate des Spielfelds drückt und welche Aktualisierungsrate tragbar ist.
- **Betroffene Module:** Oberfläche, Brücke
- **Reifegrad-Wirkung:** Oberfläche mit großen Tabellen `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Spike-Code, Messprotokoll
- **Notizen:** –

#### 1.5: Leistungsnachweis auf den Zielsystemen

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Lasttest / Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 1.2, 1.3, 1.4
- **Freigabepflichtig:** ja (Bestätigung oder Verwerfung von ADR-002)
- **Empfohlene Klasse:** Entscheidung – bestätigt oder verwirft ADR-002 (STRATEGISCH).
- **Eingangskriterien:** Durchstich aus 1.1–1.4 lauffähig; Zugang zu Testsystemen: Mittelklasse-PC (Referenz), Rechner mit integrierter Grafik (z. B. Mac des Projektverantwortlichen), Linux-System
- **Anforderungen (ab Klasse M):** FR-040
- **Zu tun:**
  - Messung im Szenario mit 10.000 Personen bei höchster Spielgeschwindigkeit: Bildrate, Rechenzeit pro Simulationsschritt, Rückstand, Speicher über einen Langzeitlauf.
  - Wiederholung auf integrierter Grafik und unter Linux (WebGL2).
  - Ergebnis gegen die Zielwerte aus `project-context.md` Abschnitt 6 bewerten.
- **Akzeptanzkriterien:** Bewertung liegt vor: Ziel erreicht / mit benannten Maßnahmen erreichbar / nicht erreichbar. Entscheidungsvorlage an den Projektverantwortlichen.
- **Betroffene Module:** alle des Durchstichs
- **Reifegrad-Wirkung:** bei Bestehen: Stack, Pattern, NFR Performance `[VORLÄUFIG]` → `[BELASTBAR]`; bei Scheitern: ADR mit Neubewertung (Rückfalloption Godot-Präsentation mit Rust-Kern)
- **Artefakte:** Messprotokoll, ADR (ERKENNTNIS), Update `architecture.md` Abschnitt 9
- **Notizen:** Falls keine Mittelklasse-Referenz verfügbar ist, Messung auf verfügbarer Hardware und Hochrechnung mit Begründung; das Fehlen ist dann als Informationslücke zu melden.

#### 1.6: Verpackung für drei Plattformen

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.1
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Routine – Prototyp der Verpackung nach festen Vorgaben; Ergebnis wird in 1.7 bewertet.
- **Eingangskriterien:** Betriebsart aus 1.1 entschieden
- **Anforderungen (ab Klasse M):** FR-045
- **Zu tun:** Builds des Durchstichs für Windows, Linux und macOS (Universal) über GitHub Actions erzeugen; Start auf jedem System prüfen; Verhalten von macOS bei unsignierter Anwendung dokumentieren.
- **Akzeptanzkriterien:** Wir wissen, ob und mit welchem Aufwand alle drei Plattformen aus der CI gebaut werden können, und was ein Spieler unter macOS ohne Signatur tun muss.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** Verpackung `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Workflow-Entwurf, Erkenntnisnotiz
- **Notizen:** Änderungen an der Build-Pipeline sind ab Phase 2 freigabepflichtig (CLAUDE.md 4.7).

#### 1.7: Auswertung und Entscheidung

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** sonstiges (Auswertung)
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.1–1.6
- **Freigabepflichtig:** ja
- **Empfohlene Klasse:** Entscheidung – Pflichtfrage am Phasenende und ERKENNTNIS-ADR zu ADR-002.
- **Eingangskriterien:** alle Messprotokolle liegen vor
- **Anforderungen (ab Klasse M):** keine (Auswertung)
- **Zu tun:** Erkenntnisse zusammenfassen, ADR-002 bestätigen oder verwerfen (ERKENNTNIS-ADR), Reifegrade nachziehen, Iterations-Reflexion schreiben, Phase 2 im vollen Schrittformat verfeinern.
- **Akzeptanzkriterien:** Entscheidung des Projektverantwortlichen liegt vor; Phase 2 ist ausgeplant.
- **Betroffene Module:** –
- **Reifegrad-Wirkung:** siehe Phasenziel
- **Artefakte:** ADR, Reflexion, verfeinerter Fahrplan
- **Notizen:** –

### Phase 2: Fundament – Typ: UMSETZUNG

**Ziel:** Produktionsreife Grundlage auf Basis der Erkenntnisse aus Phase 1.

**Abschlusskriterium:** leerer, aber vollständig lauffähiger Spielrahmen mit grüner CI auf drei Plattformen.

**Grobe Schritte (Verfeinerung im vollen Format in 1.7):**

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 2.1 | Repository-Struktur, alle CI-Gates laut `project-context.md` Abschnitt 7, Pre-Commit-Hooks, Branch-Schutz, `.editorconfig` | – |
| 2.2 | Simulationskern: Takt, Zeitsteuerung, Befehlskanal, Ereignisstrom, Determinismus-Tests | FR-003 |
| 2.3 | Weltzustand und Datenmodell, Fracht im Schema vorbereitet | FR-046 |
| 2.4 | Datenpaket-Schema (S-4) und Lader | FR-044 |
| 2.5 | Spielstand-Format (S-5) mit Autosave | FR-004 |
| 2.6 | Lokalisierung (Deutsch, Englisch, Sprachpakete), Stilleitfaden und SVG-Pipeline (ADR-008) | FR-006, FR-050 |
| 2.7 | Brücke und Plattformschicht mit Electron-Härtung, Konsole/Debug-Panel | FR-009 |

### Phase 3: Vertikaler Schnitt „Kleiner Flughafen" – Typ: UMSETZUNG

**Ziel:** Ein kleiner Flughafen ist von Anfang bis Ende spielbar.

**Abschlusskriterium:** Szenario 2 der Vision („Der Einsteiger") ist spielbar.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 3.1 | Bauen im Raster (eine Etage), Bauaufträge sofort sichtbar | FR-010, FR-011 |
| 3.2 | Kleines Terminal: Check-in, Sicherheit, Gate, Boarding, Toiletten; Passagierablauf | FR-012, FR-016, FR-019, FR-020, FR-022 |
| 3.3 | Passagierführung (Schilder, Markierungen) und erklärbare Wegprobleme | FR-018, FR-043 |
| 3.4 | Straßenverkehr, Bahn und Rollwege mit Knoten, Verkehrssteuerung; davor Erkundung Blockadevermeidung (ADR-005) | FR-014, FR-027, FR-028 |
| 3.5 | Abfertigung und Gepäck in Grundform | FR-025, FR-032 |
| 3.6 | GA und kleine Linienflüge, Flugplaner, Geld und Bewertung in Grundform | FR-030, FR-036 |

### Phase 4: Härtung des vertikalen Schnitts – Typ: STABILISIERUNG

**Ziel:** Qualität und Leistung des vertikalen Schnitts sichern, bevor in die Breite gebaut wird.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 4.1 | Referenzszenario baubar, Leistungstest als CI-Gate | FR-040 |
| 4.2 | Coverage auf Mindestwerte, Langzeittest, erste Funktionsgleichheits-Prüfung der umgesetzten Inventar-IDs | – |

### Phase 5: Ausbau in die Breite – Typ: UMSETZUNG

**Ziel:** Funktionsumfang des Originals mit den Kern-Modernisierungen abbilden. Wird bei Phasenbeginn in Teilphasen oder Teil-Fahrpläne aufgeteilt; vor Verkehr und Gepäck jeweils eine Erkundung.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 5.1 | Spielstart vollständig: Standort, Kartengröße, Sandbox, Einstellungen | FR-001, FR-002, FR-005 |
| 5.2 | Etagen, Zonen (international), Bauen vollständig | FR-010, FR-012 |
| 5.3 | Landseite: Parken, ÖPNV, Kreuzungen mit wählbarer Vorfahrt | FR-013, FR-015 |
| 5.4 | Terminal vollständig: moderner Check-in, Kontrollspuren, Grenze, Läden, Info | FR-019 bis FR-024 |
| 5.5 | Airside, Gepäck mit dreistufiger Kontrolle, Flüge, Abfertigung vollständig, Startfreigabe | FR-025, FR-026, FR-027, FR-029, FR-030, FR-032 |
| 5.6 | Personal, Wirtschaft, Forschung, Serviceziele, Notfälle, Störungen, Tutorial, Erfolge | FR-007, FR-008, FR-033 bis FR-038 |
| 5.7 | Flugzeugtypen und Airlines inklusive Add-on-Klassen | FR-031 |
| 5.8 | Benchmark mit Empfehlung, Leistungsbewusstsein | FR-041, FR-042 |

### Phase 6: Inhalte, Mods und Atmosphäre – Typ: UMSETZUNG

**Ziel:** Spielgefühl und Erweiterbarkeit.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 6.1 | Mod-Verwaltung; davor Erkundung zu Code-Mods (`project-context.md` Abschnitt 11) | FR-044 |
| 6.2 | Audio, Durchsagen, Musik, ggf. Intro-Video | FR-047, FR-049 |
| 6.3 | Abgleich der Funktionsgleichheit gegen das gesamte Inventar | – |

### Phase 7: Vorbereitung Version 1.0 – Typ: STABILISIERUNG

**Ziel:** Veröffentlichungsreife.

**Abschlusskriterium:** Erfolgskriterien der Vision erfüllt; Release v1.0.0.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 7.1 | Gate vor dem ersten öffentlichen Release (CLAUDE.md Abschnitt 12, `templates/gate-deployment.md`): Sicherheits-Review, Vertretung, Notfall-Handbuch, Signatur-Entscheidung | – |
| 7.2 | Referenzszenario-Nachweis auf allen Plattformen, Langzeitstabilität, Release-Builds v1.0.0 | FR-040, FR-045 |

### Querschnitt: wiederkehrende Schritte

#### Q.1: Versions-Nachzug (Electron, Rust, Werkzeuge)

- **Status:** OFFEN
- **Phasentyp-Kontext:** STABILISIERUNG (wiederkehrend, unabhängig von der laufenden Phase)
- **Abhängigkeiten:** keine
- **Frist:** 2027-01-02 (zwei Monate vor Lebensende Electron 44 am 2027-03-02); danach je Electron-Linie neu
- **Freigabepflichtig:** ja (neue Linien nach `CLAUDE.md` Abschnitt 15)
- **Empfohlene Klasse:** Entscheidung – Wahl neuer Linien ist eine Entscheidung über externe Abhängigkeiten.
- **Eingangskriterien:** Vorlauf im Ablaufdaten-Register erreicht
- **Anforderungen (ab Klasse M):** FR-045
- **Zu tun:** Ablaufdaten-Register (`project-context.md` Abschnitt 8) abarbeiten: neue Electron-Linie, Rust-Toolchain, TypeScript 7 bei Unterstützung durch typescript-eslint, übrige Werkzeuge; Tabelle zur Bestätigung vorlegen; ADR-002 per Folge-ADR fortschreiben.
- **Akzeptanzkriterien:** Keine fixierte Version ist über ihrem Lebensende; Register und `Verifiziert`-Vermerke aktuell; CI grün auf drei Plattformen.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Versions-Tabelle, Folge-ADR, aktualisierte Register
- **Notizen:** Nach dem ersten Lauf wird ein neuer Q.1-Termin mit der nächsten Frist angelegt.

---

<!-- ANCHOR:iterations-reflexion -->
## Iterations-Reflexion

Noch keine abgeschlossene Phase.

---

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- Schritte **ohne Abhängigkeiten zueinander**: 1.6 parallel zu 1.2–1.4 (nach 1.1); Q.1 unabhängig von allen Phasen
- Schritte **mit gemeinsamen Modulen** (Konfliktgefahr): 1.2 und 1.3 (Takt & Weltzustand, Brücke)

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-10-13 – Erstplanung in der Initialisierung (ADR-001 bis ADR-008); Nachzug nach aktuellem Regelwerk: Schritt-IDs für Phasen 2–7 mit Anforderungs-Zuordnung, Querschnitt Q.1

<!-- ANCHOR:archiv-abgeschlossene-phasen -->
## Archiv / abgeschlossene Phasen

Noch keine.
