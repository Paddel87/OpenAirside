# Spike 1.4 – Großes Tabellen-Panel neben dem Spielfeld

Fahrplan-Schritt 1.4 (Phase 1, ERKUNDUNG, Prototyp). **Wegwerf-Code.** Fragen: Drückt ein Panel im Stil des Flugmonitors mit mehreren tausend Zeilen, Sortierung und Live-Werten die Bildrate des laufenden Spielfelds? Welche Aktualisierungsrate ist tragbar?

## Aufbau

- **Spielfeld und Brücke** unverändert aus Spike 1.3 (`spikes/1.3-spielfeld`): Kern aus 1.2 mit 10.000 Personen, `bridge-core` (Simulations-Thread, S-1 als Dreifachpuffer, S-2), PixiJS-Spielfeld mit `ParticleContainer`, Übertragung „Ausschnitt" im kompakten Format.
- `panel-core/` – neue Rust-Crate ohne Anbindung (Regel-006): eine **synthetische Flugtabelle**, weil der Kern noch keine Flüge kennt. Flug *i* von *n* hat feste Werte (Flugnummer, Airline, Ziel/Herkunft, Abflug/Ankunft, Planzeit, Gate, gebuchte Passagiere) und Live-Werte, die sich aus dem Simulations-Tick ergeben (Verspätung wechselt alle 10 Spielminuten, Status, Passagiere an Bord, Gepäck, Restzeit). Ein Tick ist eine Spielsekunde; bei 60 Ticks/s ändert sich die Restzeit jeder Zeile in jedem Tick. Die Zahlen tragen keine Spielregeln. Airlines sind erfunden, Orte reale Städte (Regel-005).
  - **Format:** 32 Byte Kopf (Version, Zeilen, Tick, Spaltenzahl), dann **je Spalte ein Block** passender Breite (u8, u16, i16, i32), jeder Block ab einer 4-Byte-Grenze: 22 Byte je Zeile. Texte (Airlines, Orte, Status) kommen einmal als Wörterbuch. Der Renderer liest die Spalten über Typed-Array-Sichten ohne Umwandlung.
  - Tests: Aufbau und Größe, Werte je Spalte, Determinismus und Live-Verhalten, Boarding vor dem Abflug, Wörterbuch (`cargo test --release -p panel-core`).
- `sim-bridge/` – dünne napi-rs-Anbindung wie in 1.3, ergänzt um `readTable`, `maxTableBytes`, `tableDictionary`.
- `app/` – Electron 44.7.0 mit Härtung wie in 1.3. Neu:
  - Der Kern-Prozess (`simhost.mjs`) beantwortet Tabellen-Anfragen und kodiert die Tabelle beim Tick des zuletzt übergebenen Bildes.
  - **Panel** (`renderer/panel/index.js`): React 19.2.8, TanStack Table 9.2.8 mit `rowSortingFeature`, gebündelt mit Vite 8.3.4 zu einem ES-Modul (`renderer/dist/panel.js`), das unter der strengen CSP ohne `eval` läuft. 11 Spalten, Klick auf den Spaltenkopf sortiert. Zeilen sind Objekte `{ i }`, deren Zugriffsfunktionen aus den Spalten des neuesten Puffers lesen; jede Aktualisierung übergibt ein neues Zeilen-Array, TanStack baut die Zeilenmodelle samt Sortierung neu.
  - **Virtualisierung ohne neue Abhängigkeit:** feste Zeilenhöhe 22 px, gerendert werden nur die sichtbaren Zeilen plus 6 darüber und darunter (rund 46 Zeilen im DOM). TanStack Virtual steht nicht im Stack (ADR-002) und wurde nicht verwendet.
  - **Aktualisierung gedrosselt:** höchstens eine Tabellen-Anfrage unterwegs, nächste Anfrage frühestens nach 1/Rate Sekunden; die Antwort wird im nächsten Bild eingearbeitet, synchron (`flushSync`), damit die Kosten einer Aktualisierung in einem Stück messbar sind.
  - **Eingebaute Messreihe** (`--series=quick`): 10.000 Personen, 3 s Zeitraffer, dann je Phase 0,5 s Vorlauf und 4 s Messung, während die Kamera wie in 1.3 („Schwenk", 4 Pixel je Zelle) um die dichteste Stelle kreist und das Panel mit 600 px/s durchläuft. Phasen: ohne Panel; Panel ohne Aktualisierung; **sortiert nach einer Live-Spalte** (PAX absteigend) mit 2.000, 5.000 und 20.000 Zeilen bei 1, 4, 10, 30 und 60 Aktualisierungen/s; 20.000 Zeilen unsortiert bei 10/s; **ohne Virtualisierung** (alle Zeilen im DOM) mit 2.000 Zeilen bei 1 und 4/s; zum Schluss noch einmal ohne Panel. Gemessen werden Bildrate, Bilder über 20 und 33 ms, Kosten je Aktualisierung (gesamt und davon Zeilenmodelle/Sortierung), Kodieren im Kern-Prozess, Größe, Rundlauf und lange Aufgaben (> 50 ms) im Renderer.
  - Danach interaktiv: Spaltenkopf = sortieren, `p` = Panel ein/aus, `+`/`-` = Aktualisierungsrate (0, 1, 4, 10, 30, 60/s), `n` = Zeilenzahl (2.000, 5.000, 20.000), `v` = Virtualisierung; Spielfeld wie in 1.3.

Bauen und starten (Linux; macOS `.dylib`, Windows `.dll`):

```bash
cargo test --release -p panel-core
cargo build --release -p sim-bridge
mkdir -p app/native && cp target/release/libsim_bridge.so app/native/sim_bridge.node
cd app && npm ci && npm start                                  # interaktiv
npx vite build && npx electron . --series=quick --exit --out=ergebnis.json # Messreihe
```

Weitere Schalter: `--rows=`, `--rate=` (Messreihe auf eine Zeilenzahl bzw. Rate beschränken), `--persons=`, `--phase-seconds=`, `--draw=0` (Kontrolle ohne Zeichnen), `--screenshot=<datei>`. Unter Linux ohne Bildschirm: `xvfb-run -a -s "-screen 0 1280x800x24" … --no-sandbox --enable-unsafe-swiftshader`.

## Messumgebung

Cloud-Container wie in 1.3: 4 vCPU (Xeon 2,1 GHz), 16 GB, Linux, keine Grafikkarte (WebGL2 über SwiftShader). Fenster 1279 × 772, Pixeldichte 1; das Panel ist 760 px breit und liegt rechts über dem Spielfeld. Takt 60 Ticks/s, 4 Simulations-Threads. Zwei Durchläufe am 2026-10-13: **ohne Zeichnen** als Kontrolle (`results/container-ohne-zeichnen.json`) und **mit Zeichnen** (`results/container-swiftshader.json`).

Das Panel kostet Rechenzeit im Hauptthread des Renderers (JavaScript, DOM, Layout), kaum Grafikarbeit. Die Kontrolle ohne Zeichnen zeigt diese Kosten, ohne dass SwiftShader dieselben 4 Kerne belegt; mit SwiftShader liegt die Bildrate schon ohne Panel bei 41–44 Bildern/s und ist nicht aussagekräftig (wie in 1.3). Der Container-Prozessor ist langsamer als der Referenz-PC; die Werte sind deshalb eher zu hoch als zu niedrig. Die Messung auf echter Hardware steht aus (Abschnitt „Testprogramm").

## Ergebnisse (Kontrolle ohne Zeichnen)

Je Phase 4 s; „Update" = Einlesen, Zeilenmodelle mit Sortierung, React-Abgleich und DOM in einem Stück; „Modell" = davon `table.getRowModel()`.

| Panel | Zeilen | Rate | Bilder/s | Bilder > 20 ms | Update Mittel / p99 / max | davon Modell | Updates/s erreicht |
|---|---|---|---|---|---|---|---|
| aus | – | – | 60,0 | 0 | – | – | – |
| ohne Aktualisierung, scrollt | 5.000 | 0 | 60,0 | 0 | – | – | – |
| sortiert | 2.000 | 1–60/s | 59,6–60,1 | 0 | 4,2–6,4 / ≤ 8,6 / ≤ 12,3 ms | 2,0–3,9 ms | wie angefordert (60/s: 59) |
| sortiert | 5.000 | 1, 4/s | 60,0–60,2 | 0 | 9,2–9,8 / ≤ 13 / ≤ 15,8 ms | 6,5–7,3 ms | wie angefordert |
| sortiert | 5.000 | 10–60/s | 59,5–60,0 | 0–2 | 7,2–9,0 / ≤ 25,3 / ≤ 31 ms | 5,1–6,5 ms | 10 / 30 / 53 |
| sortiert | 20.000 | 1/s | 59,8 | 1 | 26 / 25 / 32 ms | 24 ms | 1 |
| sortiert | 20.000 | 4/s | 57,0 | 13 | 32 / 42 / 46 ms | 29 ms | 4 |
| sortiert | 20.000 | 10/s | 54,7 | 21 | 29 / 47 / 47 ms | 27 ms | 10 |
| sortiert | 20.000 | 30, 60/s | **39,5–41** | 63–66 | 30–33 / ≤ 58 / ≤ 69 ms | 28–30 ms | nur 20–22 |
| unsortiert | 20.000 | 10/s | 59,9 | 0 | 5,0 / 7,7 / 7,9 ms | 2,8 ms | 10 |
| alle Zeilen im DOM | 2.000 | 1/s | **31** | 9 | 178 / 185 / 309 ms | 2,4 ms | 1 |
| alle Zeilen im DOM | 2.000 | 4/s | **6** | 15 | 121 / 138 / 175 ms | 2,0 ms | 2,7 |
| aus (Ende) | – | – | 60,0 | 0 | – | – | – |

Lange Aufgaben (> 50 ms) gab es nur bei 20.000 sortierten Zeilen ab 4/s (1–8 je Phase, bis 77 ms) und ohne Virtualisierung (5–11, bis 595 ms).

**Im Kern-Prozess:** Kodieren der Tabelle 0,1–0,4 ms (2.000 und 5.000 Zeilen) bzw. 1,1–1,4 ms (20.000); Größe 43 / 107 / 430 KB. Rundlauf p50 5–6 ms bei 1–10/s; bei 30–60/s 4–13 ms (2.000 und 5.000 Zeilen) bzw. 39–40 ms (20.000 Zeilen, Renderer ausgelastet). Die Simulation lief in allen Phasen mit (Schritt im Mittel 1,0 ms; 7 Ticks in Rückstand über 114 s, auch in Phasen ohne Panel).

**Mit SwiftShader** zeigt sich dasselbe Muster bei niedrigerem Niveau: ohne Panel 41–44 Bilder/s; 2.000 und 5.000 Zeilen sortiert 32–39; 20.000 sortiert 22–38 mit bis zu 42 langen Aufgaben; 20.000 unsortiert 36; ohne Virtualisierung 5–11.

### Woher die Kosten kommen

- **Sortieren über TanStack** ist der größte Posten: 2,0–3,9 ms bei 2.000, 5–7 ms bei 5.000, 24–30 ms bei 20.000 Zeilen je Aktualisierung (rund 1,2–1,5 µs je Zeile). Ohne Sortierung kosten die Zeilenmodelle für 20.000 Zeilen 2,8 ms. Gegenprobe außerhalb des Spikes (Node 22, gleicher Container): Zeilennummern nach einer u16-Spalte mit `Array.prototype.sort` sortieren dauert 0,55 / 1,4 / 4,8 ms – TanStack braucht also das Fünf- bis Sechsfache des reinen Sortierens (Zeilenobjekte, Wert-Zwischenspeicher, Sortierfunktion über Spaltenzugriffe).
- **React und DOM** kosten bei Virtualisierung 2–3 ms je Aktualisierung (46 Zeilen × 11 Zellen), unabhängig von der Zeilenzahl.
- **Ohne Virtualisierung** dominiert der DOM: 2.000 Zeilen × 11 Zellen kosten 120–180 ms je Aktualisierung (Modell nur 2–2,4 ms).
- Das **Panel ohne Aktualisierung** kostet beim Durchscrollen nichts Messbares.

## Bewertung gegen die Akzeptanzkriterien

1. **Drückt das Panel die Bildrate des Spielfelds?** Im Container (Kontrolle): **nein** bis 5.000 Zeilen mit Sortierung nach einer Live-Spalte bei jeder Rate bis 60/s (höchstens 2 Bilder über 20 ms in 4 s) und bei 20.000 Zeilen ohne Sortierung. **Ja** bei 20.000 sortierten Zeilen ab 4/s (57 Bilder/s, 13 Bilder über 20 ms; ab 30/s nur noch 40 Bilder/s) und **immer ohne Virtualisierung** (schon 2.000 Zeilen: 6–31 Bilder/s, Hänger bis 0,6 s). Virtualisierung ist damit Pflicht.
2. **Tragbare Aktualisierungsrate:** bis 5.000 sortierte Zeilen ist jede Rate bis 60/s tragbar; empfohlen sind die in `project-context.md` Abschnitt 6 vorgesehenen „wenige Male pro Sekunde" (4/s kostet bei 5.000 Zeilen rund 10 ms alle 250 ms). Bei 20.000 sortierten Zeilen trägt nur 1/s; mehr verlangt eine andere Sortierung (unten). Die Grenze liegt dort, wo eine einzelne Aktualisierung in die Nähe eines Bildes (16,7 ms) kommt, weil sie den Hauptthread in einem Stück belegt.

**Vorbehalt:** Container-Prozessor statt Referenzhardware. Die Messung auf dem Referenz-PC und dem Mac mit dem Testprogramm steht aus; bis dahin bleibt „Oberfläche mit großen Tabellen" `[VORLÄUFIG]`.

### Mögliche Maßnahmen für große Tabellen (nicht umgesetzt, Entscheidung in 1.7 bzw. Phase 2)

- **Sortieren im Kern:** Rust sortiert die Zeilennummern und schickt die Reihenfolge oder nur den sichtbaren Ausschnitt; der Renderer formatiert dann nur rund 46 Zeilen. Passt zur Kern/Präsentation-Trennung, verlagert aber Anzeige-Logik (Sortierung ist nach Regel-002 erlaubte Anzeige-Logik in der Präsentation) in den Kern.
- **Seltener umsortieren als aktualisieren:** Reihenfolge z. B. einmal je Sekunde neu, Werte 4-mal je Sekunde; Zeilen springen dann seltener (auch für Spieler ruhiger).
- **TanStack nur für Spaltendefinition und Sortierzustand** (`manualSorting`), Sortieren selbst direkt auf den Typed Arrays (Gegenprobe: 4,8 statt 24–30 ms bei 20.000 Zeilen).
- Wie viele Zeilen die größten Tabellen im Spiel haben (Flüge eines Tages, Passagier- oder Personallisten mit 10.000+ Einträgen), ist noch nicht festgelegt.

### Befunde nebenbei

- **Vite 8.3.4 warnt viermal** beim Bündeln von TanStack Table 9 (`MODULE_LEVEL_DIRECTIVE`: `"use client"` in `Subscribe.js`, `createTableHook.js`, `createTableHookContexts.js`, `useTable.js` „may not be preserved"). Für eine Electron-Anwendung ohne Server-Komponenten ohne Bedeutung; für Phase 2 als benannte Ausnahme in den Warnungs-Bestand (`project-context.md` Abschnitt 7) oder über `onwarn` gezielt ausfiltern.
- **Vite im Bibliotheksmodus** ersetzt `process.env.NODE_ENV` nicht (React lädt sonst seine Entwicklungsfassung) und verkleinert ES-Module nicht (Bündel 314 KB, gzip 82 KB). Für Phase 2 regulären App-Build statt Bibliotheksmodus verwenden.
- **TanStack Table 9** ist gegenüber 8 umgebaut (`useTable` mit `tableFeatures`, Sortierung als registrierte Funktion `rowSortingFeature` mit `createSortedRowModel`, `sortFn_*` statt `sortingFn`); Beispiele für Version 8 passen nicht mehr. Die Pakete bringen eigene Agenten-Anleitungen mit (`node_modules/@tanstack/*/skills/`).
- `npm audit` meldet für die Werkzeuge nur die bekannte Schwachstelle „moderate" in `sprintf-js` über electron-builder (wie in 1.3), unter der Schwelle `high`.

## Testprogramm herunterladen und starten

Der Workflow `.github/workflows/spike-build-1-4.yml` baut dieses Programm für alle drei Systeme. Herunterladen: GitHub → Actions → „Spike-Build 1.4" → neuester Lauf → unten unter „Artifacts" `spike-1-4-win`, `spike-1-4-mac` oder `spike-1-4-linux` (Anmeldung bei GitHub nötig, 14 Tage verfügbar). Starten wie bei Spike 1.3 (Windows: SmartScreen → „Weitere Informationen" → „Trotzdem ausführen"; macOS: Systemeinstellungen → Datenschutz & Sicherheit → „Dennoch öffnen"; Linux: `./openairside-spike-1-4 --no-sandbox`).

Das Programm misst nach dem Start **rund 2 Minuten** selbständig (Fenster bitte nicht bedienen, nicht verdecken und nicht verkleinern; Netzteil anschließen). Danach steht unten links, wo das Ergebnis gespeichert wurde (Ordner „Downloads", Datei `openairside-spike-1-4-…json`). Diese Datei bitte schicken, zusammen mit der Angabe, auf welchem Rechner und Bildschirm sie entstand. Anschließend kann man im Fenster selbst sortieren, die Rate und Zeilenzahl ändern und das Panel ein- und ausblenden.
