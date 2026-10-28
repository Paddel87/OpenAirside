# Requirements

<!-- Anforderungen von OpenAirside: was das Spiel können muss, für wen, mit welcher Priorität.
     Klasse G: befüllt in Modus 1.5 (templates/projektstart.md, Schritt 1a) am 2026-10-13;
     neu abgeleitet aus `docs/vision-2.md` nach ADR-046 am 2026-10-29.
     Abschnitt 1 („Übersicht") gehört zur Mindest-Lektüre (CLAUDE.md Abschnitt 2).
     Jede Spielfunktion nennt ihre Säule aus `docs/vision-2.md` (Spalte „Säule“, Regel-010):
     S1 Umbau im Betrieb, S2 Ströme lesen, S3 Rhythmus und Fracht, S4 Störung und Erholung,
     S5 Airlines als Partner, G Grundsatz (Lernen, Bedienkomfort, Erklären), B Basis
     (Genre-Grundlage, Leistung, Plattform, Inhalte). Die Akzeptanz ist aus sich heraus prüfbar;
     `funktionsinventar.md` und `modernisierung.md` sind nur Ideenquellen. -->

<!-- ANCHOR:uebersicht -->
## 1. Übersicht (Stand vom 2026-10-29)

| Kennzahl | Wert |
|---|---|
| Anforderungen gesamt | 68 |
| davon Muss / Soll / Kann | 55 / 9 / 3 (dazu 1 verworfen) |
| Muss-Anforderungen ohne Fahrplan-Schritt | 0 |
| Muss-Anforderungen ohne Test (Pflicht ab Klasse G) | 0 (Testpfade geplant, entstehen mit dem Schritt) |
| Verworfen (mit ADR) | 1 ganz (FR-037); teilweise oder herabgestuft: FR-031 (alte Fassung), FR-033 (Hangars, Tiere), FR-002 (Muss → Soll); alle ADR-046 |

**Offene Punkte:** Prioritäten der neuen Anforderungen FR-053 bis FR-068 sind ein Vorschlag der KI vom 2026-10-29 aus der Auswahl des Projektinhabers; Spielwerte (Gebühren, Zusagen, Störwirkung, Mindestumsteigezeit) werden in den Schritten eigenständig festgelegt.

<!-- ANCHOR:beteiligte -->
## 2. Beteiligte

| Rolle / Gruppe | Interesse am System | Einfluss auf Entscheidungen | Betroffenheit | Wie wird sie gehört? |
|---|---|---|---|---|
| Projektinhaber (Auftraggeber, Visionär) | Machbarkeitstest: lässt sich ein vollständiges Aufbauspiel mit KI-Unterstützung bauen; eigener Spielspaß | hoch, entscheidet alle Freigaben | täglich | direkt in den Sessions |
| KI-Entwicklung (Coding-Assistent) | durchspezifizierte Schritte, prüfbare Akzeptanz | mittel, schlägt vor, entscheidet Umsetzungsdetails | täglich | über Dokumente und Freigaben |
| Spieler großer Flughäfen | stundenlang bauen, umbauen und betreiben ohne Ruckeln; verstehen, warum etwas hakt | mittel, über Rückmeldungen | Hauptnutzer | GitHub-Issues, Live-Tests ab Phase 3 |
| Einsteiger | verständlicher Start an einem kleinen Bestandsflughafen | gering | Nebennutzer | GitHub-Issues, Tests der Lektionen |
| Spieler mit schwächerem Rechner | ehrliche Empfehlung, spielbar auf integrierter Grafik | gering | Nebennutzer | Benchmark-Ergebnisse in Issues |
| Mod- und Inhalte-Ersteller | stabiles, dokumentiertes Datenformat für Airlines, Flugzeuge, Objekte, Grafiken | mittel ab Phase 6 | gelegentlich | GitHub-Issues, Beispiel-Mods |
| Übersetzer | Sprachpakete ohne Code-Änderung | gering | gelegentlich | GitHub-Issues |

<!-- ANCHOR:anwendungsfaelle -->
## 3. Anwendungsfälle

### Enthalten

| ID | Wer | Was | Quelle |
|---|---|---|---|
| UC-001 | Spieler | Neues Spiel anlegen: Bestandsflughafen oder leere Karte, Name, Kartengröße, Schwierigkeit, Sandbox | `docs/vision-2.md` Abschnitt 3 |
| UC-002 | Spieler | Flughafen bauen und umbauen: Gebäude, Etagen, Räume, Zonen, Objekte, Straßen | Säule 1 |
| UC-003 | Spieler | Flugbetrieb einrichten: Bahnen, Rollwege, Stellplätze, Verkehrssteuerung | Basis |
| UC-004 | Spieler | Verträge mit Airlines aushandeln, Gebühren festlegen, Flüge in Wellen planen | Säulen 3 und 5 |
| UC-005 | Spieler | Passagiere abfertigen: Anreise, Check-in, Sicherheit, Grenze, Gate, Boarding, Ankunft | Basis |
| UC-006 | Spieler | Gepäck und Beiladefracht abfertigen: Bänder, Kontrolle, Sortierung, Ausgabe | Basis, Säule 3 |
| UC-007 | Spieler | Flugzeuge abfertigen und versorgen (Turnaround, Betankung, Enteisung, Pushback) | Basis |
| UC-008 | Spieler | Personal und Wirtschaft führen, an Kennzahlen gemessen werden | Säule 5 |
| UC-009 | Spieler | Notfälle, Wetter und Störungen bewältigen und den Betrieb erholen | Säule 4 |
| UC-010 | Spieler | Großflughafen mit über 10.000 Personen flüssig betreiben | `docs/vision-2.md` Abschnitt 4 |
| UC-011 | Spieler | Verstehen, warum etwas nicht funktioniert (Wege, Wartezeit, Leistung, Verkehr) | Säule 2 |
| UC-012 | Spieler mit schwächerem Rechner | Benchmark ausführen und Kartengröße bewusst wählen | Grundsatz Untergrenze |
| UC-013 | Einsteiger | Mit Lektionen an einem kleinen Bestandsflughafen beginnen | Grundsatz „Lernen am eigenen Flughafen“ |
| UC-014 | Spieler | Spielstand speichern, laden, automatisch sichern | Basis |
| UC-015 | Mod-Ersteller | Inhalte als Datenpaket erstellen und im Spiel aktivieren | Basis |
| UC-016 | Übersetzer | Sprachpaket für eine weitere Sprache erstellen | Basis |
| UC-017 | Spieler | Spiel unter Windows, Linux oder macOS von GitHub laden und starten | Randbedingungen |
| UC-018 | Spieler | Ein Terminal im laufenden Betrieb erweitern, ohne den Betrieb anzuhalten (2026-10-29) | Säule 1 |
| UC-019 | Spieler | Ein Drehkreuz mit Umsteigern und Umsteigebank betreiben (2026-10-29) | Säule 3 |

### Bewusst ausgeschlossen

| Anwendungsfall | Begründung | Entschieden am | Wieder aufgreifen, wenn … |
|---|---|---|---|
| Kurven beim Bauen | rechtwinkliges Raster; Kurven sprengen Bau- und Wegelogik | 2026-10-13 | nie geplant; nur bei neuer Vision |
| Nachbau oder Funktionsgleichheit mit einem bestimmten Spiel | eigenes Spiel nach eigenem Entwurf (ADR-046) | 2026-10-29 | nicht vorgesehen |
| Spielstände oder Mods anderer Spiele laden | kein Material und keine Formatbindung an andere Spiele | 2026-10-13 | nicht vorgesehen |
| Vertrieb über Steam oder andere Stores | GitHub-only, betreiberneutral | 2026-10-13 | Projektinhaber entscheidet nach 1.0 |
| Reine Frachtflüge (Frachtnacht) im ersten Umfang | zuerst Beiladefracht (ADR-046) | 2026-10-29 | nach Beiladefracht, FR-066 |
| Mehrspieler | nicht Teil der Vision | 2026-10-13 | Community-Wunsch nach 1.0 |
| Code-Mods (Skripte) | Sicherheits- und Stabilitätsfrage offen (`docs/project-context.md` offene Fragen) | 2026-10-13 | Phase 6 |
| Isometrische 3D-Sicht | Kandidat, spätere Ausbaustufe | 2026-10-13 | nach 1.0 |
| Forschung, Hangars, Tiere | tragen keine Säule (Regel-010, ADR-046) | 2026-10-29 | wenn eine Säule sie braucht |
| Ideen aus `modernisierung.md` ohne Säule | nur Ideenquelle (Regel-010) | 2026-10-29 | jeweils einzeln mit Säule |

<!-- ANCHOR:kernprozesse -->
## 4. Kernprozesse

Seit ADR-046 beschreibt jeder Prozess nur noch das Soll von OpenAirside; die frühere Spalte „Ist“ (Ablauf im bisherigen Vorbild) entfällt.

### Prozess: Abflug eines Passagiers

- **Soll:** 1. Anreise mit Auto, Bus, Taxi oder Bahn, Parkhäuser mit Ebenen. 2. Online-Check-in-Anteil, nur Gepäckabgabe oder Self Bag Drop, gemeinsam genutzte Schalter. 3. Zentrale Kontrolle mit Spuren und Technikstufen, Nachkontrolle. 4. Bei Auslandsflug moderne Grenzkontrolle mit E-Gates. 5. Läden, Gastronomie, Toiletten; Walk-through-Duty-Free. 6. Boarding am Gate, auch Self-Boarding. Wege über Wegweiser und Beschilderung, auch mit über 10.000 Personen flüssig; Wartezeiten mit Ursache sichtbar.
- **Betroffene Anwendungsfälle:** UC-005, UC-010, UC-011

### Prozess: Ankunft eines Passagiers

- **Soll:** 1. Aussteigen über Brücke, Bus oder zu Fuß. 2. Passkontrolle bei Auslandsflug, Zoll. 3. Gepäckausgabe. 4. Abreise. Trennung von Ankunft und Abflug freiwillig.
- **Betroffene Anwendungsfälle:** UC-005, UC-010

### Prozess: Umsteigen (2026-10-29)

- **Soll:** 1. Aussteigen. 2. Ohne Gepäckausgabe und ohne neuen Check-in zum Anschluss; bei Wechsel zwischen Inland und Ausland Grenze und ggf. Kontrolle. 3. Gepäck wird am Vorfeld umgeladen. 4. Boarding am Anschluss-Gate. Die Mindestumsteigezeit ist eine Kennzahl des Flughafens.
- **Betroffene Anwendungsfälle:** UC-019, UC-004

### Prozess: Gepäck und Beiladefracht

- **Soll:** 1. Abgabe am Schalter oder Self Bag Drop; Fracht über die Landseite zur Frachthalle. 2. Dreistufige Kontrolle: CT-Anlage, Bildauswertung, Handkontrolle. 3. Sortierung zum Stellplatz. 4. Verladung; Beiladefracht verlängert die Bodenzeit. Ankunft umgekehrt bis zur Ausgabe.
- **Betroffene Anwendungsfälle:** UC-006

### Prozess: Flugzeug-Umlauf

- **Soll:** 1. Anflug, Landung. 2. Rollen über vom Spieler gezogene Rollwege. 3. Abfertigung. 4. Startfreigabe-Zeit am Gate, „Aufrollen und Warten“. Durchgehend automatische Verkehrssteuerung mit Vorrangregeln nach ADR-005, ohne Blockaden.
- **Betroffene Anwendungsfälle:** UC-003, UC-007, UC-009

### Prozess: Flughafen planen und im Betrieb umbauen

- **Soll:** Bauen im Raster; ein Bauauftrag erscheint sofort und wird von Bauarbeitern einer Anlieferstelle gebaut. Eine Bauphase sperrt Flächen und legt Umleitungen fest, bevor gebaut wird; Baustellen stören je nach Schwierigkeit das Erlebnis in der Nähe. Bei fehlenden Wegen erklärt das Spiel die Ursache.
- **Betroffene Anwendungsfälle:** UC-002, UC-011, UC-018

### Prozess: Vertrag mit einer Airline (2026-10-29)

- **Soll:** 1. Eine Airline mit Geschäftsmodell fragt an oder wird angeworben. 2. Verhandlung über Gebühren, Laufzeit, Slots und messbare Zusagen (etwa Bodenzeit, Pünktlichkeit, Mindestumsteigezeit). 3. Im Betrieb werden die Zusagen gemessen; Bonus oder Abschlag. 4. Nachverhandlung oder Kündigung, wenn sich Flughafen oder Zusagen ändern.
- **Betroffene Anwendungsfälle:** UC-004, UC-008

<!-- ANCHOR:funktionale-anforderungen -->
## 5. Funktionale Anforderungen

Spalte „Fahrplan-Schritt": Schritt-ID aus `docs/fahrplan.md`. Spalte „Test": geplanter Testort; `core/` = Rust-Kern (cargo test), `app/` = Präsentation (Vitest), `e2e/` = Playwright. Spalte „Säule“: siehe Kopf.

### Grundlagen und Meta

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-001 | Neues Spiel mit Startform (Bestandsflughafen oder leere Karte), Name/IATA/Logo, Kartengröße, Schwierigkeit | S1, G | UC-001 | Muss | Startform wählbar; Schwierigkeit legt u. a. die Störwirkung von Baustellen fest (FR-055) | 5.1 | `core/tests/new_game.rs` | OFFEN |
| FR-002 | Sandbox-Modus mit Regel-Schaltern | B | UC-001 | Soll | Geld, Störungen und Störwirkung von Baustellen abschaltbar; Sandbox deaktiviert Erfolge | 5.1 | `core/tests/sandbox.rs` | OFFEN (Priorität 2026-10-29 von Muss auf Soll, ADR-046) |
| FR-003 | Zeitsteuerung mit Pause, 1×–3×, Nachtsprung und Kalender | B | UC-010 | Muss | bei 3× kein Simulationsrückstand im Referenzszenario | 2.2, 4.1 | `core/tests/time.rs` | OFFEN (Zeitsteuerung, Nachtsprung und Kalender seit 2.2 im Kern; Rückstand im Referenzszenario misst 4.1) |
| FR-004 | Spielstände: manuell, Schnellspeichern, Autosave mit Rotation, Vorschaubild | B | UC-014 | Muss | Laden ergibt identischen Zustand (Prüfsumme) | 2.5 | `core/tests/savegame_roundtrip.rs`, `e2e/smoke.spec.ts` | ERLEDIGT 2026-10-19 (Intervall und Zahl der Autosaves einstellbar seit 2.6) |
| FR-005 | Einstellungen: Einheiten, Währung, Uhr, Passagieranteil, Steuerung | B | UC-001 | Muss | jede Einstellung wirkt ohne Neustart und bleibt gespeichert | 5.1 | `app/settings.test.ts` | OFFEN |
| FR-006 | Oberfläche Deutsch und Englisch, weitere über Sprachpakete | B | UC-016 | Muss | Sprachwechsel ohne Neustart; Sprachpaket aus Ordner wird erkannt | 2.6 | `app/renderer/i18n.test.ts`, `core/tests/lang.rs`, `e2e/smoke.spec.ts` | ERLEDIGT 2026-10-19 (2.6) |
| FR-007 | Lektionen am Engpass: jedes System wird beim ersten Gebrauch am Flughafen des Spielers erklärt | G | UC-013 | Muss | erste Lektion im Bestandsflughafen durchspielbar (3.2); jedes System hat eine Lektion mit eigenem Overlay; neuer Spieler betreibt danach einen kleinen Flughafen mit Gepäck ohne Nachlesen | 3.2, 5.6 | `e2e/lessons.spec.ts` | OFFEN (2026-10-29 neu gefasst, ADR-046) |
| FR-008 | Erfolge mit Sperre bei Sandbox und Konsole | B | UC-008 | Soll | eigene Liste; Sandbox und Konsole sperren | 5.6 | `core/tests/achievements.rs` | OFFEN |
| FR-009 | Konsole/Debug-Panel | B | UC-011 | Soll | Befehle wirken im Kern | 2.7 | `app/renderer/console.test.ts` | ERLEDIGT 2026-10-15 (2.7) |

### Bauen, Zonen, Landseite

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-010 | Bausystem im Raster mit Etagen, Fundamenten, Wänden, Objekten, Werkzeugen | S1 | UC-002 | Muss | Böden, Wände, Objekte und Etagen bau- und abreißbar; keine Kurven | 3.1, 5.2 | `core/tests/build.rs` | OFFEN (Erdgeschoss mit Böden, Wänden und Einzelobjekten seit 3.1; Etagen 5.2) |
| FR-011 | Bauaufträge mit Bauarbeitern, sofort sichtbar übernommen | S1 | UC-002 | Muss | Auftrag erscheint im nächsten Bild; Bauarbeiter einer Anlieferstelle bauen ihn | 3.1 | `core/tests/construction.rs` | ERLEDIGT 2026-10-28 (3.1, ADR-044, ADR-045) |
| FR-012 | Zonen und Räume | S1, B | UC-002 | Muss | Zonen und Personalzonen malbar, Räume mit Zweck; Wege beachten Zonen | 3.2, 5.2 | `core/tests/zones.rs` | OFFEN (Overlays seit 2026-10-29 in FR-057) |
| FR-013 | Landseite: Straßen, Parken (Parkhäuser mit Ebenen), ÖPNV, Taxis | B, S2 | UC-005 | Muss | Anreise mit allen Verkehrsmitteln im Szenario; Parkhaus mit Ebenen | 5.3 | `core/tests/landside.rs` | OFFEN |
| FR-014 | Straßenverkehr nach festen Regeln ohne Blockade | B | UC-005 | Muss | eigene Spuren je Richtung, Abbiegen behindert Gegenverkehr nicht, kein Einfahren in nicht räumbare Kreuzung; 1 h Spielzeit Referenzszenario ohne Dauerstau | 3.12, 3.4 | `core/tests/road_traffic.rs` | OFFEN |
| FR-015 | Vorfahrt automatisch, je Kreuzung manuell umschaltbar | B | UC-005 | Muss | Schilder, Stoppstellen, Ampeln wählbar | 5.3 | `core/tests/intersections.rs` | OFFEN |
| FR-053 | Bauphasen mit Sperrungen und Umleitungen (2026-10-29) | S1 | UC-018 | Muss | Aufträge zu einer Phase bündelbar; eine Phase sperrt Flächen vor dem Bau; Personen werden ohne Stillstand umgeleitet | 3.2, 5.2 | `core/tests/build_phases.rs` | OFFEN |
| FR-054 | Baustellenlogistik: Anlieferung kreuzt Wege, ohne den Betrieb zu blockieren (2026-10-29) | S1 | UC-018 | Muss | Material kommt per Lkw über die Landseite zur Anlieferstelle; Bauarbeiter queren Gehwege ohne Stillstand | 3.4 | `core/tests/site_logistics.rs` | OFFEN |
| FR-055 | Störwirkung von Baustellen, über die Schwierigkeit einstellbar (2026-10-29) | S1 | UC-018 | Muss | Lärm und Enge senken das Erlebnis in einem Umkreis; Stärke je Schwierigkeit, in der Sandbox abschaltbar | 5.2 | `core/tests/site_disturbance.rs` | OFFEN |
| FR-056 | Freiere Raumformen im Raster: unregelmäßige Räume, Zusammenlegen (2026-10-29) | S1 | UC-002 | Soll | Raum aus beliebigen zusammenhängenden Zellen; zwei Räume zusammenlegbar; keine Kurven | 5.2 | `core/tests/rooms.rs` | OFFEN |

### Passagiere und Terminal

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-016 | Passagiere mit Bedürfnissen, Zufriedenheit und Ablauf Abflug/Ankunft | B, S2 | UC-005 | Muss | Kernprozesse Abflug und Ankunft; 1 h Spielzeit im Szenario ohne Stillstand, der sich nicht selbst auflöst (ADR-036) | 3.2, 4.1 | `core/tests/pax_flow.rs` | OFFEN |
| FR-017 | Wegfindung nach Wegweiser-Prinzip | B | UC-010 | Muss | ADR-004; 10.000 Personen mit ≤ 4 ms je Simulationstakt auf Referenz-Hardware | 1.2, 2.3, 3.11, 3.3 | `core/benches/flowfield.rs` | OFFEN (Wegweiser im Kern und Benchmark als CI-Gate seit 2.3; Satz der Bauarbeiter seit 3.1c; Passagierführung 3.3) |
| FR-018 | Passagierführung über Schilder und Bodenmarkierungen; ohne Schilder Suchen | S2 | UC-005 | Muss | mit Beschilderung messbar kürzere Wegzeit als ohne | 3.11, 3.3 | `core/tests/signage.rs` | OFFEN |
| FR-019 | Check-in modern: Online-Anteil, Gepäckabgabe, Self Bag Drop, gemeinsam genutzte Schalter | B | UC-005 | Muss | alle vier Formen im Szenario nutzbar; Schalter mehreren Airlines zuordenbar | 3.2, 5.4 | `core/tests/checkin.rs` | OFFEN |
| FR-020 | Sicherheitskontrolle mit Spuren, Technikstufen und Nachkontrolle | B | UC-005 | Muss | Durchsatz je Technikstufe messbar verschieden; Nachkontrolle bei Auffälligkeit | 3.2, 5.4 | `core/tests/security.rs` | OFFEN |
| FR-021 | Grenzkontrolle und Zoll mit E-Gates | B, S3 | UC-005, UC-019 | Muss | Auslandsflüge und Umsteiger zwischen Inland und Ausland passieren die Grenze | 5.4 | `core/tests/border.rs` | OFFEN |
| FR-022 | Gates, Boarding (inkl. Self-Boarding), Stellplätze, Brücken, Busse | B | UC-005 | Muss | Boarding über Brücke, Bus und zu Fuß; Self-Boarding schneller als Personal | 3.2, 5.4 | `core/tests/gates.rs` | OFFEN |
| FR-023 | Läden, Gastronomie, Lounges, Toiletten mit Bestand und Lieferungen | S5, S2 | UC-005, UC-008 | Muss | Läden mit Lager; Lieferungen über Landseite oder Vorfeld als eigener Strom; leeres Lager senkt Umsatz | 5.4 | `core/tests/commercial.rs` | OFFEN (2026-10-29 um Bestand erweitert, ADR-046) |
| FR-024 | Infoschalter und digitale Beschilderung mit spürbarer Wirkung | S2 | UC-005 | Muss | Gate-Wechsel erreicht Passagiere messbar schneller | 5.4 | `core/tests/info.rs` | OFFEN |
| FR-052 | Walk-out-Positionen: Fluggäste gehen auf Vorfeld-Gehwegen zum Flugzeug an Stellplätzen ohne Fluggastbrücke | B | UC-005 | Muss | Querungen von Betriebsstraßen geregelt im Wechsel (ADR-037); Boarding und Aussteigen zu Fuß im Szenario (ADR-036, 2026-10-19) | 3.7, 3.4 | `core/tests/walk_out.rs` | OFFEN |
| FR-060 | Umsteiger und Umsteigebank mit Mindestumsteigezeit als Kennzahl (2026-10-29) | S3 | UC-019 | Muss | Prozess Umsteigen; Mindestumsteigezeit je Flughafen gemessen und angezeigt; Gepäck wird umgeladen | 5.4 | `core/tests/transfers.rs` | OFFEN |

### Gepäck und Fracht

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-025 | Gepäckbänder, Sortierung, Verladung, Ausgabe | B | UC-006 | Muss | Prozess Gepäck im Szenario ohne Stillstand | 3.5, 5.5 | `core/tests/baggage.rs` | OFFEN |
| FR-026 | Dreistufige Gepäckkontrolle | B | UC-006 | Muss | CT-Anlage, Bildauswertung, Handkontrolle mit eigenem Durchsatz | 5.5 | `core/tests/baggage_screening.rs` | OFFEN |
| FR-046 | Datenmodell bereitet Fracht vor | S3 | UC-006 | Muss | Ladungsart „Fracht" im Schema vorhanden, ohne Spielmechanik | 2.3 | `core/tests/cargo_ready.rs` | ERLEDIGT 2026-10-14 (2.3) |
| FR-048 | Beiladefracht mit den Passagierflügen | S3 | UC-006 | Muss | Fracht von der Landseite über Frachthalle und Vorfeld ins Flugzeug; Umschlag verlängert die Bodenzeit; Ankunft umgekehrt | 5.5 | `core/tests/belly_cargo.rs` | OFFEN (2026-10-29 von Kann „Fracht spielbar“ auf Muss „Beiladefracht“, ADR-046) |
| FR-066 | Frachtnacht: reine Frachtflüge in der Nacht mit Frachthalle und Lärmgrenzen (2026-10-29) | S3, S5 | UC-006 | Kann | nach FR-048 | – | – | OFFEN |

### Flugbetrieb, Flüge und Airlines

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-027 | Bahnen, Rollwege mit gezogenen Mittellinien, Stellplätze, Befeuerung, Flugsicherung | B | UC-003 | Muss | Flugzeuge landen, rollen und starten über vom Spieler gezogene Rollwege | 3.4, 5.5 | `core/tests/airside.rs` | OFFEN |
| FR-028 | Automatische Verkehrssteuerung mit Vorrangregeln nach ADR-005 | B, S4 | UC-003 | Muss | 7-stufige Rangfolge; Szenario-Test mit Notfall ohne Blockade; Entscheidungen für Spieler einsehbar | 3.12, 3.4 | `core/tests/traffic_control.rs` | OFFEN |
| FR-029 | Startfreigabe-Zeit am Gate, „Aufrollen und Warten" | S3 | UC-003 | Muss | Abflüge in der Welle ohne Stau an der Bahn | 5.5 | `core/tests/departure_sequencing.rs` | OFFEN |
| FR-030 | Verträge mit Airlines: Gebühren, Laufzeit, messbare Zusagen, Nachverhandeln; Flugplanung | S5 | UC-004 | Muss | Prozess „Vertrag mit einer Airline“; Zusage gemessen, Bonus und Abschlag gebucht | 3.6, 5.5 | `core/tests/flights.rs`, `core/tests/contracts.rs` | OFFEN (2026-10-29 neu gefasst, ADR-046) |
| FR-031 | Flugzeugtypen und Airlines als Daten, Klassen nach Bedarf der Säulen, weitere per Mod | B | UC-004, UC-015 | Muss | Regional, Mittel, Groß als eigene Typen mit Airlines; ein weiterer Typ aus dem Mod-Ordner | 5.7 | `core/tests/aircraft_data.rs` | OFFEN (alte Fassung „alle Add-on-Klassen“ VERWORFEN 2026-10-29, ADR-046) |
| FR-059 | Tagesrhythmus: Flugplan in Wellen, Spitzen planbar und sichtbar (2026-10-29) | S3 | UC-004, UC-019 | Muss | Flugplan mit Wellen; Spitzen im Overlay und in der Tabelle sichtbar | 3.6 | `core/tests/schedule_waves.rs` | OFFEN |
| FR-061 | Slots für knappe Zeiten in der Spitze als Teil der Verträge (2026-10-29) | S3, S5 | UC-004 | Soll | Spitzenzeiten begrenzt; Vergabe in Verträgen; Airline ohne Slot weicht aus | 5.5 | `core/tests/slots.rs` | OFFEN |
| FR-064 | Geschäftsmodelle der Airlines mit eigenen Kennzahlen und Wünschen (2026-10-29) | S5 | UC-004 | Muss | Billigflieger, Netzcarrier, Regional, Fracht als Daten; zwei Modelle verlangen messbar verschiedene Flughäfen | 3.6 | `core/tests/airline_models.rs` | OFFEN |
| FR-065 | Gebührenordnung: Lande-, Passagier-, Nachtentgelt, Rabatt für Umsteiger (2026-10-29) | S5 | UC-004 | Muss | jede Stellschraube ändert die Nachfrage der Geschäftsmodelle unterschiedlich | 3.6 | `core/tests/fees.rs` | OFFEN |

### Abfertigung, Störungen, Umwelt

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-032 | Turnaround: Betankung, Catering, Reinigung, Enteisung, Pushback, Fahrzeuge | B, S3 | UC-007 | Muss | Bodenzeit ergibt sich aus den Abfertigungsschritten; Beiladefracht verlängert sie | 3.5, 5.5 | `core/tests/turnaround.rs` | OFFEN |
| FR-033 | Notfälle und Wetter | S4 | UC-009 | Muss | Notfall mit Einsatzfahrzeugen; Wetter verändert Kapazität von Bahn und Vorfeld | 5.6 | `core/tests/emergencies.rs` | OFFEN (Teil „Hangars, Tiere“ VERWORFEN 2026-10-29, ADR-046) |
| FR-034 | Störungen: Drohne, IT-Ausfall, Stromausfall | S4 | UC-009 | Muss | jede Störung senkt Kapazität messbar und endet | 5.6 | `core/tests/disruptions.rs` | OFFEN |
| FR-062 | Betriebstag mit Störung als Szenario, bewertet wird die Erholung (2026-10-29) | S4 | UC-009 | Muss | Szenario mit Störung am Morgen; Kennzahl „Verspätungsminuten bis Tagesende“ | 5.6 | `core/tests/recovery_day.rs` | OFFEN |

### Management und Wirtschaft

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-035 | Personal mit Verträgen, Schichten, Zufriedenheit | B | UC-008 | Muss | Schichten decken den Tagesrhythmus; Unterbesetzung zeigt sich als Ursache in der Diagnose | 5.6 | `core/tests/staff.rs` | OFFEN |
| FR-036 | Wirtschaft: Einnahmen, Kosten, Kredite, Lieferanten | S5 | UC-008 | Muss | Einnahmen aus Gebühren und Läden, Kosten aus Personal und Bau; Kredit möglich | 3.6, 5.6 | `core/tests/economy.rs` | OFFEN |
| FR-037 | Forschung, Ziele, Bewertungen, Post/E-Mail | – | UC-008 | – | – | – | – | VERWORFEN 2026-10-29 (ADR-046: Forschung trägt keine Säule; Bewertung über FR-063, Ziele über FR-062) |
| FR-038 | Messbare Zusagen mit Bonus und Abschlag gegenüber Airlines | S5 | UC-004, UC-008 | Muss | Zusage aus dem Vertrag (FR-030) gemessen; Bonus oder Abschlag gebucht und erklärt | 3.6, 5.6 | `core/tests/service_levels.rs` | OFFEN |
| FR-039 | Große Tabellen-Panels (Flüge, Personal, Finanzen) bleiben bei Großflughafen flüssig | G | UC-010 | Muss | 10.000 Zeilen sortieren/filtern ohne spürbare Verzögerung (< 100 ms) | 1.4, 3.6, 5.6 | `app/tables.bench.ts` | OFFEN |
| FR-063 | Kennzahlen statt Sterne: Pünktlichkeit, Wartezeit, Umsteigequote, Gepäck-Fehlleitungen (2026-10-29) | S5, S2 | UC-008, UC-011 | Muss | Kennzahlen je Airline und Flughafen; Airlines entscheiden danach | 3.6 | `core/tests/kpis.rs` | OFFEN |
| FR-068 | Bedienkomfort im Großen: Vorlagen im Flugplaner, Personal in Gruppen, Sammelaktionen (2026-10-29) | G | UC-010 | Soll | Vorlage für eine Welle anwendbar; 50 Mitarbeiter in einem Schritt einstellbar | 5.6 | `app/bulk_actions.test.ts` | OFFEN |

### Ströme lesen, Leistung, Plattform, Inhalte

| ID | Anforderung | Säule | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|---|
| FR-057 | Strom-Overlays: Dichte, Wartezeit, Herkunft und Ziel; Vergleich vor und nach einem Umbau (2026-10-29) | S2 | UC-011, UC-018 | Muss | Wartezeit-Overlay in 3.2; Vergleich zweier Zeitpunkte im selben Bild | 3.2, 3.3 | `app/overlays.test.ts`, `core/tests/flow_stats.rs` | OFFEN |
| FR-058 | Engpass-Diagnose: die schlimmsten Engpässe mit Ursache (Kapazität, Weg, Personal) (2026-10-29) | S2 | UC-011 | Muss | jede Wartezeit über einer Schwelle mit Ursache abrufbar | 3.3 | `core/tests/bottlenecks.rs` | OFFEN |
| FR-067 | Bestandsflughafen als Startform und Einstieg (2026-10-29) | S1, G | UC-001, UC-013 | Muss | kleiner Bestandsflughafen auf Gelände mit einem lösbaren Engpass; Ausgangspunkt der ersten Lektion | 3.2, 5.1 | `core/tests/scenarios.rs` | OFFEN |
| FR-040 | Leistungsziel Großflughafen | B | UC-010 | Muss | Referenzszenario (3 Terminals U-Form, 2 Bahnen, ~17 Gates, ≥ 10.000 Personen): 60 FPS bei Höchstgeschwindigkeit ohne Rückstand auf Referenz-Hardware | 1.5, 4.1, 7.2 | `e2e/perf_reference.spec.ts` | OFFEN |
| FR-041 | Benchmark mit unverbindlicher Kartengrößen-Empfehlung und Warnhinweis | B | UC-012 | Muss | Benchmark läuft im Zeitraffer; Empfehlung abweisbar; Warnung bei großer Karte | 5.8 | `e2e/benchmark.spec.ts` | OFFEN |
| FR-042 | Leistungsbewusstsein: Spiel erklärt, warum es an Grenzen stößt | S2 | UC-011 | Soll | Anzeige der teuersten Bereiche/Systeme | 5.8 | `app/perf_hints.test.ts` | OFFEN |
| FR-043 | Erklärbare Wegfindungsprobleme | S2 | UC-011 | Muss | Bei fehlendem Weg wird Ursache angezeigt (Zone, Verbindung, Station) | 3.3 | `core/tests/path_diagnostics.rs` | OFFEN |
| FR-044 | Inhalte als Datenpakete; Mods plattformneutral aktivierbar | B | UC-015 | Muss | Airline, Flugzeugtyp, Objekt aus Mod-Ordner im Spiel, ohne Store-Workshop | 2.4, 3.6, 6.1 | `core/tests/datapacks.rs`, `e2e/smoke.spec.ts` | OFFEN (Objekt aus dem Mod-Ordner seit 2.4, 2026-10-18; Airline und Flugzeugtyp 3.6; Mod-Verwaltung 6.1) |
| FR-045 | Lauffähig auf Windows, Linux, macOS; Download von GitHub | B | UC-017 | Muss | CI baut und startet auf allen drei Plattformen; Release-Artefakte je Plattform | 1.6, 2.1, 7.2 | `e2e/smoke.spec.ts` in `.github/workflows/ci.yml` und `release.yml` (seit 2.1, x64 bzw. macOS arm64) | OFFEN (arm64 für Windows/Linux und v1.0.0-Builds in 7.2) |
| FR-047 | Musik, Geräusche, Atmosphäre | B | UC-005 | Soll | freie Bibliotheken; Durchsagen und Stimmengewirr | 6.2 | `app/audio.test.ts` | OFFEN |
| FR-049 | Intro-/Menü-Video | B | UC-001 | Kann | – | 6.2 | – | OFFEN |
| FR-050 | Grafiken als eigene SVG, im Build gerastert | B | UC-002 | Soll | ADR-008; keine Grafik anderer Spiele oder ihrer Mods | 2.10 | `scripts/assets.test.mjs`, `tools/assets` (cargo test), `e2e/smoke.spec.ts` | ERLEDIGT 2026-10-19 (ADR-033, ADR-034) |
| FR-051 | Isometrische Sicht | B | UC-002 | Kann | spätere Ausbaustufe | – | – | OFFEN |

**Regeln:**

- IDs werden nie wiederverwendet, auch nicht nach Streichung.
- Eine Muss-Anforderung ohne Fahrplan-Schritt ist ein Bug (Drift-Prüfung, CLAUDE.md Abschnitt 16).
- Eine Muss-Anforderung zu streichen oder herabzustufen ist ein Descope: Status `VERWORFEN` mit ADR (CLAUDE.md Abschnitt 6, „Keine Verschiebung ohne Landeplatz").
- Neue Anforderungen während des Projekts bekommen Datum und Quelle (z. B. „Live-Test 2026-06-20").
- Jede Spielfunktion nennt ihre Säule (Regel-010).

<!-- ANCHOR:nicht-funktionale-anforderungen -->
## 6. Nicht-funktionale Anforderungen

Werte stehen ausschließlich in `docs/architecture.md` Abschnitt 6.

- **Leistung (60 FPS, kein Rückstand, ≥ 10.000 Personen):** besonders UC-010, UC-005, UC-002 (Bauaufträge), FR-017, FR-039, FR-040; zusätzliche Ströme (FR-023 Lieferungen, FR-048 Fracht, FR-054 Lkw) gehen in das Referenzszenario 4.1 ein.
- **Plattformen:** UC-017, FR-045.
- **Untergrenze integrierte Grafik:** UC-012, FR-041.
- **Datenformat-Stabilität:** UC-014, UC-015.
