# Spike 1.3 – Zustandsübergabe und Spielfeld-Darstellung

Fahrplan-Schritt 1.3 (Phase 1, ERKUNDUNG, Prototyp). **Wegwerf-Code.** Fragen: Wie lange dauert die Übergabe des Zustands je Bild vom Kern-Prozess an den Renderer (ADR-009)? Was ist zu übertragen – alles, nur Sichtbares oder nur Änderungen? Wie viele Bilder pro Sekunde erreicht ein PixiJS-Spielfeld mit 10.000 Personen? Ist eine Bauaktion im nächsten Bild sichtbar?

## Aufbau

- **Kern:** die Crate aus Spike 1.2 (`spikes/1.2-wegweiser`, Raster 512 × 512 × 2 Etagen, 29 Wegweiser). Für diesen Spike additiv ergänzt: `set_walls` (Wände an beliebigen Zellen, sofort im Raster), `poll_async` (neue Wegweiser werden übernommen, **sobald sie fertig sind** – ADR-010 Option A; weitere Bauaktionen während einer Neuberechnung werden gesammelt und danach nachgerechnet) und `visit_hot` (Lesezugriff auf die Positionen). Test: Nach zwei schnell aufeinanderfolgenden Bauaktionen und nach deren Rücknahme stimmen die Distanzen mit der Vollberechnung überein. Der Messpfad von 1.2 (fester Übernahme-Tick) ist unverändert.
- `bridge-core/` – Rust-Seite der Brücke ohne JavaScript-Anbindung (Regel-006):
  - eigener Simulations-Thread mit rayon-Pool, Takt einstellbar (Standard 60 Ticks/s, 0 = ungebremst);
  - **S-1 als Dreifachpuffer:** Der Simulations-Thread schreibt in seinen Puffer, der Leser hat seinen eigenen; beide tauschen unter einer kurzen Sperre nur Zeiger mit dem mittleren Puffer. Keiner wartet auf Kopier- oder Kodierarbeit des anderen, der Leser bekommt immer den neuesten fertigen Tick;
  - **Bildformat:** 48 Byte Kopf (Tick, Anzahl, Rasterversion, zuletzt ausgeführter Befehl, Messwerte), dann Personen entweder **kompakt** (x, y als u16 in 1/32 Zelle, Etage, Wegweiser-Nummer: 6 Byte) oder **f32** (12 Byte), dann die Rasteränderungen, die der Renderer noch nicht kennt (nur ganze Befehle);
  - **Filter auf Anfrage:** alles oder nur eine Etage innerhalb des Kamera-Ausschnitts (+4 Zellen Rand);
  - **S-2 in Minimalform:** Bauaktion „Wand setzen/entfernen" (3 × 3 Zellen). Der Befehl wird zu Beginn des nächsten Ticks ausgeführt; das Bild, das ihn enthält, trägt seine Nummer und die geänderten Zellen.
  - Tests: Filter, f32-Kodierung, nur unbekannte Rasteränderungen, Bauaktion erscheint im Bild (`cargo test --release -p bridge-core`).
- `sim-bridge/` – dünne napi-rs-Anbindung (Node-API 10), keine Logik.
- `app/` – Electron 44.7.0 mit Härtung wie in 1.1 (contextIsolation, Sandbox, kein nodeIntegration, eigenes Protokoll `app://`, strenge CSP **ohne** `unsafe-eval`). Kern im Utility-Prozess (`simhost.mjs`), Bilder per `MessagePort`, je Bild eine Anfrage (höchstens eine unterwegs).
  - **Spielfeld** mit PixiJS 8.20.1 (WebGL2): Raster je Etage als Textur (eine Zelle = ein Pixel, Darstellung „nearest"), Personen als Punkte, Kamera mit Zoom und Verschieben, Etagenwechsel, **zwei Detailstufen** (nah ab 6 Pixel je Zelle: runde Punkte 0,8 Zellen; fern: Quadrate 1,2 Zellen). Zwei Zeichenarten zum Vergleich: `particles` (ParticleContainer) und `sprites` (je Person ein Sprite).
  - **Eingebaute Messreihe:** je Personenzahl (10.000, 50.000) erst 3 s Zeitraffer, damit sich die Personen verteilen, dann je Kombination aus Übertragung (`view`/`full`) und Zeichenart vier Phasen à 4 s: Übersicht Etage 0, Nahansicht (16 Pixel je Zelle) an der dichtesten Stelle, Schwenk (4 Pixel je Zelle) um diese Stelle, Übersicht Etage 1. Dazu einmal je Personenzahl 12 Bauaktionen und deren Rücknahme im Abstand von 300 ms.
  - Danach interaktiv: rechte Maustaste ziehen = verschieben, Mausrad = zoomen, Linksklick = Wand bauen, mit Umschalt = Wand entfernen, `1`/`2` = Etage, `0` = ganze Karte, `t`/`f`/`r` = Übertragung/Format/Zeichenart umschalten.

Bauen und starten (Linux; macOS `.dylib`, Windows `.dll`):

```bash
cargo test --release -p bridge-core
cargo build --release -p sim-bridge
mkdir -p app/native && cp target/release/libsim_bridge.so app/native/sim_bridge.node
cd app && npm ci && npx electron .                      # interaktiv
npx electron . --series=full --exit --out=ergebnis.json # Messreihe
```

Weitere Schalter: `--persons=`, `--transfer=view|full`, `--format=compact|f32`, `--render=particles|sprites`, `--hz=`, `--phase-seconds=`, `--draw=0` (Kontrolle ohne Zeichnen), `--screenshot=<datei>`. Unter Linux ohne Bildschirm: `xvfb-run -a -s "-screen 0 1280x800x24" … --no-sandbox --enable-unsafe-swiftshader`.

## Messumgebung

Cloud-Container, 4 vCPU (Xeon 2,1 GHz), 16 GB, Linux, **keine Grafikkarte**: WebGL2 läuft über SwiftShader (Software-Rendering auf der CPU). Fenster 1280 × 772, Pixeldichte 1. Takt 60 Ticks/s, 4 Simulations-Threads, 1 Hintergrund-Thread für Wegweiser. Zwei Durchläufe der vollen Messreihe am 2026-10-13: **mit Zeichnen** (`results/container-swiftshader.json`) und **ohne Zeichnen** als Kontrolle (`results/container-ohne-zeichnen.json`).

**Bildraten aus diesem Container sind nicht aussagekräftig.** SwiftShader rechnet die Grafik auf denselben 4 Kernen, auf denen Simulation und Übergabe laufen: Mit Zeichnen steigt der Simulationsschritt von 0,7 auf 3,5 ms und der Rundlauf der Übergabe von 0,7 auf 20–90 ms. Die Kontrolle ohne Zeichnen zeigt die Übergabe ohne diese Verfälschung. Die Bildrate auf echter Grafik misst das Testprogramm (unten) auf den Rechnern des Projektinhabers.

## Ergebnisse

### Übergabe je Bild (Kontrolle ohne Zeichnen, Mittel der Phasen)

Rundlauf = Anfrage im Renderer bis Ankunft des Bildes (enthält Kodierung im Kern-Prozess, Kopie und Übertragung über die Prozessgrenze).

| Personen | Übertragung | Format | Größe je Bild | Rundlauf p50 / p99 | Kodieren im Kern | Einlesen im Renderer¹ |
|---|---|---|---|---|---|---|
| 10.000 | Ausschnitt, Übersicht | kompakt | 38 KB | 0,7 / 1,5 ms | 0,07 ms | 0,4 ms |
| 10.000 | Ausschnitt, nah | kompakt | 3,7 KB | 0,5 / 1,4 ms | 0,09 ms | 0,04 ms |
| 10.000 | alles | kompakt | 59 KB | 0,7 / 1,9 ms | 0,04 ms | 0,45 ms |
| 10.000 | alles | f32 | 117 KB | 1,1 / 2,8 ms | 0,06 ms | 0,5 ms |
| 50.000 | Ausschnitt, Übersicht | kompakt | 175 KB | 1,6 / 3,9 ms | 0,38 ms | 0,7 ms |
| 50.000 | Ausschnitt, nah | kompakt | 23 KB | 1,1 / 3,9 ms | 0,45 ms | 0,55 ms |
| 50.000 | alles | kompakt | 293 KB | 1,8 / 5,0–8,7 ms | 0,19 ms | 2,1–2,9 ms |
| 50.000 | alles | f32 | 586 KB | 2,6 / 4,8–7,0 ms | 0,25 ms | 2,9–3,2 ms |

¹ Dekodieren, Etagenfilter (bei „alles"), Positionen in die Partikel schreiben; Zeichenart `particles`.

Zusätzlich im Simulations-Thread je Tick: Packen der Positionen 0,1 ms (10.000) bzw. 0,6 ms (50.000). Bei 60 Ticks/s und 60 Bildern/s kam jeder Tick beim Renderer an (56–60 Zustände je Sekunde). In der Übersicht über Etage 0 ist knapp ein Drittel der Personen auf Etage 1 und wird beim Ausschnitt gar nicht übertragen.

**Nur Änderungen übertragen lohnt nicht:** Je Tick bewegen sich 97–99 % der Personen (10.000) bzw. 49–68 % (50.000, mehr Gedränge und Wartende). Eine Änderungsliste bräuchte je bewegter Person zusätzlich eine Kennung (4 Byte) und wäre damit größer als das kompakte Vollbild oder kaum kleiner, bei mehr Aufwand. Gemessen wurde der Anteil bewegter Personen; ein Änderungsformat wurde nicht gebaut.

### Zeichenart (Rechenzeit im Renderer, mit SwiftShader)

Die Zeit für `app.render()` misst nur, was der Renderer-Prozess selbst tut (Zeichenbefehle vorbereiten); die eigentliche Grafikarbeit kommt auf echter Hardware hinzu. Trotzdem zeigt sie den Abstand der beiden Zeichenarten:

| Personen gezeichnet | `particles`: render / Einlesen | `sprites`: render / Einlesen |
|---|---|---|
| ~6.500 (10.000, Übersicht) | 0,6 / 0,6 ms | 1,8–2,0 / 1,1–1,3 ms |
| ~2.000 (10.000, Schwenk) | 0,3 / 0,2 ms | 3,1 / 0,5 ms |
| ~30.000 (50.000, Übersicht) | 1,2–2,3 / 0,8–3,1 ms | 10–11 / 5,7–7,4 ms |
| ~16.000 (50.000, Schwenk) | 1,7 / 2,5 ms | 18,6 / 4,3 ms |

Sprites werden beim Schwenken teurer, weil Pixi für jedes Sprite die Transformation neu berechnet; Partikel nicht. **Für Personen (und später Gepäck, Fahrzeuge) ist `ParticleContainer` nötig;** mit Sprites ist bei 50.000 Objekten schon die CPU-Seite über dem Budget von 16,7 ms je Bild.

### Bauaktion

| | 10.000 Personen | 50.000 Personen |
|---|---|---|
| Klick bis zum gezeichneten Bild mit der Wand (Kontrolle) | p50 27 ms, max 31 ms | p50 33 ms, max 37 ms |
| in Bildern (60 Bilder/s) | 2 (max 2) | 2 (max 3) |
| davon Warten auf den nächsten Tick | Mittel 7,6 ms, max 11,7 ms | Mittel 10,3 ms, max 14,4 ms |
| Personen folgen neuen Wegweisern nach | Mittel 82 ms, max 150 ms (10 Ticks) | Mittel 84 ms, max 151 ms |

Mit SwiftShader: p50 81–89 ms, 3 Bilder (bei 20–40 Bildern/s). Der Ablauf ist immer: Klick → Befehl wartet auf den nächsten Tick (bis 16,7 ms bei 60 Ticks/s) → das nächste angeforderte Bild enthält die Wand → sie wird im darauffolgenden Bild gezeichnet. **Eine Bauaktion ist damit nicht im nächsten, sondern im übernächsten Bild sichtbar.** Die Wand erscheint so, wie der Kern sie gesetzt hat (Regel-002); die Personen weichen nach der Hintergrundberechnung der Wegweiser innerhalb von höchstens 150 ms aus, das Spiel wartet dabei nicht (ADR-010 Option A bestätigt).

### Befunde nebenbei

- **PixiJS 8.20.1, Fehler im globalen Build von `unsafe-eval`:** Die Ergänzung ersetzt den eval-basierten Code der Partikel nur in ihrer eigenen gebündelten Kopie von `ParticleBuffer`, nicht in der von `pixi.min.js`. Unter strenger CSP bricht `ParticleContainer` deshalb ab. Umgehung im Spike: eine Zeile in `renderer/main.js` setzt `PIXI.ParticleBuffer.prototype.generateParticleUpdate` auf die mitgelieferte Ersatzfunktion. Für Phase 2 (Bündelung mit Vite, ES-Module) erneut prüfen; mit ES-Modulen teilen sich Pixi und `pixi.js/unsafe-eval` dieselben Klassen.
- **Die Testkarte aus 1.2 stürzt bei 96 × 96 Zellen ab** (Index außerhalb des Rasters in `Grid::walkable`); ab 128 × 128 läuft sie. Wegwerf-Code, nicht behoben.
- **Zu Spielbeginn** stehen alle Personen am Eingang; ohne Zeitraffer misst die Reihe ein leeres Spielfeld. Deshalb 3 s ungebremster Takt vor den Messphasen.
- Windows- und Linux-Paket sowie Rauchtest liefen im ersten CI-Lauf grün (Lauf 38389783470).

## Bewertung gegen die Akzeptanzkriterien

1. **Dauer der Übergabe je Bild:** beantwortet. 0,5–1,1 ms bei 10.000 Personen, 1,1–2,6 ms bei 50.000 (p99 bis 8,7 ms bei 50.000 und vollständiger Übertragung). Die Kopie je Bild aus ADR-009 trägt. Empfehlung für Phase 2: kompaktes Format und Filter auf Etage und Kamera-Ausschnitt; keine Änderungslisten.
2. **Bauaktion im nächsten Bild:** beantwortet mit **nein** – im übernächsten (27–37 ms). Abweichung von der Regel in `project-context.md` Abschnitt 6 („im nächsten dargestellten Bild"). Mögliche Maßnahmen, nicht umgesetzt: Befehle in der Brücke sofort statt zu Beginn des nächsten Ticks ausführen; den Bauauftrag in der Präsentation sofort als Vorschau zeigen (wie die Bauplanung im Original, bevor Arbeiter bauen) – reine Darstellung, ändert keinen Zustand; höherer Takt. Entscheidung gehört in die Auswertung 1.7.
3. **Bilder pro Sekunde mit 10.000 Personen:** **offen.** Im Container nur mit Software-Grafik messbar (26–47 Bilder/s, nicht aussagekräftig). Messung mit dem Testprogramm auf Referenz-PC und Mac des Projektinhabers.

## Testprogramm herunterladen und starten

Der Workflow `.github/workflows/spike-build-1-3.yml` baut dieses Programm für alle drei Systeme. Herunterladen: GitHub → Actions → „Spike-Build 1.3" → neuester Lauf → unten unter „Artifacts" `spike-1-3-win`, `spike-1-3-mac` oder `spike-1-3-linux` (Anmeldung bei GitHub nötig, 14 Tage verfügbar). Starten wie bei Spike 1.1 (Windows: SmartScreen → „Weitere Informationen" → „Trotzdem ausführen"; macOS: Rechtsklick → „Öffnen", notfalls `xattr -cr "/Pfad/OpenAirside Spike 1.3.app"`; Linux: `./openairside-spike-1-3 --no-sandbox`).

Das Programm misst nach dem Start **rund 3 Minuten** selbständig (Fenster bitte nicht bedienen, nicht verdecken und nicht verkleinern; Netzteil anschließen). Danach steht oben rechts, wo das Ergebnis gespeichert wurde (Ordner „Downloads", Datei `openairside-spike-1-3-…json`). Diese Datei bitte schicken, zusammen mit der Angabe, auf welchem Rechner und Bildschirm sie entstand. Anschließend kann man im Fenster selbst bauen und zoomen.
