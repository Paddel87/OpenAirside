# Spike 1.5 – Leistungsnachweis auf den Zielsystemen

Fahrplan-Schritt 1.5 (Phase 1, ERKUNDUNG, Lasttest / Messung). **Wegwerf-Code.** Fragen: Welche Rastergröße hat das Referenzszenario mit Reserve (ADR-010)? Was kosten Wegweiser mit fünf Etagen an Speicher und Neuberechnung? Hält die Anwendung mit 10.000 Personen bei höchster Spielgeschwindigkeit über einen Langzeitlauf 60 Bilder/s ohne Rückstand der Simulation, und wächst der Speicher?

## Aufbau

- **Kern:** Crate aus Spike 1.2 (`spikes/1.2-wegweiser`), additiv auf beliebig viele Etagen verallgemeinert. Neu `Grid::reference_scenario`: die beiden Etagen aus 1.2 (Halle mit Check-in und Sicherheitslinie, Abflugebene mit 17 Gates) als Etagen 2 und 3, dazu Etage 0 U-Bahn-Station (landseitig), Etage 1 Gepäckausgabe (luftseitig, 4 Bänder), Etage 4 Lounges (luftseitig, 2). Treppen verbinden alle Etagen; 36 Wegweiser bei 17 Gates. Personen: drei von zehn Abfliegenden kommen mit der U-Bahn, jeder Vierte besucht eine Lounge; Ankommende gehen zur Hälfte zur Gepäckausgabe und verlassen den Flughafen zu drei Zehnteln mit der U-Bahn.
  - `Fields::build_opts(grid, keep_dist = false)` speichert keine Distanzen (4 Byte je Zelle und Wegweiser, nur für die inkrementelle Reparatur nötig, die ADR-010 nicht übernimmt).
  - Der Zwei-Etagen-Pfad ist unverändert: Prüfsummen bitgleich zu 1.2 (`711616a0fb3f808b` inkrementell, `7a9422eb4f817758` voll). Neuer Test `five_floor_scenario_uses_all_floors`.
- **Brücke:** `bridge-core` aus 1.3 und `sim-bridge` aus 1.4 um optionale Einstellungen ergänzt (Etagen, Gates, volle Neuberechnung, Zahl der Hintergrund-Threads); 1.3 und 1.4 laufen unverändert.
- **`app/`:** Testprogramm aus 1.4 (Spielfeld, Panel, gehärtetes Electron 44.7.0) mit der Messreihe **Langzeitlauf**:
  1. Szenario anlegen (Standard: 10.000 Personen, Raster 768 × 768 × 5 Etagen, 17 Gates, volle Neuberechnung im Hintergrund auf allen Kernen außer zweien), 5 s Zeitraffer.
  2. **Höchste Tickrate:** 5 s ungebremst mit Zeichnen.
  3. **Langzeitlauf** (Standard 10 Minuten) bei **180 Ticks/s**: Kamera wechselt je 30-s-Fenster zwischen Übersicht über die Halle und Kreisen über der dichtesten Stelle der Abflugebene; Panel mit 5.000 sortierten Zeilen, 4 Aktualisierungen/s; alle 10 s eine Bauaktion in der Halle (Wand setzen, beim nächsten Mal entfernen), die eine volle Neuberechnung aller Wegweiser auslöst.
  4. Je Fenster: Bilder/s, Bildzeiten, Ticks/s, Schrittzeit, Ticks im Rückstand, Neuberechnungen, Bauaktion bis zum Bild; der Hauptprozess erfasst alle 5 s den Speicher aller Prozesse (Hauptprozess, Fenster, GPU, Kern-Prozess).

**Annahme „höchste Spielgeschwindigkeit":** Das Original kennt 1×, 2×, 3× (Inventar META-08). Die Tickrate bei 1× ist nicht festgelegt; die Spikes takten mit 60 Ticks/s. 3× wird hier als **180 Ticks/s bei gleicher Tick-Länge** gemessen (die strengere Auslegung). Unabhängig davon misst Schritt 2 der Reihe die höchste erreichbare Tickrate.

Bauen und starten (Linux; macOS `.dylib`, Windows `.dll`):

```bash
(cd ../1.4-tabellen && cargo build --release -p sim-bridge)
mkdir -p app/native && cp ../1.4-tabellen/target/release/libsim_bridge.so app/native/sim_bridge.node
cd app && npm ci && npx vite build
npx electron .                                         # interaktiv, 1–5 = Etage
npx electron . --series=longrun --exit --out=e.json    # Langzeitlauf
```

Weitere Schalter: `--minutes=`, `--size=`, `--floors=2|5`, `--gates=`, `--hz=`, `--persons=`, `--incremental=1`, `--threads=`, `--bg-threads=`, `--rows=`, `--rate=`. Container-Messreihe des Kerns: `./run-container.sh` (Rohdaten `results/fields-5.jsonl`, `results/run-5.jsonl`).

## Messung im Container (Kern, ohne Grafik)

Cloud-Container, 4 vCPU (Xeon 2,1 GHz), 16 GB, Linux, 2026-10-13. Nicht die Referenzhardware (8 Kerne).

### Wegweiser mit fünf Etagen (volle Berechnung, ohne Distanzen)

Begehbare Zellen: 512² → 0,64 Mio., 768² → 1,44 Mio., 1024² → 2,56 Mio. (alle fünf Etagen).

| Raster × 5 Etagen | Wegweiser | Speicher Wegweiser | 1 Thread | 2 Threads | 4 Threads | Prozess-Spitze (4 Threads) |
|---|---|---|---|---|---|---|
| 512² | 36 | 45 MB | 1,53 s | 0,66 s | 0,47 s | 116 MB |
| 512² | 64 | 80 MB | 2,36 s | 1,48 s | 0,62 s | 163 MB |
| 768² | 36 | 101 MB | 3,66 s | 1,81 s | 0,98 s | 258 MB |
| 768² | 64 | 180 MB | 7,07 s | 3,85 s | 1,71 s | 292 MB |
| 1024² | 36 | 180 MB | 8,54 s | 4,15 s | 2,72 s | 451 MB |
| 1024² | 64 | 320 MB | 14,09 s | 8,25 s | 4,11 s | 686 MB |

Speicher = Raster × 5 Etagen × Wegweiser × 1 Byte; die Ebenen sind voll belegt, auch wo nichts gebaut ist (bei 1024² ist nur die Hälfte der Zellen begehbar). Rechenzeit wächst etwa linear mit begehbaren Zellen × Wegweisern (rund 65–85 ns je begehbarer Zelle und Wegweiser auf einem Kern) und ist auf 4 Threads 3,1- bis 4,1-mal so schnell wie auf einem.

### Simulationsschritt im Fünf-Etagen-Szenario (10.000 Personen, 4 Threads)

| Raster | ohne Bauaktionen: Schritt Mittel / p99 | höchste Tickrate | Speicher (Prozess) |
|---|---|---|---|
| 512² × 5 | 0,25 / 0,80 ms | 3.930 Ticks/s | 101 MB |
| 1024² × 5 | 0,29 / 0,53 ms | 3.490 Ticks/s | 377 MB |

180 Ticks/s brauchen damit rund 5 % eines Kerns; die Rastergröße wirkt kaum auf den Schritt. Mit Bauaktionen (Messpfad aus 1.2 mit festem Übernahme-Tick, nur zur Kontrolle): Prozess-Spitze 252 MB (512²), 525 MB (768²), 911 MB (1024²) – alte und neue Wegweiser liegen während der Neuberechnung gleichzeitig im Speicher. Über 12.000 Ticks kein gerichteter Anstieg von Schrittzeit oder Speicher.

### Testprogramm im Container (nur Funktionsprobe)

`results/container-probe.json`, 1 Minute, 768² × 5, SwiftShader ohne Grafikkarte: läuft durch, misst alle Werte; Bildraten (20–29) und Rückstand der Simulation (145–155 statt 180 Ticks/s) sind verfälscht, weil die Software-Grafik dieselben 4 Kerne belegt wie Simulation und Neuberechnung. Volle Neuberechnung bei 768² × 5 mit 2 Hintergrund-Threads 2,7–2,9 s, Bauaktion bis zum Bild 86–133 ms (3 Bilder). Speicher aller Prozesse 0,82 → 1,0 GB in 74 s, davon Kern-Prozess 0,33 GB.

## Befunde

1. **Fünf Etagen sind tragbar bis 768², teuer ab 1024².** Mit 36 Wegweisern: 768² × 5 belegt 101 MB und rechnet auf 4 Threads 1 s neu; 1024² × 5 belegt 180 MB und rechnet 2,7 s. Mit 64 Wegweisern (Reserve für mehr Airlines, Gepäckbänder, Läden) 180 bzw. 320 MB und 1,7 bzw. 4,1 s.
2. **Der Simulationsschritt ist kein Engpass.** 10.000 Personen auf fünf Etagen kosten 0,25–0,29 ms je Tick; 180 Ticks/s sind ein Bruchteil der Rechenzeit, die höchste Tickrate liegt im Container bei rund 3.500–3.900.
3. **Speicher während der Neuberechnung doppelt.** Alte Wegweiser bleiben in Gebrauch, bis die neuen fertig sind (ADR-010 Option A); die Spitze liegt bei rund dem Doppelten des Wegweiser-Speichers plus Arbeitsfeldern. Maßnahme bei Bedarf: Wegweiser nur über bebauten Bereichen speichern (Blöcke statt voller Ebenen), halbiert bei 1024² etwa den Bedarf.
4. **Abweichung in 1.3/1.4:** Die Spikes 1.3 und 1.4 nutzten für Bauaktionen die **inkrementelle Reparatur** (Voreinstellung der Crate aus 1.2), nicht die von ADR-010 gewählte volle Neuberechnung. Die dort gemessenen 39–54 ms bis zur Übernahme gelten deshalb nur für die Reparatur. Das Testprogramm 1.5 misst die volle Neuberechnung.
5. **Rastergröße:** In keinem Projektdokument ist festgelegt, wie viele Meter eine Zelle darstellt; die Rastergröße des Referenzszenarios mit Reserve ist daher eine Entscheidung (Vorlage im Fahrplan, Schritt 1.5).

## Testprogramm herunterladen und starten

Der Workflow `.github/workflows/spike-build-1-5.yml` baut das Programm für alle drei Systeme. Herunterladen: GitHub → Actions → „Spike-Build 1.5" → neuester Lauf → unten unter „Artifacts" `spike-1-5-win` oder `spike-1-5-mac` (Anmeldung bei GitHub nötig, 14 Tage verfügbar). Starten wie bei Spike 1.4 (Windows: ZIP entpacken, SmartScreen → „Weitere Informationen" → „Trotzdem ausführen"; macOS: DMG öffnen, Programm in „Programme" ziehen, beim ersten Start Systemeinstellungen → Datenschutz & Sicherheit → „Dennoch öffnen").

Das Programm misst nach dem Start **rund 11 Minuten** selbständig. Bitte währenddessen nichts bedienen, das Fenster nicht verdecken, nicht verkleinern und keine anderen rechenintensiven Programme laufen lassen; Netzteil anschließen. Danach steht oben rechts, wo das Ergebnis gespeichert wurde (Ordner „Downloads", Datei `openairside-spike-1-5-…json`). Diese Datei bitte schicken, mit Rechner und Bildschirm.

## Messung auf echter Grafik

Ausstehend: Referenz-PC (Windows) und Mac M3 Pro. Linux auf echter Hardware: 7.2 (ADR-011).
