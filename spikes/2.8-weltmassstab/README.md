# Messung 2.8 – Weltmaßstab, Luftseite und Zeitmodell

Fahrplan-Schritt 2.8 (Phase 2, Entwurfsschritt mit Phasentyp-Kontext ERKUNDUNG, Vergleichsstudie mit Messung). **Wegwerf-Code.** Fragen: Was kosten Wegweiser, wenn Bahnen, Vorfeld, Rollwege und Straßen im selben Raster liegen wie die Gebäude? Hilft es, Wegweiser nur über bebauten Blöcken zu speichern (ADR-013, Konsequenzen)? Welche Zellgröße und welches Zeitmodell passen?

## Aufbau

- Nutzt die Crate aus Spike 1.2/1.5 (`spikes/1.2-wegweiser`) unverändert als Pfad-Abhängigkeit.
- `world`: bettet das Fünf-Etagen-Referenzszenario aus 1.5 (Terminal `t × t` Zellen, 17 Gates, 36 Wegweiser) in eine Welt von `W × H` Zellen ein. Außerhalb des Terminals ist das Erdgeschoss Luftseite (Vorfeld, Rollwege, Bahnen) und für Personen nicht begehbar; südlich des Terminals liegt ein begehbarer landseitiger Streifen von 128 Reihen (Vorplatz, Parkplätze, Bahnhof). Obere und untere Etagen gibt es nur über bzw. unter dem Terminal.
  - **Dicht:** Wegweiser über die ganze Welt (heutige Speicherung, ADR-013).
  - **Blöcke:** Zahl der 32 × 32-Blöcke je Etage mit mindestens einer begehbaren Zelle; Speicher = Blöcke × 1.024 × Wegweiser × 1 Byte.
  - **Ersatzmessung Rechenzeit Blöcke:** volle Neuberechnung auf dem Ausschnitt Terminal + Streifen, also genau den begehbaren Zellen, die eine Block-Speicherung berechnen würde. Der Mehraufwand der Block-Adressierung in der Wegweiser-Berechnung ist darin nicht enthalten (siehe `lookup`).
  - **Grunddaten je Zelle:** Belegung, Zone, Kosten (3 Byte) und Dichte (2 Byte). Dicht = alle Etagen voll; Blöcke = Erdgeschoss voll, übrige Etagen nur belegte Blöcke.
- `step`: Simulationsschritt mit 10.000 Personen in der eingebetteten Welt (dichte Speicherung).
- `lookup`: Kosten eines Lesezugriffs je Person über eine Blocktabelle gegenüber dem dichten Index (zufällige begehbare Zellen, 10.000 Personen × 2.000 Runden).

```bash
cargo build --release
./target/release/weltmassstab world W=2048 H=2048 t=512 band=128 gates=17
./target/release/weltmassstab step W=2048 H=2048 t=512 band=128
./target/release/weltmassstab lookup W=2048 H=2048 t=768 band=128 block=32
```

Rohdaten: `results/world.jsonl`, `results/step.jsonl`, `results/lookup.jsonl`.

## Messung im Container

Cloud-Container, 4 vCPU, 15 GB, Linux, 2026-10-13, 4 Threads. Nicht die Referenzhardware (8 Kerne); Verhältnisse sind übertragbar, absolute Zeiten auf dem Referenz-PC kürzer (vgl. 1.5).

### Wegweiser: dicht über die ganze Welt gegen Blöcke

| Welt | Terminal | Wegweiser | begehbare Zellen | dicht: Speicher / Neuberechnung | Blöcke: Speicher / Neuberechnung (Ersatz) | Grunddaten dicht / Blöcke |
|---|---|---|---|---|---|---|
| 768² (ADR-013, nur Terminal, aus 1.5) | 768² | 36 / 64 | 1,44 Mio. | 101 / 180 MB, 0,98 / 1,71 s | – | – |
| 1536² | 512² | 36 | 0,71 Mio. | 405 MB, 1,65 s | 25,5 MB, 0,42 s | 56 / 13 MB |
| 2048² | 512² | 36 | 0,71 Mio. | 720 MB, 2,34 s | 25,5 MB, 0,41 s | 100 / 22 MB |
| 3072² | 512² | 36 | 0,71 Mio. | 1.620 MB, 5,18 s | 25,5 MB, 0,47 s | 225 / 47 MB |
| 2048² | 768² | 36 | 1,54 Mio. | 720 MB, 3,03 s | 55,0 MB, 1,30 s | 100 / 24 MB |
| 2048² | 768² | 64 | 1,54 Mio. | 1.280 MB, 5,66 s | 97,8 MB, 1,61 s | 100 / 24 MB |
| 2048² | 1024² | 36 | 2,69 Mio. | 720 MB, 4,18 s | 95,8 MB, 2,02 s | 100 / 28 MB |

Belegte Blöcke je Etage (0–4) bei Terminal 512²: 16 / 132 / 320 / 128 / 128; bei 768²: 24 / 292 / 672 / 288 / 288. Prozess-Spitze nach der dichten Berechnung: 585 MB (1536²), 1.052 MB (2048²), 2.246 MB (3072²), 1.650 MB (2048², 64 Wegweiser). Während einer Neuberechnung liegen alte und neue Wegweiser gleichzeitig im Speicher (ADR-010), die Spitze verdoppelt den Wegweiser-Anteil.

### Simulationsschritt (10.000 Personen, dichte Speicherung)

| Welt | Schritt Mittel / p99 |
|---|---|
| 640 × 641 (nur Terminal + Streifen) | 0,229 / 0,480 ms |
| 2048² | 0,259 / 0,598 ms |
| 3072² | 0,276 / 0,794 ms |

### Blocktabelle gegenüber dichtem Index

Lesezugriff je Person: dicht 5,0–5,2 ns, über eine Blocktabelle 8,2 ns (Blockgröße 32 und 64 gleich). Bei einigen Zugriffen je Person und Schritt sind das unter 0,1 ms je Schritt auf einem Kern, verteilt auf die Threads weniger.

## Befunde

1. **Luftseite im selben Raster ist mit dichter Speicherung nicht tragbar.** Die Rechenzeit der dichten Neuberechnung wächst mit der Weltfläche, nicht mit der begehbaren Fläche (2,3 s bei 2048², 5,2 s bei 3072² für dasselbe Terminal), der Speicher ebenfalls (720 MB bzw. 1,6 GB, Spitze doppelt).
2. **Blöcke entkoppeln die Kosten von der Weltgröße.** Speicher und Rechenzeit hängen nur noch von der begehbaren Fläche ab: 25,5 MB und 0,41–0,47 s für das Terminal 512² in jeder Weltgröße. Mit einem Terminalblock 768² (64 Wegweiser) 98 MB und 1,6 s, im Bereich der in ADR-013 freigegebenen Werte für 768².
3. **Die Grenze verschiebt sich auf die begehbare Fläche.** Ein Terminalblock 1024² × 5 kostet mit Blöcken 96 MB und 2,0 s bei 36 Wegweisern (Container). Die Neuberechnung bleibt linear in begehbaren Zellen × Wegweisern; eine Obergrenze ergibt sich nicht mehr aus der Kartengröße, sondern aus der bebauten Fläche (Benchmark-Empfehlung, Vision).
4. **Der Simulationsschritt bleibt unberührt** (0,23–0,28 ms); die Blocktabelle kostet rund 3 ns je Zugriff.
5. **Vorfeld nicht pauschal begehbar.** Die Messung setzt die Luftseite außerhalb der Gebäude als nicht begehbar an. Wäre das ganze Vorfeld begehbar, wüchse die begehbare Fläche um ein Vielfaches und mit ihr Speicher und Neuberechnung.

## Orientierung am Original (nicht übernommen, nur Spielprinzip)

- Inventar: Stellplätze 5 × 7, 10 × 10, 16 × 16 Kacheln (GATE-02), Terminal-Mindestgröße 16 × 16 (BAU-13, unsicher), Toiletten mindestens 3 × 3 (WC-01). Eine Meterangabe je Kachel ist nicht bekannt.
- Spieltag bei 1×: keine offizielle Angabe; Spieler im Forum des Herstellers nennen 1 bis 4 Sekunden je Spielminute (Spieltag 24 bis 96 Minuten, Versionen unterschiedlich). Die Entwickler beschreiben, dass Objekte „in Echtzeit" simuliert werden, die Uhr aber viel schneller läuft; Spieler kritisieren dadurch zu lange Abläufe in Spielzeit (Sicherheitskontrolle, Fahrten, Abfertigung). Quellen: forum.apoglabs.com, Themen 3953, 6953, 15746 (abgerufen 2026-10-13).
