# Spike 3.8 – Weltdarstellung im Spielfeld

Fahrplan-Schritt 3.8 (Phase 3, Phasentyp-Kontext ERKUNDUNG, Vergleichsstudie mit Messung). **Wegwerf-Code.** Entscheidung: ADR-038; Vorlage, Messplan und Auswertung: `docs/research/3.8-weltdarstellung.md`; Messdateien: `docs/research/3.8-ergebnisse/`.

## Wo der Code liegt

Der Prototyp steckt im Spiel selbst (Kern, Brücke, Spielfeld, Messwerkzeug), damit die Messung den echten Weg Kern-Prozess → Port → Renderer → GPU nimmt. Er liegt nur auf dem Branch `erkundung/3-8-messaufbau` und wird nicht gemergt:

| Commit | Inhalt |
|---|---|
| `257a498` | Kern: `core/src/display.rs` – synthetische Darstellungsdaten je Block, Kodierung (gleichförmig, Palette 4 Bit, roh), Versand gegen die übergebenen Blöcke (W-A) oder Ankündigung mit Abfrage `blocks` (W-B); Messtest `display::tests::first_transfer_of_the_reference_terminal` |
| `8f557d2` | Renderer: `app/renderer/world-view.ts` (Datentextur mit Shader Z-1, vorgerenderte Blöcke Z-2, Objekte je Block), Bildleser, Messreihe mit vier Ansichten, Konsolenbefehl `world wa\|wb z1\|z2 [Minuten]`; `spikes/3.8-weltdarstellung/gen-graphics.mjs` (512 Testgrafiken), `probe.mjs` (Probelauf im Container mit Playwright) |
| `74ceec9` | 512 erzeugte Testgrafiken unter `assets/graphics/base/spike_g_*.svg` |
| `f23e154` | Korrektur W-B: Abfrage der Blöcke erst mit der neuen Sitzung |

Pakete: Release-Läufe 39449159828 (`74ceec9`) und 39459607051 (`f23e154`).

## Was in die Umsetzung (3.1) übernommen werden kann

Die Idee, nicht der Code: Erkennung geänderter Blöcke über die Identität der geteilten Blöcke (der Sender hält die übergebenen Blöcke, damit `Arc::make_mut` beim Ändern kopiert), Palette mit 4 Bit je Block, Datentextur je 512² Zellen mit `texelFetch` im Shader (`#version 300 es` selbst setzen, PixiJS 8.20.1 tut es bei eigenem `GlProgram` nicht).
