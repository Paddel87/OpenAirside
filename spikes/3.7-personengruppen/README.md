# Messung 3.7 – Personengruppen, Begehbarkeit und Querungen

Fahrplan-Schritt 3.7 (Phase 3, Entwurfsschritt mit Phasentyp-Kontext ERKUNDUNG, Entwurf mit Messung). **Wegwerf-Code.** Fragen: Was kosten Wegweiser, wenn Gruppen von Personen verschiedene Flächen betreten dürfen (Fluggäste, Personal, Vorfeldpersonal, Bauarbeiter)? Lohnt ein gemeinsamer Feldsatz für alle Gruppen? Kann Personal statt über Wegweiser über eine Einzelweg-Suche je Fahrt geführt werden?

Herleitung, Optionen und Vorlage: [`docs/research/3.7-personengruppen.md`](../../docs/research/3.7-personengruppen.md).

## Aufbau

- Nutzt den Produktions-Kern (`openairside-core`) als Pfad-Abhängigkeit: Referenz-Terminal 768² × 5 mit 17 Gates und 36 Zielen (`scenario::reference_terminal`), Wegweiser mit `FieldSet::compute` wie im Benchmark aus 2.3. Die Zahlen sind damit mit `core/benches/flowfield.rs` vergleichbar.
- **Welt 2048²**, Terminal in der Mitte (640 … 1407). Nördlich davon das Vorfeld im Erdgeschoss: Betriebsstraße (12 Zellen), 17 Stellplätze je 44 × 44 Zellen (88 m), Rollgasse und Rollweg (je 40 Zellen), Bahn 1.500 × 30 Zellen. Südlich ein landseitiger Vorplatz von 128 Reihen mit Türen alle 32 Zellen.
- **Walk-out (FR-052):** 6 der 17 Stellplätze haben einen Vorfeld-Gehweg von 3 Zellen Breite: Tür in der Nordwand, Querung der Betriebsstraße, Gehweg auf dem Stellplatz bis zur Flugzeugtür (Ziel).
- **Personalräume:** 4 Räume von 63 × 63 Zellen im Erdgeschoss der Luftseite.
- **Zugangsprofile** (je Profil ein eigenes Raster mit den Zellen, die das Profil betreten darf):

| Profil | Begehbar | Ziele |
|---|---|---|
| `base` | Terminal (heutiger Stand) | 36 Ziele des Referenz-Terminals |
| `pax` | Terminal, Vorplatz, Gehwege, Querungen | 36 + 6 Flugzeugtüren = 42 |
| `staff` | wie `pax` (Personalräume liegen im Terminal) | 4 Personalräume + 12 Arbeitsplätze = 16 |
| `ramp` | `pax` + alle 17 Stellplatzflächen | 17 Stellplätze + 4 Personalräume = 21 |
| `build` | ganzes Erdgeschoss außer der Bahn, Terminal-Etagen | 8 Baustellen |
| `union` | Zellen von `build` | alle 87 Ziele (ein Feldsatz für alle Gruppen) |
| `people` | Zellen von `ramp` | 79 Ziele von `pax`, `staff`, `ramp` (ein Feldsatz für Fluggäste und Personal) |
| `build2` | Zellen von `build` | 2 Ziele: Anlieferstelle, alle Baustellen als ein Ziel |

- Die Messung bildet die Begehbarkeit je Profil über eigene Raster ab, nicht über Regeln in der Wegweiser-Berechnung. Für Speicher und Rechenzeit ist das gleichwertig; die Regel „Personalzone nur verlassen“ für Fluggäste ändert die Zellenzahl nicht (Personalräume liegen in Blöcken, die ohnehin begehbar sind).
- **Einzelweg-Suche (`astar`):** A* auf einer dichten Kopie des Rasters mit denselben Schrittregeln und Kosten wie die Wegweiser (Ecken, Zonen, Treppen, Kostenfaktoren). Zwei Schätzfunktionen: zulässig (Kostenfaktor 0,7, liefert den kürzesten Weg) und mit Normalkosten (schneller, nicht immer kürzester Weg). Die zulässigen Kosten werden gegen die Wegweiser des Profils `ramp` geprüft. Ein Thread, 1.000 Fahrten je Fall, Startwert 1.

```bash
cargo build --release
./target/release/personengruppen fields threads=4 runs=3
./target/release/personengruppen astar trips=1000 seed=1
```

Rohdaten: `results/fields-reihe1.jsonl` (erste Messreihe, ohne `people` und `build2`), `results/fields-reihe2.jsonl` (zweite Messreihe mit allen Profilen), `results/astar.jsonl`.

## Messung im Container

Cloud-Container, 4 vCPU, 15 GB, Linux, 2026-10-20. Nicht die Referenzhardware (8 Kerne); Verhältnisse sind übertragbar, absolute Zeiten auf dem Referenz-PC kürzer (vgl. 1.5). Zeiten der Wegweiser: Median aus 3 Läufen auf 4 Threads; wo zwei Werte stehen, sind es die Mediane der beiden Messreihen. Die Läufe streuen im Container um bis zu 25 %.

### Wegweiser je Profil

| Profil | Ziele | Blöcke | Zellen | Speicher | je Ziel | Neuberechnung (Median) |
|---|---|---|---|---|---|---|
| `base` | 36 | 1.468 | 1,50 Mio. | 54,1 MB | 1,50 MB | 0,95–1,18 s |
| `pax` | 42 | 1.576 | 1,61 Mio. | 67,8 MB | 1,61 MB | 1,25–1,37 s |
| `staff` | 16 | 1.576 | 1,61 Mio. | 25,8 MB | 1,61 MB | 0,56–0,57 s |
| `ramp` | 21 | 1.612 | 1,65 Mio. | 34,7 MB | 1,65 MB | 0,59–0,67 s |
| `build` | 8 | 4.988 | 5,11 Mio. | 40,9 MB | 5,11 MB | 0,84–0,89 s |
| `union` | 87 | 4.988 | 5,11 Mio. | 444,4 MB | 5,11 MB | 9,4–11,2 s |
| `people` | 79 | 1.612 | 1,65 Mio. | 130,4 MB | 1,65 MB | 2,59 s |
| `build2` | 2 | 4.988 | 5,11 Mio. | 10,2 MB | 5,11 MB | 0,57 s |

Alle Profile ohne unerreichbare Zellen. Prozess-Spitze bis `build`: 230 MB; nach `union`: 1.013 MB.

### Einzelweg-Suche je Fahrt (ein Thread)

| Fall | Schätzfunktion | Median | p90 | p99 | Max. | Mittel | besuchte Zellen (Mittel) | Weglänge (Zellen) |
|---|---|---|---|---|---|---|---|---|
| Vorfeldpersonal: Personalraum ↔ Stellplatz, Stellplatz ↔ Stellplatz | zulässig | 4,9 ms | 63 ms | 298 ms | 426 ms | 22,4 ms | 71.824 | 302 |
| | Normalkosten | 0,07 ms | 6,8 ms | 19 ms | 24 ms | 1,9 ms | 7.647 | 302 |
| Personal im Terminal, zufällige Zelle → zufällige Zelle (alle Etagen) | zulässig | 61 ms | 167 ms | 272 ms | 332 ms | 76 ms | 270.524 | 424 |
| | Normalkosten | 18 ms | 62 ms | 170 ms | 667 ms | 28 ms | 111.009 | 421 |
| Bauarbeiter: Anlieferstelle → zufällige Zelle im Erdgeschoss | zulässig | 146 ms | 1.162 ms | 1.718 ms | 2.355 ms | 404 ms | 1.104.594 | 918 |
| | Normalkosten | 21 ms | 896 ms | 1.309 ms | 1.594 ms | 202 ms | 622.916 | 918 |

Prüfung: Die zulässigen Kosten stimmen in allen 1.000 Fahrten mit den Kosten entlang der Wegweiser des Profils `ramp` überein (0 Abweichungen). Prozess-Spitze 364 MB (dichte Kopie und Suchfelder).

## Befunde

1. **Begehbarkeit kostet wenig, Ziele kosten.** Vorplatz, Walk-out-Gehwege und alle 17 Stellplatzflächen erhöhen die Zellen der Wegweiser um 10 % (1,50 → 1,65 Mio.). Speicher und Neuberechnung wachsen mit der Zahl der Ziele: 36 → 79 Ziele heißt 54 → 130 MB und 1,0 → 2,6 s im Container.
2. **Ein Feldsatz für Fluggäste und Personal kostet dasselbe wie getrennte Sätze** (`people` 130,4 MB / 2,6 s gegen `pax` + `staff` + `ramp` 128,3 MB / 2,6 s). Getrennte Topologien bringen hier nichts.
3. **Ein Feldsatz für alle Gruppen ist zu teuer** (`union`: 444 MB, 9–11 s), weil jedes Ziel dann über das ganze Erdgeschoss reicht. Bauarbeiter brauchen einen eigenen kleinen Satz. Mit einem Feld zur Anlieferstelle und einem zu allen Baustellen (`build2`) kostet er 10 MB und 0,6 s, unabhängig von der Zahl der Baustellen.
4. **Einzelweg-Suche je Fahrt trägt Personal nicht.** Im Terminal kostet eine Fahrt im Median 18–61 ms, p99 170–272 ms, bei Bauarbeitern p99 1,3–1,7 s. Das liegt weit über dem Zeitbudget eines Simulationsschritts (4 ms). Im Hintergrund gerechnet wären es bei angenommen 1.000 Mitarbeitern mit einer neuen Fahrt je 10 Spielminuten bei 3× rund 100 Fahrten je Sekunde, also im Mittel 2,8 Kern-Sekunden je Sekunde. Wegweiser je Ziel teilen sich dagegen alle Personen mit demselben Ziel.
5. **Die Suche wird teuer durch Wände und Einbahn-Zonen.** Kurze Wege auf dem offenen Vorfeld sind billig (Median 0,07 ms mit Normalkosten). Im Terminal besucht die Suche bis zu 2,9 Mio. Zellen für einen Weg von rund 420 Zellen.
