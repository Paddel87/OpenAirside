# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Laufende Phase im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-19 (Session 16:18–17:10 UTC)
- **Laufende Phase:** Phase 2 – Fundament
- **Phasentyp:** UMSETZUNG (mit Entwurfsschritten 2.8 und 2.9, Phasentyp-Kontext ERKUNDUNG)
- **Aktiver Schritt:** keiner; Q.3 ERLEDIGT am 2026-10-19 (ADR-035, PR #26, `d4e457f`); 2.10 ERLEDIGT am 2026-10-19 (ADR-033, ADR-034), mit PR #25 nach `main` gemergt (`302e3ce`) – damit sind alle 11 Schritte von Phase 2 erledigt
- **Nächster Schritt:** Phasenabschluss Phase 2 in einer neuen Session auf einem neuen Branch vom aktuellen `main` (Merge-Vermerke von `ci/q-3-rustup-toolchain-install` übernehmen): Vision-Abgleich gegen `docs/vision.md`, Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“ mit Bewertung durch eine getrennte Instanz (`CLAUDE.md` Abschnitt 12), Archivierung und Logbuch-Verdichtung beim `[PHASEN-WECHSEL]`
- **Offene STOPP-Situationen:** Reaktiv-Quote 7 / 10 nach ADR-032 über dem Schwellenwert – Reflexion zu ADR-032 siehe „Iterations-Reflexion“, zur Kenntnis genommen am 2026-10-19, kein Umbau

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

**Reihenfolge:** 2.1 und 2.8 parallel → 2.9 → 2.2 → 2.3 → 2.7 → 2.4 → 2.5 → 2.6 → 2.11 → 2.10. 2.11 kam am 2026-10-19 hinzu (Maßnahme (10), Reflexion zu ADR-031); Phase 2 damit 11 Schritte bei 10 geplanten, unter der Wucherungs-Schwelle. Die Nummern 2.1–2.7 stammen aus der Grobplanung (Verweise in `requirements.md` und `project-context.md`), 2.8–2.10 kamen in 1.7 hinzu.

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

- **Status:** ERLEDIGT 2026-10-14 (ADR-023); CI-Lauf auf `01f9779` grün auf drei Plattformen
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
- **Ergebnis (Stand 2026-10-13, Routine-Klasse):** Module `grid`, `network`, `target`, `flowfield`, `scenario` und der neue `world` in `core/src/` (Umsetzung in `architecture.md`, Module Welt & Bau, Wegfindung, Takt). Tests: Distanzen der Wegweiser gleich einer dichten Referenzberechnung (Zufallslayouts über Blockgrenzen und drei Etagen, vor und nach Bauaktionen), bitgleich für 1 bis 8 Threads; `core/tests/cargo_ready.rs`; `core/tests/determinism.rs` mit Neuberechnung nach Bauaktionen (3 Threadzahlen, Prüfmodus). Coverage Kern 99,0 % Lines (82 Tests), TypeScript und Skripte 87,0 %. `core/benches/flowfield.rs`: 64 Ziele, 1,5 Mio. Zellen, 101 MB, Neuberechnung 1,65 s (1,6–1,8 s) auf 4 Kernen; Schritt mit 10.000 Personen im Echtzeit-Takt während Neuberechnungen: Mittel 0,3 ms, p99 0,9 ms (≤ 4 ms); `tick.rs` unverändert grün (Mittel 0,13 ms). Mit mehr Threads als Kernen (4 + 3 auf 4 Kernen) schwankte p99 zwischen 2,8 und 4,6 ms – Befund zur Threadaufteilung (je Kern ein Thread, `background_threads` getrennt von `threads`). Block-Speicherung aus ADR-020 bestätigt (101 MB statt 98 MB geschätzt, 1,65 s statt 1,6 s). Gates lokal grün: rustfmt, clippy `-D warnings`, cargo-deny, ESLint, Prettier, tsc, Vitest. Landeplätze der Platzhalter: 3.2 (Personenverhalten und kalte Felder, `TODO` in `world.rs`), 3.4 (Verkehrsregeln der Wegenetze), 4.1 (Referenzszenario aus Daten).
- **Abschluss auf der Entscheidungs-Klasse (2026-10-13 23:50 bis 2026-10-14 00:05 UTC):** Freigabe Option A (ADR-023), Schritt „Wegweiser im Referenz-Terminal“ im Job „Kern-Benchmark“; erzwungener Fehlerfall belegt (Lauf 38486734288: Grenze 1 ms → Schritt rot, Zusammenfassung und Artefakte trotzdem erzeugt, zurückgenommen in `01f9779`). Erster Runner-Wert 3,59 s lag zu nah an der Grenze von 4 s; Optimierung der Nachbarzugriffe (`af84dfb`, Ergebnis unverändert) → Runner 2,48 s, lokal 1,13–1,23 s; Schritt p99 auf dem Runner 0,38 ms. Modul Welt & Bau (Grundraster) `[BELASTBAR]`.
- **Aufgehobener STOPP – MODELLWECHSEL ERFORDERLICH (2026-10-13 23:50 UTC):** Aktuelle Klasse Routine; benötigt Entscheidung. Auslöser 1 (`ENTSCHEIDUNG ERFORDERLICH`, siehe Logbuch `[ENTSCHEIDUNG-VORGELEGT]` 23:50): Kategorie 7 für den Wegweiser-Benchmark als Schritt im CI-Job „Kern-Benchmark“ mit Grenzen; Auslöser 4: Beförderung Welt & Bau `[VORLÄUFIG]` → `[BELASTBAR]`. Offen danach: CI-Schritt einbauen und durch erzwungenen Fehler belegen, ADR, Beförderung, Schritt abschließen.

#### 2.7: Brücke und Plattformschicht

- **Status:** ERLEDIGT 2026-10-15 (ADR-024: napi `serde-json` mit `serde_json =1.0.151`); CI-Lauf 38526061047 auf `bbdd5c8` grün auf drei Plattformen; Messung auf Mac und Referenz-PC am 2026-10-15
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
- **Zwischenstand (2026-10-14, Entscheidungs-Klasse):** umgesetzt und mit Tests belegt, CI-Lauf 38516083109 grün auf drei Plattformen (Stand `4edcdab`); Lauf für `86bb713` (S-7) stand bei Sessionende noch aus.
  - Kern (`core/src/`): `frame` (Bildformat S-1, Layout-Version 1: 32 Byte Kopf, 6 Byte je Person, Filter auf Etage und Ausschnitt mit 4 Zellen Rand), Dreifachpuffer in `runner` (Bild und Ereignisse unter einer Sperre, Ereignisse nach Sitzung gruppiert), `port` (Nachrichten, Handshake, Sitzungsprüfung, Zähler), `host` (Port-Zustand mit Takt-Thread, Ereignisse ohne Bild spätestens alle 250 ms), `query` (S-7: Test-Tabelle `debugPersons` mit Filter, Sortierung, Fenster, Wörterbuch, Detail), Befehl `debug` mit `spawnPersons`, `clearPersons`, `setWalls`, `loadScenario` (FR-009), Antwort `invalidArgument` für unlesbare Befehle bekannter Art, `default_threads` (Tick ein Viertel der Kerne, 1–4; Hintergrund der Rest minus ein Kern).
  - Brücke: Klasse `Core` mit napi `serde-json` (ADR-024); Nachrichten über 1 MB werden verworfen und gezählt.
  - Plattform: frischer Port je geladener Seite, neue Sitzung nach Neuladen, Neustart des Kern-Prozesses höchstens 3-mal in 60 s, Absenderprüfung per `senderFrame` und URL-Parser, `app.info`/`app.quit`, Preload `window.openairside` (apiVersion 1), Navigation per URL-Parser, Release-Builds ohne Menüpunkte „Reload" und Entwicklerwerkzeuge.
  - Renderer: PixiJS 8.20.1 mit Personen im `ParticleContainer` (CSP ohne `unsafe-eval`), Kamera, Etagenwechsel, Konsole (`speed`, `night`, `spawn`, `clear`, `walls`, `scenario`, `table`), Leistungsanzeige; Start mit `--scenario=referenceTerminal --persons=10000`.
  - Tests: Kern 96 Unit-Tests grün; TypeScript 120 Tests, Coverage gesamt 94 % Lines (Renderer-Module 99 %, Hauptprozess-Module 100 %; ausgenommen Einstiegspunkte und WebGL-Zeichnen); Rauchtest mit Handshake, Bildern, Befehlen, S-6, S-7 und neuer Sitzung nach Neuladen.
  - Messung im Container (4 vCPU, ohne Grafikkarte, nicht aussagekräftig für Bildraten): Bild kodieren 0,1 ms, Tabellenabfrage 10.000 Zeilen mit Fenster 1.000 im Kern 1,7–1,9 ms; mit SwiftShader WebGL2 läuft das Spielfeld unter der strengen CSP.
- **Messwerkzeug (2026-10-14, Entscheidungs-Klasse):** umgesetzt und im Container Ende-zu-Ende gelaufen.
  - Kern: `TickStats` im Status des Takts (`core/src/runner.rs`: Ticks/s, Schrittzeit Mittel/p99/Maximum je Sekunde, Rückstand, Neuberechnung mit Dauer von Anforderung bis Übernahme aus `core/src/simulation.rs`, Threads); Port-Nachricht `coreStats` etwa je Sekunde (`core/src/host.rs`, additiv). Kern 100 Unit-Tests.
  - Renderer: Konsolenbefehl `measure [Minuten 1–60]` (Standard 10) mit der Messreihe in `app/renderer/measure.ts` (Ablauf siehe `architecture.md`, Modul Benchmark & Leistung); Leistungsanzeige mit Ticks/s, Schrittzeit und Dauer der Neuberechnung.
  - Plattform: S-6 `measure.start`/`measure.report` (additiv): Speicher aller Prozesse alle 5 s, Datei `openairside-measure-<Plattform>-<Zeit>.json` im Download-Ordner (`app/src/main/measure.ts`).
  - Tests: TypeScript 178 Tests, Coverage 96 % Lines (`measure.ts` im Renderer 98 %, im Hauptprozess 100 %); Rauchtest prüft `coreStats` in der Leistungsanzeige und die Ablehnung eines Berichts ohne gestartete Messung.
  - Probelauf im Container (4 vCPU, SwiftShader, 1 Minute): 2 Fenster, 178–180 Ticks/s, je Fenster 3 Neuberechnungen mit 2,9–3,3 s, Bauaktion bis zum Bild 143–252 ms, Tabellenabfrage Median 51–62 ms, Speicher aller Prozesse 676 → 765 MB. Bildraten 12–16 und Schrittzeiten p99 bis 16 ms sind durch die Software-Grafik auf denselben 4 Kernen verfälscht, ohne Aussage.
  - Abweichung von Spike 1.5: Die Kamera kreist über dem Erdgeschoss statt über der Etage darüber, weil nach 10 s Aufwärmen bei 3× noch alle Personen im Erdgeschoss stehen (Spike 1.5 spulte ungebremst vor; das gibt es im Produktionskern nicht). Höchste Tickrate ohne Bremse wird nicht gemessen; maßgeblich ist die Schrittzeit.
- **Abschluss (2026-10-15, Entscheidungs-Klasse):** Akzeptanzkriterien erfüllt – Härtung durch Tests belegt, `app/renderer/console.test.ts` grün, Szenario aus Spike 1.5 auf Produktionscode gleichwertig oder besser auf Mac und Referenz-PC (Auswertung in [`docs/research/2.7-messung.md`](research/2.7-messung.md)). Störung in Fenster 9 bis 12 der PC-Messung geklärt: Bildschirm war aus (Angabe Projektinhaber), das Fenster zeichnete deshalb nicht; die Werte dieser Fenster gehen nicht in die Bewertung ein. Schwelle der Rückstandsmeldung (3 Ticks) auf beiden Rechnern bestätigt; Standardaufteilung der Threads (Mac 2 + 8, PC 4 + 19) beibehalten; zweiter PC-Lauf mit 2 + 4 Threads (nachgereicht 2026-10-15): Simulation ruhiger (Einzelschritt höchstens 0,94 ms), Neuberechnung bis 1,59 s, Zielwerte erfüllt. Brücke und Plattform `[BELASTBAR]`, FR-009 erledigt. Offener Befund mit Landeplatz 2.10: zwei Aussetzer bis 110 ms auf dem Mac ohne Einwirkung von außen (laut Projektinhaber am Mac nichts bedient).
- **Stand vor dem Abschluss (2026-10-15):**
  - **Referenz-PC gemessen am 2026-10-15** (Datei `docs/research/2.7-ergebnisse/referenz-pc-windows.json`, Auswertung in [`docs/research/2.7-messung.md`](research/2.7-messung.md)): in 16 ungestörten Fenstern 60 Bilder/s, 180 Ticks/s ohne Rückstand, Schritt Mittel 0,13–0,14 ms, Neuberechnung höchstens 366 ms, Bauaktion 1–2 Bilder, Kern-Prozess 123–155 MB ohne Anstieg. Offen: Ursache der Störung in Fenster 9 bis 12 (kein Bild in Fenster 10 und 11, Kern-Schritte bis 75 ms) – Rückfrage an den Projektinhaber; zweiter Lauf mit festen Threads liegt nicht vor (optional).
  - **Mac M3 Pro gemessen am 2026-10-15** (Datei `docs/research/2.7-ergebnisse/mac-m3pro.json`): 120 Bilder/s, 180 Ticks/s im Mittel jedes Fensters, Schritt Mittel 0,26–0,56 ms, Neuberechnung höchstens 422 ms, Bauaktion 1–3 Bilder, Speicher ab 200 s ohne Anstieg (933–989 MB) – gleichwertig oder besser als Spike 1.5; Schwelle der Rückstandsmeldung (3 Ticks) bestätigt. Offen: zwei Aussetzer bis 110 ms (Fenster 8 und 9, Rückstand bis 19 Ticks, aufgeholt), Ursache nicht belegt.
  - `app/console.test.ts` liegt als `app/renderer/console.test.ts` neben dem Modul (Ablage wie alle Renderer-Tests).
- **Landeplätze der übrigen S-6-Funktionen:** `saves.*` und `logs.*` in 2.5, `packs.*` in 2.4, `settings.*` in 2.6. Darstellung des Rasters im Spielfeld (`TODO(fahrplan-ref: 3.1)` in `app/renderer/playfield.ts`) in 3.1.

#### 2.4: Datenpaket-Schema und Lader

- **Status:** ERLEDIGT 2026-10-18 (ADR-025, ADR-026); CI-Lauf 39176235417 auf `89fa783` grün auf drei Plattformen
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.3, 2.9
- **Freigabepflichtig:** ja, für den JSON-Lader und die Schema-Erzeugung als neue Abhängigkeiten (Kategorie 3, ADR-021)
- **Empfohlene Klasse:** Routine – Umsetzung des Vertrags S-4.
- **Eingangskriterien:** S-4 `[BELASTBAR]`
- **Anforderungen (ab Klasse M):** FR-044
- **Zu tun:** Lader für Datenpakete im Kern nach S-4 (ADR-021): Manifest, zweistufige Prüfung, Grenzen, Pfadprüfung, ganzes Paket aus bei Fehlern, verständliche Fehlermeldung; JSON-Schema-Dateien für Modder; S-6 `packs.list`/`packs.setActive`; Basis-Datenpaket des Spiels (Regel-003); Inhaltsprüfung von `build`/`buildArea` (S-2) auf die geladenen Pakete umstellen (`TODO(fahrplan-ref: 2.4)` in `core/src/simulation.rs`).
- **Akzeptanzkriterien:** `core/tests/datapacks.rs` grün, inklusive fehlerhafter Pakete ohne Absturz; Coverage Kern ≥ 85 %.
- **Betroffene Module:** Inhalte & Mods, Takt & Weltzustand; für S-6 `packs.*` (im „Zu tun" vorgesehen) zusätzlich Brücke, Plattform und Spielfeld (Konsole) – bei der Planung nicht aufgeführt, nachgetragen am 2026-10-18
- **Reifegrad-Wirkung:** Modul Inhalte & Mods (Lader) → `[BELASTBAR]`; Code-Mods bleiben `[OFFEN]` (6.1)
- **Artefakte:** Lader, Schema, Basis-Datenpaket, Tests
- **Ergebnis (2026-10-18, Entscheidungs-Klasse):** Lader `core/src/datapack/` (Vorprüfung der Grenzen, Aufzählen und Lesen ohne Links, zwei Prüfstufen mit Datei, Zeile, Spalte und JSON-Pfad, Abhängigkeiten mit Reihenfolge und Zyklen, `replaces`, ganzes Paket aus bei Fehlern); Basis-Datenpaket `content/base/` mit `base:floor_concrete`, `base:wall_concrete`, `base:bench`; Schema-Dateien `content/schema/` aus `npm run gen:schema`, CI-Schritt „Erzeugte Schemas (content/schema) aktuell" mit erzwungenem Fehlerfall (Lauf 39173571403, nur dieser Schritt rot; zurückgenommen in `f59fe24`); `build`/`buildArea` prüfen die ID gegen den Katalog der Sitzung (`TODO(fahrplan-ref: 2.4)` aufgelöst, Wirkung 3.1); neue Paketauswahl ab der nächsten Sitzung; S-6 `packs.list`/`packs.setActive` mit Speicherung in `packs-active.json`, `busy`, Anfrage-Nummern; Konsolenbefehle `packs`, `packs set`, `build`; Basispaket im Programmpaket. Tests: `core/tests/datapacks.rs` 26 Fälle (fehlerhafte Pakete ohne Absturz, Junctions unter Windows, Links und harte Links unter Unix), Kern 120 Unit-Tests; Coverage Kern 98,74 % Lines (`core/src/datapack/` 90,5–100 % je Datei), TypeScript 252 Tests mit 96,0 % Lines; Rauchtest lädt ein Mod-Paket aus einem frischen Datenordner und baut `kiosks:kiosk` (FR-044 für Objekte). Getrennte Prüfung (Sonnet 5.5): 14 Befunde, keiner hoch; Erledigung in [`docs/research/2.4-getrennte-pruefung.md`](research/2.4-getrennte-pruefung.md), Landeplätze 2.5 und 6.1, Befund 8 per ADR-026 entschieden. Präzisierungen von S-4 in `architecture.md` Abschnitt 4. Modul Inhalte & Mods (Lader) `[BELASTBAR]`. Abweichung: Zwischenstand `ac90598` war in der CI rot (erzeugte Typen einen Commit zu spät, Logbuch 22:37 UTC).
- **Vorlage zur Freigabe (2026-10-18, Entscheidungs-Klasse; freigegeben: A, ADR-025):**
  - Kategorie 3, Versionen belegt auf crates.io am 2026-10-18 (Regel-001): A `serde_json =1.0.151` auch im Kern (bisher nur Brücke, ADR-024), `serde_path_to_error =0.1.20` (JSON-Pfad in Fehlern), `semver =1.0.28` (Versionsbereiche; schon im Lockfile über napi-derive-backend), `schemars =1.2.2` nur hinter Kern-Feature `schema` (Schema-Dateien für Modder; nicht 1.2.3, 6 Tage alt); B wie A ohne schemars, Schema-Dateien erst in 6.1 (ändert ADR-021); C wie A, JSON-Pfad und Versionsbereiche selbst programmiert. Empfehlung A.
  - Kategorie 7: CI-Schritt „Erzeugte Schemas aktuell" im Job „Rust Lint & Format" (wie `check:types`, durch erzwungenen Fehler zu belegen); Basis-Datenpaket im Programmpaket (electron-builder `files`).
  - Probeaufbau in einer Kopie des Workspace: neu im Lockfile `schemars`, `schemars_derive`, `serde_derive_internals`, `dyn-clone`, `ref-cast`, `ref-cast-impl` (nur mit Feature `schema`) und `serde_path_to_error`; alle MIT bzw. MIT OR Apache-2.0, syn 3 wie serde; `cargo deny check` grün.
  - Umfang zur Kenntnis: Basispaket in 2.4 nur mit Objekttypen (ID, Textschlüssel, Bauart Fläche oder Einzelobjekt, Grundfläche in Metern); Schema der übrigen Arten mit dem Schritt, der sie zuerst nutzt (Fahrzeuge 3.4, Flugzeugtypen und Airlines 3.6, Forschung 5.6, Sprachen 2.6); bis dahin weist der Lader solche Ordner mit Meldung ab. Wirkung des Bauens bleibt 3.1.
  - Klassifikation: ADR-025 hebt die Einschränkung „nur in der Brücken-Crate" aus ADR-024 (4 Tage alt) auf und ist damit `[REAKTIV]` (`CLAUDE.md` Abschnitt 6); Reaktiv-Quote bleibt 4 / 10, Reflexion nach der Freigabe.

#### 2.5: Spielstand mit Autosave

- **Status:** ERLEDIGT 2026-10-19 (ADR-027, ADR-028, ADR-029, ADR-030); CI-Lauf 39253155836 auf `a4fff12` grün auf drei Plattformen (17 Jobs)
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.3, 2.7, 2.9
- **Freigabepflichtig:** ja, für Serialisierungsformat, Prüfsumme und ggf. Kompression (Kategorie 3, ADR-021)
- **Empfohlene Klasse:** Entscheidung – Freigabe neuer Abhängigkeiten; die Umsetzung des Vertrags S-5 ist Routine und abgebbar.
- **Eingangskriterien:** S-5, S-6 `[BELASTBAR]`
- **Anforderungen (ab Klasse M):** FR-004
- **Zu tun:** Fehlerform aller S-6-Aufrufe nach Vertrag (`{ code, textKey, params }`; Electron überträgt bei abgewiesenen Aufrufen nur den Meldungstext), bestehende Aufrufe `measure.*` und `packs.*` nachziehen (Befund 9 der getrennten Prüfung 2.4); Spielstand-Container nach S-5 (ADR-021) mit Grenzen, Prüfreihenfolge und Fehlerkatalog; Höchstzahlen für Objekte, Personen und Fahrzeuge als Konstanten; Speichern und Laden über S-6 (`saves.*`); `logs.write` und `logs.openFolder` mit Rotation (S-6, aus 2.7); Schnellspeichern, Autosave mit Rotation (1–20), Vorschaubild; fehlendes Paket: Laden verweigern mit Liste; Laden validiert und meldet Fehler verständlich.
- **Akzeptanzkriterien:** `core/tests/savegame_roundtrip.rs` grün: identische Prüfsumme nach Laden, auch während einer laufenden Neuberechnung der Wegweiser; manipulierter Spielstand führt zu einer Meldung statt zum Absturz.
- **Betroffene Module:** Takt & Weltzustand, Plattform; in 2.5a zusätzlich Welt & Bau (Hilfen in `grid`/`network`), Wegfindung & Passagierführung (`FieldSet` behält ihre Eingabe) und Inhalte & Mods (aktive Pakete im Katalog) – nur interne Hilfen ohne neue Verantwortung, nachgetragen am 2026-10-19, Abweichung nach Stopp-Kriterium 5 vom Projektinhaber zur Kenntnis genommen; 2.5b zusätzlich Brücke, Oberfläche und Spielfeld (Konsole, Liste)
- **Reifegrad-Wirkung:** S-5 durch Umsetzung bestätigt
- **Artefakte:** Spielstand-Code, Tests
- **Zwischenstand Stufe 2.5a (2026-10-19, Entscheidungs-Klasse; Routine-Arbeit oberhalb der Empfehlung):** Kern umgesetzt in `07188d5`: `core/src/savegame/` (Container, Grenzen, Prüfreihenfolge, Fehlerkatalog, `read_info` für die Liste), `Simulation::save_snapshot` und `Simulation::load_game` (Pakete gegen den Inhalt der nächsten Sitzung, aktive Wegweiser aus ihrer gespeicherten Eingabe, laufende Neuberechnung mit Revision und Übernahme-Tick), `FieldSet` behält ihre Eingabe, Katalog mit aktiven Paketen (`ActivePack`), Befehlsprüfung gegen die Karte der Welt statt der Einstellung (geladene Spielstände dürfen eine andere Kartengröße haben). Erzeugte Typen `SaveKind`, `SaveMeta`, `SavedPack`. Tests: `core/tests/savegame_roundtrip.rs` 10 Fälle (Zustands-Prüfsumme gleich nach Laden, auch während einer Neuberechnung im Prüfmodus mit bitgleicher Fortsetzung und im normalen Spiel; fehlgeschlagenes Laden lässt die Sitzung unverändert; `packMissing`, `packVersionMismatch`; Liste ohne Weltzustand; beschädigte und manipulierte Dateien in der Reihenfolge des Vertrags, 300 zufällige Byte-Änderungen ohne Absturz; Kartengrößen 1536/2048; Prüfung der Anfrage), 4 Unit-Tests der Invarianten in `core/src/savegame/state.rs`; Kern 128 Unit-Tests. Coverage Kern 98,49 % Lines (`savegame/container.rs` 93,8 %, `state.rs` 97,5 %, `mod.rs` 97,5 %). Gates lokal grün: rustfmt, clippy `-D warnings`, cargo-deny, `check:types`, `check:schema`, tsc, Pre-Commit-Hooks. Offen: 2.5b (Plattform).
- **Abschluss (2026-10-19, Entscheidungs-Klasse):** Akzeptanzkriterien erfüllt – `core/tests/savegame_roundtrip.rs` grün (gleiche Zustands-Prüfsumme nach Laden, auch während einer Neuberechnung; manipulierte Spielstände führen zu einer Meldung statt zum Absturz). Getrennte Prüfung (Sonnet 5.5) mit 12 Befunden, keiner hoch; Erledigung in [`docs/research/2.5-getrennte-pruefung.md`](research/2.5-getrennte-pruefung.md) (`a6763e9`), Befund 5 per ADR-030 entschieden (Grenzen 512 MB, `ad16a93`). Nach den Korrekturen: TypeScript 300 Tests, Coverage 90,5 % Lines; Rauchtest zusätzlich mit Link auf `/dev/zero` und FIFO im Spielstand-Ordner (beide „beschädigt“, Laden abgewiesen). S-5 durch Umsetzung bestätigt, FR-004 erledigt. Landeplätze: Autosave-Intervall und -Zahl einstellbar 2.6; Flughafenname 5.1; Spielmenü 5.1; Nachmessung der Grenzen 4.1. Nicht Ende-zu-Ende belegt bleibt der 10-Minuten-Takt des Autosaves (Unit-Test).
- **Zwischenstand Stufe 2.5b (2026-10-19, Entscheidungs-Klasse; Routine-Arbeit oberhalb der Empfehlung):** umgesetzt in `2997f0d` (Kern: `SaveLink` im Takt-Thread, `Simulation::reject_waiting`, Prüfung des Vorschaubilds öffentlich), `6c971c8` (Brücke: `saveGame`/`loadGame` als `AsyncTask`, `saveInfo`, `checkPreview`, `running` im Status), `a8e43b9` (Plattform: Fehlerform aller S-6-Aufrufe über die Antwort-Hülle, auch `measure.*` und `packs.*`; `saves.*` mit `saves.preview`; `logs.*`; Autosave; `onCoreExit` mit `restarting`, ADR-029), `cb6e554` (Konsole `saves`/`save`/`quicksave`/`load`/`delete`, F5, einfache Liste mit `blob:`-Vorschau, Hinweis nach Absturz, Rauchtest). Fund beim Testen und behoben: Die Antwort auf das Laden ging vor dem Veröffentlichen des neuen Status hinaus (Test `a_failed_load_keeps_the_session` rot in 3 von 15 Läufen, danach 25 von 25 grün). Tests: Kern 128 Unit-Tests (neu: 3 im Takt, 1 in `simulation`), `core/tests/savegame_roundtrip.rs` 10 Fälle; Coverage Kern 98,42 % Lines (`runner.rs` 97,8 %); TypeScript 293 Tests, Coverage 90,2 % Lines (`app/src/main/saves.ts` 100 %, `logs.ts` 100 %, `platform-error.ts` 100 %, `app/renderer/saves.ts` 100 %; `app/renderer/saves-panel.ts` nur über den Rauchtest, DOM ohne Unit-Test-Umgebung). Rauchtest (dreimal grün im Container): Speichern, F5, Autosave-Rotation über `kind: "auto"`, Liste mit Vorschau 480 px als `blob:`-URL, Laden in neue Sitzung, Löschen, Fehlercodes `unknownSave`/`invalidArgument`, Logdatei mit maskierten Zeilen; **erzwungener Absturz** (Kern-Prozess per `SIGKILL`): Angebot erscheint, „Load last autosave“ lädt `auto-1`; nach drei Neustarts in 60 s Hinweis auf die Logdatei, Logzeile „no further restart“. Nicht Ende-zu-Ende belegt: der 10-Minuten-Takt des Autosaves (Unit-Test `AutosaveClock`, im Rauchtest zu lang). Gates lokal grün: rustfmt, clippy `-D warnings`, cargo-deny, `check:types`, `check:schema`, ESLint, Prettier, tsc. Offen: getrennte Prüfung (Kategorie 6), CI-Lauf.
- **Abgleich vor Stufe 2.5b (2026-10-19 11:06 UTC, Maßnahmen 7 und 8; Entscheidungs-Klasse):**
  - **Freigabe (Projektinhaber 2026-10-19 per Auswahl):** Abweichung A (ADR-029), Festlegungen (a)–(l) ja.
  - **Abweichung, vorgelegt:** S-6 „Absturz des Kerns“ verlangt das Angebot „letzten Autosave laden“ nach einem Neustart und nach dem dritten Absturz in 60 s stattdessen eine Fehlermeldung mit Verweis auf die Logdatei. `onCoreExit(callback)` liefert nur den Code; die Oberfläche kann beide Fälle nicht unterscheiden (Maßnahme 8: S-6 setzt eine Information voraus, die S-6 selbst nicht liefert). Optionen: A zweites Argument `restarting` (additiv, eigener ADR, nach der 14-Tage-Regel `[REAKTIV]`); B die Oberfläche zählt Abstürze selbst nach derselben Regel (`restartDecision`), ohne Vertragsänderung, ungenau nach Neuladen des Fensters.
  - **Ohne Abweichung, zur Kenntnis:** (a) CSP enthält `img-src 'self' data: blob:` schon seit 2.7; 2.5b ändert sie nicht und belegt sie mit einem Test. (b) Autosave nach 10 Minuten Echtzeit bei laufendem Spiel (nicht pausiert), gezählt im Kern-Prozess, neu ab jeder Sitzung; fällig während einer anderen Operation → nach ihr. (c) Dateien `<ID>.oasave` im Ordner `saves` des Datenordners: Schnellspeichern ein Platz `quick` (Taste F5 und Konsole), Autosave Plätze `auto-1` … `auto-5` (überschreibt den ältesten), manuell `m-<Zufall>`; IDs vergibt der Hauptprozess. (d) Eintrag der Liste: ID mit Metadaten oder Zustand „beschädigt“ mit Fehler. (e) Unbekannte oder ungültige ID → `invalidArgument` (kein neuer Code) – korrigiert bei der Anlage von ADR-029: ADR-028 legt für gut geformte, unbekannte IDs schon `unknownSave` fest; es gilt ADR-028. (f) `busy` gilt für alle `saves.*` und `packs.setActive` gemeinsam, wörtlich nach S-6. (g) Fehler des Hauptprozesses mit Textschlüssel `platform.error.<code>`, Fehler des Kerns unverändert (`save.error.<code>`); `measure.*`: `alreadyRunning`, `notStarted`, `notAnObject`, `tooLarge`; fremder Absender `untrustedSender`. (h) Vorschaubild per `capturePage`, 480 Pixel breit, PNG; geprüft vom Kern wie beim Laden. (i) Laden liest höchstens 1 GB (Größe vor dem Lesen geprüft). (j) Befehle während des Ladens werden mit `notReady` abgelehnt (der Takt arbeitet die während des Ladens eingegangenen Befehle danach im Zustand `loading` ab; bei Erfolg gehören sie zur alten Sitzung und werden verworfen). (k) Logdatei `logs/openairside.log` im Datenordner, Rotation 5 × 10 MB, Stufen `debug`/`info`/`warn`/`error` (Releases ab `info`); Warnungen des Hauptprozesses gehen ebenfalls hinein, ohne Pfade. (l) Kodieren und Prüfen des Spielstands in der Brücke als napi `AsyncTask` außerhalb des JavaScript-Threads (in napi 3.12.7 enthalten, keine neue Abhängigkeit).
  - **„Betroffene Module“ gegen den geplanten Code:** Takt & Weltzustand (`core/src/runner.rs`: Speichern und Laden im Takt-Thread; `core/src/savegame/`: Prüfung des Vorschaubilds öffentlich), Brücke (`bridge/src/lib.rs`), Plattform (`app/src/main/`: Hauptprozess, Preload, Kern-Prozess, neue Dateien für Spielstände und Logs), Spielfeld (Konsole in `app/renderer/console.ts`, `main.ts`), Oberfläche (einfache Liste, Angebot nach Absturz). Keine weiteren Module; `app/renderer/measure.ts` (Benchmark & Leistung) bleibt unberührt, die Fehleranzeige liegt in `main.ts`.
- **Vorlage zur Freigabe (2026-10-19, Entscheidungs-Klasse; freigegeben: A, ADR-027; Abgleich (1)–(9) und Aufteilung ja, ADR-028):**
  - Kategorie 3, Versionen belegt auf crates.io am 2026-10-19 (Regel-001), Probeaufbau in einer Kopie des Workspace mit `cargo deny check` grün: A `postcard =1.1.3` (ohne Standard-Features, `use-std`) für den Weltzustand, `miniz_oxide =0.9.1` (Deflate, `forbid(unsafe_code)`) für den Weltzustand-Abschnitt, Datei-Prüfsumme FNV-1a 64 aus `core/src/hash.rs` (keine Abhängigkeit); neu im Lockfile `postcard`, `cobs` 0.3.0, `miniz_oxide`, `adler2` (dazu `embedded-io` 0.4/0.6 nur als Lockfile-Eintrag, nicht gebaut); alle MIT oder Apache-2.0 bzw. Zlib/0BSD. B wie A mit `zstd =0.14.2` (libzstd 1.5.7 in C) statt `miniz_oxide`. C nur `postcard`, ohne Kompression. Empfehlung A.
  - Nicht vorgeschlagen: `bincode` (RUSTSEC-2025-0141, nicht mehr gepflegt), `lz4_flex` (RUSTSEC-2026-0041 im Entpacker, März 2026), `ruzstd` (RUSTSEC-2026-0344 Dekompressionsbombe, behoben vor 8 Tagen), `rmp-serde` und `ciborium` (selbstbeschreibend und größer; `ciborium` zuletzt 2024-01), JSON für den Weltzustand (zu groß und langsam für das Raster).
  - Messung im Container (4 vCPU, Release-Build), Raster des Referenz-Terminals 768² × 5 in Welt 2048²: 4.988 Blöcke, 15,4 MB als postcard, Schreiben 23 ms, Lesen 23 ms; Deflate Stufe 6: 59 KB, Packen 33 ms, Entpacken 19 ms; zstd Stufe 3: 24 KB, 6 ms / 10 ms; FNV-1a 64 über 15 MB 21 ms (über die gepackte Datei unter 1 ms). Das synthetische Terminal ist sehr gleichförmig; echte Flughäfen packen schlechter.
  - Abgleich mit S-5/S-6 (Maßnahme 7): (1) Den Container (Kopf, Metadaten als JSON mit `serde_json`, Vorschaubild, Weltzustand, Prüfsumme) baut und prüft der Kern (`core/src/savegame/`); der Hauptprozess liefert das Vorschaubild, schreibt die Datei und reicht sie beim Laden und für die Liste (Kopf und Metadaten, höchstens 64 KB) an den Kern. (2) Neu und additiv: `saves.preview(id)` → PNG-Bytes für die `blob:`-URL der Liste (S-6 nennt nur `list`/`save`/`load`/`delete`, die Liste liefert keine Bilder). (3) „Flughafenname“ gibt es im Weltzustand noch nicht: Feld bleibt leer bis 5.1 (META-02). (4) Autosave-Intervall und -Anzahl („einstellbar“): in 2.5 feste Werte (alle 10 Minuten laufender Spielzeit, 5 Dateien), einstellbar mit `settings.*` in 2.6. (5) Ältere Spielstände vor 1.0 werden mit `migrationFailed` abgewiesen (kein eigener Code). (6) Die Liste prüft nur Kopf und Metadaten; Schäden im Rest meldet erst das Laden (`checksumMismatch`). (7) Höchstzahlen: Personen 200.000 (`MAX_PERSONS`), Objekte 1.000.000 (wirksam ab 3.1), Fahrzeuge 50.000 (wirksam ab 3.4). (8) Oberfläche in 2.5 minimal: Konsolenbefehle und eine einfache Spielstand-Liste mit Vorschau, Angebot „letzten Autosave laden“ nach Absturz des Kerns; das Spielmenü folgt in 5.1. (9) Fehlerform `{ code, textKey, params }` für alle S-6-Aufrufe über eine Antwort-Hülle im IPC, auch `measure.*` und `packs.*`.
  - Aufteilung in zwei Stufen (zählt als ein Schritt): 2.5a Kern (Weltzustand, Container, Grenzen, Prüfreihenfolge, Fehlerkatalog, `core/tests/savegame_roundtrip.rs`); 2.5b Plattform (Fehlerform, `saves.*`, `logs.*`, Autosave, Schnellspeichern, Vorschaubild, CSP `img-src blob:`, Liste, Rauchtest). Getrennte Prüfung am Ende (Kategorie 6: CSP, `logs.openFolder`, nicht vertrauenswürdige Dateien).
  - Klassifikation: ADR-027 (Bibliotheken) füllt den in ADR-021 offen gelassenen Punkt und ist `[OPERATIV]`; die Ergänzung (2) ändert S-6 aus ADR-021 (6 Tage alt) und wird vorsichtshalber als eigener ADR-028 `[REAKTIV]` geführt. Reaktiv-Quote danach 5 / 10 (ADR-019 bis ADR-028).

#### 2.6: Lokalisierung

- **Status:** ERLEDIGT 2026-10-19 (ADR-031); CI-Lauf 39270974482 auf `ff53c01` grün auf drei Plattformen (17 Jobs)
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.7
- **Freigabepflichtig:** ja, falls eine Bibliothek für Lokalisierung eingeführt wird (Kategorie 3)
- **Empfohlene Klasse:** Routine – klar spezifizierte Umsetzung; Entscheidung nur für eine neue Abhängigkeit.
- **Eingangskriterien:** Oberfläche startet (2.7)
- **Anforderungen (ab Klasse M):** FR-006
- **Zu tun:** Deutsch und Englisch, Sprachwechsel ohne Neustart, Sprachpakete aus einem Ordner; `settings.get`/`settings.set` über S-6 mit der Sprache als erster Einstellung (aus 2.7); Autosave-Intervall und Zahl der Autosave-Dateien 1–20 als Einstellungen (aus 2.5, dort feste Werte: 10 Minuten laufender Spielzeit, 5 Dateien; META-10); Schema der Inhaltsart `lang/` in Datenpaketen nach S-4 (Landeplatz aus 2.4, ADR-025).
- **Akzeptanzkriterien:** `app/i18n.test.ts` grün; ein Sprachpaket aus dem Ordner wird erkannt.
- **Betroffene Module:** Lokalisierung, Oberfläche; zusätzlich Inhalte & Mods, Brücke, Plattform, Spielfeld (Konsole) – Abweichung vor dem Code vorgelegt und freigegeben am 2026-10-19 (ADR-031); in 2.6b zusätzlich der Kern-Crate mit dem Typ `Settings` (`core/src/settings.rs`, Takt & Weltzustand; vorgelegt und freigegeben am 2026-10-19, Option A)
- **Reifegrad-Wirkung:** Modul Lokalisierung → `[BELASTBAR]`
- **Artefakte:** Lokalisierungs-Code, Sprachdateien, Tests
- **Vorlage zur Freigabe (2026-10-19, Entscheidungs-Klasse; Abgleich nach Maßnahmen 7, 8, 9; freigegeben: A und alle Festlegungen, ADR-031):**
  - **Keine Bibliothek:** eigenes Modul `app/renderer/i18n.ts` (Nachschlagen, Platzhalter `{name}` nur als Text eingesetzt, Zahlen über `Intl.NumberFormat` der gewählten Sprache, Rückfall gewählte Sprache → Englisch → Schlüssel); Kategorie 3 wird nicht berührt. Nicht vorgeschlagen: i18next und ähnliche (Funktionen wie Pluralformen und Namensräume werden in 2.6 nicht gebraucht; jede Bibliothek wäre eine Abhängigkeit mehr im Renderer).
  - **Lücke 1 (Maßnahme 8):** S-3 („Texte … auch für Texte aus Datenpaketen (`lang/`)“) und S-4 (`lang/` als Inhaltsart, Konsument Takt & Weltzustand) setzen voraus, dass Texte aus Paketen die Oberfläche erreichen. Weder S-6 noch der Port (S-1/S-2/S-3/S-7) bietet dafür einen Weg; die Modul-Karte kennt keine Beziehung Inhalte → Lokalisierung. Optionen: **A** neue Funktionen in S-6 (additiv): `lang.list()` → verfügbare Sprachen `{ code, name, keys }`, `lang.texts(code)` → Texte einer Sprache (zusammengeführt über die Pakete); der Hauptprozess holt sie vom Kern-Prozess wie `packs.*`; **B** neue Nachricht auf dem Port (Renderer ↔ Kern direkt), ändert die gemeinsamen Grundlagen von S-1 bis S-7; **C** nur Deutsch und Englisch beim Build eingebunden, keine Sprachpakete aus Ordnern – erfüllt FR-006 nicht (bräuchte Descope-ADR). Empfehlung A. Klassifikation: ändert S-6 und die Modul-Karte aus ADR-021 (6 Tage alt) → nach der 14-Tage-Regel `[REAKTIV]`, Reaktiv-Quote danach 6 / 10 (ADR-022 bis ADR-031).
  - **Lücke 2 (Maßnahme 8):** Meldungen müssen auch ohne laufenden Kern lesbar sein (S-6 „Absturz des Kerns“, Fehler beim Start). Festlegung: Die Sprachdateien des Basispakets werden zusätzlich beim Build in den Renderer eingebunden (dieselben Dateien, keine zweite Quelle); `lang.texts` ersetzt sie, sobald der Kern bereit ist.
  - **Inhaltsart `lang/` (S-4):** Datei `{ "$schema"?, "languages": [ { "id", "name"?, "texts": { Schlüssel: Text } } ] }`; `id` ist der Sprachcode (`[a-z]{2,3}` mit optionalem Zusatz `-[a-z0-9]{2,8}`, z. B. `de`, `pt-br`); mehrere Pakete tragen zur selben Sprache bei; `name` muss die Sprache mindestens einmal benennen (Basispaket: `Deutsch`, `English`). Schlüssel `[a-z0-9_.-]{1,128}` (korrigiert vor dem Code auf `[A-Za-z0-9_.-]{1,128}`, siehe ADR-031), Text höchstens 4.096 Zeichen (Grenze aus S-4), jeder Text zählt als Eintrag (Grenze 100.000 je Paket aus S-4). Derselbe Schlüssel zweimal in einer Sprache: `duplicateId` für das später geladene Paket. Überschreiben fremder Texte (wie `replaces`) nicht in 2.6, Landeplatz 6.1. `content/base/lang/de.json`, `en.json`; Schema-Datei `content/schema/lang.schema.json`.
  - **Einstellungen (S-6 `settings.*`, S-5, META-10):** erzeugter Typ `Settings` (Rust, ts-rs) mit `language`, `autosaveMinutes` (1–60, Standard 10), `autosaveCount` (1–20, Standard 5). Herkunft der Grenzen (Maßnahme 9): 1–20 aus S-5; 60 Minuten ist eine Schätzung (seltener schützt der Autosave kaum noch), Überprüfung mit dem Spielmenü in 5.1. Standard-Sprache beim ersten Start aus der Systemsprache (Deutsch → `de`, sonst `en`). Datei `settings.json` im Datenordner (Schreiben und Umbenennen); unlesbar oder ungültig → Standardwerte, Warnung im Log. `settings.set(teilweise)` prüft jedes Feld (Sprache muss in `lang.list` stehen), liefert die vollständigen Einstellungen und weist ab mit `invalidArgument` oder `ioError`; kein `busy` (unabhängig von Spielständen). Intervall gilt sofort (der Hauptprozess meldet es dem Kern-Prozess); bei kleinerer Zahl bleiben vorhandene Autosaves über der Zahl erhalten (kein Löschen von Spielerdaten), die Rotation läuft nur noch über `auto-1` … `auto-N`.
  - **Umfang der Übersetzung:** übersetzt werden alle Texte für den Spieler – Spielstand-Liste, Angebot nach Absturz, Hinweis auf die Logdatei, Fehlermeldungen über `textKey` (`save.error.*`, `platform.error.*`, `datapack.error.*`, `command.rejected.*`), Objektnamen (`object.*`) und ein neues, minimales Einstellungs-Panel (Sprache, Autosave-Intervall, Autosave-Zahl; Ausbau im Spielmenü 5.1). Konsole und Leistungsanzeige bleiben englische Entwicklerwerkzeuge (FR-009); neuer Konsolenbefehl `lang [Code]`. Ein Test prüft, dass `de` und `en` jeden im Code erzeugten Schlüssel enthalten.
  - **Gültigkeit nach `packs.setActive`:** Die Texte folgen dem zuletzt geladenen Katalog (wie `packs.list`). Objekte einer laufenden Sitzung aus einem abgewählten Paket zeigen bis zur nächsten Sitzung ihren Schlüssel.
  - **„Betroffene Module“ gegen den geplanten Code (Abweichung, Stopp-Kriterium 5):** geplant Lokalisierung, Oberfläche. Tatsächlich zusätzlich **Inhalte & Mods** (Format `lang/` im Lader `core/src/datapack/`, Schema, Basispaket), **Brücke** (Texte und Sprachliste aus dem Ladeergebnis, `bridge/src/lib.rs`), **Plattform** (`settings.*`, `lang.*`, `settings.json`, Autosave-Intervall an `core-host.ts`, Rotation in `saves.ts`, Preload) und **Spielfeld** (Konsole `lang`, Verdrahtung in `main.ts`). Takt & Weltzustand bleibt unberührt (die Texte gehen nicht in den Katalog der Simulation).
  - **Aufteilung in zwei Stufen (zählt als ein Schritt):** 2.6a Kern (Format `lang/`, Lader, Schema, Basispaket `de`/`en`, Brücke; Tests in `core/tests/datapacks.rs`); 2.6b Plattform und Oberfläche (`settings.*`, `lang.*`, `i18n.ts`, Panels, Rauchtest mit Sprachwechsel ohne Neustart und Sprachpaket aus dem Ordner `packs`). Getrennte Prüfung am Ende (Kategorie 6: neue IPC-Funktionen, `settings.json` als nicht vertrauenswürdige Eingabe, Texte aus Mods in der Oberfläche).

- **Zwischenstand Stufe 2.6a (2026-10-19, Entscheidungs-Klasse; Routine-Arbeit oberhalb der Empfehlung):** Inhaltsart `lang/` im Lader (`core/src/datapack/languages.rs`, Stufe 2 in `load.rs`, `LangFile`/`LanguageEntry`/`Texts` in `model.rs`; `Texts` behält doppelte Schlüssel einer Datei, damit sie als `duplicateId` gemeldet werden), `LoadOutcome.languages`; Schema `content/schema/lang.schema.json`; Basispaket `content/base/lang/de.json` und `en.json` mit 54 Texten je Sprache (Objektnamen und alle Schlüssel des Kerns); `text::core_text_keys()` aus Listen `ALL` in den Code-Modulen von S-2, S-4, S-5, S-7; Brücke `Core.languages()`/`Core.texts(code)`; erzeugter Typ `LanguageInfo`. Jeder Text zählt als Eintrag des Pakets (Basispaket jetzt 111 Einträge). Tests: `core/tests/lang.rs` 9 Fälle (Abdeckung aller Kern-Schlüssel und Objektnamen in `de` und `en`, gleiche Schlüssel und Platzhalter, neue Sprache aus einem Paket, Texte zu vorhandener Sprache, Schlüssel eines anderen Pakets und doppelter Schlüssel in einer Datei → ganzes Paket aus, fehlerhafte Einträge mit Stelle, strenge Struktur), Unit-Tests in `languages.rs` und `model.rs`; Kern 133 Unit-Tests, Rust gesamt 198 Tests grün; Coverage Kern 98,33 % Lines (`languages.rs` 100 %, `load.rs` 96,5 %). Erwartungen „3 Einträge“ des Basispakets in `core/tests/datapacks.rs` und im Rauchtest auf die neue Zählung umgestellt (Rauchtest leitet sie aus den Sprachdateien ab, grün im Container). Gates lokal grün: rustfmt, clippy `-D warnings`, cargo-deny, ESLint, Prettier, tsc, Vitest (300 Tests). Offen: 2.6b.

- **Zwischenstand Stufe 2.6b (2026-10-19, Entscheidungs-Klasse; Routine-Arbeit oberhalb der Empfehlung):** umgesetzt in `62dd9fe` (Kern: Typ `Settings` in `core/src/settings.rs`, nur für ts-rs, Ort freigegeben Option A), `5905d75` (Plattform: `app/src/main/settings.ts` mit Prüfung je Feld, `settings.json` mit Schreiben und Umbenennen nacheinander, `settings:*` und `lang:*` in `main.ts` und Preload, Sprachliste mit `ready`/`packs` vom Kern-Prozess, Anfrage `lang:texts`, Autosave-Intervall `autosave:interval` an `AutosaveClock.setInterval`, Rotation über `autosaveCount`; `PLATFORM_CODES` mit Test gegen den Quelltext; Basis-Sprachdateien jetzt 96 Texte je Sprache), `eb216a8` (Renderer: `app/renderer/i18n.ts` ohne Bibliothek mit eingebundenen Basis-Texten und Rückfall gewählte Sprache → Englisch → Schlüssel, nur der jüngste Sprachwechsel wirkt; Einstellungs-Panel `settings-panel.ts` und Knopf; Spielstand-Liste und Absturz-Hinweise übersetzt; Konsolenbefehle `lang [Code]`, `settings`), `ebd3db8` (Rauchtest). Tests: `app/renderer/i18n.test.ts` (Format, Rückfall, Wechsel mit Benachrichtigung, gleiche Schlüssel und Platzhalter `de`/`en`, Abdeckung aller `ui.*`-Schlüssel und Fehlercodes des Hauptprozesses, Laden mit Rückfall auf eingebundene Texte), `settings.test.ts`, `settings-panel.test.ts`, Ergänzungen in `core-status`, `saves`, `console`, `platform-error`; TypeScript 338 Tests, Coverage 85,5 % Lines (DOM-Panels `saves-panel.ts` 0 %, `settings-panel.ts` 13 %, nur über den Rauchtest). Rauchtest dreimal grün im Container: `settings.json` vorab mit Englisch, Wechsel `lang de` und `lang fr` ohne Neuladen und in derselben Sitzung, Sprachpaket `french` aus dem Ordner `packs` erkannt (`lang` listet „Français – 2 texts“), fehlende Texte auf Englisch, Abweisung ungültiger Einstellungen mit `invalidArgument`, Datei `settings.json` geschrieben. Nicht Ende-zu-Ende belegt: Wirkung des Autosave-Intervalls (Unit-Test `AutosaveClock`). Abweichung beim Committen: der Typ-Check des Pre-Commit-Hooks prüft den ganzen Arbeitsbaum; Renderer und Rauchtest wurden dafür kurz per `git stash` beiseitegelegt. Offen: getrennte Prüfung (Kategorie 6), CI-Lauf, Abschluss.

- **Abschluss (2026-10-19, Entscheidungs-Klasse):** Akzeptanzkriterien erfüllt – `app/renderer/i18n.test.ts` grün; ein Sprachpaket aus dem Ordner `packs` wird erkannt (Rauchtest auf drei Plattformen, dazu `core/tests/lang.rs`); Sprachwechsel ohne Neustart im Rauchtest belegt. Getrennte Prüfung (Sonnet 5.5) mit 7 Befunden, keiner hoch; Erledigung in [`docs/research/2.6-getrennte-pruefung.md`](research/2.6-getrennte-pruefung.md) (`d53f808`, `ba64ad2`, `72eab85`, `4ad8d38`): Befunde 1, 3, 4, 5, 6 behoben, 2 teilweise behoben und Rest per Entscheidung des Projektinhabers in 6.1, 7 begründet nicht geändert. Nach den Korrekturen: Kern 133 Unit-Tests plus `core/tests/lang.rs` 11 Fälle, Coverage Kern 98,43 % Lines; TypeScript 338 Tests, Coverage 85,5 % Lines; Rauchtest dreimal grün im Container. Modul Lokalisierung `[BELASTBAR]`, FR-006 erledigt. Onboarding-Pfad nicht berührt (keine neue Voraussetzung, keine Änderung an `package.json` oder Skripten); README „Verwendung“ ergänzt. Landeplätze: Einstellungs-Menü und Prüfung der 60-Minuten-Grenze 5.1; Überschreiben fremder Texte und Gesamtgrenze über alle Pakete 6.1.

#### 2.11: Sammelprüfung der Vertragssätze S-1 bis S-7 (Maßnahme 10)

- **Status:** ERLEDIGT (2026-10-19)
- **Phasentyp-Kontext:** ERKUNDUNG (Entwurfsschritt in Phase 2, wie 2.9)
- **Schritt-Art (nur ERKUNDUNG):** sonstiges (Prüfung von Verträgen)
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 2.6
- **Freigabepflichtig:** ja, falls Funde Verträge ändern (Kategorie 5, ggf. 1)
- **Empfohlene Klasse:** Entscheidung – Ergebnis ist ein ADR zu Schnittstellenverträgen.
- **Eingangskriterien:** 2.6 erledigt
- **Anforderungen (ab Klasse M):** keine (Infrastruktur der Verträge)
- **Zu tun:** Jeden Satz in `architecture.md` Abschnitt 4 (S-1 bis S-7 und gemeinsame Grundlagen), der Daten oder Funktionen einer anderen Schnittstelle oder eines anderen Moduls voraussetzt, gegen deren Funktionslisten und gegen die Modul-Karte lesen; Funde gesammelt mit Optionen vorlegen (Maßnahme (10), Reflexion zu ADR-031).
- **Akzeptanzkriterien:** Liste aller geprüften Sätze mit Ergebnis (passt / Lücke); Lücken per ADR entschieden oder mit Landeplatz versehen.
- **Betroffene Module:** Brücke, Plattform, Oberfläche, Inhalte & Mods, Takt & Weltzustand (nur Verträge, kein Code)
- **Reifegrad-Wirkung:** keine (S-1 bis S-7 bleiben `[BELASTBAR]`, ggf. per ADR fortgeschrieben)
- **Artefakte:** Prüfliste in `docs/research/`, ggf. ADR
- **Stand 2026-10-19 15:19 UTC:** Prüfliste `docs/research/2.11-sammelpruefung.md`: 81 Sätze geprüft, 41 ohne Fund; 10 Lücken (L-1 … L-10), 5 Kartenfunde (K-1 … K-5), 8 redaktionelle Funde (R-1 … R-8). Vorlage als `ENTSCHEIDUNG ERFORDERLICH` (Kategorie 5 und 1) mit Empfehlungen: L-1 Entwurfsschritt 3.8 vor 3.1 (Weltdarstellung), L-2 Katalog über S-7 (3.1), L-3 `game.new` in S-6 (Grundform 3.1, voll 5.1), L-4 Grafik über die volle Objekt-ID (2.10), L-5 Parameter `{ key }` (3.1), L-6 Wörterbuch aus Textschlüsseln und Tick im Detail (3.6), L-7 Debug-Vermerk in 5.6, L-8 Kanal als S-8 beschreiben, L-9 `sessionEnded` streichen, L-10 Texte der laufenden Sitzung (6.1); Modul-Karte nach K-1 … K-5; R-1 … R-8 mit dem ADR. Ein gemeinsamer ADR-032 wäre `[REAKTIV]` (ändert ADR-021 binnen 14 Tagen), Quote danach 7 / 10. Wartet auf Freigabe.
- **Ergebnis (2026-10-19):** freigegeben vom Projektinhaber per Auswahl „Alle Empfehlungen“; ADR-032 `[REAKTIV]`. `architecture.md` fortgeschrieben (Modul-Karte K-1 … K-5, Grundlagen, S-1 bis S-7, neu S-8 Kern-Prozess-Steuerung `[BELASTBAR]`, Weltdarstellung `[OFFEN]`); Landeplätze in 2.10, 3.1, 3.6, 3.8 (neu), 5.1, 5.6, 6.1. Akzeptanzkriterien: Liste aller geprüften Sätze mit Ergebnis in der Prüfliste; jede Lücke per ADR-032 entschieden und mit Schritt versehen. DoD für einen Entwurfsschritt ohne Code: markdownlint über die Hooks grün, CI des Pushs grün (siehe Logbuch); Linter, Typprüfung, Tests und Coverage nicht anwendbar.

#### 2.10: Stilleitfaden, SVG-Pipeline und Messung der Darstellung

- **Status:** ERLEDIGT (2026-10-19)
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 2.7, 2.11 (Zuordnung Objekt → Bild, ADR-032)
- **Freigabepflichtig:** ja (Kategorie 3: SVG-Rasterer als neue Abhängigkeit)
- **Empfohlene Klasse:** Entscheidung – neue Abhängigkeit und Bewertung einer Leistungsmessung (ADR-017).
- **Eingangskriterien:** Spielfeld-Grundgerüst (2.7)
- **Anforderungen (ab Klasse M):** FR-050
- **Zu tun:** Stilleitfaden (Palette, Strichstärken, Maßstab 2 m je Zelle nach ADR-020, Detailgrad); SVG-Quellen im Repository, Rasterung in mehreren Auflösungen im Build (ADR-008), Atlanten; Bildname im Atlas = volle Objekt-ID (`base:bench`), neutrale Ersatzgrafik für Objekte ohne Bild (ADR-032, L-4); für die Messung platziert das Messwerkzeug die Objekt-Sprites selbst, solange der Weg der Weltdarstellung offen ist (3.8); eigenes Programmsymbol für Fenster, Installer und Pakete (Befund 2.1-N2); Messung des Spielfelds mit Objekt-Sprites aus den Atlanten, Overlays und 10.000 Personen auf Referenz-PC und Mac; dabei auf dem Mac prüfen, ob die Aussetzer der Simulation bis 110 ms aus 2.7 wiederkehren, und bei Wiederholung die Ursache klären (Vermutung: Verteilung von Tick- und Hintergrund-Threads auf Leistungs- und Effizienzkerne), dazu ein Vergleichslauf mit weniger Hintergrund-Threads (z. B. `--background-threads=4`; auf dem Referenz-PC liefen die Schritte damit ruhiger, 2.7).
- **Akzeptanzkriterien:** `scripts/assets.test.ts` grün (umgesetzt als `scripts/assets.test.mjs` nach dem Muster der übrigen Skript-Tests, dazu cargo-Tests in `tools/assets`); Messprotokoll zeigt, ob 60 Bilder/s mit echten Grafiken halten; Ergebnis per ADR (NFR Darstellung `[VORLÄUFIG]` → `[BELASTBAR]` oder Maßnahme).
- **Betroffene Module:** Spielfeld, Build/CI
- **Reifegrad-Wirkung:** NFR Performance Darstellung mit echten Grafiken `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen
- **Artefakte:** Stilleitfaden, Pipeline-Skript, Atlanten, Messprotokoll, ADR
- **Stand 2026-10-19 16:22 UTC:** Vorlage als `ENTSCHEIDUNG ERFORDERLICH` (Kategorie 3, mit Kategorie 7 für den neuen Bauschritt): A resvg 0.48.1 (Rust, crates.io, Apache-2.0 OR MIT) in einem eigenen Werkzeug-Crate `tools/assets` im Cargo-Workspace, nur im Build, mit Atlas-Packer und Bildliste ohne weitere Abhängigkeit; B @resvg/resvg-js 2.6.2 (npm, MPL-2.0, vorgebaute native Pakete; letzte stabile Version 2024-03-26, seither nur Alpha-Versionen); C Rasterung in Electron/Chromium ohne neue Abhängigkeit. Verworfen ohne Option: sharp (vorgebaute libvips-Binärdateien unter LGPL-3.0, nicht erlaubt). Empfehlung A. Belege: Probe-Projekt mit `default-features = false` – 37 Crates, alle Lizenzen erlaubt, eine doppelte Version `miniz_oxide` 0.8.9 neben 0.9.1 (über png 0.18.1) als benannte Ausnahme in `deny.toml` nötig; Rasterung eines Test-SVG in 1× und 2× lief. Wartet auf Freigabe.
- **Freigabe (2026-10-19 16:28 UTC):** Option A per Auswahl, ADR-033 `[OPERATIV]`.
- **Stand 2026-10-19 17:02 UTC:** umgesetzt und lokal geprüft: Werkzeug-Crate `tools/assets` (Atlanten in 8/16/32 px je Zelle, Bildliste, Programmsymbol), Stilleitfaden `docs/stilleitfaden.md` mit Palette `assets/palette.json`, SVG-Quellen in `assets/` (außerhalb des Paketordners, S-4 unverändert), Prüfer `scripts/assets.mjs`, Atlanten im Spielfeld (`app/renderer/atlas.ts`, ohne `fetch`, CSP unverändert), Szene des Messwerkzeugs (20.000 Objekte, 64 Zonen), Fenstersymbol unter Linux, Rauchtest mit `data-atlas`. Onboarding im frischen Worktree validiert. Messanleitung `docs/research/2.10-messung.md`. CI-Lauf 39290398392 grün (17 Jobs); Messpakete aus Release-Lauf 39291102397 (`62f3ac0`).
- **Ergebnis (2026-10-19):** Messung durch den Projektinhaber auf Referenz-PC (60 Hz) und Mac M3 Pro (120 Hz, zwei Läufe mit 2 + 8 und 2 + 4 Threads), `docs/research/2.10-messung.md`: Bildrate hält auf beiden Rechnern mit 20.000 Sprites, Zonen und 10.000 Personen. ADR-034 (freigegeben per Auswahl A): NFR Darstellung geteilt, Gemessenes `[BELASTBAR]`, Rest `[VORLÄUFIG]` mit Landeplätzen 3.8, 3.4/5.5, 4.1, 5.8, 7.2; Schrittzeit-Spitzen auf Apple-Chips in neuem Schritt 4.3. Programmsymbol auf beiden Plattformen sichtbar (Befund 2.1-N2 erledigt). Akzeptanzkriterien: `scripts/assets.test.mjs` grün, Messprotokoll mit Ergebnis, ADR-034. DoD: Linter, Formatter, Typprüfung, cargo-deny, Tests mit Coverage (Vitest 86,5 % gesamt, `atlas.ts` 100 %, `tools/assets` 96,0 %), Rauchtest auf drei Plattformen, Onboarding-Validierung, CI grün (Lauf 39290398392).

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
