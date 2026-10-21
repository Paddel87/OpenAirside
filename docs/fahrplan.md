# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Laufende Phase im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-21 (Session ab 22:30 UTC, Vorlage zu 3.9)
- **Laufende Phase:** Phase 3 – Vertikaler Schnitt „Kleiner Flughafen“; Phase 2 abgeschlossen am 2026-10-19 (ADR-036, archiviert in [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md))
- **Phasentyp:** UMSETZUNG (mit Entwurfs- und Erkundungsschritten 3.7, 3.8, 3.11, 3.12, 3.13, Phasentyp-Kontext ERKUNDUNG)
- **Aktiver Schritt:** 3.9 Kernstruktur der Fachmodule – Umbau nach ADR-039 (Option B); 3.15 erledigt am 2026-10-20
- **Nächster Schritt:** Umbau 3.9 abschließen; danach 3.10
- **Offene STOPP-Situationen:** keine. Reaktiv-Quote 7 / 10 (ADR-029 bis ADR-038) über dem Schwellenwert; Reflexion zu ADR-038 (neues Muster nach Regel-009) zur Kenntnis genommen am 2026-10-20, siehe „Iterations-Reflexion“

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

**Ziel:** Ein kleiner Flughafen ist von Anfang bis Ende spielbar, auf einem Kern, der mehrere Fachmodule geordnet trägt (ADR-036).

**Abschlusskriterium:** Szenario 2 der Vision („Der Einsteiger") ist spielbar.

**Reifegrad-Erwartung am Phasenende:** Weltdarstellung im Spielfeld per ADR in 3.8 `[BELASTBAR]` (erreicht, ADR-038); Speicher der Weltdarstellung nach Nachmessung in 3.1; NFR Darstellung unter Windows nach 3.15 (erreicht 2026-10-20, Auflage 3.1); Module Personen, Wegfindung & Passagierführung, Verkehr & Verkehrssteuerung, Flugbetrieb, Abfertigung & Gepäck, Spielfeld und Oberfläche durch Umsetzung `[BELASTBAR]` im umgesetzten Umfang; NFR „Bauaktion im nächsten Bild" gemessen (3.1); Verkehr/Blockadevermeidung per ADR aus 3.12.

**Ursprünglicher Schrittplan:** 13 Schritte, festgehalten am 2026-10-19 beim Phasenabschluss 2 (ADR-036): 3.1–3.13. Seit 2026-10-20 (ADR-038) 14 Schritte mit 3.14, seit 2026-10-20 (Ergebnis 3.14, Antwort des Projektinhabers) 15 mit 3.15; Wucherungs-Schwelle (Faktor 2 und mindestens 5 zusätzliche, also 26) nicht erreicht.

**Reihenfolge:** 3.7 → 3.8 → 3.14 → 3.15 → 3.9 → 3.10 → 3.1 → 3.2 → 3.11 → 3.3 → 3.12 → 3.13 → 3.4 → 3.5 → 3.6. 3.7, 3.8, 3.11, 3.12 und 3.13 sind Entwurfs- bzw. Erkundungsschritte mit Phasentyp-Kontext ERKUNDUNG (wie 2.8 und 2.9); ebenso 3.14 (seit ADR-038); sie liefern Entscheidungen oder Messungen, keinen Spielcode.

**Form der Einträge (ADR-036):** Je Schritt Ergebnis, Belege und Landeplätze; Herleitungen und Vorlagen stehen in `docs/research/`. Vor jedem Schritt, der eine Schnittstelle berührt, läuft der Vertragsabgleich nach den Maßnahmen 7–9 (Iterations-Reflexion); additive Ergänzungen nach Regel-008.

#### 3.7: Personengruppen, Begehbarkeit und Querungen

- **Status:** ERLEDIGT (2026-10-20, ADR-037)
- **Phasentyp-Kontext:** ERKUNDUNG (Entwurfsschritt)
- **Schritt-Art (nur ERKUNDUNG):** Entwurf mit Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 2.3
- **Freigabepflichtig:** ja (Kategorie 1 und 4, Ergänzung von ADR-020)
- **Empfohlene Klasse:** Entscheidung – Ergebnis ist ein ADR zu Datenmodell und Architektur.
- **Eingangskriterien:** Datenmodell im Kern `[BELASTBAR]` – erfüllt
- **Anforderungen (ab Klasse M):** FR-018 (vorbereitend), FR-052
- **Zu tun:** Gruppen aus dem Inventar (PAX-*, PAX-09, STAFF-01, SHOP-03, BAU-03; offen: Crew, Einweiser) mit Begehbarkeit je Gruppe einschließlich Personalzonen (ZON-02); Personal auf dem Vorfeld (nur per Fahrzeug, Stellplatzfläche begehbar oder beides, jeweils mit Erklärung bei Unerreichbarkeit; TURN-03, GEP-06, GATE-06); Bauarbeiter und laufende Bahnen; Walk-out-Positionen für Fluggäste (MOD-13 seit ADR-036 Kern, FR-052) mit Querungen und Sperrung von Betriebsstraßen; Messung, wie Personal neben den Wegweisern geführt wird.
- **Akzeptanzkriterien:** Wir wissen, welche Gruppe wo gehen darf und wie Personal und Fluggäste das Vorfeld queren; per ADR in Ergänzung von ADR-020 freigegeben, mit Speicher- und Zeitwerten belegt.
- **Betroffene Module:** Welt & Bau, Wegfindung & Passagierführung, Personen, Verkehr & Verkehrssteuerung
- **Reifegrad-Wirkung:** Datenmodell im Kern bleibt `[BELASTBAR]` (per ADR fortgeschrieben)
- **Artefakte:** ADR, Messnotiz in `docs/research/`
- **Ergebnis:** Gruppen Fluggäste, Besucher, Personal und Franchise-Personal, Sicherheitsbeamte, Vorfeldpersonal, Bauarbeiter; keine Crew, keine Einweiser. Flächenart und Personalzone je Zelle, Zugangsprofil je Ziel. Ein Feldsatz für alle Profile außer Bauarbeitern, ein eigener für Bauarbeiter. Personal folgt Wegweisern, keine Einzelweg-Suche. Kontrolle für alle außer Sicherheitsbeamten und Bauarbeitern; Vorfeldpersonal zu Fuß und per Servicewagen; Walk-out-Querungen im Wechsel; Bauarbeiter überall außer auf Bahnen in Betrieb (ADR-037, freigegeben 2026-10-20).
- **Belege:** `spikes/3.7-personengruppen/` (Produktions-Code der Wegweiser, 4 vCPU): Flächen +10 % Zellen; 79 Ziele 130 MB, 2,6 s; Satz für alle Gruppen 444 MB, 9–11 s; Bauarbeiter zwei Felder 10 MB, 0,6 s; A* je Fahrt im Terminal Median 18–61 ms, p99 170–272 ms, Kosten gleich den Wegweisern. Herleitung und Vorlage: `docs/research/3.7-personengruppen.md`.
- **Landeplätze:** 3.1 (Flächenart beim Bauen, Felder der Bauarbeiter, S-5-Format), 3.2 (Personalzonen, Profile der Ziele, Sicherheitsbeamte), 3.3 (Ursachen der Unerreichbarkeit), 3.11 und 4.1 (Zahl der Ziele, Neuberechnung), 3.12 (Reservierung der Querungen), 3.4 (Walk-out, Querungszustand), 3.5 (Vorfeldpersonal), 5.6 (Aufgabenfelder)

#### 3.8: Weltdarstellung im Spielfeld und S-1 Layout 2

- **Status:** ERLEDIGT (2026-10-20, ADR-038)
- **Phasentyp-Kontext:** ERKUNDUNG (Entwurfsschritt)
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie mit Messung
- **Zeitbox (nur ERKUNDUNG):** maximal zwei Sessions (Messung auf Referenz-PC und Mac durch den Projektinhaber)
- **Abhängigkeiten:** 3.7
- **Freigabepflichtig:** ja (Kategorie 5, ggf. 1: Fortschreibung von S-1 bzw. S-7, ADR-032 L-1)
- **Empfohlene Klasse:** Entscheidung – Schnittstellenvertrag und Bewertung einer Leistungsmessung.
- **Eingangskriterien:** 3.7 entschieden (Gruppen, die gezeichnet werden)
- **Anforderungen (ab Klasse M):** FR-010 (vorbereitend)
- **Zu tun:** Wie Raster, Böden, Wände, Objekte und Zonen das Spielfeld erreichen – Ausgangshypothese geänderte Blöcke seit dem quittierten Stand im Bild (S-1), Gegenmodell Abruf über S-7; ob die Darstellung nach Übernahme neuer Wegweiser Zellen neu erhält. **S-1 Layout 2** für bewegte Objekte mit Typ und Ausrichtung (Fahrzeuge, Flugzeuge, Gepäck, Personal) mitentwerfen, damit der nicht additive Schnitt an S-1 nur einmal fällt (ADR-036). Messung von Erstübertragung (ganze Karte), Bauaktion bis zum Bild und Speicher im Renderer im Referenz-Terminal, dabei die Atlas-Stufen 16 und 32 px je Zelle (Nahansicht) und Atlanten mit vielen Grafiken (ADR-034).
- **Akzeptanzkriterien:** Weg der ruhenden Welt und Layout 2 per ADR entschieden, mit Messwerten auf Referenz-PC und Mac; S-1 bzw. S-7 in `docs/architecture-schnittstellen.md` fortgeschrieben.
- **Betroffene Module:** Brücke, Spielfeld, Takt & Weltzustand
- **Reifegrad-Wirkung:** Weltdarstellung im Spielfeld `[OFFEN]` → `[BELASTBAR]` per ADR; NFR Darstellung Stufen 16/32 und große Atlanten nach Messung
- **Artefakte:** ADR, Messprotokoll, Fortschreibung S-1/S-7
- **Ergebnis:** W-A (Blöcke im Bild gegen den an den Port übergebenen Stand) mit Z-1 (Datentextur je 512² Zellen, Shader); S-1 Layout 2 mit Personen (Gruppe nach ADR-037), Fahrzeugen, Flugzeugen, Gepäck und Welt-Abschnitt, `protocol` 2; S-7-Katalog mit fester Reihenfolge (additiv); neue Wegweiser übertragen nichts. Verworfen: Abruf über S-7 (Bauaktion ein Bild später, Bruch von S-2), vorgerenderte Blöcke (Speicher). Speicherschwelle bleibt, Nachmessung in 3.1; späte Bilder auf dem Referenz-PC in 3.14 (ADR-038, freigegeben 2026-10-20).
- **Belege:** sechs Läufe auf Referenz-PC und Mac (`docs/research/3.8-ergebnisse/`), Prototyp auf `erkundung/3-8-messaufbau` (nicht gemergt; Aufzeichnung `spikes/3.8-weltdarstellung/`): ganze Welt 0,12–0,35 s, sichtbarer Ausschnitt ≤ 0,14 s; Bauaktion bis zum Welt-Bild ≤ 57 ms im Bild des Ticks; Mac 120 Bilder/s in allen Ansichten; Referenz-PC zeitweise späte Bilder unabhängig vom Weg; Speicher W-A + Z-1 +161 MB (PC) bzw. +339 MB (Mac). Herleitung und Vorlage: `docs/research/3.8-weltdarstellung.md`.
- **Landeplätze:** 3.14 (späte Bilder Referenz-PC), 3.1 (Layout 2, Welt-Abschnitt, Z-1, Objekte je Block, vereinfachte Übersicht, Katalog-Index, Speicher-Nachmessung), 3.2 (Gruppe und Zustand im Bild), 3.4 (Fahrzeuge, Flugzeuge), 3.5 (Gepäck), 4.1 (Budget je Bild, Speicher im Referenzszenario), 5.1 (Einstellung „alle Einzelheiten“)

#### 3.14: Späte Bilder auf dem Referenz-PC (Windows)

- **Status:** ERLEDIGT (2026-10-20, ohne ADR: Maßnahme ohne Kategorie aus `CLAUDE.md` Abschnitt 4)
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session (Messläufe auf dem Referenz-PC durch den Projektinhaber)
- **Abhängigkeiten:** 3.8; vor 3.1 (dessen Messung der Bauaktion sonst verfälscht wird)
- **Freigabepflichtig:** ja, falls die Maßnahme die Threadaufteilung, die Betriebsart oder den Grafikweg berührt (Kategorie 1)
- **Empfohlene Klasse:** Entscheidung – Bewertung einer Leistungsmessung (ADR-038).
- **Eingangskriterien:** Referenz-PC des Projektinhabers verfügbar
- **Anforderungen (ab Klasse M):** – (NFR Performance)
- **Zu tun:** In allen sechs Läufen von 3.8 kam auf dem Referenz-PC zeitweise jede hundertste Antwort auf eine Bildanforderung ein oder zwei Bildtakte zu spät (Rundlauf p99 14–31 ms statt 1–3 ms in 2.10), unabhängig von Weg, Zeichenart und Ansicht, bei W-B erst nach 4 Minuten; sichtbar als Ruckeln beim Kreisen in der Nahansicht; auf dem Mac nie. Ursache eingrenzen mit Zeitstempeln auf beiden Seiten (Absenden im Kern-Prozess, Eingang im Renderer, Zeichnen), Vergleich mit und ohne Weltdarstellung, Grafikweg (ANGLE/D3D11 gegen D3D11on12 bzw. Vulkan, sofern ohne neue Abhängigkeit schaltbar), Fenster- und Vollbildgröße.
- **Akzeptanzkriterien:** Ursache belegt oder eingegrenzt; Ergebnis Maßnahme oder ADR; Bildabstand p99 auf dem Referenz-PC wieder wie 2.10 oder begründete Grenze.
- **Betroffene Module:** Spielfeld, Brücke, Benchmark & Leistung
- **Reifegrad-Wirkung:** NFR Darstellung unter Windows `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen
- **Artefakte:** Messprotokoll, ggf. ADR
- **Ergebnis:** Ursache belegt: Die Leistungsanzeige las alle 250 ms den GPU-Namen über `getParameter(UNMASKED_RENDERER_WEBGL)`. Der synchrone Rundlauf zum GPU-Prozess blockierte den Hauptthread des Renderers ein oder zwei Bildtakte lang, wenn der GPU-Prozess unter ANGLE/D3D11 einen Flush bis zum nächsten Takt hielt. Das trat auch ohne Weltdarstellung auf. Kern-Prozess, Port und Zeichnen sind ausgeschlossen. Maßnahme: GPU-Namen einmal lesen, keine synchronen WebGL-Abfragen im laufenden Betrieb. Grafikweg, Threads und Betriebsart bleiben unverändert. Die Läufe 3–5 (Vollbild, D3D11on12, Vulkan) entfallen.
- **Belege:** Lauf 2 vorher 2,0 % späte Antworten im 250-ms-Raster. Zeitspuren aus Lauf 1 und 2 mit `GLES2::GetString` → `CommandBufferHelper::Finish` bei jeder späten Antwort. Nachmessung mit `9d01469`: mit Welt 0,01 %, ohne Welt 0 späte Antworten; Rundlauf p99 ≤ 1,1 ms; Bildabstand p99 17,3–18,1 ms; 59,8–60,0 Bilder/s. Messprototyp `erkundung/3-14-messaufbau` (nicht gemergt, Aufzeichnung `spikes/3.14-spaete-bilder/`), Herleitung `docs/research/3.14-spaete-bilder.md`, Messdateien `docs/research/3.14-ergebnisse/`.
- **Landeplätze:** 3.15 (Korrektur auf `main`, Reifegrad NFR Darstellung unter Windows); 3.1 und 4.1 (Bauaktion und Budget je Bild werden ohne die synchrone Abfrage gemessen)

#### 3.15: GPU-Namen nur einmal lesen (Korrektur aus 3.14)

- **Status:** ERLEDIGT (2026-10-20, ohne ADR)
- **Phasentyp-Kontext:** UMSETZUNG (Fehlerbehebung innerhalb des Moduls)
- **Abhängigkeiten:** 3.14; vor 3.1 (dessen Messungen sonst die späten Bilder enthalten)
- **Freigabepflichtig:** nein (Fehlerbehebung im Spielfeld ohne Schnittstellenänderung; Antwort des Projektinhabers vom 2026-10-20: eigener Schritt)
- **Empfohlene Klasse:** Routine – klar spezifizierte Fehlerbehebung mit Test; die Beförderung des Reifegrads ist Entscheidung (Auslöser 4).
- **Eingangskriterien:** Ursache belegt (3.14) – erfüllt
- **Anforderungen (ab Klasse M):** – (NFR Performance)
- **Zu tun:** Der Getter `gpu` in `app/renderer/playfield.ts` liest den Namen einmal und hält ihn, wie Spike-Commit `9d01469`. Die Leistungsanzeige in `app/renderer/main.ts` ruft keine synchrone WebGL-Abfrage mehr auf. Suche nach weiteren synchronen WebGL-Abfragen im laufenden Betrieb (`getParameter`, `getError`, `readPixels`, `getExtension`); die Regel „nur beim Start“ wird als Kommentar am Spielfeld und in `docs/architecture.md` (Spielfeld) festgehalten. Unit-Test: Ein zweiter Zugriff fragt den Kontext nicht erneut ab.
- **Akzeptanzkriterien:** Test grün; Vitest gesamt ≥ 70 %; Rauchtest grün auf drei Plattformen; Messung mit dem Paket des Schritts auf dem Referenz-PC (`measure 6`): Bildabstand p99 ≤ 19,7 ms und unter 0,1 % späte Antworten, wie die Nachmessung 3.14.
- **Betroffene Module:** Spielfeld, Benchmark & Leistung
- **Reifegrad-Wirkung:** NFR Darstellung unter Windows `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen
- **Artefakte:** Spielfeld-Code, Test, Messdatei
- **Ergänzung (2026-10-20, Antwort des Projektinhabers „Zähler ergänzen“):** Das Messwerkzeug auf `main` zählte keine späten Antworten (nur im Spike, `latency.ts`). `app/renderer/measure.ts` meldet je Fenster `answers` und `rttLate` (Rundlauf länger als ein Bildtakt = Bildabstand p50), rein additiv im Messbericht, mit Unit-Test; damit ist das Kriterium „unter 0,1 % späte Antworten“ prüfbar.
- **Ergebnis:** Der GPU-Name wird beim Anlegen des Spielfelds einmal gelesen (`app/renderer/gpu-name.ts`, Klasse `GpuName`); die Leistungsanzeige liest nur noch den gespeicherten Wert. Weitere synchrone WebGL-Abfragen im laufenden Betrieb: keine im eigenen Code; PixiJS 8.20.1 fragt nur beim Anlegen von Kontext, Shader-Programmen und Texturen. Regel „synchrone WebGL-Abfragen nur beim Start“ im Kopf von `app/renderer/playfield.ts` und in `docs/architecture.md` (Modul Spielfeld). Reifegrad „NFR Performance: dasselbe unter Windows“ `[VORLÄUFIG]` → `[BELASTBAR]` nach Antwort des Projektinhabers („Befördern“), Auflage Nachmessung in 3.1.
- **Belege:** Unit-Tests `app/renderer/gpu-name.test.ts` (4, darunter „zweiter Zugriff fragt nicht erneut“) und zwei Tests zu `rttLate` in `measure.test.ts`; `gpu-name.ts` 100 % Lines, Vitest gesamt 86,58 % Lines (362 Tests); CI-Lauf 39508710449 grün (17 Jobs, Rauchtest auf drei Plattformen); Release-Lauf 39508715951 grün. Messung auf dem Referenz-PC (`measure 6`, Paket `41786ec`): 0 von 21.668 Antworten spät, Bildabstand p99 17,3–17,5 ms, 59,9–60,0 Bilder/s, Rundlauf p99 0,7–1,0 ms, keine langen Aufgaben (`docs/research/3.15-messung.md`, Datei `docs/research/3.15-ergebnisse/referenz-pc-measure.json`).
- **Landeplätze:** 3.1 (Nachmessung der Weltdarstellung auf `main` auf dem Referenz-PC gegen die Schwellen aus 3.15, sonst Rückstufung)

#### 3.9: Kernstruktur der Fachmodule

- **Status:** IN ARBEIT (Entwurf B freigegeben am 2026-10-21, ADR-039; Vorlage `docs/research/3.9-kernstruktur.md`)
- **Phasentyp-Kontext:** UMSETZUNG (Umbau des bestehenden Kerns; Entwurf als Vorlage vor dem Code)
- **Abhängigkeiten:** 3.7, 3.8
- **Freigabepflichtig:** ja (Kategorie 1 und 4) – Entwurf als `ENTSCHEIDUNG ERFORDERLICH` vor dem Code
- **Empfohlene Klasse:** Entscheidung – Architekturentscheidung über die Struktur des Kerns; der Umbau selbst ist Routine und abgebbar.
- **Eingangskriterien:** Stack, Pattern, Datenmodell `[BELASTBAR]` – erfüllt; 3.7 und 3.8 entschieden
- **Anforderungen (ab Klasse M):** keine eigene (Infrastruktur für FR-010, FR-011, FR-016)
- **Zu tun:** Befund der getrennten Instanz A (`docs/research/phasenabschluss-2.md`, Befund 6): feste Reihenfolge der Fachmodule im Takt; Zustand, Spielstand-Teil, Invarianten beim Laden und Beitrag zur Zustands-Prüfsumme je Modul statt zentral in `core/src/world.rs` und `core/src/savegame/state.rs`; Weitergabe von Wirkungen zwischen Modulen (interne Ereignisse oder Alternative; `architecture.md` Abschnitt 2 danach berichtigen); Ablage je Fachmodul in `core/src`; Zielwahl der Personen aus `Person` in das Modul Personen (`TODO(fahrplan-ref: 3.2)` in `world.rs` bleibt bis 3.2). Zugeschnitten auf das, was 3.1 und 3.2 brauchen, kein allgemeines Rahmenwerk. Umbau des bestehenden Codes ohne Verhaltensänderung.
- **Akzeptanzkriterien:** Entwurf per ADR freigegeben; nach dem Umbau `core/tests/determinism.rs`, `core/tests/savegame_roundtrip.rs` und alle übrigen Kern-Tests grün; Kern-Benchmarks unverändert innerhalb ihrer Grenzen; Coverage Kern ≥ 85 %; eine neue Objektart braucht nachweislich nur noch Änderungen in ihrem Modul (an einer Probe-Objektart in einem Test belegt).
- **Betroffene Module:** Takt & Weltzustand, Welt & Bau, Wegfindung & Passagierführung, Personen
- **Reifegrad-Wirkung:** Takt & Weltzustand bleibt `[BELASTBAR]` (per ADR fortgeschrieben)
- **Artefakte:** ADR, Kern-Code, Tests, ggf. neue Spielstand-Formatversion (vor 1.0 zulässig, ADR-021)

#### 3.10: Plattform: `main.ts` nach Zuständigkeiten teilen

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG (Refactoring innerhalb des Moduls)
- **Abhängigkeiten:** keine (vor 3.1, weil 3.1 `game.new` in `main.ts` hinzufügt)
- **Freigabepflichtig:** nein (lokales Refactoring, öffentliche Schnittstellen S-6 und S-8 unverändert)
- **Empfohlene Klasse:** Routine – klar spezifiziertes Refactoring ohne Schnittstellenänderung.
- **Eingangskriterien:** S-6, S-8 `[BELASTBAR]` – erfüllt
- **Anforderungen (ab Klasse M):** keine (Befund 2 der getrennten Instanz A)
- **Zu tun:** Dateizugriff auf Spielstände (`openSave`, `readExactly`, `saveFiles`), Pakete und Einstellungen sowie die IPC-Handler aus `app/src/main/main.ts` in Module nach dem Muster von `saves.ts` und `settings.ts`; `main.ts` behält nur die Verdrahtung. Nebenbei: Begründungs-Kommentar an `#[allow(clippy::type_complexity)]` in `core/src/scenario.rs:76` nachtragen (Verstoß gegen `CLAUDE.md` Abschnitt 15 seit 2.3).
- **Akzeptanzkriterien:** Neue Module unter Unit-Tests mit Coverage ≥ 90 % Lines; `main.ts` deutlich kürzer und ohne Dateizugriff; Rauchtest grün auf drei Plattformen; Vitest gesamt ≥ 70 %.
- **Betroffene Module:** Plattform; Takt & Weltzustand nur für den Kommentar in `scenario.rs`
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Plattform-Module, Tests

#### 3.1: Bauen im Raster, Bauaufträge sofort sichtbar, Dauerbau

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.7, 3.8, 3.9, 3.10, 3.14
- **Freigabepflichtig:** ja, falls der Dauerbau-Benchmark als CI-Gate kommt (Kategorie 7); sonst nein (Verträge aus ADR-032)
- **Empfohlene Klasse:** Entscheidung – Bewertung der NFR „Bauaktion im nächsten Bild“ und ggf. neues CI-Gate; die Umsetzung ist Routine und abgebbar.
- **Eingangskriterien:** Weltdarstellung `[BELASTBAR]` (3.8); Kernstruktur umgebaut (3.9)
- **Anforderungen (ab Klasse M):** FR-010, FR-011
- **Zu tun:** Bauen im Raster (eine Etage), Bauaufträge sofort sichtbar; Wirkung von `build`, `buildArea` und `demolish` im Kern (`TODO(fahrplan-ref: 3.1)` aus 2.2); Darstellung des Rasters im Spielfeld (aus 2.7) auf dem Weg aus 3.8 (ADR-038): S-1 Layout 2 mit `protocol` 2, Welt-Abschnitt im Bild (Blöcke gegen den übergebenen Stand, höchstens 1 MB je Bild), Personen-Abschnitt mit Gruppe (0 bis 3.2) und leeren Abschnitten für Fahrzeuge, Flugzeuge und Gepäck, Zeichnung mit Datentextur und Shader (Z-1), Objekte je Block mit Zuschnitt, vereinfachte Übersicht unter 4 Bildpunkten je Zelle, Katalog mit fester Reihenfolge als Index für S-1 (S-7, additiv), Vertragsabgleich S-1 und S-7 nach Maßnahme (7) und (8), Messprototyp-Prüfung nach Maßnahme (11); Katalog der Sitzung über S-7 `catalog` für Bau-Werkzeug und Vorschau, `game.new({ map })` in S-6 und S-8 (Grundform), Parameter `{ key }` in `ParamValue` (ADR-032, L-2, L-3, L-5); Bewegung der Bauarbeiter nach 3.7 (ADR-037: eigener Feldsatz über dem Erdgeschoss, je Anlieferstelle ein Feld und ein Feld zu allen Baustellen; Bauarbeiter überall außer auf Bahnen in Betrieb); Flächenart und Personalzone im Raster, Zugangsprofil am Ziel und Regeln der Wegweiser-Berechnung (ADR-037), neue Format-Version des Spielstands mit Vertragsabgleich S-5 nach Maßnahme (7) und (8); Objekte im Weltzustand und Spielstand mit der Höchstzahl `MAX_OBJECTS` (aus 2.5). **Dauerbau (ADR-036):** überholte Wegweiser-Anfragen zusammenfassen bzw. nicht weiterrechnen (`core/src/adoption.rs`), Benchmark mit einer Bauaktion alle 0,25–0,5 s.
- **Akzeptanzkriterien:** `core/tests/build.rs` und `core/tests/construction.rs` grün; Bauaktion bis zum Bild mit vorläufigem Zeichnen auf Referenz-PC und Mac gemessen (Ziel nächstes Bild, FR-011 < 100 ms); Dauerbau-Benchmark: Wegweiser werden während Dauerbau innerhalb einer festgelegten Frist übernommen, Speicherspitze unter einer festgelegten Grenze (Frist und Grenze in der Vorlage vor dem Code); Speicher der Weltdarstellung mit Referenz-Terminal in Welt 2048² und 20.000 Objekten höchstens 256 MB über 2.10 auf Referenz-PC und Mac (ADR-038); Weltdarstellung auf dem Referenz-PC mit Bildabstand p99 ≤ 19,7 ms, unter 0,1 % späten Antworten (`rttLate`) und ≥ 59,5 Bildern/s (Auflage aus 3.15); Coverage Kern ≥ 85 %; neue Grafiken vom Projektinhaber abgenommen (`docs/stilleitfaden.md` Abschnitt 7).
- **Betroffene Module:** Welt & Bau, Takt & Weltzustand, Personen (Bauarbeiter), Brücke, Spielfeld, Oberfläche, Plattform
- **Reifegrad-Wirkung:** Welt & Bau (Bauen) durch Umsetzung; NFR Bauaktion im nächsten Bild `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen; Speicher der Weltdarstellung `[VORLÄUFIG]` → `[BELASTBAR]` bei Bestehen (ADR-038); NFR Performance unter Windows bleibt `[BELASTBAR]` nur bei Bestehen der Auflage aus 3.15, sonst `[VORLÄUFIG]`
- **Artefakte:** Kern-Code, Spielfeld, Bau-Werkzeug, Benchmark, Messprotokoll

#### 3.2: Kleines Terminal und Passagierablauf

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.1
- **Freigabepflichtig:** nein, sofern keine neue Schnittstelle oder Abhängigkeit (sonst Vorlage vor dem Code)
- **Empfohlene Klasse:** Routine – Umsetzung nach Inventar und Kernprozessen; Entscheidung nur bei Vertragsfragen.
- **Eingangskriterien:** 3.1 erledigt
- **Anforderungen (ab Klasse M):** FR-012, FR-016, FR-019, FR-020, FR-022
- **Zu tun:** Kleines Terminal mit Check-in, Sicherheit, Gate, Boarding und Toiletten; Zonen und Räume in Grundform; Passagierablauf Abflug und Ankunft mit Bedürfnissen und Zielwahl im Modul Personen (`TODO(fahrplan-ref: 3.2)` in `world.rs`); Personalzonen (ZON-02, nur verlassen für Fluggäste), Zugangsprofile der Ziele, Personal und Sicherheitsbeamte (ohne Zonenregel) an Schalter, Kontrolle und Gate, landseitige Ziele ohne Weg über die Kontrolle (ADR-037); Gruppe und Zustand der Personen im Bild (S-1 Layout 2, ADR-038); Inventar-IDs nach Regel-004.
- **Akzeptanzkriterien:** `core/tests/pax_flow.rs` grün; 1 h Spielzeit im Szenario ohne Stillstand im Passagierverkehr, der sich nicht selbst auflöst (FR-016, ADR-036); `core/tests/zones.rs`, `checkin.rs`, `security.rs`, `gates.rs` in Grundform grün; Coverage Kern ≥ 85 %; neue Grafiken vom Projektinhaber abgenommen (`docs/stilleitfaden.md` Abschnitt 7).
- **Betroffene Module:** Personen, Welt & Bau, Wegfindung & Passagierführung, Takt & Weltzustand, Spielfeld, Oberfläche
- **Reifegrad-Wirkung:** Modul Personen → `[BELASTBAR]` im umgesetzten Umfang
- **Artefakte:** Kern-Code, Datenpaket-Inhalte, Tests

#### 3.11: Erkundung Suchen ohne Schilder und Feinwege

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Spike mit Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 3.2
- **Freigabepflichtig:** ja, falls das Ergebnis ADR-004 oder ADR-010 berührt (Kategorie 1)
- **Empfohlene Klasse:** Entscheidung – kann die Grundannahme aus ADR-004 berühren (Aufwand nach Zahl der Ziele, nicht der Personen).
- **Eingangskriterien:** Passagierablauf aus 3.2
- **Anforderungen (ab Klasse M):** FR-017, FR-018
- **Zu tun:** Modell für „Suchen ohne Schilder“ und für Feinwege zum Einzelziel (Sitzplatz, Schalter; aus 1.2); `MAX_TARGETS = 255` (`core/src/target.rs`) und Speicher je Ziel gegen ein realistisch bestücktes kleines Terminal (Check-in, Toiletten, Gates, Läden, Arbeitsplätze des Personals nach ADR-037) prüfen.
- **Akzeptanzkriterien:** Wir wissen, wie Suchen und Feinwege funktionieren und was sie bei 10.000 Personen kosten; Ergebnis als ADR oder begründet ohne Änderung.
- **Betroffene Module:** Wegfindung & Passagierführung, Personen
- **Reifegrad-Wirkung:** Wegfindung & Passagierführung: offene Fragen geschlossen
- **Artefakte:** Messnotiz, ggf. ADR

#### 3.3: Passagierführung und erklärbare Wegprobleme

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.11
- **Freigabepflichtig:** nein, sofern 3.11 ohne Architekturänderung endet
- **Empfohlene Klasse:** Routine – Umsetzung nach dem Ergebnis von 3.11.
- **Eingangskriterien:** 3.11 entschieden
- **Anforderungen (ab Klasse M):** FR-017, FR-018, FR-043
- **Zu tun:** Schilder und Bodenmarkierungen als Kostenfaktoren der Wegweiser; Suchen ohne Schilder nach 3.11; Ursache der Unerreichbarkeit (Zonenregel, eingeschlossen, fehlende Station; nach ADR-037 zusätzlich Personalzone, kein Gehweg oder keine Querung, nur über Bahn in Betrieb) für den Spieler.
- **Akzeptanzkriterien:** `core/tests/signage.rs` (mit Beschilderung messbar kürzere Wegzeit) und `core/tests/path_diagnostics.rs` grün; Benchmark Wegweiser unverändert in seinen Grenzen.
- **Betroffene Module:** Wegfindung & Passagierführung, Personen, Spielfeld, Oberfläche
- **Reifegrad-Wirkung:** Modul Wegfindung & Passagierführung → `[BELASTBAR]`
- **Artefakte:** Kern-Code, Overlays, Tests

#### 3.12: Erkundung Blockadevermeidung

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie mit Spike
- **Zeitbox (nur ERKUNDUNG):** maximal zwei Sessions
- **Abhängigkeiten:** 3.7
- **Freigabepflichtig:** ja (Kategorie 1; Ergebnis per ADR in Ergänzung von ADR-005)
- **Empfohlene Klasse:** Entscheidung – Architekturentscheidung zum Verkehr.
- **Eingangskriterien:** Wegenetze im Weltzustand (2.3), Querungen aus 3.7
- **Anforderungen (ab Klasse M):** FR-014, FR-028
- **Zu tun:** Verfahren, mit dem Straßen- und Rollverkehr ohne Stillstand bleibt, der sich nicht selbst auflöst (Reservierung von Abschnitten, kein Einfahren in nicht räumbare Kreuzungen, Vorrang-Rangfolge nach ADR-005); Querungen im Wechsel nach ADR-037 (Personen warten bei belegtem oder reserviertem Abschnitt, Fahrzeuge bei Personen auf der Querung, Rang 1 sperrt neues Betreten); Wartegründe abfragbar; Kosten bei Fahrzeug- und Flugzeugzahlen des Referenzszenarios.
- **Akzeptanzkriterien:** Verfahren per ADR entschieden, im Spike mit einem Notfall-Szenario ohne Blockade belegt.
- **Betroffene Module:** Verkehr & Verkehrssteuerung
- **Reifegrad-Wirkung:** Verkehr, Blockadevermeidung `[VORLÄUFIG]` → `[BELASTBAR]` per ADR
- **Artefakte:** Spike, Messnotiz, ADR

#### 3.13: Schrittzeit-Spitzen der Simulation auf Apple-Chips

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG (vorgezogen aus 4.3, ADR-036)
- **Schritt-Art (nur ERKUNDUNG):** Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session (Messläufe auf dem Mac des Projektinhabers)
- **Abhängigkeiten:** keine; vor 3.4 (Fahrzeuge erhöhen die Last)
- **Freigabepflichtig:** ja, falls die Maßnahme die Threadaufteilung oder die Betriebsart berührt (Kategorie 1)
- **Empfohlene Klasse:** Entscheidung – Bewertung einer Leistungsmessung (ADR-034).
- **Eingangskriterien:** Mac des Projektinhabers verfügbar
- **Anforderungen (ab Klasse M):** – (NFR Performance)
- **Zu tun:** Auf dem Mac M3 Pro lag das Sekunden-p99 in 11 von 20 Fenstern über 4 ms, Einzelschritte bis 75 ms (2.7, 2.10); mit 4 Hintergrund-Threads gemildert, nicht beseitigt. Ursache klären: Verteilung auf Leistungs- und Effizienzkerne, Priorität der Tick-Threads, Speicherbereinigung im Renderer; die langen Aufgaben im Renderer aus Mac-Lauf 1 mitprüfen.
- **Akzeptanzkriterien:** Ursache belegt oder eingegrenzt; Ergebnis Maßnahme oder ADR.
- **Betroffene Module:** Takt & Weltzustand, Benchmark & Leistung
- **Reifegrad-Wirkung:** keine, außer per ADR
- **Artefakte:** Messprotokoll, ggf. ADR

#### 3.4: Verkehr, Bahn und Rollwege, Walk-out

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.12, 3.13
- **Freigabepflichtig:** ja, für das Schema der Inhaltsart `vehicles/` (Kategorie 5, additiv nach Regel-008 falls rückwärtskompatibel) und für S-1 Layout 2 nur, wenn es von ADR-038 abweicht
- **Empfohlene Klasse:** Entscheidung – größter Umsetzungsschritt der Phase mit neuen Fachmodulen; Teilarbeiten Routine.
- **Eingangskriterien:** Blockadevermeidung `[BELASTBAR]` (3.12); S-1 Layout 2 entschieden (3.8)
- **Anforderungen (ab Klasse M):** FR-014, FR-027, FR-028, FR-052
- **Zu tun:** Straßenverkehr, Bahn und Rollwege mit Knoten, Verkehrssteuerung; Vorfeld-Gehwege zu Stellplätzen ohne Fluggastbrücke und Walk-out-Boarding (ADR-020, ADR-036, FR-052); Querungen nach 3.7 (Querungszustand im Verkehr, Flugzeugtür am Ende des Gehwegs als Ziel der Fluggäste, ADR-037); Nachtsprung bei Flugzeugen in Bewegung sperren (`TODO(fahrplan-ref: 3.4)` aus 2.2, META-08); Fahrzeuge im Spielstand mit der Höchstzahl `MAX_VEHICLES` (aus 2.5); Schema der Inhaltsart `vehicles/` in Datenpaketen (aus 2.4, ADR-025); Abschnitte Fahrzeuge und Flugzeuge in S-1 Layout 2 (ADR-038).
- **Akzeptanzkriterien:** `core/tests/road_traffic.rs`, `airside.rs`, `traffic_control.rs` und `walk_out.rs` grün; 1 h Spielzeit ohne Dauerstau; Benchmarks in ihren Grenzen; Coverage Kern ≥ 85 %; neue Grafiken vom Projektinhaber abgenommen (`docs/stilleitfaden.md` Abschnitt 7).
- **Betroffene Module:** Verkehr & Verkehrssteuerung, Flugbetrieb, Welt & Bau, Personen, Brücke, Spielfeld, Inhalte & Mods
- **Reifegrad-Wirkung:** Modul Verkehr & Verkehrssteuerung → `[BELASTBAR]` im umgesetzten Umfang
- **Artefakte:** Kern-Code, Schema, Tests

#### 3.5: Abfertigung und Gepäck in Grundform

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.4
- **Freigabepflichtig:** nein, sofern keine neue Schnittstelle oder Abhängigkeit
- **Empfohlene Klasse:** Routine – Umsetzung nach Inventar.
- **Eingangskriterien:** 3.4 erledigt
- **Anforderungen (ab Klasse M):** FR-025, FR-032
- **Zu tun:** Turnaround-Services und Gepäckanlage in Grundform; Wege des Vorfeldpersonals nach 3.7 (ADR-037: zu Fuß über Stellplätze, Gehwege und Querungen, sonst per Servicewagen, Erklärung bei Unerreichbarkeit); Abschnitt Gepäck in S-1 Layout 2 (ADR-038); Inventar-IDs nach Regel-004.
- **Akzeptanzkriterien:** `core/tests/baggage.rs` und `core/tests/turnaround.rs` in Grundform grün; Coverage Kern ≥ 85 %; neue Grafiken vom Projektinhaber abgenommen (`docs/stilleitfaden.md` Abschnitt 7).
- **Betroffene Module:** Abfertigung & Gepäck, Personen, Verkehr & Verkehrssteuerung, Spielfeld
- **Reifegrad-Wirkung:** Modul Abfertigung & Gepäck → `[BELASTBAR]` im umgesetzten Umfang
- **Artefakte:** Kern-Code, Tests

#### 3.6: Flüge, Flugplaner, Geld und Bewertung in Grundform

- **Status:** OFFEN
- **Phasentyp-Kontext:** UMSETZUNG
- **Abhängigkeiten:** 3.5
- **Freigabepflichtig:** ja, für die Schemas `aircraft/` und `airlines/` (Kategorie 5, additiv nach Regel-008 falls rückwärtskompatibel) und für die Darstellung der Lackierungen (Kategorie 1 und 4: Einfärben im Spielfeld, Lackierung als Inhaltsart; Erweiterung von `docs/stilleitfaden.md` um Airline-Farben und Logos)
- **Empfohlene Klasse:** Routine – Umsetzung nach Inventar; Entscheidung bei Vertragsfragen und beim Entwurf der Lackierungen (Architektur und Datenmodell).
- **Eingangskriterien:** 3.5 erledigt
- **Anforderungen (ab Klasse M):** FR-030, FR-036, FR-039, FR-044 (Airline und Flugzeugtyp aus dem Mod-Ordner)
- **Zu tun:** GA und kleine Linienflüge (Propellermaschinen, Szenario „Einsteiger“), Flugplaner als große Tabelle über S-7 mit Sortierung im Kern, Geld und Bewertung in Grundform; Wörterbuch- und Detail-Texte als `{ key }`/`{ text }`, `tick` im Detail (ADR-032, L-6); Schema der Inhaltsarten `aircraft/` und `airlines/` (aus 2.4, ADR-025). **Lackierungen** (AIR-01, MOD-01; Antwort des Projektinhabers vom 2026-10-21): Entwurf als `ENTSCHEIDUNG ERFORDERLICH` vor dem Code. Ausgangshypothese: je Flugzeugtyp eine Grundgrafik mit markierten Flächen für Airline-Farbe 1 und 2 (Seitenleitwerk, Triebwerke, Flügelspitzen, Rumpfstreifen), je Airline Farben und ein Logo als eigene SVG-Grafik auf dem Rumpfdach, Einfärben und Logo beim Zeichnen im Spielfeld statt vorgerenderter Grafiken je Airline und Typ (Speicher der Atlanten). Mehrere Lackierungen je Typ und Airline, seltene Lackierungen. Nur erfundene Airlines und Logos im Basispaket (Markenrecht, `docs/vision.md` Risiken). Stilleitfaden: Airline-Farben außerhalb der Palette, Logos mit Schriftzug als Ausnahme von „kein Text in Grafiken“. Abnahme der Grafiken nach `docs/stilleitfaden.md` Abschnitt 7.
- **Akzeptanzkriterien:** `core/tests/flights.rs` und `core/tests/economy.rs` in Grundform grün; `app/tables.bench.ts`: 10.000 Zeilen sortieren und filtern < 100 ms (FR-039); Rauchtest lädt Airline und Flugzeugtyp aus dem Mod-Ordner; Szenario „Einsteiger“ spielbar (Abschlusskriterium der Phase, Spieltest des Projektinhabers); Lackierungen per ADR entschieden, zwei Airlines mit unterschiedlicher Lackierung auf demselben Flugzeugtyp im Spielfeld unterscheidbar in allen drei Atlas-Stufen, Airline aus dem Mod-Ordner mit eigener Lackierung im Rauchtest; neue Grafiken vom Projektinhaber abgenommen (`docs/stilleitfaden.md` Abschnitt 7).
- **Betroffene Module:** Flugbetrieb, Wirtschaft & Management, Oberfläche, Inhalte & Mods, Brücke, Spielfeld
- **Reifegrad-Wirkung:** Module Flugbetrieb und Oberfläche → `[BELASTBAR]` im umgesetzten Umfang
- **Artefakte:** Kern-Code, Schemas, Flugplaner, Tests, ADR zu den Lackierungen, Grundgrafiken und Airline-Logos

### Phase 4: Härtung des vertikalen Schnitts – Typ: STABILISIERUNG

**Ziel:** Qualität und Leistung des vertikalen Schnitts sichern, bevor in die Breite gebaut wird.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 4.1 | Referenzszenario baubar, Leistungstest als CI-Gate; Messung auf echter 8-Kern-Hardware (Auflage ADR-014); Größe des Weltzustands im Spielstand messen und die Grenzen von S-5 (512 MB, ADR-030) prüfen; Neuberechnung der Wegweiser mit den Zielen aller Personengruppen (ADR-037); Budget des Welt-Abschnitts je Bild (1 MB) und Speicher der Weltdarstellung im Referenzszenario (ADR-038) | FR-040, FR-016 |
| 4.2 | Coverage auf Mindestwerte, Langzeittest, erste Funktionsgleichheits-Prüfung der umgesetzten Inventar-IDs | – |
| 4.3 | Aufgegangen in 3.13 (vorgezogen vor 3.4, ADR-036) | – |

### Phase 5: Ausbau in die Breite – Typ: UMSETZUNG

**Ziel:** Funktionsumfang des Originals mit den Kern-Modernisierungen abbilden. Wird bei Phasenbeginn in Teilphasen oder Teil-Fahrpläne aufgeteilt; vor Verkehr und Gepäck jeweils eine Erkundung.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 5.1 | Spielstart vollständig: Standort, Kartengröße, Sandbox, Einstellungen (`game.new` vollständig, ADR-032, L-3); lokalisierte Meldung bei `incompatible` statt Konsolenzeile (ADR-032, R-8); Flughafenname im Weltzustand und in den Spielstand-Metadaten (META-02, Feld leer seit 2.5); Spielmenü mit Speichern, Laden und Spielstand-Liste (ersetzt die einfache Liste aus 2.5); Einstellungs-Menü (ersetzt das minimale Panel aus 2.6) mit der Grafik-Einstellung „alle Einzelheiten in der Übersicht“ (ADR-038, Voreinstellung vereinfacht), dabei Obergrenze des Autosave-Intervalls (60 Minuten, Schätzung aus 2.6, ADR-031) überprüfen | FR-001, FR-002, FR-005 |
| 5.2 | Etagen, Zonen (international), Bauen vollständig | FR-010, FR-012 |
| 5.3 | Landseite: Parken, ÖPNV, Kreuzungen mit wählbarer Vorfahrt | FR-013, FR-015 |
| 5.4 | Terminal vollständig: moderner Check-in, Kontrollspuren, Grenze, Läden, Info | FR-019 bis FR-024 |
| 5.5 | Airside, Gepäck mit dreistufiger Kontrolle, Flüge, Abfertigung vollständig, Startfreigabe | FR-025, FR-026, FR-027, FR-029, FR-030, FR-032 |
| 5.6 | Personal, Wirtschaft, Forschung, Serviceziele, Notfälle, Störungen, Tutorial, Erfolge mit Vermerk der Debug-Nutzung im Spielstand (META-15, ADR-032, L-7); große Tabellen für Personal und Finanzen; Schema der Inhaltsart `research/` in Datenpaketen (aus 2.4, ADR-025); Aufgabenfelder für wechselnde Einzelziele des Personals (Technik, Reinigung) mit Vermeiden doppelter Zuteilung (ADR-037) | FR-007, FR-008, FR-033 bis FR-039 |
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

### Phase 2 (2026-10-19, ADR-036; zugleich Reflexion zur Reaktiv-Quote für Phase 2 nach Regel-009)

- **Gelernt:** Das Fundament trägt mit großer Reserve: Schritt mit 10.000 Personen 0,13–0,56 ms, Bild an der Grenze des Bildschirms auf Referenz-PC und Mac, Spielstand bitgleich nach Laden. Die getrennten Prüfungen fanden in jedem Schritt echte Mängel (2.4: 14, 2.5: 12, 2.6: 7 Befunde). Die Sammelprüfung 2.11 fand 10 Vertragslücken auf einmal statt einzeln.
- **Gekippte Annahmen:**
  - Verträge, die am Schreibtisch je Schnittstelle entworfen sind, tragen das Gebaute nicht vollständig (ADR-026 bis ADR-032).
  - „Kern-Module über Weltzustand und interne Ereignisse“ ist noch nicht gebaut (Entwurf 3.9).
  - Die Neuberechnung im Hintergrund genügt bei Dauerbau nicht ohne Zusammenfassen der Anfragen (3.1).
- **Reaktiv-Quote (6 / 10 nach ADR-036):**
  - Muster in Phase 2: additive Ergänzungen oder Konstanten an Verträgen aus 2.9 (ADR-026, ADR-028 bis ADR-032) und Verlegung von Landeplätzen (ADR-036).
  - Kein reaktiver ADR änderte Code in Takt, Welt & Bau, Wegfindung oder Brücke; keine Modulgrenze hat sich verschoben.
  - Maßnahme: Regel-008 (additive Ergänzungen ohne ADR, `CLAUDE.md` Abschnitt 4 Nr. 5). Regel-009 regelt Reflexionen künftig.
  - Die Zählregel aus `CLAUDE.md` Abschnitt 6 bleibt unverändert; Rückmeldung an das Vorlagen-Repo, dass sie formal reaktive Fortschreibungen nicht von Strukturmängeln unterscheidet.
- **Neue Erkundungen:** 3.11 Suchen ohne Schilder und Feinwege, 3.12 Blockadevermeidung, 3.13 Schrittzeit-Spitzen auf Apple-Chips; Entwurf 3.9 Kernstruktur.

### Ergänzung Reaktiv-Quote (2026-10-20, Anlass ADR-037; neues Muster nach Regel-009)

- **Befund:** 7 / 10 `[REAKTIV]` (ADR-028, ADR-029, ADR-030, ADR-031, ADR-032, ADR-036, ADR-037) über ADR-028 bis ADR-037, 70 %.
- **Ursache von ADR-037:** formal die 14-Tage-Regel (`CLAUDE.md` Abschnitt 6). ADR-037 ändert 7 Tage nach ADR-020 dessen Satz „Personen betreten Vorfeld, Rollwege und Bahnen nicht“. Inhaltlich war die Frage mit dem Entwurfsschritt 3.7 geplant (ADR-036, `docs/research/phasenabschluss-2.md`). Der Satz in ADR-020 galt aber ohne Einschränkung für alle Personen, bevor die Gruppen entworfen waren.
- **Neues Muster nach Regel-009:** ADR-037 führt zu Code in Welt & Bau (Flächenart, Personalzone, Profil am Ziel) und Wegfindung (Regeln je Profil). Deshalb neue Reflexion mit Stopp statt Verweis auf die Phasen-Reflexion.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). Die Änderung liegt hinter derselben Schnittstelle „Richtung je Ziel und Zelle“ (ADR-010). Raster und Wegweiser bekommen Merkmale und Regeln, keine neue Struktur. Bisher hängt kein Code davon ab, die Umsetzung beginnt in 3.1.
- **Muster:** wie bei ADR-020 selbst (Maßnahme 5): eine Festlegung, deren Geltungsbereich noch nicht bestimmt war – damals die Einheit, jetzt die Gruppe.
- **Maßnahme (10):** Eine Festlegung über „Personen“ oder „Fahrzeuge“ nennt die Gruppen, für die sie gilt. Ist die Gruppenfrage offen, gilt sie nur für die genannten Gruppen und trägt einen Landeplatz für die übrigen.
- **Ausblick:** Sind die folgenden ADRs nicht reaktiv, sinkt die Quote auf 6 / 10 ab ADR-038 und auf 2 / 10 (Schwellenwert eingehalten) ab ADR-042; ADR-036 und ADR-037 fallen ab ADR-046 bzw. ADR-047 aus dem Fenster.
- **Kenntnisnahme:** Projektinhaber am 2026-10-20 per Auswahl: „Zur Kenntnis genommen“; kein Umbau, Maßnahme (10) gilt.

### Ergänzung Reaktiv-Quote (2026-10-20, Anlass ADR-038; neues Muster nach Regel-009)

- **Befund:** 7 / 10 `[REAKTIV]` (ADR-029, ADR-030, ADR-031, ADR-032, ADR-036, ADR-037, ADR-038) über ADR-029 bis ADR-038, 70 %.
- **Ursache von ADR-038:** formal die 14-Tage-Regel (`CLAUDE.md` Abschnitt 6): S-1 aus ADR-021 (7 Tage) und ADR-032 (1 Tag) fortgeschrieben. Inhaltlich der geplante Entwurfsschritt 3.8 (ADR-032 L-1, ADR-036).
- **Neues Muster nach Regel-009:** ADR-038 führt zu Code in der Brücke (Layout 2, Welt-Abschnitt). Deshalb neue Reflexion mit Stopp.
- **Architektur-Refactoring nötig?** Nein (Einschätzung der bauenden KI). Der Weg nutzt die bestehende Beziehung Zustandspuffer → Spielfeld; das Raster liefert seine geteilten Blöcke wie für die Wegweiser; Kern-Module bleiben unberührt.
- **Muster:** wie ADR-032 und ADR-037: ein geplanter Entwurf schreibt einen jungen Vertrag fort.
- **Lehre aus dem Messaufbau:** Der Fehler im W-B-Prototyp (Abfrage mit der alten Sitzung) fiel im Container nur durch Zufall nicht auf und kostete einen Messlauf auf dem Mac und neue Pakete.
- **Maßnahme (11):** Ein Messprototyp mit neuem Nachrichtenweg wird im Container mit mindestens drei Sitzungswechseln geprüft, bevor Pakete an den Projektinhaber gehen.
- **Ausblick:** Sind die folgenden ADRs nicht reaktiv, sinkt die Quote auf 6 / 10 ab ADR-039 und auf 2 / 10 (Schwellenwert eingehalten) ab ADR-046; ADR-038 fällt ab ADR-048 aus dem Fenster.
- **Kenntnisnahme:** Projektinhaber am 2026-10-20 per Auswahl: „Kenntnis + Maßnahme 11“; kein Umbau, Maßnahme (11) gilt.

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- Schritte **ohne Abhängigkeiten zueinander**: 2.1 parallel zu 2.8; 2.4 parallel zu 2.5 und 2.6 (nach 2.7); Q.1 und Q.2 unabhängig von allen Phasen
- Schritte **mit gemeinsamen Modulen** (Konfliktgefahr): 2.2 und 2.3 (Takt & Weltzustand); 2.5 und 2.7 (Plattform)

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-10-20 – Schritt 3.8 abgeschlossen (ADR-038): neuer Erkundungsschritt 3.14 (späte Bilder auf dem Referenz-PC) vor 3.9; Phase 3 jetzt 14 Schritte bei ursprünglichem Plan 13; Landeplätze in 3.1, 3.2, 3.4, 3.5, 4.1, 5.1
- 2026-10-19 – Phasenabschluss 2 (ADR-036): Phase 2 archiviert; Phase 3 im vollen Format mit ursprünglichem Schrittplan von 13 Schritten, neu 3.9 Kernstruktur, 3.10 Aufteilung `main.ts`, 3.11 und 3.12 als Erkundungen mit eigener ID (bisher „davor Erkundung“ in 3.3 und 3.4), 3.13 aus 4.3 vorgezogen; 3.8 um S-1 Layout 2, 3.1 um Dauerbau, 3.2 um „kein Stillstand“, 3.4 um Walk-out (FR-052), 4.1 um Wiederholung von FR-016 ergänzt
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
