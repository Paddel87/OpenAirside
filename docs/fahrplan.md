# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Laufende Phase im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-13 (Sessionstart 22:17 UTC)
- **Laufende Phase:** Phase 2 – Fundament
- **Phasentyp:** UMSETZUNG (mit Entwurfsschritten 2.8 und 2.9, Phasentyp-Kontext ERKUNDUNG)
- **Aktiver Schritt:** keiner (2.2 erledigt am 2026-10-13, ADR-022; Nachlauf 2.2-N1 beim Projektinhaber)
- **Nächster Schritt:** 2.3 (Weltzustand, Datenmodell und Wegweiser)
- **Offene STOPP-Situationen:** keine (Reaktiv-Quote 5 / 10 nach ADR-021 weiter über dem Schwellenwert, aber sinkend; Reflexion zu ADR-020 vom Projektinhaber zur Kenntnis genommen, Ergänzung zu ADR-021 siehe „Iterations-Reflexion")

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

### Phase 2: Fundament – Typ: UMSETZUNG

**Ziel:** Produktionsreife Grundlage auf Basis der Erkenntnisse aus Phase 1, mit vorab festgelegtem Weltmaßstab und belastbaren Schnittstellenverträgen (ADR-016).

**Abschlusskriterium:** leerer, aber vollständig lauffähiger Spielrahmen mit grüner CI auf drei Plattformen; das Szenario aus Spike 1.5 (10.000 Personen, Terminal 768² × 5 in einer Welt 2048² nach ADR-020, Bauaktionen) läuft auf Produktionscode mit gleichwertigen Kernwerten (Simulationsschritt, Neuberechnung der Wegweiser, Speicher ohne Anstieg).

**Reifegrad-Erwartung am Phasenende:** S-2 bis S-7 `[BELASTBAR]` (per ADR in 2.9, bestätigt durch Umsetzung); Module Takt & Weltzustand, Welt & Bau (Grundraster), Brücke, Plattform, Inhalte (Lader), Lokalisierung `[BELASTBAR]` durch Umsetzung; NFR Darstellung mit echten Grafiken nach Messung in 2.10.

**Ursprünglicher Schrittplan:** 10 Schritte, festgehalten am 2026-10-13 in Schritt 1.7 (ADR-016).

**Reihenfolge:** 2.1 und 2.8 parallel → 2.9 → 2.2 → 2.3 → 2.7 → 2.4 → 2.5 → 2.6 → 2.10. Die Nummern 2.1–2.7 stammen aus der Grobplanung (Verweise in `requirements.md` und `project-context.md`), 2.8–2.10 kamen in 1.7 hinzu.

**Phasentyp-Hinweis:** 2.8 und 2.9 sind Entwurfsschritte mit Phasentyp-Kontext ERKUNDUNG innerhalb der Umsetzungsphase (wie die Erkundungen vor 3.3 und 3.4). Sie liefern Entscheidungen, keinen Code.

**Übernahme aus den Spikes:** Spike-Code wird nicht kopiert, sondern je Schritt neu geschrieben und getestet. Vorlagen: Wegweiser `spikes/1.2-wegweiser/src/field.rs` (2.3), Dreifachpuffer und Bildformat `spikes/1.3-spielfeld/bridge-core` (2.7), Electron-Härtung mit CSP und MessagePort-Handshake `spikes/1.5-leistung/app/main.mjs` (2.7), Personen als `ParticleContainer` (2.7), arm64-Build mit Ad-hoc-Signatur (ADR-015, 2.1).

#### 2.1: Repository-Struktur, CI-Gates und Werkzeuge

- **Status:** ERLEDIGT 2026-10-13 (ADR-018, ADR-019); CI-Lauf 38440669665 und Release-Lauf 38440601610 grün auf drei Plattformen
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 1.7
- **Freigabepflichtig:** ja (Kategorie 7 Build- und Deploy-Pipeline; Kategorie 3 bei Änderung der napi-Versionen)
- **Empfohlene Klasse:** Entscheidung – CI-Gates und Festsetzung von Abhängigkeiten stehen als Freigabe an; Teilarbeiten (Konfigurationsdateien, Runbook) sind Routine.
- **Eingangskriterien:** Stack und Betriebsart `[BELASTBAR]` (ADR-014) – erfüllt
- **Anforderungen (ab Klasse M):** FR-045
- **Zu tun:**
  - Verzeichnisse `core/` (Kern-Crates), `bridge/` (napi-Crate, Regel-006), `app/` (Electron, TypeScript); Cargo-Workspace, `rust-toolchain.toml`, `package.json` mit den Versionen aus `project-context.md` Abschnitt 3 und 7.
  - napi-derive-backend bewusst festsetzen (`=6.1.4` im Manifest) oder eine verträgliche napi-derive-Version wählen (Regel-001, Tabelle zur Bestätigung).
  - `ci.yml`: Code-Jobs aktiv; Coverage-Gate 85 % nur für die Kern-Crates, 70 % global (`project-context.md` Abschnitt 7); Lizenzprüfung npm und `cargo-deny`.
  - `release.yml`: arm64-Mac mit Ad-hoc-Signatur nach ADR-015; Startweg eines heruntergeladenen Pakets prüfen (Maßnahme 1 der Reaktiv-Reflexion).
  - Spike-Workflows (`spike-build*.yml`) einfrieren oder entfernen – Entscheidung in der Freigabe.
  - `.editorconfig`, Pre-Commit-Hooks für alle Sprachen, Branch-Schutz auf `main` (durch den Projektinhaber in den GitHub-Einstellungen).
  - `docs/onboarding-runbook.md` erstbefüllen und gegen einen frischen Klon validieren.
- **Akzeptanzkriterien:** Ein frischer Klon lässt sich nach Runbook auf Linux einrichten; alle Pflicht-Gates laufen lokal und in der CI auf drei Plattformen grün; ein leeres Electron-Fenster startet mit geladenem napi-Modul im Utility-Prozess; ein frischer Workspace baut ohne vorhandene `Cargo.lock` mit der festgesetzten Version.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** keine (Verpackung bleibt `[VORLÄUFIG]` bis 7.1/7.2)
- **Artefakte:** Workspace-Gerüst, Workflows, `.editorconfig`, Runbook, ggf. ADR zur napi-Version
- **Ergebnis:** `core/` (openairside-core), `bridge/` (napi, `test = false`), `app/` (Electron mit Kern im Utility-Prozess, leeres gehärtetes Fenster), `scripts/check-licenses.mjs`; Gates lokal und in der CI grün. Coverage: Rust-Kern 100 % Lines (1 Test), TypeScript und Skripte 87,2 % Lines (53 Tests; App-Module 100 %, Lizenz-Skript 81,7 %). Rauchtest auf Windows, Linux und macOS (Entwicklungs-Build in `ci.yml`, gepackte Programme in `release.yml`); erzwungener Fehlerfall belegt (ohne natives Modul rot). Runbook am frischen Worktree unter Linux validiert. Abweichung: `c13ebd9` trägt Typ `chore`, enthält aber auch eine CI-Änderung (Logbuch 17:36 UTC).
- **Nachlauf (zählt nicht zur Phase):**
  - **2.1-N1 `[ERLEDIGT]` 2026-10-13 – Branch-Schutz auf `main` (Projektinhaber):** Ruleset `main-schutz` (aktiv, Ziel Standard-Branch): Pull Request vor Merge mit 0 Freigaben, 18 Pflicht-Checks (alle Jobs aus `ci.yml` und beide CodeQL-Jobs), Force-Push und Löschen gesperrt; Branch nicht zwingend aktuell. Eingerichtet laut Projektinhaber; von der KI nicht direkt einsehbar. Nachweis bei PR Paddel87/OpenAirside#15 (2026-10-13): Merge war gesperrt, solange eine Prüfung rot angezeigt wurde, und nach grünem Lauf freigegeben (Beobachtung des Projektinhabers); die Einzelliste der 18 Pflicht-Checks wurde nicht geprüft.
  - **2.1-N2 `[ERLEDIGT]` 2026-10-13 – Startweg heruntergeladener Pakete (Projektinhaber, Maßnahme 1):** Artefakte des Release-Laufs 38440601610. **Windows** 11 Pro 25H2 (Build 26200.9457): keine Browser-Warnung; SmartScreen-Warnung, Zulassen möglich; Installer mit Wahl „für mich / alle Nutzer" und Zielordner; Fenster „OpenAirside" startet. **macOS** (Mac des Projektinhabers, Version nicht erfasst): DMG ohne Browser-Warnung; Gatekeeper-Dialog „In den Papierkorb legen / Fertig", keine Meldung „beschädigt"; Systemeinstellungen → Datenschutz & Sicherheit → „Dennoch öffnen" mit Touch ID; Fenster startet. Ad-hoc-Signatur (ADR-015) wirkt auch beim Paket aus `release.yml`. Befunde: Electron-Standardmenü mit „Reload" und „Toggle Developer Tools" (→ 2.7), Standard-Symbol (→ 2.10).

#### 2.8: Weltmaßstab, Luftseite und Zeitmodell

- **Status:** ERLEDIGT 2026-10-13 (ADR-020)
- **Phasentyp-Kontext:** ERKUNDUNG (Entwurfsschritt in Phase 2, ADR-016)
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie mit Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 1.7 (parallel zu 2.1)
- **Freigabepflichtig:** ja (Kategorie 4 Datenmodell; ggf. Änderung von ADR-013)
- **Empfohlene Klasse:** Entscheidung – Datenmodell-Entscheidung mit ADR.
- **Eingangskriterien:** Datenmodell im Kern `[BELASTBAR]` (ADR-010, ADR-014) – erfüllt
- **Anforderungen (ab Klasse M):** FR-001, FR-003, FR-010, FR-027 (vorbereitend)
- **Zu tun (zu klärende Fragen):**
  - Zellgröße in Metern, abgeleitet aus Personen-Dichte, Gate- und Stellplatzgrößen und Spannweiten (Inventar BAU-*, GATE-*, RWY-*).
  - Kartenausdehnung mit zwei Start- und Landebahnen: Bahnlänge in Zellen, Fläche der größten Karte (Inventar META-04).
  - Lage von Bahnen, Vorfeld, Rollwegen und Straßen: im selben Raster wie die Gebäude, eigene Ebene oder Graph (Rollweg-Knoten, ADR-005). Graphen für Straßen und Rollwege im Weltzustand vorsehen.
  - Wegweiser nur über bebauten Blöcken speichern? Speicher und Neuberechnung mit dem Code aus Spike 1.5 messen; ADR-013 bestätigen oder per Folge-ADR anpassen.
  - Zeitmodell: Ticks je Sekunde bei 1×, Spielzeit je Tick, Auslegung von 2× und 3× (Inventar META-08; Annahme aus 1.5: 60 Ticks/s bei 1×, 180 bei 3×).
- **Akzeptanzkriterien:** Wir wissen Zellgröße, Kartenausdehnung, Lage der Luftseite und das Zeitmodell; die Wahl ist per ADR freigegeben und mit Speicher- und Zeitwerten belegt; `architecture.md` Abschnitt 7 ist nachgezogen.
- **Betroffene Module:** Takt & Weltzustand, Welt & Bau, Wegfindung & Passagierführung, Verkehr & Verkehrssteuerung
- **Reifegrad-Wirkung:** Datenmodell im Kern bleibt `[BELASTBAR]` (bei Änderung per ADR fortgeschrieben)
- **Artefakte:** ADR(s), Messnotiz, Architektur-Update
- **Ergebnis:** ADR-020 (freigegeben 2026-10-13): 2 m je Zelle, ein Raster für Gebäude, Straßen, Vorfeld, Rollwege und Bahnen in echten Maßen; Karten 1536² und 2048²; Raster und Wegweiser in 32 × 32-Blöcken; Wegenetze für Fahrzeuge und Flugzeuge im Weltzustand; Vorfeld nur auf Vorfeld-Gehwegen begehbar; Tick = 1/3 Spielsekunde, 60/120/180 Ticks/s (Spieltag 72 min bei 1×). Messnotiz `spikes/2.8-weltmassstab/README.md`: dicht 720 MB / 2,3 s (2048²) bis 1.620 MB / 5,2 s (3072²), in Blöcken 25,5 MB / 0,4 s (Terminal 512²) bzw. 98 MB / 1,6 s (Terminal 768², 64 Wegweiser) bei jeder Weltgröße; Rechenzeit der Blöcke ersatzweise auf dem Ausschnitt gemessen, Bestätigung durch den Benchmark in 2.3. `architecture.md` Abschnitte 3, 6, 7 und 9 nachgezogen. ADR-013 überholt; ADR-020 ist nach `CLAUDE.md` Abschnitt 6 `[REAKTIV]` (Änderung binnen 14 Tagen).

#### 2.9: Schnittstellenverträge S-2 bis S-7

- **Status:** ERLEDIGT 2026-10-13 (ADR-021)
- **Phasentyp-Kontext:** ERKUNDUNG (Entwurfsschritt in Phase 2, ADR-016)
- **Schritt-Art (nur ERKUNDUNG):** sonstiges (Entwurf)
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 2.8
- **Freigabepflichtig:** ja (Kategorie 5 API-Verträge; Kategorie 1 für die neue Schnittstelle S-7)
- **Empfohlene Klasse:** Entscheidung – Schnittstellenverträge mit ADR.
- **Eingangskriterien:** Weltmaßstab und Zeitmodell aus 2.8 entschieden
- **Anforderungen (ab Klasse M):** FR-004, FR-011, FR-039, FR-044 (vorbereitend)
- **Zu tun (zu klärende Fragen):**
  - S-2 Befehlskanal: Befehlskennung, Bestätigung oder Ablehnung mit Bezug über S-3; vorläufiges Zeichnen einer Bauaktion im Fenster und Abgleich (NFR „Bauaktion im nächsten Bild", ADR-014 Auflage 1).
  - S-3 Ereignisstrom: Ereignistypen, Reihenfolge, Rückstandsmeldung.
  - S-7 Abfrage (neu): Tabellendaten mit Filter, Seiten und Sortierung im Kern; Aktualisierung wenige Male je Sekunde (Spike 1.4, `readTable`).
  - S-4 Datenpaket-Schema: Format, Versionierung, Validierung, Fehlermeldung statt Absturz (`project-context.md` Abschnitt 6).
  - S-5 Spielstand-Format: Versionierung, Prüfsumme; identischer Zustand nach Laden auch bei laufender Neuberechnung der Wegweiser (FR-004, ADR-010).
  - S-6 Plattform-API: Dateizugriff, Spielstände, Logs über die Preload-Schnittstelle.
  - TypeScript-Typen aus Rust-Typen (ts-rs).
  - Adressierung in Weltkoordinaten (x, y, Etage) bei 2 m je Zelle und Karten bis 2048² (ADR-020).
- **Akzeptanzkriterien:** Für jede Schnittstelle steht in `architecture.md` Abschnitt 4 ein Vertrag mit Eingabe, Ausgabe, Fehlern und Versionierung; per ADR freigegeben; S-2 bis S-7 sind damit `[BELASTBAR]` (Regel-007).
- **Betroffene Module:** Brücke, Plattform, Oberfläche, Inhalte & Mods, Takt & Weltzustand
- **Reifegrad-Wirkung:** S-2, S-3, S-6 `[VORLÄUFIG]` → `[BELASTBAR]`; S-4, S-5 `[OFFEN]` → `[BELASTBAR]`; S-7 neu `[BELASTBAR]`
- **Artefakte:** ADR(s), Architektur-Update Abschnitt 4
- **Ergebnis:** ADR-021 (freigegeben 2026-10-13, Option A): Verträge mit gemeinsamen Grundlagen (ein `MessagePort` mit Handshake, Sitzung in jeder Nachricht, Kern-Zustände, atomares Laden, Typen aus Rust per ts-rs, Weltkoordinaten nach ADR-020), S-2 (Befehle mit Kennung und genau einer Antwort, auch in der Pause, vorläufiges Zeichnen), S-3 (Ereignisse mit dem Bild, Flusssteuerung, Rückstandsmeldung), S-7 (Tabellen mit Filter, Sortierung und Fenster im Kern; Detailabfrage), S-4 (JSON-Pakete mit Manifest, ganzes Paket aus bei Fehlern, Grenzen und Pfadprüfung), S-5 (Binärcontainer, Prüfreihenfolge, Fehlerkatalog; Zusage für alte Spielstände ab 1.0; fehlendes Paket: verweigern, Entfernen in 6.1), S-6 (benannte Preload-Funktionen, Absenderprüfung per `senderFrame`). Text in `architecture.md` Abschnitt 4, Herleitung mit belegten Vorbildern in `docs/research/2.9-schnittstellenvertraege.md`. Getrennte Prüfung (Unteragent Sonnet 5.5): 15 Befunde, 13 eingearbeitet. S-2 bis S-7 `[BELASTBAR]`.

#### 2.2: Simulationskern – Takt, Zeitsteuerung, Befehle, Ereignisse, Determinismus

- **Status:** ERLEDIGT 2026-10-13 (ADR-022); CI-Lauf 38480239819 grün auf drei Plattformen
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.1, 2.8, 2.9
- **Freigabepflichtig:** ja, für den neuen CI-Job mit Leistungsschwelle und die Prüfung der erzeugten Typen (Kategorie 7)
- **Empfohlene Klasse:** Entscheidung – Freigabe des Benchmark-Gates; die Umsetzung selbst ist Routine und abgebbar.
- **Eingangskriterien:** S-2, S-3 `[BELASTBAR]` (2.9); Zeitmodell (2.8)
- **Anforderungen (ab Klasse M):** FR-003
- **Zu tun:** Verträge S-2 und S-3 nach ADR-021 im Kern: Befehle mit Kennung, Prüfung und genau einer Antwort, Verarbeitung auch in der Pause, Ereignisse mit `seq` und Rückstandsmeldung, Sitzungen und Kern-Zustände; ts-rs hinter Feature `ts` mit CI-Prüfung der erzeugten Typen (Kategorie 7, zusammen mit dem Benchmark-Job zur Freigabe). Takt mit festen Simulationsschritten auf eigenen Threads (rayon), Tick = 1/3 Spielsekunde mit 60/120/180 Ticks/s (ADR-020), Zeitsteuerung Pause/1×–3×/Nachtsprung/Kalender, Befehlsverarbeitung mit Kennung und Bestätigung, Ereignisstrom; Prüfmodus mit festem Übernahme-Tick; Kern-Benchmark „Tick mit 10.000 Personen" headless in der CI mit Schwelle und Trend.
- **Akzeptanzkriterien:** `core/tests/time.rs` grün; Simulation bitgleich über verschiedene Threadzahlen im Prüfmodus; Benchmark-Job läuft und schlägt bei Überschreitung der Schwelle fehl (durch erzwungenen Fehler belegt, `CLAUDE.md` Abschnitt 6); Coverage Kern ≥ 85 %.
- **Betroffene Module:** Takt & Weltzustand, Benchmark & Leistung
- **Reifegrad-Wirkung:** Modul Takt & Weltzustand → `[BELASTBAR]` durch Umsetzung
- **Artefakte:** Kern-Crate, Tests, Benchmark, CI-Job
- **Ergebnis:** Kern-Module `time`, `command`, `event`, `simulation`, `adoption`, `pacer`, `runner`, `world`, `hash` in `core/src/` (Umsetzung in `architecture.md`, Modul Takt & Weltzustand); Tests `core/tests/time.rs` (FR-003), `core/tests/commands.rs` (S-2/S-3), `core/tests/determinism.rs` (bitgleich mit 1, 2, 4 und 7 Threads im Prüfmodus, auch mit Neuberechnungen); Coverage Kern 98,5 % Lines (59 Tests), TypeScript und Skripte 85,5 % Lines (75 Tests). Typen aus ts-rs in `app/src/generated/` (16 Dateien), geprüft im Job „Rust Lint & Format". Kern-Benchmark `core/benches/tick.rs` im Job „Kern-Benchmark": auf dem GitHub-Runner Mittel 0,089 ms, p99 0,104 ms (Lauf 38480239819; Container 4 Kerne 0,12–0,13 ms); erzwungener Fehlerfall belegt (Lauf 38479926291: Grenze 0,001 ms → Job rot, alle anderen grün; zurückgenommen in `9b3d4f7`). Onboarding-Pfad (neue Prüfungen im Runbook Schritt 5) im frischen Worktree validiert. Benannte Ausnahme `syn@2.0.119` in `deny.toml` (Warnungs-Bestand, Q.1). Landeplätze der markierten Platzhalter: 2.3, 2.4, 2.7, 3.1, 3.4.
- **Nachlauf (zählt nicht zur Phase):**
  - **2.2-N1 `[ERLEDIGT]` 2026-10-13 – Pflicht-Check „Kern-Benchmark" im Ruleset `main-schutz` (Projektinhaber):** eingetragen vom Projektinhaber; von der KI über `GET /repos/Paddel87/OpenAirside/rules/branches/main` geprüft: Ruleset 25019894 mit 19 Pflicht-Checks, darunter „Kern-Benchmark" (Quelle GitHub Actions, `integration_id` 15368). Dabei festgestellt: „Require branches to be up to date" war aktiv, abweichend vom Text bei 2.1-N1; der Projektinhaber hat den Haken am 2026-10-13 entfernt, geprüft: `strict_required_status_checks_policy: false`, weiter 19 Pflicht-Checks. 2.1-N1 stimmt damit wieder.

#### 2.3: Weltzustand, Datenmodell und Wegweiser

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.2, 2.8
- **Freigabepflichtig:** nein (Datenmodell in 2.8 entschieden)
- **Empfohlene Klasse:** Routine – klar spezifizierte Umsetzung nach ADR-010 und ADR-020.
- **Eingangskriterien:** Datenmodell `[BELASTBAR]`, Weltmaßstab aus 2.8
- **Anforderungen (ab Klasse M):** FR-046, FR-017 (Kern des Wegweiser-Prinzips; Passagierführung in 3.3)
- **Zu tun:** Raster mit fünf Etagen in 32 × 32-Blöcken (2 m je Zelle, Erdgeschoss voll, übrige Etagen nur belegt; ADR-020), Wegenetze für Straßen, Rollwege und Bahnen als Datenstrukturen im Weltzustand (ohne Verkehrslogik, die folgt in 3.4), Zonen-Grundlage, heiße und kalte Felder je Objektart, Ladungsart „Fracht" im Schema; Wegweiser-Berechnung neu aufgebaut nach `spikes/1.2-wegweiser/src/field.rs` mit Neuberechnung im Hintergrund und Übernahme sobald fertig (ADR-010), Wegweiser nur in Blöcken mit begehbaren Zellen (ADR-020); synthetische Bewegung und `Guidance` aus 2.2 (`core/src/world.rs`, `TODO(fahrplan-ref: 2.3)`) durch Wegweiser ersetzen, Neuberechnung über `core/src/adoption.rs` (Prüfmodus aus 2.2); Benchmark Wegweiser „Terminal 768² × 5 in Welt 2048²" in der CI.
- **Akzeptanzkriterien:** `core/tests/cargo_ready.rs` und Wegweiser-Tests grün (Distanzen gleich der Vollberechnung nach Bauaktionen); `core/benches/flowfield.rs` mit 10.000 Personen ≤ 4 ms je Simulationsschritt (FR-017) und Neuberechnung im Bereich der Werte aus 1.5; Coverage Kern ≥ 85 %.
- **Betroffene Module:** Takt & Weltzustand, Welt & Bau, Wegfindung & Passagierführung
- **Reifegrad-Wirkung:** Modul Welt & Bau (Grundraster) → `[BELASTBAR]`
- **Artefakte:** Kern-Crates, Tests, Benchmarks

#### 2.7: Brücke und Plattformschicht

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.3, 2.9
- **Freigabepflichtig:** ja, nur für die Abhängigkeit zur Umwandlung JS ↔ Rust (Kategorie 3, ADR-021); Härtung nach `project-context.md` Abschnitt 6, Verträge aus ADR-021
- **Empfohlene Klasse:** Routine – Umsetzung festgelegter Verträge und Härtungsregeln.
- **Eingangskriterien:** S-1, S-2, S-3, S-6, S-7 `[BELASTBAR]`
- **Anforderungen (ab Klasse M):** FR-009
- **Zu tun:** Verträge nach ADR-021: Port-Handshake mit Sitzungen, Ereignisse mit dem Bild und Flusssteuerung (S-3), Abfragedienst mit Test-Tabelle (S-7), Preload-Grundgerüst S-6 mit Absenderprüfung per `senderFrame`; URL-Prüfung in `app/src/main/security.ts` per Parser statt `startsWith` (Electron-Checkliste Punkt 13). Brücken-Crate mit napi-rs (Regel-006); Dreifachpuffer und kompaktes Bildformat mit Filter auf Etage und Ausschnitt; MessagePort-Handshake; Electron-Härtung (`contextIsolation`, Sandbox, CSP, schmale Preload-API; Electron-Standardmenü ersetzen oder entfernen, „Reload" und Entwicklerwerkzeuge nur im Entwicklungs-Build – Befund 2.1-N2); Spielfeld-Grundgerüst mit PixiJS und Personen im `ParticleContainer`; Konsole/Debug-Panel mit Befehl `debug { action: DebugAction }` (S-2, Aktionen legt dieser Schritt fest); Leistungsanzeige; Takt-Thread `core/src/runner.rs` aus 2.2 an den Port anbinden; Schwelle der Rückstandsmeldung (S-3: 1 Spielsekunde = 3 Ticks) bei der Messung auf Referenz-PC und Mac prüfen – Spike 1.5 zeigte Rückstände bis 8 bzw. 33 Ticks ohne wachsenden Trend.
- **Akzeptanzkriterien:** Härtung durch Tests belegt (`nodeIntegration` aus, Sandbox an, keine entfernten Inhalte); `app/console.test.ts` grün; das Szenario aus Spike 1.5 läuft auf Produktionscode mit gleichwertigen Kernwerten (Abschlusskriterium der Phase), gemessen auf Referenz-PC und Mac des Projektinhabers.
- **Betroffene Module:** Brücke, Plattform, Spielfeld
- **Reifegrad-Wirkung:** Brücke und Plattform → `[BELASTBAR]`
- **Artefakte:** Brücken-Crate, Electron-Hauptprozess, Preload, Renderer-Gerüst, Messprotokoll

#### 2.4: Datenpaket-Schema und Lader

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.3, 2.9
- **Freigabepflichtig:** ja, für den JSON-Lader und die Schema-Erzeugung als neue Abhängigkeiten (Kategorie 3, ADR-021)
- **Empfohlene Klasse:** Routine – Umsetzung des Vertrags S-4.
- **Eingangskriterien:** S-4 `[BELASTBAR]`
- **Anforderungen (ab Klasse M):** FR-044
- **Zu tun:** Lader für Datenpakete im Kern nach S-4 (ADR-021): Manifest, zweistufige Prüfung, Grenzen, Pfadprüfung, ganzes Paket aus bei Fehlern, verständliche Fehlermeldung; JSON-Schema-Dateien für Modder; S-6 `packs.list`/`packs.setActive`; Basis-Datenpaket des Spiels (Regel-003); Inhaltsprüfung von `build`/`buildArea` (S-2) auf die geladenen Pakete umstellen (`TODO(fahrplan-ref: 2.4)` in `core/src/simulation.rs`).
- **Akzeptanzkriterien:** `core/tests/datapacks.rs` grün, inklusive fehlerhafter Pakete ohne Absturz; Coverage Kern ≥ 85 %.
- **Betroffene Module:** Inhalte & Mods, Takt & Weltzustand
- **Reifegrad-Wirkung:** Modul Inhalte & Mods (Lader) → `[BELASTBAR]`; Code-Mods bleiben `[OFFEN]` (6.1)
- **Artefakte:** Lader, Schema, Basis-Datenpaket, Tests

#### 2.5: Spielstand mit Autosave

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.3, 2.7, 2.9
- **Freigabepflichtig:** ja, für Serialisierungsformat, Prüfsumme und ggf. Kompression (Kategorie 3, ADR-021)
- **Empfohlene Klasse:** Entscheidung – Freigabe neuer Abhängigkeiten; die Umsetzung des Vertrags S-5 ist Routine und abgebbar.
- **Eingangskriterien:** S-5, S-6 `[BELASTBAR]`
- **Anforderungen (ab Klasse M):** FR-004
- **Zu tun:** Spielstand-Container nach S-5 (ADR-021) mit Grenzen, Prüfreihenfolge und Fehlerkatalog; Höchstzahlen für Objekte, Personen und Fahrzeuge als Konstanten; Speichern und Laden über S-6 (`saves.*`), Schnellspeichern, Autosave mit Rotation (1–20), Vorschaubild; fehlendes Paket: Laden verweigern mit Liste; Laden validiert und meldet Fehler verständlich.
- **Akzeptanzkriterien:** `core/tests/savegame_roundtrip.rs` grün: identische Prüfsumme nach Laden, auch während einer laufenden Neuberechnung der Wegweiser; manipulierter Spielstand führt zu einer Meldung statt zum Absturz.
- **Betroffene Module:** Takt & Weltzustand, Plattform
- **Reifegrad-Wirkung:** S-5 durch Umsetzung bestätigt
- **Artefakte:** Spielstand-Code, Tests

#### 2.6: Lokalisierung

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.7
- **Freigabepflichtig:** ja, falls eine Bibliothek für Lokalisierung eingeführt wird (Kategorie 3)
- **Empfohlene Klasse:** Routine – klar spezifizierte Umsetzung; Entscheidung nur für eine neue Abhängigkeit.
- **Eingangskriterien:** Oberfläche startet (2.7)
- **Anforderungen (ab Klasse M):** FR-006
- **Zu tun:** Deutsch und Englisch, Sprachwechsel ohne Neustart, Sprachpakete aus einem Ordner.
- **Akzeptanzkriterien:** `app/i18n.test.ts` grün; ein Sprachpaket aus dem Ordner wird erkannt.
- **Betroffene Module:** Lokalisierung, Oberfläche
- **Reifegrad-Wirkung:** Modul Lokalisierung → `[BELASTBAR]`
- **Artefakte:** Lokalisierungs-Code, Sprachdateien, Tests

#### 2.10: Stilleitfaden, SVG-Pipeline und Messung der Darstellung

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.7
- **Freigabepflichtig:** ja (Kategorie 3: SVG-Rasterer als neue Abhängigkeit)
- **Empfohlene Klasse:** Entscheidung – neue Abhängigkeit und Bewertung einer Leistungsmessung (ADR-017).
- **Eingangskriterien:** Spielfeld-Grundgerüst (2.7)
- **Anforderungen (ab Klasse M):** FR-050
- **Zu tun:** Stilleitfaden (Palette, Strichstärken, Maßstab 2 m je Zelle nach ADR-020, Detailgrad); SVG-Quellen im Repository, Rasterung in mehreren Auflösungen im Build (ADR-008), Atlanten; eigenes Programmsymbol für Fenster, Installer und Pakete (Befund 2.1-N2); Messung des Spielfelds mit Objekt-Sprites aus den Atlanten, Overlays und 10.000 Personen auf Referenz-PC und Mac.
- **Akzeptanzkriterien:** `scripts/assets.test.ts` grün; Messprotokoll zeigt, ob 60 Bilder/s mit echten Grafiken halten; Ergebnis per ADR (NFR Darstellung `[VORLÄUFIG]` → `[BELASTBAR]` oder Maßnahme).
- **Betroffene Module:** Spielfeld, Build/CI
- **Reifegrad-Wirkung:** NFR Performance Darstellung mit echten Grafiken `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen
- **Artefakte:** Stilleitfaden, Pipeline-Skript, Atlanten, Messprotokoll, ADR

### Phase 3: Vertikaler Schnitt „Kleiner Flughafen" – Typ: UMSETZUNG

**Ziel:** Ein kleiner Flughafen ist von Anfang bis Ende spielbar.

**Abschlusskriterium:** Szenario 2 der Vision („Der Einsteiger") ist spielbar.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 3.1 | Bauen im Raster (eine Etage), Bauaufträge sofort sichtbar; Wirkung von `build`, `buildArea` und `demolish` im Kern (`TODO(fahrplan-ref: 3.1)` aus 2.2) | FR-010, FR-011 |
| 3.2 | Kleines Terminal: Check-in, Sicherheit, Gate, Boarding, Toiletten; Passagierablauf | FR-012, FR-016, FR-019, FR-020, FR-022 |
| 3.3 | Passagierführung (Schilder, Markierungen) und erklärbare Wegprobleme; davor Erkundung „Suchen ohne Schilder" und Feinwege zum Einzelziel (aus 1.2) | FR-017, FR-018, FR-043 |
| 3.4 | Straßenverkehr, Bahn und Rollwege mit Knoten, Verkehrssteuerung, Vorfeld-Gehwege zu Stellplätzen ohne Fluggastbrücke (ADR-020); Nachtsprung bei Flugzeugen in Bewegung sperren (`TODO(fahrplan-ref: 3.4)` aus 2.2, META-08); davor Erkundung Blockadevermeidung (ADR-005) | FR-014, FR-027, FR-028 |
| 3.5 | Abfertigung und Gepäck in Grundform | FR-025, FR-032 |
| 3.6 | GA und kleine Linienflüge, Flugplaner (große Tabelle über S-7, Sortierung im Kern), Geld und Bewertung in Grundform | FR-030, FR-036, FR-039 |

### Phase 4: Härtung des vertikalen Schnitts – Typ: STABILISIERUNG

**Ziel:** Qualität und Leistung des vertikalen Schnitts sichern, bevor in die Breite gebaut wird.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 4.1 | Referenzszenario baubar, Leistungstest als CI-Gate; Messung auf echter 8-Kern-Hardware (Auflage ADR-014) | FR-040 |
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
| 5.6 | Personal, Wirtschaft, Forschung, Serviceziele, Notfälle, Störungen, Tutorial, Erfolge; große Tabellen für Personal und Finanzen | FR-007, FR-008, FR-033 bis FR-039 |
| 5.7 | Flugzeugtypen und Airlines inklusive Add-on-Klassen | FR-031 |
| 5.8 | Benchmark mit Empfehlung, Leistungsbewusstsein (Empfehlung und Warnung nach bebauter, begehbarer Fläche und Zahl der Ziele, ADR-020); Messung auf schwacher integrierter Grafik (Auflage ADR-014) | FR-041, FR-042 |

### Phase 6: Inhalte, Mods und Atmosphäre – Typ: UMSETZUNG

**Ziel:** Spielgefühl und Erweiterbarkeit.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 6.1 | Mod-Verwaltung; davor Erkundung zu Code-Mods (`project-context.md` Abschnitt 11); Grafiken und Töne in Mods; Spielstände mit fehlenden Paketen laden unter Entfernen der betroffenen Objekte (ADR-021) | FR-044 |
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
- **Zu tun:** Ablaufdaten-Register (`project-context.md` Abschnitt 8) abarbeiten: neue Electron-Linie, Rust-Toolchain, TypeScript 7 bei Unterstützung durch typescript-eslint, übrige Werkzeuge, abgekündigte Unterpakete von electron-builder (Register-Eintrag vom 2026-10-13), benannte Ausnahme `syn@2.0.119` in `deny.toml` (ts-rs auf syn 3 umgestellt?, Warnungs-Bestand); Tabelle zur Bestätigung vorlegen; ADR-002 per Folge-ADR fortschreiben.
- **Akzeptanzkriterien:** Keine fixierte Version ist über ihrem Lebensende; Register und `Verifiziert`-Vermerke aktuell; CI grün auf drei Plattformen.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Versions-Tabelle, Folge-ADR, aktualisierte Register
- **Notizen:** Nach dem ersten Lauf wird ein neuer Q.1-Termin mit der nächsten Frist angelegt.

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

---

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- Schritte **ohne Abhängigkeiten zueinander**: 2.1 parallel zu 2.8; 2.4 parallel zu 2.5 und 2.6 (nach 2.7); Q.1 und Q.2 unabhängig von allen Phasen
- Schritte **mit gemeinsamen Modulen** (Konfliktgefahr): 2.2 und 2.3 (Takt & Weltzustand); 2.5 und 2.7 (Plattform)

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-10-13 – Schritt 2.8 abgeschlossen (ADR-020): 2.2, 2.3, 2.9, 2.10, 3.4, 5.8 und das Abschlusskriterium von Phase 2 um Weltmaßstab, Blöcke, Wegenetze, Vorfeld-Gehwege und Zeitmodell ergänzt; keine neuen Schritte
- 2026-10-13 – Schritt 1.7 abgeschlossen (ADR-016, ADR-017): Phase 2 im vollen Format mit 10 Schritten (neu 2.8, 2.9, 2.10; 2.6 geteilt); FR-039 in 3.6 und 5.6; neuer Schritt 7.3; Phase 1 archiviert
- 2026-10-13 – Schritt 1.2 abgeschlossen (ADR-010); 1.5 um Rastergröße und fünf Etagen ergänzt; 3.3 um vorgeschaltete Erkundung „Suchen ohne Schilder"/Feinwege ergänzt (Landeplatz aus 1.2)
- 2026-10-13 – Schritt 1.1 abgeschlossen (ADR-009); Querschnitt Q.2 als Landeplatz für die zurückgestellte WebAssembly-Variante angelegt (zählt nicht zum Schrittplan von Phase 1); 1.3 um Übertragungsumfang ergänzt
- 2026-10-13 – Erstplanung in der Initialisierung (ADR-001 bis ADR-008); Nachzug nach aktuellem Regelwerk: Schritt-IDs für Phasen 2–7 mit Anforderungs-Zuordnung, Querschnitt Q.1

<!-- ANCHOR:archiv-abgeschlossene-phasen -->
## Archiv / abgeschlossene Phasen

- Phase 1: [`docs/archiv/fahrplan-phase-1.md`](archiv/fahrplan-phase-1.md)
