# Requirements

<!-- Anforderungen von OpenAirside: was das Spiel können muss, für wen, mit welcher Priorität.
     Klasse G: befüllt in Modus 1.5 (templates/projektstart.md, Schritt 1a) am 2026-10-13.
     Abschnitt 1 („Übersicht") gehört zur Mindest-Lektüre (CLAUDE.md Abschnitt 2).
     Maßstab für den Funktionsumfang des Originals: docs/funktionsinventar.md (IDs wie CHK-01).
     Entschiedene Abweichungen vom Original: docs/modernisierung.md (IDs wie MOD-05).
     Die Anforderungen bündeln Inventar-Einträge je Spielbereich; die Akzeptanz verweist auf
     die Inventar-IDs, damit nichts doppelt gepflegt wird. -->

<!-- ANCHOR:uebersicht -->
## 1. Übersicht (Stand vom 2026-10-13)

| Kennzahl | Wert |
|---|---|
| Anforderungen gesamt | 51 |
| davon Muss / Soll / Kann | 43 / 5 / 3 |
| Muss-Anforderungen ohne Fahrplan-Schritt | 0 |
| Muss-Anforderungen ohne Test (Pflicht ab Klasse G) | 0 (Testpfade geplant, entstehen mit dem Schritt) |
| Verworfen (mit ADR) | 0 |

**Offene Punkte:** Genaue Formeln und Werte des Originals sind teils unbekannt (`docs/funktionsinventar.md` Abschnitt 18); sie werden in den Schritten der Phasen 3 und 5 eigenständig festgelegt, nicht nachgerechnet.

<!-- ANCHOR:beteiligte -->
## 2. Beteiligte

| Rolle / Gruppe | Interesse am System | Einfluss auf Entscheidungen | Betroffenheit | Wie wird sie gehört? |
|---|---|---|---|---|
| Projektinhaber (Auftraggeber, Visionär) | Machbarkeitstest: lässt sich ein vollständiges Aufbauspiel mit KI-Unterstützung bauen; eigener Spielspaß | hoch, entscheidet alle Freigaben | täglich | direkt in den Sessions |
| KI-Entwicklung (Coding-Assistent) | durchspezifizierte Schritte, prüfbare Akzeptanz | mittel, schlägt vor, entscheidet Umsetzungsdetails | täglich | über Dokumente und Freigaben |
| Spieler großer Flughäfen | stundenlanges Bauen ohne Ruckeln, Passagiere und Fahrzeuge kommen an | mittel, über Rückmeldungen | Hauptnutzer | GitHub-Issues, Live-Tests ab Phase 3 |
| Einsteiger | verständlicher Start mit kleinem Flughafen | gering | Nebennutzer | GitHub-Issues, Tutorial-Tests |
| Spieler mit schwächerem Rechner | ehrliche Empfehlung, spielbar auf integrierter Grafik | gering | Nebennutzer | Benchmark-Ergebnisse in Issues |
| Mod- und Inhalte-Ersteller | stabiles, dokumentiertes Datenformat für Airlines, Flugzeuge, Objekte, Grafiken | mittel ab Phase 6 | gelegentlich | GitHub-Issues, Beispiel-Mods |
| Übersetzer | Sprachpakete ohne Code-Änderung | gering | gelegentlich | GitHub-Issues |

<!-- ANCHOR:anwendungsfaelle -->
## 3. Anwendungsfälle

### Enthalten

| ID | Wer | Was | Quelle |
|---|---|---|---|
| UC-001 | Spieler | Neues Spiel anlegen: Standort, Name, Kartengröße, Schwierigkeit, Sandbox | `docs/vision.md` Abschnitt 3; Inventar Abschnitt 1 |
| UC-002 | Spieler | Flughafen bauen und umbauen: Gebäude, Etagen, Räume, Zonen, Objekte, Straßen | Inventar Abschnitte 2, 3, 5 |
| UC-003 | Spieler | Flugbetrieb einrichten: Bahnen, Rollwege, Stellplätze, Flugsicherung | Inventar Abschnitt 11 |
| UC-004 | Spieler | Flüge mit Airlines verhandeln und planen | Inventar Abschnitt 12 |
| UC-005 | Spieler | Passagiere abfertigen: Anreise, Check-in, Sicherheit, Grenze, Gate, Boarding, Ankunft | Inventar Abschnitte 4–10; Szenario „Großflughafen-Bauer" |
| UC-006 | Spieler | Gepäck abfertigen: Bänder, Kontrolle, Sortierung, Ausgabe | Inventar Abschnitt 8 |
| UC-007 | Spieler | Flugzeuge abfertigen und versorgen (Turnaround, Betankung, Enteisung, Pushback) | Inventar Abschnitt 13 |
| UC-008 | Spieler | Personal, Wirtschaft, Verträge, Forschung und Ziele führen | Inventar Abschnitt 15 |
| UC-009 | Spieler | Notfälle, Wetter und Störungen bewältigen | Inventar Abschnitt 14 |
| UC-010 | Spieler | Großflughafen mit über 10.000 Personen flüssig betreiben | `docs/vision.md` Abschnitte 3 und 4 |
| UC-011 | Spieler | Verstehen, warum etwas nicht funktioniert (Wege, Leistung, Verkehr) | `docs/vision.md` Kernumfang |
| UC-012 | Spieler mit schwächerem Rechner | Benchmark ausführen und Kartengröße bewusst wählen | Szenario „schwächerer Rechner" |
| UC-013 | Einsteiger | Mit Tutorial einen kleinen Flughafen beginnen | Szenario „Einsteiger"; META-13 |
| UC-014 | Spieler | Spielstand speichern, laden, automatisch sichern | META-10 |
| UC-015 | Mod-Ersteller | Inhalte als Datenpaket erstellen und im Spiel aktivieren | `docs/vision.md` Kernumfang; Inventar Abschnitt 16 |
| UC-016 | Übersetzer | Sprachpaket für eine weitere Sprache erstellen | `docs/vision.md` Kernumfang |
| UC-017 | Spieler | Spiel unter Windows, Linux oder macOS von GitHub laden und starten | `docs/vision.md` Constraints |

### Bewusst ausgeschlossen

| Anwendungsfall | Begründung | Entschieden am | Wieder aufgreifen, wenn … |
|---|---|---|---|
| Kurven beim Bauen | Raster-Spielprinzip wie im Original; Kurven sprengen Bau- und Wegelogik | 2026-10-13 | nie geplant; nur bei neuer Vision |
| Spielstände oder Mods des Originals laden | kein Material und keine Formatbindung an das Original | 2026-10-13 | nicht vorgesehen |
| Vertrieb über Steam oder andere Stores | GitHub-only, betreiberneutral | 2026-10-13 | Projektinhaber entscheidet nach 1.0 |
| Fracht/Cargo spielbar | erst nach Funktionsgleichheit; Datenmodell bereitet es vor (FR-046) | 2026-10-13 | Phase 7 abgeschlossen |
| Mehrspieler | nicht Teil von Original und Vision | 2026-10-13 | Community-Wunsch nach 1.0 |
| Code-Mods (Skripte) | Sicherheits- und Stabilitätsfrage offen (`docs/project-context.md` offene Fragen) | 2026-10-13 | Phase 6 |
| Isometrische 3D-Sicht | Kandidat, spätere Ausbaustufe | 2026-10-13 | nach 1.0 |
| Modernisierungen der Kategorie „Kandidat" (`docs/modernisierung.md`) | unverbindlich | 2026-10-13 | jeweils einzeln nach 1.0 |

<!-- ANCHOR:kernprozesse -->
## 4. Kernprozesse

„Ist" beschreibt das Original, „Soll" OpenAirside.

### Prozess: Abflug eines Passagiers

- **Ist:** 1. Anreise mit Auto, Bus, Taxi oder Bahn. 2. Check-in am fest zugeordneten Schalter oder Automaten. 3. Sicherheitskontrolle in Größenstufen. 4. Bei Auslandsflug Passkontrolle. 5. Läden, Gastronomie, Toiletten. 6. Boarding am Gate. Wegsuche je Person; bei großen Terminals Stau und Ruckeln.
- **Soll:** 1. Anreise wie Original, Parkhäuser mit Ebenen. 2. Online-Check-in-Anteil, nur Gepäckabgabe oder Self Bag Drop, gemeinsam genutzte Schalter (MOD-01–03). 3. Zentrale Kontrolle mit Spuren und Technikstufen, Nachkontrolle (MOD-05–07). 4. Moderne Grenzkontrolle mit E-Gates (MOD-10). 5. Walk-through-Duty-Free als Vorteil (MOD-16). 6. Boarding mit Self-Boarding (MOD-12). Wege über Wegweiser und Beschilderung, auch mit über 10.000 Personen flüssig.
- **Betroffene Anwendungsfälle:** UC-005, UC-010, UC-011

### Prozess: Ankunft eines Passagiers

- **Ist:** 1. Aussteigen über Brücke oder Bus. 2. Passkontrolle bei Auslandsflug, Zoll. 3. Gepäckausgabe. 4. Abreise.
- **Soll:** wie Original; Trennung von Ankunft und Abflug freiwillig; Wege über Wegweiser.
- **Betroffene Anwendungsfälle:** UC-005, UC-010

### Prozess: Gepäck

- **Ist:** 1. Abgabe am Check-in. 2. Bänder zur Kontrolle (Stufe I, Spezialscanner in Reihe, manuelle Station). 3. Sortierung zum Stellplatz. 4. Verladung. Ankunft umgekehrt bis zur Ausgabe.
- **Soll:** 1. Abgabe am Schalter oder Self Bag Drop. 2. Dreistufige Kontrolle: CT-Anlage, Bildauswertung, Handkontrolle (MOD-22). 3–4. wie Original.
- **Betroffene Anwendungsfälle:** UC-006

### Prozess: Flugzeug-Umlauf

- **Ist:** 1. Anflug, Landung. 2. Rollen über vom Spieler gezogene Rollwege. 3. Abfertigung in fester Reihenfolge. 4. Pushback, Rollen, Start, sobald die Abfertigung fertig ist; Abflug wartet, bis die Bahn ganz frei ist. Fahrzeuge blockieren sich an Kreuzungen.
- **Soll:** 1–3. wie Original. 4. Startfreigabe-Zeit am Gate (MOD-41), „Aufrollen und Warten" (MOD-42). Durchgehend automatische Verkehrssteuerung mit Vorrangregeln nach ADR-005, ohne Blockaden.
- **Betroffene Anwendungsfälle:** UC-003, UC-007, UC-009

### Prozess: Flughafen planen und erweitern

- **Ist:** Bauen im Raster, Bauauftrag wird von Bauarbeitern umgesetzt; bei großen Flughäfen werden Bauaufträge erst nach Sekunden übernommen.
- **Soll:** wie Original, Bauaufträge sofort sichtbar übernommen; bei fehlenden Wegen erklärt das Spiel die Ursache.
- **Betroffene Anwendungsfälle:** UC-002, UC-011

<!-- ANCHOR:funktionale-anforderungen -->
## 5. Funktionale Anforderungen

Spalte „Fahrplan-Schritt": Schritt-ID aus `docs/fahrplan.md`. Spalte „Test": geplanter Testort; `core/` = Rust-Kern (cargo test), `app/` = Präsentation (Vitest), `e2e/` = Playwright.

### Grundlagen und Meta

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-001 | Neues Spiel mit Standort, Name/IATA/Logo, Kartengröße, Schwierigkeit | UC-001 | Muss | META-01 bis META-04 und META-06/07 abgebildet | 5.1 | `core/tests/new_game.rs` | OFFEN |
| FR-002 | Sandbox-Modus mit Regel-Schaltern | UC-001 | Muss | META-05, META-12 abgebildet; Sandbox deaktiviert Erfolge | 5.1 | `core/tests/sandbox.rs` | OFFEN |
| FR-003 | Zeitsteuerung mit Pause, 1×–3×, Nachtsprung und Kalender | UC-010 | Muss | META-08, META-09; bei 3× kein Simulationsrückstand im Referenzszenario | 2.2, 4.1 | `core/tests/time.rs` | OFFEN (Zeitsteuerung, Nachtsprung und Kalender seit 2.2 im Kern; Rückstand im Referenzszenario misst 4.1) |
| FR-004 | Spielstände: manuell, Schnellspeichern, Autosave mit Rotation, Vorschaubild | UC-014 | Muss | META-10; Laden ergibt identischen Zustand (Prüfsumme) | 2.5 | `core/tests/savegame_roundtrip.rs`, `e2e/smoke.spec.ts` | ERLEDIGT 2026-10-19 (Intervall und Zahl der Autosaves einstellbar seit 2.6) |
| FR-005 | Einstellungen: Einheiten, Währung, Uhr, Passagieranteil, Steuerung | UC-001 | Muss | META-11 abgebildet | 5.1 | `app/settings.test.ts` | OFFEN |
| FR-006 | Oberfläche Deutsch und Englisch, weitere über Sprachpakete | UC-016 | Muss | Sprachwechsel ohne Neustart; Sprachpaket aus Ordner wird erkannt | 2.6 | `app/renderer/i18n.test.ts`, `core/tests/lang.rs`, `e2e/smoke.spec.ts` | ERLEDIGT 2026-10-19 (2.6) |
| FR-007 | Tutorial als Aufgabenkette | UC-013 | Muss | META-13; Einsteiger-Szenario durchspielbar | 5.6 | `e2e/tutorial.spec.ts` | OFFEN |
| FR-008 | Erfolge mit Sperre bei Sandbox und Konsole | UC-008 | Soll | ACH-01 sinngemäß, eigene Liste | 5.6 | `core/tests/achievements.rs` | OFFEN |
| FR-009 | Konsole/Debug-Panel | UC-011 | Soll | META-15 | 2.7 | `app/renderer/console.test.ts` | ERLEDIGT 2026-10-15 (2.7) |

### Bauen, Zonen, Landseite

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-010 | Bausystem im Raster mit Etagen, Fundamenten, Wänden, Objekten, Werkzeugen | UC-002 | Muss | Inventar Abschnitt 2 (BAU-*) abgebildet; keine Kurven | 3.1, 5.2 | `core/tests/build.rs` | OFFEN |
| FR-011 | Bauaufträge mit Bauarbeitern, sofort sichtbar übernommen | UC-002 | Muss | Auftrag erscheint im nächsten Bild; im Referenzszenario < 100 ms bis zur Anzeige | 3.1 | `core/tests/construction.rs` | OFFEN |
| FR-012 | Zonen, Räume und Overlays | UC-002 | Muss | Inventar Abschnitt 3 (ZON-*) abgebildet | 3.2, 5.2 | `core/tests/zones.rs` | OFFEN |
| FR-013 | Landseite: Straßen, Parken (Parkhäuser mit Ebenen), ÖPNV, Taxis | UC-005 | Muss | Inventar Abschnitt 5 (LAND-*), MOD-21 | 5.3 | `core/tests/landside.rs` | OFFEN |
| FR-014 | Straßenverkehr nach festen Regeln ohne Blockade | UC-005 | Muss | eigene Spuren je Richtung, Abbiegen behindert Gegenverkehr nicht, kein Einfahren in nicht räumbare Kreuzung; 1 h Spielzeit Referenzszenario ohne Dauerstau | 3.4 | `core/tests/road_traffic.rs` | OFFEN |
| FR-015 | Vorfahrt automatisch, je Kreuzung manuell umschaltbar | UC-005 | Muss | Schilder, Stoppstellen, Ampeln wählbar | 5.3 | `core/tests/intersections.rs` | OFFEN |

### Passagiere und Terminal

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-016 | Passagiere mit Bedürfnissen, Zufriedenheit und Ablauf Abflug/Ankunft | UC-005 | Muss | Inventar Abschnitt 4 (PAX-*); Kernprozesse Abflug und Ankunft | 3.2 | `core/tests/pax_flow.rs` | OFFEN |
| FR-017 | Wegfindung nach Wegweiser-Prinzip | UC-010 | Muss | ADR-004; 10.000 Personen mit ≤ 4 ms je Simulationstakt auf Referenz-Hardware | 1.2, 2.3, 3.3 | `core/benches/flowfield.rs` | OFFEN (Wegweiser im Kern und Benchmark als CI-Gate seit 2.3: p99 0,38 ms mit 10.000 Personen auf dem GitHub-Runner; Passagierführung 3.3) |
| FR-018 | Passagierführung über Schilder und Bodenmarkierungen; ohne Schilder Suchen | UC-005 | Muss | mit Beschilderung messbar kürzere Wegzeit als ohne | 3.3 | `core/tests/signage.rs` | OFFEN |
| FR-019 | Check-in modern: Online-Anteil, Gepäckabgabe, Self Bag Drop, Common Use | UC-005 | Muss | Inventar Abschnitt 6 (CHK-*), MOD-01 bis MOD-03 | 3.2, 5.4 | `core/tests/checkin.rs` | OFFEN |
| FR-020 | Sicherheitskontrolle mit Spuren, Technikstufen und Nachkontrolle | UC-005 | Muss | Inventar Abschnitt 7 (SEC-*), MOD-05 bis MOD-07 | 3.2, 5.4 | `core/tests/security.rs` | OFFEN |
| FR-021 | Grenzkontrolle und Zoll mit E-Gates | UC-005 | Muss | MOD-10, SEC-03 | 5.4 | `core/tests/border.rs` | OFFEN |
| FR-022 | Gates, Boarding (inkl. Self-Boarding), Stellplätze, Brücken, Busse | UC-005 | Muss | Inventar Abschnitt 9 (GATE-*), MOD-12 | 3.2, 5.4 | `core/tests/gates.rs` | OFFEN |
| FR-023 | Läden, Gastronomie, Lounges, Toiletten, Franchises | UC-005 | Muss | Inventar Abschnitt 10, MOD-16 | 5.4 | `core/tests/commercial.rs` | OFFEN |
| FR-024 | Infoschalter und digitale Beschilderung mit spürbarer Wirkung | UC-005 | Muss | MOD-14; Gate-Wechsel erreicht Passagiere messbar schneller | 5.4 | `core/tests/info.rs` | OFFEN |

### Gepäck

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-025 | Gepäckbänder, Sortierung, Verladung, Ausgabe | UC-006 | Muss | Inventar Abschnitt 8 (GEP-*) | 3.5, 5.5 | `core/tests/baggage.rs` | OFFEN |
| FR-026 | Dreistufige Gepäckkontrolle | UC-006 | Muss | MOD-22 | 5.5 | `core/tests/baggage_screening.rs` | OFFEN |

### Flugbetrieb und Flüge

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-027 | Bahnen, Rollwege mit gezogenen Mittellinien, Stellplätze, Befeuerung, Flugsicherung | UC-003 | Muss | Inventar Abschnitt 11 (RWY-*, TWY-*, ATC-*) | 3.4, 5.5 | `core/tests/airside.rs` | OFFEN |
| FR-028 | Automatische Verkehrssteuerung mit Vorrangregeln nach ADR-005 | UC-003 | Muss | 7-stufige Rangfolge; Szenario-Test mit Notfall ohne Blockade; Entscheidungen für Spieler einsehbar | 3.4 | `core/tests/traffic_control.rs` | OFFEN |
| FR-029 | Startfreigabe-Zeit am Gate, „Aufrollen und Warten" | UC-003 | Muss | MOD-41, MOD-42 | 5.5 | `core/tests/departure_sequencing.rs` | OFFEN |
| FR-030 | Airlines, Verhandlungen, Verträge, Flugplanung | UC-004 | Muss | Inventar Abschnitt 12 (FLT-*, AIR-*) | 3.6, 5.5 | `core/tests/flights.rs` | OFFEN |
| FR-031 | Flugzeugtypen inklusive aller Add-on-Klassen (Vintage, Ost, Überschall, Hubschrauber) als Daten | UC-004 | Muss | ACFT-01 bis ACFT-05 mit eigenen Typen und Airlines | 5.7 | `core/tests/aircraft_data.rs` | OFFEN |

### Abfertigung, Notfälle, Umwelt

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-032 | Turnaround: Betankung, Catering, Reinigung, Enteisung, Pushback, Fahrzeuge | UC-007 | Muss | Inventar Abschnitt 13 (TURN-*, FUEL-*, ICE-*, CAT-*, CLEAN-*, PUSH-*, VEH-*) | 3.5, 5.5 | `core/tests/turnaround.rs` | OFFEN |
| FR-033 | Hangars, Notfälle, Wetter, Tiere | UC-009 | Muss | Inventar Abschnitt 14 (HAN-*, EMG-*, WX-*, ANI-*), MOD-46 | 5.6 | `core/tests/emergencies.rs` | OFFEN |
| FR-034 | Neue Störungen: Drohne, IT-Ausfall, Stromausfall | UC-009 | Muss | MOD-48, MOD-49 | 5.6 | `core/tests/disruptions.rs` | OFFEN |

### Management und Wirtschaft

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-035 | Personal mit Verträgen, Schichten, Zufriedenheit, Führungskräften | UC-008 | Muss | Inventar Abschnitt 15 (STAFF-*, EXEC-*) | 5.6 | `core/tests/staff.rs` | OFFEN |
| FR-036 | Wirtschaft: Einnahmen, Kosten, Kredite, Lieferanten, moderne Einnahmequellen | UC-008 | Muss | ECO-*, MOD-51 | 3.6, 5.6 | `core/tests/economy.rs` | OFFEN |
| FR-037 | Forschung, Ziele, Bewertungen, Post/E-Mail | UC-008 | Muss | RD-*, OPS-*, RATE-*, MAIL-* | 5.6 | `core/tests/progression.rs` | OFFEN |
| FR-038 | Messbare Serviceziele mit Rückzahlungen an Airlines | UC-008 | Muss | MOD-19 | 5.6 | `core/tests/service_levels.rs` | OFFEN |
| FR-039 | Große Tabellen-Panels (Flüge, Personal, Finanzen) bleiben bei Großflughafen flüssig | UC-010 | Muss | 10.000 Zeilen sortieren/filtern ohne spürbare Verzögerung (< 100 ms) | 1.4, 3.6, 5.6 | `app/tables.bench.ts` | OFFEN |

### Leistung, Erklärbarkeit, Plattform, Inhalte

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-040 | Leistungsziel Großflughafen | UC-010 | Muss | Referenzszenario (3 Terminals U-Form, 2 Bahnen, ~17 Gates, ≥ 10.000 Personen): 60 FPS bei Höchstgeschwindigkeit ohne Rückstand auf Referenz-Hardware | 1.5, 4.1, 7.2 | `e2e/perf_reference.spec.ts` | OFFEN |
| FR-041 | Benchmark mit unverbindlicher Kartengrößen-Empfehlung und Warnhinweis | UC-012 | Muss | Benchmark läuft im Zeitraffer; Empfehlung abweisbar; Warnung bei großer Karte | 5.8 | `e2e/benchmark.spec.ts` | OFFEN |
| FR-042 | Leistungsbewusstsein: Spiel erklärt, warum es an Grenzen stößt | UC-011 | Soll | Anzeige der teuersten Bereiche/Systeme | 5.8 | `app/perf_hints.test.ts` | OFFEN |
| FR-043 | Erklärbare Wegfindungsprobleme | UC-011 | Muss | Bei fehlendem Weg wird Ursache angezeigt (Zone, Verbindung, Station) | 3.3 | `core/tests/path_diagnostics.rs` | OFFEN |
| FR-044 | Inhalte als Datenpakete; Mods plattformneutral aktivierbar | UC-015 | Muss | Airline, Flugzeugtyp, Objekt aus Mod-Ordner im Spiel; Inventar MOD-01 sinngemäß ohne Workshop | 2.4, 3.6, 6.1 | `core/tests/datapacks.rs`, `e2e/smoke.spec.ts` | OFFEN (Objekt aus dem Mod-Ordner seit 2.4, 2026-10-18; Airline und Flugzeugtyp 3.6; Mod-Verwaltung 6.1) |
| FR-045 | Lauffähig auf Windows, Linux, macOS; Download von GitHub | UC-017 | Muss | CI baut und startet auf allen drei Plattformen; Release-Artefakte je Plattform | 1.6, 2.1, 7.2 | `e2e/smoke.spec.ts` in `.github/workflows/ci.yml` und `release.yml` (seit 2.1, x64 bzw. macOS arm64) | OFFEN (arm64 für Windows/Linux und v1.0.0-Builds in 7.2) |
| FR-046 | Datenmodell bereitet Fracht vor | UC-004 | Muss | Ladungsart „Fracht" im Schema vorhanden, ohne Spielmechanik | 2.3 | `core/tests/cargo_ready.rs` | ERLEDIGT 2026-10-14 (2.3) |
| FR-047 | Musik, Geräusche, Atmosphäre | UC-005 | Soll | AUDIO-01 sinngemäß, freie Bibliotheken | 6.2 | `app/audio.test.ts` | OFFEN |
| FR-048 | Fracht spielbar | UC-004 | Kann | nach 1.0 | – | – | OFFEN |
| FR-049 | Intro-/Menü-Video | UC-001 | Kann | – | 6.2 | – | OFFEN |
| FR-050 | Grafiken als eigene SVG, im Build gerastert | UC-002 | Soll | ADR-008; keine Grafik aus Original oder Mods | 2.10 | `scripts/assets.test.mjs`, `tools/assets` (cargo test), `e2e/smoke.spec.ts` | OFFEN (Pipeline seit 2.10 umgesetzt, ADR-033; erledigt mit dem Schritt nach der Messung) |
| FR-051 | Isometrische Sicht | UC-002 | Kann | spätere Ausbaustufe | – | – | OFFEN |

**Regeln:**

- IDs werden nie wiederverwendet, auch nicht nach Streichung.
- Eine Muss-Anforderung ohne Fahrplan-Schritt ist ein Bug (Drift-Prüfung, CLAUDE.md Abschnitt 16).
- Eine Muss-Anforderung zu streichen oder herabzustufen ist ein Descope: Status `VERWORFEN` mit ADR (CLAUDE.md Abschnitt 6, „Keine Verschiebung ohne Landeplatz").
- Neue Anforderungen während des Projekts bekommen Datum und Quelle (z. B. „Live-Test 2026-06-20").

<!-- ANCHOR:nicht-funktionale-anforderungen -->
## 6. Nicht-funktionale Anforderungen

Werte stehen ausschließlich in `docs/architecture.md` Abschnitt 6.

- **Leistung (60 FPS, kein Rückstand, ≥ 10.000 Personen):** besonders UC-010, UC-005, UC-002 (Bauaufträge), FR-017, FR-039, FR-040.
- **Plattformen:** UC-017, FR-045.
- **Untergrenze integrierte Grafik:** UC-012, FR-041.
- **Datenformat-Stabilität:** UC-014, UC-015.
