# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Laufende Phase im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-19 (Session ab 19:40 UTC, Phasenabschluss 2)
- **Laufende Phase:** Phasengrenze Phase 2 → Phase 3; Phase 2 abgeschlossen (alle 11 Schritte erledigt, archiviert in [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md))
- **Phasentyp:** Phase 3 UMSETZUNG (mit Entwurfsschritten im Phasentyp-Kontext ERKUNDUNG)
- **Aktiver Schritt:** keiner; Phasenabschluss 2 nach `CLAUDE.md` Abschnitt 12: Vision-Abgleich, Bewertung durch zwei getrennte Instanzen und Stellungnahme in [`docs/research/phasenabschluss-2.md`](research/phasenabschluss-2.md), Onboarding-Re-Validation
- **Nächster Schritt:** Antwort des Projektinhabers auf die Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“ (Vorlage `ENTSCHEIDUNG ERFORDERLICH`, Logbuch); danach ADR, ursprünglicher Schrittplan von Phase 3 und `[PHASEN-WECHSEL]`; erster Schritt von Phase 3 erst danach
- **Offene STOPP-Situationen:** Pflichtfrage an der Phasengrenze wartet auf Antwort (blockiert den Beginn von Phase 3). Reaktiv-Quote 6 / 10 (ADR-026 bis ADR-035) über dem Schwellenwert; Reflexion für Phase 2 in der Vorlage zum Phasenabschluss

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

### Phase 1: Technischer Durchstich und Leistungsnachweis – Typ: ERKUNDUNG – ABGESCHLOSSEN (2026-10-13)

**Phasen-Bilanz:** 7 Schritte nach ursprünglichem Plan (1.1–1.7), alle `[ERLEDIGT]`; Nachlauf 1.6-N1 erledigt. Stack, Pattern, Betriebsart, Datenmodell im Kern, Wegweiser-Prinzip, S-1 und große Tabellen `[BELASTBAR]` (ADR-009, ADR-010, ADR-013, ADR-014); NFR Performance auf das Gemessene begrenzt (ADR-017); Pflichtfrage „gezielt umbauen" (ADR-016). Reaktive ADRs: ADR-011, ADR-012, ADR-015, ADR-017. Volltext der Schritte: [`docs/archiv/fahrplan-phase-1.md`](archiv/fahrplan-phase-1.md).

### Phase 2: Fundament – Typ: UMSETZUNG – ABGESCHLOSSEN (2026-10-19)

**Phasen-Bilanz:** 11 Schritte bei ursprünglichem Plan 10 (2.11 kam am 2026-10-19 hinzu, unter der Wucherungs-Schwelle), alle `[ERLEDIGT]`; Nachläufe 2.1-N1, 2.1-N2, 2.2-N1 erledigt. Abschlusskriterium erfüllt: Spielrahmen mit grüner CI auf drei Plattformen, Szenario aus Spike 1.5 auf Produktionscode gleichwertig oder besser auf Referenz-PC und Mac (2.7). Durch Umsetzung `[BELASTBAR]`: Module Takt & Weltzustand, Welt & Bau (Grundraster), Brücke, Plattform, Inhalte & Mods (Lader), Lokalisierung, Grafik-Pipeline; S-2 bis S-7 per ADR-021 und bestätigt durch Umsetzung, S-8 neu (ADR-032); NFR Darstellung im gemessenen Umfang (ADR-034). Offen: Weltdarstellung im Spielfeld `[OFFEN]` (3.8), Bauaktion im nächsten Bild `[VORLÄUFIG]` (3.1). ADRs der Phase: ADR-018 bis ADR-034; reaktiv ADR-019, ADR-020, ADR-025, ADR-026, ADR-028 bis ADR-032. Pflichtfrage: siehe `docs/research/phasenabschluss-2.md`. Volltext der Schritte: [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md).

### Phase 3: Vertikaler Schnitt „Kleiner Flughafen" – Typ: UMSETZUNG

**Ziel:** Ein kleiner Flughafen ist von Anfang bis Ende spielbar.

**Abschlusskriterium:** Szenario 2 der Vision („Der Einsteiger") ist spielbar.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 3.7 | **Vor 3.1:** Entwurfsschritt (Phasentyp-Kontext ERKUNDUNG) „Personengruppen, Begehbarkeit und Querungen“: Gruppen aus dem Inventar (PAX-*, PAX-09, STAFF-01, SHOP-03, BAU-03; offen: Crew, Einweiser) mit Begehbarkeit je Gruppe einschließlich Personalzonen (ZON-02); Personal auf dem Vorfeld (nur per Fahrzeug, Stellplatzfläche begehbar oder beides, jeweils mit Erklärung bei Unerreichbarkeit; TURN-03, GEP-06, GATE-06); Bauarbeiter und laufende Bahnen; Querungen und Sperrung von Betriebsstraßen beim Walk-out-Boarding (MOD-13); Messung, wie Personal neben den Wegweisern geführt wird; Ergebnis ADR in Ergänzung von ADR-020 (Kategorie 1 und 4) | FR-018 (vorbereitend) |
| 3.8 | **Nach 3.7, vor 3.1:** Entwurfsschritt (Phasentyp-Kontext ERKUNDUNG) „Weltdarstellung im Spielfeld“ (ADR-032, L-1): wie Raster, Böden, Wände, Objekte und Zonen das Spielfeld erreichen – Ausgangshypothese geänderte Blöcke seit dem quittierten Stand im Bild (S-1), Gegenmodell Abruf über S-7; Messung von Erstübertragung (ganze Karte), Bauaktion bis zum Bild und Speicher im Renderer im Referenz-Terminal, dabei die Atlas-Stufen 16 und 32 px je Zelle (Nahansicht) und Atlanten mit vielen Grafiken (ADR-034); Ergebnis ADR (Kategorie 5, ggf. 1) mit Fortschreibung von S-1 bzw. S-7 | FR-010 (vorbereitend) |
| 3.1 | Bauen im Raster (eine Etage), Bauaufträge sofort sichtbar; Wirkung von `build`, `buildArea` und `demolish` im Kern (`TODO(fahrplan-ref: 3.1)` aus 2.2); Darstellung des Rasters im Spielfeld (aus 2.7) auf dem Weg aus 3.8; Katalog der Sitzung über S-7 `catalog` für Bau-Werkzeug und Vorschau, `game.new({ map })` in S-6 und S-8 (Grundform), Parameter `{ key }` in `ParamValue` (ADR-032, L-2, L-3, L-5); Bewegung der Bauarbeiter nach 3.7; Objekte im Weltzustand und Spielstand mit der Höchstzahl `MAX_OBJECTS` (aus 2.5) | FR-010, FR-011 |
| 3.2 | Kleines Terminal: Check-in, Sicherheit, Gate, Boarding, Toiletten; Passagierablauf | FR-012, FR-016, FR-019, FR-020, FR-022 |
| 3.3 | Passagierführung (Schilder, Markierungen) und erklärbare Wegprobleme; davor Erkundung „Suchen ohne Schilder" und Feinwege zum Einzelziel (aus 1.2) | FR-017, FR-018, FR-043 |
| 3.4 | Straßenverkehr, Bahn und Rollwege mit Knoten, Verkehrssteuerung, Vorfeld-Gehwege zu Stellplätzen ohne Fluggastbrücke (ADR-020), Querungen nach 3.7; Nachtsprung bei Flugzeugen in Bewegung sperren (`TODO(fahrplan-ref: 3.4)` aus 2.2, META-08); Fahrzeuge im Spielstand mit der Höchstzahl `MAX_VEHICLES` (aus 2.5); Schema der Inhaltsart `vehicles/` in Datenpaketen (aus 2.4, ADR-025); davor Erkundung Blockadevermeidung (ADR-005) | FR-014, FR-027, FR-028 |
| 3.5 | Abfertigung und Gepäck in Grundform; Wege des Vorfeldpersonals nach 3.7 | FR-025, FR-032 |
| 3.6 | GA und kleine Linienflüge, Flugplaner (große Tabelle über S-7, Sortierung im Kern), Geld und Bewertung in Grundform; Wörterbuch- und Detail-Texte als `{ key }`/`{ text }`, `tick` im Detail (ADR-032, L-6); Schema der Inhaltsarten `aircraft/` und `airlines/` in Datenpaketen (aus 2.4, ADR-025) | FR-030, FR-036, FR-039, FR-044 (Airline und Flugzeugtyp aus dem Mod-Ordner) |

### Phase 4: Härtung des vertikalen Schnitts – Typ: STABILISIERUNG

**Ziel:** Qualität und Leistung des vertikalen Schnitts sichern, bevor in die Breite gebaut wird.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 4.1 | Referenzszenario baubar, Leistungstest als CI-Gate; Messung auf echter 8-Kern-Hardware (Auflage ADR-014); Größe des Weltzustands im Spielstand messen und die Grenzen von S-5 (512 MB, ADR-030) prüfen | FR-040 |
| 4.2 | Coverage auf Mindestwerte, Langzeittest, erste Funktionsgleichheits-Prüfung der umgesetzten Inventar-IDs | – |
| 4.3 | Schrittzeit-Spitzen der Simulation auf Apple-Chips (ADR-034): auf dem Mac M3 Pro in 11 von 20 Fenstern Sekunden-p99 über 4 ms, Einzelschritte bis 75 ms (2.7, 2.10), mit 4 Hintergrund-Threads gemildert, nicht beseitigt; Ursache klären (Verteilung auf Leistungs- und Effizienzkerne, Priorität der Tick-Threads, Speicherbereinigung im Renderer) und die langen Aufgaben im Renderer aus Mac-Lauf 1 mitprüfen; Ergebnis Maßnahme oder ADR | – (NFR Performance) |

### Phase 5: Ausbau in die Breite – Typ: UMSETZUNG

**Ziel:** Funktionsumfang des Originals mit den Kern-Modernisierungen abbilden. Wird bei Phasenbeginn in Teilphasen oder Teil-Fahrpläne aufgeteilt; vor Verkehr und Gepäck jeweils eine Erkundung.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 5.1 | Spielstart vollständig: Standort, Kartengröße, Sandbox, Einstellungen (`game.new` vollständig, ADR-032, L-3); lokalisierte Meldung bei `incompatible` statt Konsolenzeile (ADR-032, R-8); Flughafenname im Weltzustand und in den Spielstand-Metadaten (META-02, Feld leer seit 2.5); Spielmenü mit Speichern, Laden und Spielstand-Liste (ersetzt die einfache Liste aus 2.5); Einstellungs-Menü (ersetzt das minimale Panel aus 2.6), dabei Obergrenze des Autosave-Intervalls (60 Minuten, Schätzung aus 2.6, ADR-031) überprüfen | FR-001, FR-002, FR-005 |
| 5.2 | Etagen, Zonen (international), Bauen vollständig | FR-010, FR-012 |
| 5.3 | Landseite: Parken, ÖPNV, Kreuzungen mit wählbarer Vorfahrt | FR-013, FR-015 |
| 5.4 | Terminal vollständig: moderner Check-in, Kontrollspuren, Grenze, Läden, Info | FR-019 bis FR-024 |
| 5.5 | Airside, Gepäck mit dreistufiger Kontrolle, Flüge, Abfertigung vollständig, Startfreigabe | FR-025, FR-026, FR-027, FR-029, FR-030, FR-032 |
| 5.6 | Personal, Wirtschaft, Forschung, Serviceziele, Notfälle, Störungen, Tutorial, Erfolge mit Vermerk der Debug-Nutzung im Spielstand (META-15, ADR-032, L-7); große Tabellen für Personal und Finanzen; Schema der Inhaltsart `research/` in Datenpaketen (aus 2.4, ADR-025) | FR-007, FR-008, FR-033 bis FR-039 |
| 5.7 | Flugzeugtypen und Airlines inklusive Add-on-Klassen | FR-031 |
| 5.8 | Benchmark mit Empfehlung, Leistungsbewusstsein (Empfehlung und Warnung nach bebauter, begehbarer Fläche und Zahl der Ziele, ADR-020); Messung auf schwacher integrierter Grafik (Auflage ADR-014) | FR-041, FR-042 |

### Phase 6: Inhalte, Mods und Atmosphäre – Typ: UMSETZUNG

**Ziel:** Spielgefühl und Erweiterbarkeit.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 6.1 | Mod-Verwaltung; davor Erkundung zu Code-Mods (`project-context.md` Abschnitt 11); Grafiken und Töne in Mods; Spielstände mit fehlenden Paketen laden unter Entfernen der betroffenen Objekte (ADR-021); aus der getrennten Prüfung 2.4: Pakete im Hintergrund laden statt auf dem JavaScript-Thread des Kern-Prozesses, neu abgelegte Pakete ohne Neustart erkennen (`docs/research/2.4-getrennte-pruefung.md`, Befunde 7 und 14); Überschreiben von Texten anderer Pakete in `lang/` (wie `replaces`, ADR-031); `lang.*` folgt der laufenden Sitzung statt der Auswahl für die nächste (ADR-032, L-10); prüfen, ob eine Gesamtgrenze für Einträge und Texte über alle Pakete nötig ist (Befund 2 der getrennten Prüfung von 2.6, `docs/research/2.6-getrennte-pruefung.md`) | FR-044 |
| 6.2 | Audio, Durchsagen, Musik, ggf. Intro-Video | FR-047, FR-049 |
| 6.3 | Abgleich der Funktionsgleichheit gegen das gesamte Inventar | – |

### Phase 7: Vorbereitung Version 1.0 – Typ: STABILISIERUNG

**Ziel:** Veröffentlichungsreife.

**Abschlusskriterium:** Erfolgskriterien der Vision erfüllt; Release v1.0.0.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 7.1 | Gate vor dem ersten öffentlichen Release (CLAUDE.md Abschnitt 12, `templates/gate-deployment.md`): Sicherheits-Review, Vertretung, Notfall-Handbuch, Signatur-Entscheidung; Lizenzhinweise aller ausgelieferten Abhängigkeiten im Paket (ADR-019) | – |
| 7.2 | Referenzszenario-Nachweis auf allen Plattformen inklusive Linux-Leistungsmessung auf echter Hardware mit gängigen Treibern (ADR-011, früher gern durch Mitwirkende), Langzeitstabilität, Release-Builds v1.0.0, auch für Windows und Linux auf arm64 (`project-context.md` Abschnitt 6; 2.1 baut dort nur x64); Linux-Pakete mit funktionierender Chromium-Sandbox (z. B. deb mit SUID-`chrome-sandbox` oder AppArmor-Profil, Befund aus 1.6); ab v1.0.0 Zusage für ältere Spielstände, Spielstände aus 1.0 als Prüfdaten aufbewahren (ADR-021) | FR-040, FR-045 |
| 7.3 | Erfolgskriterien der Vision ohne eigene Messgröße: Spieltest des Projektinhabers über mehrere Stunden („Gleichwertiges Spielerlebnis") und Bilanz des Machbarkeitstests der KI-gestützten Entwicklung (ADR-016) | – |

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
- **Zu tun:** Ablaufdaten-Register (`project-context.md` Abschnitt 8) abarbeiten: neue Electron-Linie, Rust-Toolchain, TypeScript 7 bei Unterstützung durch typescript-eslint, übrige Werkzeuge, abgekündigte Unterpakete von electron-builder (Register-Eintrag vom 2026-10-13), Audit-Befund „moderate“ sprintf-js über electron-builder (GHSA-hp3w-g68c-fv3c, Warnungs-Bestand seit 2026-10-19), benannte Ausnahme `syn@2.0.119` in `deny.toml` (ts-rs auf syn 3 umgestellt?, Warnungs-Bestand), benannte Ausnahme `miniz_oxide@0.8.9` (png auf miniz_oxide 0.9 umgestellt?, ADR-033); Tabelle zur Bestätigung vorlegen; ADR-002 per Folge-ADR fortschreiben.
- **Akzeptanzkriterien:** Keine fixierte Version ist über ihrem Lebensende; Register und `Verifiziert`-Vermerke aktuell; CI grün auf drei Plattformen.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Versions-Tabelle, Folge-ADR, aktualisierte Register
- **Notizen:** Nach dem ersten Lauf wird ein neuer Q.1-Termin mit der nächsten Frist angelegt.

#### Q.3: Toolchain in den Workflows mit `rustup toolchain install`

- **Status:** ERLEDIGT (2026-10-19)
- **Phasentyp-Kontext:** STABILISIERUNG (Querschnitt)
- **Abhängigkeiten:** keine
- **Freigabepflichtig:** ja (Kategorie 7), freigegeben am 2026-10-19 (ADR-035)
- **Empfohlene Klasse:** Routine – klar spezifizierte Umstellung eines Befehls nach Freigabe.
- **Eingangskriterien:** Freigabe
- **Anforderungen (ab Klasse M):** – (Infrastruktur)
- **Zu tun:** `run: rustup show` in `ci.yml` (6), `release.yml` (1) und `security.yml` (1) durch `run: rustup toolchain install` ersetzen (Abkündigung rustup#4836, Ablaufdaten-Register).
- **Akzeptanzkriterien:** CI auf drei Plattformen grün ohne den Hinweis „auto-installation is deprecated" im Protokoll.
- **Betroffene Module:** Build/CI
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Workflow-Dateien, ADR-035
- **Ergebnis (2026-10-19):** 8 Stellen umgestellt (`0cf1724`); grün auf drei Plattformen: Push-Lauf 39311772065 (17 Jobs), PR-Läufe CI 39311921712 und Security 39311921810; im Protokoll (Job „Rust Lint & Format“) installiert `rustup toolchain install` 1.98.1 ohne den Hinweis „auto-installation is deprecated“. Mit PR #26 nach `main` gemergt (`d4e457f`). DoD für eine Workflow-Änderung ohne Code: Prettier und pre-commit grün, CI grün; Tests und Coverage nicht anwendbar.

#### Q.2: Neubewertung WebAssembly mit Threads als Betriebsart

- **Status:** OFFEN (wartet auf Eingangskriterium)
- **Phasentyp-Kontext:** ERKUNDUNG (wiederkehrende Prüfung, unabhängig von der laufenden Phase)
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie
- **Zeitbox (nur ERKUNDUNG):** Prüfung des Eingangskriteriums je Q.1-Lauf 15 Minuten; Vergleichsmessung maximal eine Session
- **Abhängigkeiten:** 1.1 (Spike als Messgrundlage)
- **Frist:** erste Nachprüfung 2027-01-02 zusammen mit Q.1, danach bei jedem Q.1-Lauf
- **Freigabepflichtig:** ja, für einen Wechsel der Betriebsart (Architekturänderung)
- **Empfohlene Klasse:** Entscheidung – kann ADR-009 ändern (Architekturentscheidung).
- **Eingangskriterien:** WebAssembly mit Threads ist mit stabilem Rust baubar (Zielfunktion `atomics` stabil und Standardbibliothek ohne `-Z build-std` threadfähig, oder ein gleichwertiger stabiler Weg); bis dahin wird nur das Kriterium geprüft und im Logbuch vermerkt.
- **Anforderungen (ab Klasse M):** FR-040, FR-045
- **Zu tun:**
  - Bei jedem Q.1: Eingangskriterium anhand der Rust-Release-Notes und der wasm-bindgen-Doku prüfen.
  - Ist es erfüllt: Spike `spikes/1.1-betriebsart/` mit dem aktuellen Kern neu messen (wasmN gegen napi), zusätzlich auf Referenzhardware und unter Windows/macOS; Build-Aufwand für Variante A aus Phase 2 dagegenstellen.
  - Ergebnis als Entscheidungsvorlage: bei A bleiben, auf B wechseln oder Hybrid (z. B. WebAssembly für Teilaufgaben im Renderer).
- **Akzeptanzkriterien:** Wir wissen, ob B mit stabilem Rust baubar ist; falls ja, liegen Vergleichswerte und eine begründete Empfehlung vor.
- **Betroffene Module:** Takt & Weltzustand, Brücke, Plattform
- **Reifegrad-Wirkung:** keine, außer bei Wechsel (dann Betriebsart per Folge-ADR neu bewertet)
- **Artefakte:** Logbuch-Vermerk je Prüfung; bei Messung Messprotokoll und ggf. Folge-ADR zu ADR-009
- **Notizen:** Herkunft: Freigabe von ADR-009 („B perspektivisch“). Regel-006 hält den Kern dafür ohne Anbindungscode baubar für `wasm32-unknown-unknown`.

---

<!-- ANCHOR:iterations-reflexion -->
## Iterations-Reflexion

### Phase 1 (2026-10-13)

- **Gelernt:** Der Kern trägt mit großer Reserve (Schritt 0,3–0,8 ms bei 10.000 Personen von 5,5 ms Budget); Kopie je Bild über `MessagePort` genügt, Änderungslisten lohnen nicht; Neuberechnung der Wegweiser im Hintergrund statt inkrementell genügt bis 768² × 5.
- **Gekippte Annahmen:** Universal-Build für macOS (ADR-012: nur Apple-Chips); Linux-Messung in Phase 1 (ADR-011: 7.2); „Bauaktion im nächsten Bild" ohne Vorschau im Fenster nicht erreichbar.
- **Lücken:** Gemessen wurden nur Fußgänger mit einfacher Darstellung (ADR-017); Weltmaßstab, Luftseite, Zeitmodell und Schnittstellenverträge fehlten für Phase 2 (ADR-016).
- **Neue Erkundungen:** 2.8, 2.9; Messung der Darstellung mit echten Grafiken in 2.10.

### Reflexion Reaktiv-Quote (2026-10-13, Anlass ADR-015)

- **Befund:** 3 / 10 `[REAKTIV]` (ADR-011, ADR-012, ADR-015) über ADR-006 bis ADR-015, 30 % bei Schwellenwert 20 %.
- **Gemeinsamkeit:** Alle drei betreffen Testlogistik und Verpackung (Linux-Testhardware nicht verfügbar; Mac-Paketgröße und Ende von Intel-Macs; fehlende Signatur des arm64-Pakets als Folge von ADR-012). Kein reaktiver ADR betrifft Kern, Brücke, Datenmodell, Wegfindung oder Präsentation; diese sind per Spike (`[ERKENNTNIS]`) entschieden und seit ADR-014 `[BELASTBAR]`.
- **Ursache:** Verpackung ist in Phase 1 bewusst nur auf „baut und startet" erkundet (1.6), Signatur auf 7.1 gelegt. ADR-012 hat die Folgen für den Startweg auf macOS nicht geprüft – ADR-015 repariert diese Lücke.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI; vom Projektinhaber am 2026-10-13 bestätigt: „Zustimmung"). Die Häufung zeigt eine unreife Verpackung, die als `[VORLÄUFIG]` geführt ist, keine schwache Modularchitektur.
- **Maßnahmen:** (1) Jede Änderung an Paketziel oder Signatur prüft vor dem ADR den Startweg eines heruntergeladenen Pakets auf jeder betroffenen Plattform (Gatekeeper, SmartScreen). (2) Weitere Verpackungsfragen sammeln, nicht einzeln entscheiden; gebündelt in 7.1, sofern sie nicht Tests blockieren.
- **Ausblick:** Die Quote bleibt über dem Schwellenwert, bis ADR-011 aus dem Fenster fällt (ab ADR-017, falls nicht reaktiv) bzw. ADR-012 (ab ADR-018).

### Ergänzung Reaktiv-Quote (2026-10-13, Anlass ADR-017)

- **Befund:** 4 / 10 `[REAKTIV]` (ADR-011, ADR-012, ADR-015, ADR-017) über ADR-008 bis ADR-017, 40 %.
- **Ursache von ADR-017:** Die Beförderung in ADR-014 hat eine Sammelzeile („NFR Performance Gesamtsystem") befördert, obwohl die Messung nur Personen, Übergabe und einfache Darstellung abdeckte. Kein Fehler in Architektur oder Messung, sondern eine zu breite Reifegrad-Aussage; die getrennte Instanz hat sie gefunden.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI und der getrennten Instanz, `docs/research/phasenabschluss-1.md` Befund 4 und Teil 2). Die reaktiven ADRs betreffen Verpackung, Testlogistik und eine Reifegrad-Formulierung, nicht Kern, Brücke oder Datenmodell.
- **Kenntnisnahme:** Projektinhaber am 2026-10-13: „Zustimmung, Reflexion zur Kenntnis genommen".
- **Maßnahme (3):** Eine Beförderung auf `[BELASTBAR]` nennt den gemessenen Umfang in der Reifegrad-Zeile; was die Messung nicht abdeckt, steht als eigene Zeile `[VORLÄUFIG]`.
- **Ausblick:** Die Quote sinkt erst, wenn ADR-011, ADR-012 und ADR-015 aus dem Fenster fallen (ab ADR-018, ADR-019, ADR-022 bei nicht reaktiven Folge-ADRs). Phase 2 plant mit 2.8 und 2.9 die bekannten Entscheidungen vorab, damit dort keine reaktiven ADRs entstehen.

### Ergänzung Reaktiv-Quote (2026-10-13, Anlass ADR-019)

- **Befund:** 5 / 10 `[REAKTIV]` (ADR-011, ADR-012, ADR-015, ADR-017, ADR-019) über ADR-010 bis ADR-019, 50 %.
- **Ursache von ADR-019:** Die erlaubte Lizenzliste in Modus 2 wurde ohne Abgleich mit den Lizenzen der Werkzeug-Abhängigkeiten festgelegt; die Spikes haben npm-Lizenzen nicht geprüft. Der Befund kam erst mit der ersten echten Lizenzprüfung in 2.1. Wie bei ADR-011, ADR-012 und ADR-015 betrifft er Werkzeuge und Verpackung, nicht Kern, Brücke, Datenmodell oder Präsentation.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). ADR-019 ändert eine Compliance-Liste, keine Modulgrenze, Schnittstelle oder Datenhaltung.
- **Muster:** Vier der fünf reaktiven ADRs (ADR-011, ADR-012, ADR-015, ADR-019) entstehen an der Grenze zu Werkzeugen, Plattformen und Verpackung; ADR-017 an einer zu breiten Reifegrad-Aussage. Die Häufung zeigt, dass Annahmen über das Werkzeug-Umfeld vor der ersten Nutzung nicht geprüft wurden.
- **Maßnahme (4):** Vor jeder neuen Prüfung oder Liste mit Freigabe-Wirkung (Lizenzen, Audits, Plattformziele) läuft sie einmal gegen den tatsächlichen Bestand, und der Befund geht in dieselbe Freigabe ein. Für 2.1 bereits angewandt: `cargo deny check` und die npm-Zählung liefen vor dem Lizenz-Skript.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs sinkt die Quote ab ADR-021 (ADR-011 fällt heraus) auf 4 / 10, ab ADR-022 auf 3 / 10, ab ADR-025 auf 2 / 10 (20 %, Schwellenwert eingehalten).
- **Kenntnisnahme:** Projektinhaber am 2026-10-13: „Zur Kenntnis genommen"; kein Umbau, Maßnahme (4) gilt.

### Ergänzung Reaktiv-Quote (2026-10-13, Anlass ADR-020)

- **Befund:** 6 / 10 `[REAKTIV]` (ADR-011, ADR-012, ADR-015, ADR-017, ADR-019, ADR-020) über ADR-011 bis ADR-020, 60 %.
- **Ursache von ADR-020:** formal die 14-Tage-Regel (`CLAUDE.md` Abschnitt 6): ADR-020 ändert ADR-013 am Tag seiner Anlage. Inhaltlich war die Neubewertung geplant – ADR-013 (Konsequenzen) und ADR-016 haben sie an 2.8 übergeben. Die eigentliche Ursache liegt in 1.5: Die Kartengröße wurde festgelegt, bevor Zellgröße und Lage der Luftseite geklärt waren (Befund 8 der getrennten Instanz, `docs/research/phasenabschluss-1.md`: „Maßeinheit der Welt zuerst").
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). ADR-020 ändert die Speicherung der Wegweiser hinter derselben Schnittstelle („Richtung je Ziel und Zelle", ADR-010) und ergänzt Wegenetze, die der Verkehr ohnehin vorsah; Kern, Brücke, Takt und Prinzip der Wegfindung bleiben. Noch kein Weltcode betroffen (2.3 offen).
- **Muster:** Neben dem Werkzeug-Umfeld (Maßnahme 4) zeigt sich eine zweite Quelle: Festlegungen auf einer Größe, deren Einheit noch nicht bestimmt war.
- **Maßnahme (5):** Eine Größenfestlegung (Raster, Karte, Puffer, Grenzwert) nennt ihre Einheit und den Ausschnitt des Systems, an dem sie bemessen wurde; fehlt eines davon, wird sie als vorläufig mit Landeplatz geführt, nicht per ADR fixiert.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 5 / 10 ab ADR-021, 4 / 10 ab ADR-022, 3 / 10 ab ADR-025, 2 / 10 (Schwellenwert eingehalten) ab ADR-027. 2.9 entscheidet die geplanten Verträge, ohne zuvor fixierte ADRs zu ändern.
- **Kenntnisnahme:** Projektinhaber am 2026-10-13: „Reflexion zur Kenntnis genommen"; kein Umbau, Maßnahme (5) gilt.

### Ergänzung Reaktiv-Quote (2026-10-13, Anlass ADR-021)

- **Befund:** ADR-021 ist `[ERKENNTNIS]` (geplanter Entwurfsschritt 2.9, ändert keinen ADR binnen 14 Tagen). Quote 5 / 10 über ADR-012 bis ADR-021 (ADR-012, ADR-015, ADR-017, ADR-019, ADR-020), 50 % – weiter über dem Schwellenwert, sinkend wie im Ausblick zu ADR-020 vorhergesagt.
- **Architektur-Refactoring nötig?** Nein; keine neue Ursache. Maßnahme (5) wurde angewendet: alle Grenzen in ADR-021 nennen Einheit und Bezug.

### Ergänzung Reaktiv-Quote (2026-10-18, Anlass ADR-025)

- **Befund:** 4 / 10 `[REAKTIV]` über ADR-016 bis ADR-025 (ADR-017, ADR-019, ADR-020, ADR-025), 40 %. Der Ausblick zu ADR-020 („3 / 10 ab ADR-025“) ist damit nicht eingetreten.
- **Ursache von ADR-025:** formal die 14-Tage-Regel (`CLAUDE.md` Abschnitt 6): ADR-025 hebt die Aussage „`serde_json` nur in der Brücken-Crate“ aus ADR-024 vier Tage nach dessen Anlage auf. Inhaltlich war die Einschränkung enger als nötig: Ihre Begründung (Regel-006, kein Anbindungscode im Kern) betrifft eine JSON-Bibliothek nicht, und der JSON-Lader im Kern stand schon in ADR-021 (Konsequenzen, 2.4) und im Fahrplan.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). Keine Modulgrenze, kein Vertrag und kein Datenformat ändert sich, nur der Geltungsbereich einer Bibliothek.
- **Muster:** Neben dem Werkzeug-Umfeld (Maßnahme 4) und fehlenden Einheiten (Maßnahme 5) eine dritte Quelle: Aussagen zum Geltungsbereich in Abhängigkeits-ADRs ohne Abgleich mit den folgenden Fahrplan-Schritten.
- **Maßnahme (6):** Ein ADR, der eine Abhängigkeit auf einen Teil des Systems beschränkt, prüft vorher die Fahrplan-Schritte, die dieselbe Art Abhängigkeit vorsehen, und begründet die Beschränkung mit einer Regel. Ohne solche Begründung nennt er nur den derzeitigen Nutzer und legt keine Beschränkung fest.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 4 / 10 ab ADR-026, 3 / 10 ab ADR-027, 2 / 10 (Schwellenwert eingehalten) ab ADR-029.
- **Kenntnisnahme:** Projektinhaber am 2026-10-18: „Reflexionen zur Kenntnis genommen"; kein Umbau, Maßnahme (6) gilt.

### Ergänzung Reaktiv-Quote (2026-10-18, Anlass ADR-026)

- **Befund:** 5 / 10 `[REAKTIV]` über ADR-017 bis ADR-026 (ADR-017, ADR-019, ADR-020, ADR-025, ADR-026), 50 %; steigend.
- **Ursache von ADR-026:** formal die 14-Tage-Regel (ändert S-6 aus ADR-021 nach fünf Tagen). Inhaltlich zwei Ursachen: (a) Der Vertragstext aus 2.9 regelt Spielstände und Pakete in einem Satz, obwohl die Begründung (Dateinamen nie aus Eingaben des Renderers) nur für Spielstände trägt; Pakete brauchen eine stabile, vom Autor vergebene Kennung, damit die Auswahl einen Neustart übersteht. (b) Die bauende KI hat 2.4 umgesetzt, ohne den Satz aus S-6 gegen ihren Plan zu prüfen; erst die getrennte Prüfung fand die Abweichung. Ursache (b) ist ein Fehler im Vorgehen, kein Fehler der Architektur.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). Keine Modulgrenze, kein Datenformat; der Schutzzweck der Regel bleibt erfüllt.
- **Muster:** Wie bei ADR-025 eine Festlegung, die weiter reicht als ihre Begründung (Geltungsbereich einer Bibliothek dort, Geltungsbereich einer Vertragsregel hier).
- **Maßnahme (7):** Vor der Umsetzung eines Schritts, der einen Vertrag umsetzt, listet die KI die Sätze des Vertrags zu den berührten Funktionen und prüft jeden gegen ihren Plan; eine Abweichung wird vor dem Code vorgelegt, nicht danach. Ergänzend zu Maßnahme (6): Eine Vertragsregel, die mehrere Arten von Objekten betrifft, nennt ihre Begründung je Art.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 4 / 10 ab ADR-027 bis ADR-028, 3 / 10 ab ADR-029, 2 / 10 (Schwellenwert eingehalten) ab ADR-030.
- **Kenntnisnahme:** Projektinhaber am 2026-10-18: „Reflexionen zur Kenntnis genommen"; kein Umbau, Maßnahme (7) gilt.

---

### Ergänzung Reaktiv-Quote (2026-10-19, Anlass ADR-028)

- **Befund:** 5 / 10 `[REAKTIV]` über ADR-019 bis ADR-028 (ADR-019, ADR-020, ADR-025, ADR-026, ADR-028), 50 %; gleichbleibend (ADR-017 fällt heraus, ADR-028 kommt hinzu; ADR-027 ist `[OPERATIV]`).
- **Ursache von ADR-028:** formal die 14-Tage-Regel (ergänzt S-6 aus ADR-021 nach sechs Tagen); inhaltlich eine Lücke im Vertrag aus 2.9: S-5 verlangt Vorschaubilder in der Liste, S-6 enthält keinen Aufruf, der sie liefert. Gefunden vor dem Code durch Maßnahme (7) – die Maßnahme wirkt: Die Abweichung kam als Vorlage, nicht als Befund der getrennten Prüfung.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). Eine additive Funktion nach dem bestehenden Muster; keine Modulgrenze, kein Datenformat.
- **Muster:** Verträge aus 2.9 wurden je Schnittstelle geschrieben, aber nicht gegeneinander gelesen (S-5 setzt etwas voraus, das S-6 nicht anbietet).
- **Maßnahme (8):** Beim Abgleich nach Maßnahme (7) prüft die KI zusätzlich jeden Satz eines Vertrags, der eine andere Schnittstelle voraussetzt, gegen deren Text.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 4 / 10 ab ADR-029, 3 / 10 ab ADR-030, 2 / 10 (Schwellenwert eingehalten) ab ADR-035 (ADR-025 fällt erst mit ADR-035 aus dem Fenster).
- **Kenntnisnahme:** Projektinhaber am 2026-10-19: „Zur Kenntnis genommen, Reflexion und Abweichung“; kein Umbau, Maßnahme (8) gilt.

---

### Ergänzung Reaktiv-Quote (2026-10-19, Anlass ADR-029)

- **Befund:** 5 / 10 `[REAKTIV]` über ADR-020 bis ADR-029 (ADR-020, ADR-025, ADR-026, ADR-028, ADR-029), 50 %; gleichbleibend (ADR-019 fällt heraus, ADR-029 kommt hinzu).
- **Ursache von ADR-029:** dasselbe Muster wie bei ADR-028 – ein Satz in S-6 („Absturz des Kerns“) setzt eine Angabe voraus, die die Funktionsliste von S-6 nicht liefert. Gefunden vor dem Code durch Maßnahme (8); die Maßnahme wirkt.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI): ein zusätzliches Argument eines bestehenden Ereignisses, keine Modulgrenze, kein Datenformat.
- **Eigener Fehler beim Abgleich:** Punkt (e) widersprach ADR-028 (`unknownSave`); bemerkt beim Schreiben von ADR-029, vor dem Code. Lehre: Beim Abgleich auch die ADRs desselben Schritts gegen die eigenen Festlegungen lesen.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 4 / 10 ab ADR-030, 2 / 10 (Schwellenwert eingehalten) ab ADR-036.
- **Kenntnisnahme:** Projektinhaber am 2026-10-19: „Zur Kenntnis genommen, Reflexionen ADR-029 und ADR-030“; kein Umbau, Maßnahme (9) gilt.

---

### Ergänzung Reaktiv-Quote (2026-10-19, Anlass ADR-030)

- **Befund:** 5 / 10 `[REAKTIV]` über ADR-021 bis ADR-030 (ADR-025, ADR-026, ADR-028, ADR-029, ADR-030), 50 %; gleichbleibend (ADR-020 fällt heraus).
- **Ursache von ADR-030:** Die Grenzen von S-5 aus 2.9 waren ohne Messung der Größe echter Spielstände gewählt (1 GB Datei, 2 GB ausgepackt); erst die getrennte Prüfung zeigte die Speicherverstärkung durch eine kleine präparierte Datei. Kein Fehler der Architektur, sondern eine zu großzügig geschätzte Konstante im Vertrag.
- **Muster:** Wie bei 2.4 (Befund 1, Dateizahl) findet die getrennte Prüfung Grenzen, die zwar vertragsgemäß, aber nicht an gemessene Größen angelehnt sind.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI): zwei Konstanten, Nachmessung in 4.1 eingeplant.
- **Maßnahme (9):** Neue Obergrenzen in Verträgen nennen künftig die Messung oder Schätzung, aus der sie stammen, und den Schritt, der sie nachmisst.
- **Ausblick:** Mit nicht reaktiven Folge-ADRs bleibt der Wert 5 / 10 bis ADR-034; 4 / 10 ab ADR-035, 3 / 10 ab ADR-036, 2 / 10 (Schwellenwert eingehalten) ab ADR-038. Der Ausblick zu ADR-029 ist damit überholt.
- **Kenntnisnahme:** Projektinhaber am 2026-10-19: „Zur Kenntnis genommen, Reflexionen ADR-029 und ADR-030“; kein Umbau, Maßnahme (9) gilt.

---

### Ergänzung Reaktiv-Quote (2026-10-19, Anlass ADR-031)

- **Befund:** 6 / 10 `[REAKTIV]` über ADR-022 bis ADR-031 (ADR-025, ADR-026, ADR-028, ADR-029, ADR-030, ADR-031), 60 %; steigend (ADR-021 `[ERKENNTNIS]` fällt heraus, ADR-031 kommt hinzu).
- **Ursache von ADR-031:** dasselbe Muster wie bei ADR-028 und ADR-029 – S-3 und S-4 setzen voraus, dass Texte aus `lang/` die Oberfläche erreichen; S-6 bietet keinen Weg, die Modul-Karte keine Beziehung. Gefunden vor dem Code durch Maßnahme (8). Inhaltlich eine additive Funktion; reaktiv nur nach der 14-Tage-Regel, weil ADR-021 sechs Tage alt ist.
- **Muster:** Dritte Lücke derselben Art in S-6 binnen eines Tages. Die Verträge aus 2.9 beschreiben je Schnittstelle, was sie liefert, aber nicht, welche Daten zwischen den Schnittstellen fließen müssen. Die übrigen offenen Sätze dieser Art lassen sich jetzt vollständig prüfen statt je Schritt.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI): keine Modulgrenze ändert sich, der neue Weg folgt `packs.*`.
- **Maßnahme (10), Vorschlag:** Vor 2.10 einmal alle Sätze in S-1 bis S-7, die Daten einer anderen Schnittstelle oder eines anderen Moduls voraussetzen, gegen die Funktionslisten und die Modul-Karte lesen und die Funde gesammelt in einem ADR vorlegen (statt einzeln je Schritt).
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 6 / 10 bis ADR-034; 5 / 10 ab ADR-035, 2 / 10 (Schwellenwert eingehalten) ab ADR-039.
- **Kenntnisnahme:** Projektinhaber am 2026-10-19 per Auswahl: „Kenntnis + Maßnahme (10)“; kein Umbau; Maßnahme (10) als Schritt 2.11 vor 2.10 angelegt.

---

### Ergänzung Reaktiv-Quote (2026-10-19, Anlass ADR-032)

- **Befund:** 7 / 10 `[REAKTIV]` über ADR-023 bis ADR-032 (ADR-025, ADR-026, ADR-028, ADR-029, ADR-030, ADR-031, ADR-032), 70 %; steigend (ADR-022 `[OPERATIV]` fällt heraus, ADR-032 kommt hinzu).
- **Ursache von ADR-032:** Maßnahme (10) selbst – die Sammelprüfung 2.11 hat die Lücken zwischen den Verträgen aus 2.9 auf einmal gesucht und in einem ADR entschieden; reaktiv nach der 14-Tage-Regel, weil er ADR-021 (6 Tage alt) fortschreibt. Ohne Sammelprüfung wären dieselben 10 Lücken voraussichtlich einzeln in 2.10, 3.1, 3.6, 5.1, 5.6 und 6.1 als je eigener reaktiver ADR aufgetaucht.
- **Muster:** bestätigt die Diagnose zu ADR-031 – die Verträge aus 2.9 beschreiben je Schnittstelle, was sie liefert, aber nicht durchgängig, welche Daten zwischen Schnittstellen und Modulen fließen; dazu kamen ein Darstellungsfehler der Modul-Karte (doppelte Kennung `ABF`) und ein ungenannter Kanal (jetzt S-8).
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI): keine Modulgrenze verschoben, alle Ergänzungen additiv oder beschreibend; die einzige offene Grundsatzfrage (Weltdarstellung, L-1) hat mit 3.8 einen Entwurfsschritt vor ihrer ersten Umsetzung.
- **Maßnahme:** keine neue; (7) bis (9) gelten weiter. Erwartung: Funde nach (7)/(8) in 2.10 und Phase 3 sollten jetzt selten werden – tritt dennoch eine Vertragslücke der Art „setzt voraus, was nirgends angeboten wird“ auf, ist das ein Hinweis, dass die Prüfliste 2.11 unvollständig war (dann Ergänzung der Prüfliste, nicht nur ADR).
- **Ausblick:** Mit nicht reaktiven Folge-ADRs 7 / 10 bis ADR-034; 6 / 10 ab ADR-035, 5 / 10 ab ADR-036, 2 / 10 (Schwellenwert eingehalten) ab ADR-040.
- **Kenntnisnahme:** Projektinhaber am 2026-10-19 per Auswahl „Kenntnis + PR bei grün“; kein Umbau, Maßnahmen (7)–(9) gelten.

---

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- Schritte **ohne Abhängigkeiten zueinander**: 2.1 parallel zu 2.8; 2.4 parallel zu 2.5 und 2.6 (nach 2.7); Q.1 und Q.2 unabhängig von allen Phasen
- Schritte **mit gemeinsamen Modulen** (Konfliktgefahr): 2.2 und 2.3 (Takt & Weltzustand); 2.5 und 2.7 (Plattform)

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-10-14 – Phase 3 um Entwurfsschritt 3.7 „Personengruppen, Begehbarkeit und Querungen“ (vor 3.1) ergänzt; Anlass: ADR-020 regelt die Begehbarkeit des Vorfelds nur für Fluggäste, Personal und Bauarbeiter fehlen (Hinweis des Projektinhabers); 3.1, 3.4, 3.5 verweisen auf 3.7. Phase 3 hat noch keinen festgehaltenen ursprünglichen Schrittplan
- 2026-10-13 – Schritt 2.8 abgeschlossen (ADR-020): 2.2, 2.3, 2.9, 2.10, 3.4, 5.8 und das Abschlusskriterium von Phase 2 um Weltmaßstab, Blöcke, Wegenetze, Vorfeld-Gehwege und Zeitmodell ergänzt; keine neuen Schritte
- 2026-10-13 – Schritt 1.7 abgeschlossen (ADR-016, ADR-017): Phase 2 im vollen Format mit 10 Schritten (neu 2.8, 2.9, 2.10; 2.6 geteilt); FR-039 in 3.6 und 5.6; neuer Schritt 7.3; Phase 1 archiviert
- 2026-10-13 – Schritt 1.2 abgeschlossen (ADR-010); 1.5 um Rastergröße und fünf Etagen ergänzt; 3.3 um vorgeschaltete Erkundung „Suchen ohne Schilder"/Feinwege ergänzt (Landeplatz aus 1.2)
- 2026-10-13 – Schritt 1.1 abgeschlossen (ADR-009); Querschnitt Q.2 als Landeplatz für die zurückgestellte WebAssembly-Variante angelegt (zählt nicht zum Schrittplan von Phase 1); 1.3 um Übertragungsumfang ergänzt
- 2026-10-13 – Erstplanung in der Initialisierung (ADR-001 bis ADR-008); Nachzug nach aktuellem Regelwerk: Schritt-IDs für Phasen 2–7 mit Anforderungs-Zuordnung, Querschnitt Q.1

<!-- ANCHOR:archiv-abgeschlossene-phasen -->
## Archiv / abgeschlossene Phasen

- Phase 1: [`docs/archiv/fahrplan-phase-1.md`](archiv/fahrplan-phase-1.md)
- Phase 2: [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md)
