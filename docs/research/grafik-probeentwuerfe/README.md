# Grafik-Probeentwürfe (2026-10-21)

Probematerial für den Stil der Spielgrafiken, vorgelegt nach `docs/stilleitfaden.md` Abschnitt 7. Noch nicht im Spiel: Erst die vom Projektinhaber gewählte Fassung kommt mit dem zugehörigen Fahrplan-Schritt nach `assets/graphics/` (Wand und Boden 3.1, Check-in-Schalter 3.2, Bus und Flugzeug 3.4).

| Objekt | Größe | Variante A | Variante B |
|---|---|---|---|
| Wand | 1 Zelle, Fläche | `wand-a.svg` Beton | `wand-b.svg` Glasfassade |
| Boden | 1 Zelle, Fläche | `boden-a.svg` Fliesen 1 m | `boden-b.svg` Teppich |
| Check-in-Schalter | 2 × 1 Zellen (4 m × 2 m) | `checkin-a.svg` Holztheke mit Waage | `checkin-b.svg` Theke mit Gepäckband |
| Bus | 7 × 1,5 Zellen (14 m × 3 m) | `bus-a.svg` Vorfeldbus | `bus-b.svg` Shuttlebus |
| Flugzeug | 19 × 18 Zellen (38 m × 36 m) | `flugzeug-a.svg` neutral | `flugzeug-b.svg` mit Akzent |

- Alle zehn Dateien bestehen `checkSvg` aus `scripts/assets.mjs`.
- Übersichtsseite mit den drei Atlas-Stufen und einer Beispielszene: <https://claude.ai/artifact/LcxV23uZmJj8KNjZHyeYDm> (privat, nur für den Projektinhaber).
- **Offene Festlegung:** Für Fahrzeuge und Flugzeuge regelt der Stilleitfaden die Ausrichtung bei Drehung 0 noch nicht. Die Entwürfe zeigen die Front nach rechts (+x). Festlegung mit 3.4, passend zur Ausrichtung in S-1 Layout 2 (ADR-038).
- **Lackierungen nach Airline:** Entwurf in 3.6 (Grundgrafik mit Farbflächen, Logo auf dem Rumpfdach, Farben der Airline; Fahrplan). Die Grundgrafik des Flugzeugs aus 3.4 wird dafür um markierte Farbflächen ergänzt.
- **Wahl des Projektinhabers:** steht aus (2026-10-21: „vielversprechend“, noch keine abschließende Entscheidung).
