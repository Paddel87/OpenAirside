# Spike 3.14 – Späte Bilder auf dem Referenz-PC

Fahrplan-Schritt 3.14 (Phase 3, Phasentyp-Kontext ERKUNDUNG, Messung). **Wegwerf-Code.** Vorlage, Messplan und Auswertung: `docs/research/3.14-spaete-bilder.md`; Messdateien: `docs/research/3.14-ergebnisse/`. Korrektur auf `main`: Schritt 3.15.

## Wo der Code liegt

Der Messprototyp liegt nur auf dem Branch `erkundung/3-14-messaufbau`, abgezweigt vom Messprototyp 3.8 (`f23e154`), und wird nicht gemergt:

| Commit | Inhalt |
|---|---|
| `6808a35` | Renderer: `app/renderer/latency.ts` (Zeitstempel je Bild, Selbst-Nachricht, späte Antworten), Haken um den PixiJS-Ticker; Kern-Prozess: Eingang, Absenden und Verzögerung der Ereignisschleife je Bild (`core-host.ts`); Hauptprozess: Zeitspur über `contentTracing`, Konsolenbefehle `angle`, `fullscreen`, `trace`; `spikes/3.14-spaete-bilder/probe.mjs` (Probelauf mit Playwright) und `analyze.py` (Auswertung von Messdateien und Zeitspuren) |
| `9d01469` | Korrektur: GPU-Name nur einmal lesen (`playfield.ts`) |

Pakete: Release-Läufe 39471593590 (`6808a35`) und 39483411097 (`9d01469`).

## Was in die Umsetzung übernommen werden kann

Die Korrektur in `9d01469` (Schritt 3.15). Die Zeitstempel, die Selbst-Nachricht und die Auswertung der Zeitspuren (`analyze.py trace`) eignen sich als Vorlage, falls im Leistungstest 4.1 wieder späte Bilder auftreten.
