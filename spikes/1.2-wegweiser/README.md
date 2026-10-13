# Spike 1.2 – Wegweiser-Prinzip mit 10.000 Personen

Fahrplan-Schritt 1.2 (Phase 1, ERKUNDUNG, Prototyp). **Wegwerf-Code**, reines Rust ohne Electron (Übergabe zur Anzeige ist Schritt 1.3).

## Aufbau

- `src/grid.rs` – Testraster mit **zwei Etagen**, Außenwänden, Innenwänden alle 64 Zellen mit Türen, Landseite und Luftseite, Grenzlinie mit 6 Sicherheitsspuren und 2 Einbahn-Ausgängen, 8 Treppen, Bodenmarkierungen (Kostenfaktor 0,7 „bevorzugt" und 2,0 „meiden"). Ziele: 17 Gates (Etage 1), 8 Check-in-Gruppen, Toiletten land- und luftseitig, Läden luftseitig, Ausgang. Ergibt 29 Wegweiser.
- `src/field.rs` – ein Wegweiser je Ziel: Dijkstra rückwärts vom Ziel mit Bucket-Queue, 1 Byte je Zelle (Richtung, Treppe hoch/runter, angekommen, unerreichbar). Zonenregeln: Landseite → Kontrolle → Luftseite, Luftseite → Ausgang → Landseite, sonst kein Wechsel. Unerreichbarkeit mit Ursache: „nur ohne Zonenregeln erreichbar" (z. B. alle Kontrollen geschlossen) oder „von Wänden eingeschlossen". Dazu eine **inkrementelle Reparatur**, die nur Zellen neu berechnet, deren Weg über eine geänderte Zelle führt.
- `src/sim.rs` – Personen mit Ablaufplan (Abflug: Check-in → ggf. Toilette → ggf. Laden → Gate → Boarding; Ankunft: Gate → Ausgang), Wartezeiten, 90 Ticks an der Kontrolle, Verlangsamung bei Gedränge. Ein Tick hat zwei Phasen: Bewegung (liest die Personendichte des Vortick) und Dichtezählung (vertauschbare Additionen). Bauänderungen wirken sofort auf das Raster; neue Wegweiser rechnet ein Hintergrund-Pool und übernimmt sie **genau 30 Ticks später** (bei Verzug wartet der Takt), damit der Ablauf deterministisch bleibt.
- Zwei Datenlayouts für Personen: `aos` (ein Datensatz je Person, 52 Byte) und `split` (16 Byte „heiße" Daten je Tick, Rest getrennt).
- `run-all.sh` – Messreihe; Rohdaten in `results/`.
- Nachträglich für Spike 1.3 ergänzt (additiv, Messpfad oben unverändert): `Sim::set_walls`, `Sim::poll_async` (Übernahme neuer Wegweiser, sobald fertig – ADR-010 Option A) und `Sim::visit_hot`; siehe `spikes/1.3-spielfeld/README.md`.

Ausführen: `./run-all.sh` oder einzeln `cargo run --release --bin run -- run ticks=6000 bg=4`. Tests: `cargo test --release` (Wegweiser stabil; Reparatur liefert dieselben Distanzen wie die Vollberechnung, jede gespeicherte Richtung ist ein gültiger kürzester Schritt).

## Messumgebung

Cloud-Container, 4 vCPU (Xeon 2,1 GHz), Linux, 2026-10-13. **Nicht die Referenzhardware** (8 Kerne). Raster 512 × 512 × 2 Etagen (524.288 Zellen) als Hauptfall, dazu 256² und 1024².

## Ergebnisse

### Simulationsschritt (ohne Bauänderungen)

| Personen | Layout | 1 Thread | 2 Threads | 4 Threads (p99) |
|---|---|---|---|---|
| 10.000 | aos | 524 µs | 369 µs | 223 µs (590) |
| 10.000 | split | 548 µs | 386 µs | 201 µs (457) |
| 50.000 | aos | 2219 µs | 1305 µs | 705 µs (1302) |
| 50.000 | split | 2337 µs | 1299 µs | 677 µs (1208) |

Ein Tick mit 10.000 Personen kostet rund 0,2 ms auf 4 Threads – bei 60 Ticks pro Sekunde rund 1 % der Rechenzeit. Die Layouts unterscheiden sich um höchstens 10 %, in keiner Richtung durchgängig.

### Wegweiser: Rechenzeit und Speicher (alle Wegweiser, vollständig)

| Raster (× 2 Etagen) | Wegweiser | 1 Thread | 4 Threads | Speicher |
|---|---|---|---|---|
| 256² | 29 / 64 | 185 / 497 ms | 64 / 129 ms | 3,6 / 8 MB |
| 512² | 29 / 64 | 576–698 / 1736 ms | 195 / 434 ms | 14,5 / 32 MB |
| 1024² | 29 / 64 | 2890 / 6847 ms | 889 / 1663–1939 ms | 58 / 128 MB |

Linear in Zellen × Wegweiser, rund 40–50 ns je Zelle und Wegweiser auf einem Kern. Eine Optimierung der inneren Schleife brachte 18 % bei identischem Ergebnis; die Berechnung ist speichergebunden. **Fünf Etagen** wie im Original (BAU-07) bedeuten rund das 2,5-Fache an Speicher und Rechenzeit, soweit die Etagen bebaut sind.

### Bauänderung bis zur Wirkung auf die Wege (512², 29 Wegweiser)

| Hintergrund-Threads | voll | inkrementell |
|---|---|---|
| 1 | 600–1350 ms | 295–1690 ms |
| 2 | 353–783 ms | 167–965 ms |
| 4 | 200–394 ms | 110–598 ms |

Bei 1024² mit 64 Wegweisern: 1,9–6,8 s voll, 1,6–4,5 s inkrementell (4 Hintergrund-Threads). Wände selbst wirken sofort (Personen weichen aus oder warten); nur die Wegwahl folgt verzögert. Mit der deterministischen Übernahme nach 30 Ticks wartet der Takt fast die ganze Rechenzeit, weil 30 Ticks bei Höchsttempo nur Millisekunden sind.

**Inkrementelle Reparatur:** schneller bei Flächenänderungen (Bodenmarkierung: 110–200 statt 240–390 ms), aber **langsamer** beim Öffnen und Schließen von Türen (1,2- bis 1,8-mal so lange), weil Türen Engstellen sind, über die fast alle Wege eines Bereichs laufen. Sie braucht zusätzlich 4 Byte Distanz je Zelle und Wegweiser (Faktor 5 Speicher, 512 MB bei 1024² mit 64 Wegweisern).

### Unerreichbarkeit

Bei geschlossenen Sicherheitsspuren meldeten wartende Personen 47.453-mal „Ursache Zonenregel" (Wiederholung alle 60 Ticks je Person), nach dem Öffnen keine neuen Meldungen dieser Art. 9 Meldungen „eingeschlossen" stammen von Räumen, deren Türen der Testbefehl alle schloss – ebenfalls korrekt.

### Determinismus

Prüfsumme nach 6000 Ticks mit allen 6 Bauänderungen: **bitgleich** für Layout aos/split, 1 und 4 Simulations-Threads, 2 und 4 Hintergrund-Threads (`711616a0fb3f808b`). Voraussetzung: Übernahme neuer Wegweiser zu einem festen Tick.

### Langzeitlauf (60.000 Ticks, 10.000 Personen, 54.268 abgeschlossene Wege)

- Ohne Bauänderungen: Mittel je 10.000-Tick-Fenster 221, 249, 251, 242, 246, 284 µs – kein gerichteter Anstieg erkennbar, Streuung im Rahmen der Container-Messung. Speicher 78,4 → 78,8 MB.
- Mit 72 Bauänderungen: Schrittzeit p99 je Fenster 583–822 µs ohne Trend; Speicher 118 → 122,4 MB, davon +1,1 MB zwischen Tick 10.000 und 60.000. **Vermutung, nicht geprüft:** Seiten des Dichte-Arrays werden erst beim ersten Betreten eines Bereichs belegt, dazu Fragmentierung durch wiederholte große Allokationen der Wegweiser.

## Befunde

1. **Personen sind billig, Wegweiser sind teuer.** Die Rechenlast hängt an Zellen × Zielen, nicht an Personen (wie ADR-004 vorhersagt).
2. **Datenlayout ist nicht entscheidend.** Getrennte Felder (heiß/kalt) und ein Datensatz je Person liegen innerhalb von 10 %.
3. **Zweiphasiger Tick ist deterministisch** über Threadzahlen hinweg und kostet fast nichts.
4. **Konflikt Determinismus gegen „Bauen ohne Wartezeit":** Feste Übernahme-Ticks erzwingen Warten, wenn die Neuberechnung länger dauert. Ohne festen Tick (übernehmen, sobald fertig) gibt es kein Warten, aber der Spielverlauf ist nicht mehr bitgenau wiederholbar.
5. **Kartengröße ist unbestimmt.** Weder Vision noch Projektdokumente nennen die Rastergröße des Referenzszenarios oder der größten Karte. Bis 512² × 2 reicht die einfache Lösung; ab 1024² oder fünf voll bebauten Etagen dauern Neuberechnungen Sekunden und belegen 100+ MB.
6. **Inkrementelle Reparatur lohnt nicht als Standard** (uneinheitlicher Gewinn, fünffacher Speicher). Für sehr große Karten wäre eine hierarchische Aufteilung in Sektoren der bekannte Weg (nicht gemessen).
