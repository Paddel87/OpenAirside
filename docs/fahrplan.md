# Fahrplan

<!-- Zentrales Arbeitsdokument von OpenAirside. Wird vor jeder Änderung gelesen (CLAUDE.md Abschnitt 2)
     und nach jedem Arbeitsschritt sowie zu Sessionende aktualisiert (Abschnitt 12).
     Klasse G: 7 Phasen (ADR-001). Phase 1 im vollen Schrittformat, spätere Phasen grob,
     Verfeinerung jeweils vor Phasenbeginn. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-13
- **Laufende Phase:** Phase 1 – Technischer Durchstich und Leistungsnachweis
- **Phasentyp:** ERKUNDUNG
- **Aktiver Schritt:** 1.5 – Leistungsnachweis auf den Zielsystemen
- **Nächster Schritt:** 1.7 – Auswertung und Entscheidung
- **Offene STOPP-Situationen:** keine

<!-- ANCHOR:phasen-typen -->
## Phasen-Typen

Jede Phase ist genau einem Typ zugeordnet. Der Typ bestimmt das Akzeptanzformat der Schritte.

### ERKUNDUNG

**Zweck:** Erkenntnis gewinnen, Annahmen validieren, Risiken vor der Umsetzung reduzieren.

- Akzeptanzkriterien sind **wissensbasiert**.
- Code ist Wegwerf-Code oder Spike, sofern nicht anders gekennzeichnet.
- Reduzierte Definition of Done; Erkenntnisse müssen in `decisions.md` oder `architecture.md` dokumentiert sein.
- Spike-Code, der weiterverwendet werden soll, durchläuft eine Stabilisierungsphase.

**Typische Schritt-Arten:** Spike, Prototyp, Vergleichsstudie, Lasttest / Messung.

### UMSETZUNG

**Zweck:** Geplante Funktionalität auf Basis belastbarer Architektur produktiv bauen.

- Akzeptanzkriterien sind **funktionsbasiert**.
- Berührte Architektur-Bestandteile müssen vor Schrittbeginn `[BELASTBAR]` sein – sonst Stopp.
- Volle Definition of Done (CLAUDE.md Abschnitt 9).
- Architektur-Lücke während der Umsetzung: Schritt stoppen, Lücke als `[OFFEN]` markieren, ERKUNDUNG-Schritt anlegen.

### STABILISIERUNG

**Zweck:** Härten, was in vorherigen Phasen entstanden ist.

- Akzeptanzkriterien sind **qualitätsbasiert**.
- Volle Definition of Done.
- Nach jeder Erkundungsphase, deren Ergebnisse weiterverwendet werden, Pflicht.

<!-- ANCHOR:schritt-format -->
## Schritt-Format

Jeder Schritt folgt diesem Schema. Abweichungen nur nach Freigabe.

```text
### [Phase].[Nummer]: Kurztitel

- **Status:** [OFFEN | VERSCHOBEN | IN ARBEIT | WARTET-AUF-FREIGABE | BLOCKIERT | ERLEDIGT | CI-AUSSTEHEND (mit Nachprüf-Schritt und Frist) | VERWORFEN]
- **Landeplatz (nur VERSCHOBEN):** [Ziel-Schritt-ID, z. B. „6.4" – ein Phasen-Verweis ohne Schritt-ID ist unzulässig, siehe CLAUDE.md Abschnitt 6]
- **Phasentyp-Kontext:** [ERKUNDUNG | UMSETZUNG | STABILISIERUNG] – ergibt sich aus der Phase
- **Schritt-Art (nur ERKUNDUNG):** [Spike | Prototyp | Vergleichsstudie | Lasttest | sonstiges]
- **Zeitbox (nur ERKUNDUNG):** [z. B. „maximal 4h Arbeit, dann Zwischenstand"]
- **Abhängigkeiten:** [Schritt-IDs, die vorher abgeschlossen sein müssen, oder "keine"]
- **Frist (Pflicht bei Abkündigungen, Ablaufdaten, Secret-Rotation):** [YYYY-MM-DD]
- **Freigabepflichtig:** [ja/nein – siehe CLAUDE.md Abschnitt 4]
- **Empfohlene Klasse:** [Mechanik | Routine | Entscheidung | Ausnahme] – [ein Satz Begründung aus CLAUDE.md Abschnitt 0]
- **Eingangskriterien:** [was muss gegeben sein, bevor der Schritt begonnen werden kann; bei UMSETZUNG: alle berührten Architektur-Bestandteile auf [BELASTBAR]]
- **Anforderungen (ab Klasse M):** [FR-IDs aus `docs/requirements.md`, die dieser Schritt umsetzt, oder „keine"]
- **Zu tun:** [konkrete Arbeitsanweisung; bei UMSETZUNG implementierungsnah, bei ERKUNDUNG: zu klärende Fragen]
- **Akzeptanzkriterien:** [phasentypabhängig – wissensbasiert / funktionsbasiert / qualitätsbasiert]
- **Betroffene Module:** [Modulnamen – wenn >1, ggf. aufsplitten]
- **Reifegrad-Wirkung:** [welche Architektur-Bestandteile werden durch diesen Schritt befördert oder zurückgestuft]
- **Artefakte:** [erwartete Dateien/Änderungen; bei ERKUNDUNG: ADRs, Architektur-Updates, Erkenntnisdokumente]
- **Notizen:** [optional: Hinweise, bekannte Fallstricke]
```

<!-- ANCHOR:aktuelle-phasen -->
## Aktuelle Phasen

### Phase 1: Technischer Durchstich und Leistungsnachweis – Typ: ERKUNDUNG

**Ziel:** Nachweis, dass der gewählte Stack (ADR-002) und das Architektur-Pattern (ADR-003) mit dem Wegweiser-Prinzip (ADR-004) das Leistungsziel tragen: 10.000 Personen, 60 Bilder pro Sekunde, kein Rückstand bei höchster Spielgeschwindigkeit auf der Referenzhardware, lauffähig auf integrierter Grafik, auf Windows, Linux und macOS.

**Abschlusskriterium:** Messergebnisse liegen vor und sind ausgewertet; ADR-002 ist per ERKENNTNIS-ADR bestätigt oder verworfen; die offenen Architekturfragen aus Schritt 1.1 und 1.2 sind entschieden.

**Reifegrad-Erwartung am Phasenende:** Stack, Architektur-Pattern, Wegfindung, S-1 Zustandspuffer und NFR Performance auf `[BELASTBAR]` (bei Bestehen); Betriebsart des Kerns und Datenmodell im Kern von `[OFFEN]` auf `[VORLÄUFIG]` oder `[BELASTBAR]`.

**Ursprünglicher Schrittplan:** 7 Schritte, festgehalten am 2026-10-13.

**Pflichtfrage am Phasenende:** ADR „Weiterbauen, umbauen oder neu aufsetzen" in Schritt 1.7 (CLAUDE.md Abschnitt 12).

**Hinweis:** Gesamter Code dieser Phase ist Spike-Code in einem eigenen Verzeichnis (`spikes/`). Was weiterverwendet wird, wird in Phase 2 sauber neu aufgebaut oder durchläuft eine Stabilisierung.

#### 1.1: Betriebsart des Rust-Kerns

- **Status:** ERLEDIGT (2026-10-13) – Option A freigegeben, ADR-009; Option B zurückgestellt nach Q.2
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session, dann Zwischenstand
- **Abhängigkeiten:** keine
- **Freigabepflichtig:** ja, für das Ergebnis (Architekturentscheidung)
- **Empfohlene Klasse:** Entscheidung – Ergebnis ist eine Architekturentscheidung (Eskalations-Auslöser 1 und 3).
- **Eingangskriterien:** Repository angelegt; Versionen nach ADR-002 (Versions-Verifikation vom 2026-10-13)
- **Anforderungen (ab Klasse M):** FR-017, FR-040 (Vorarbeit)
- **Zu tun:**
  - Minimalen Rust-Kern bauen, der einen Zustand mit 10.000 Einträgen je Simulationsschritt fortschreibt und mehrere Kerne nutzt.
  - Variante A: als natives Node-Modul (napi-rs) im Electron-Hauptprozess oder einem Utility-Prozess, Übergabe des Zustands über gemeinsamen Speicher.
  - Variante B: als WebAssembly in einem Web Worker mit gemeinsamem Speicher und Threads.
  - Vergleichen: Rechenleistung, Mehrkern-Nutzung, Übergabe an den Renderer ohne Serialisierung, Build-Aufwand für drei Plattformen (inkl. macOS Universal), Debugbarkeit, Komplexität für spätere Arbeit.
- **Akzeptanzkriterien:** Beide Varianten laufen; Messwerte liegen vor; eine Variante ist begründet empfohlen und dem Projektverantwortlichen vorgelegt.
- **Betroffene Module:** Takt & Weltzustand, Brücke, Plattform
- **Reifegrad-Wirkung:** Betriebsart des Kerns `[OFFEN]` → `[VORLÄUFIG]`
- **Artefakte:** Spike-Code, ADR (ERKENNTNIS), Update `architecture.md`
- **Notizen:** Electron-Härtung (contextIsolation, Sandbox) von Anfang an beachten, sonst sind Messungen nicht repräsentativ.
- **Zwischenstand 2026-10-13:** Spike und Messprotokoll in `spikes/1.1-betriebsart/README.md` (Rohdaten `results/matrix.jsonl`). Beide Varianten laufen in gehärtetem Electron 44.7.0; Akzeptanzkriterien „beide laufen" und „Messwerte liegen vor" erfüllt, „begründet empfohlen und vorgelegt" mit der Vorlage unten. Messung auf 4-vCPU-Container ohne GPU, nicht auf Referenzhardware; Windows und macOS nicht gemessen.
- **Entscheidung:** am 2026-10-13 vom Projektinhaber: „A, freigegeben, B perspektivisch“ → ADR-009, Regel-006, Landeplatz für B: Q.2.
- **Entscheidungsvorlage (beantwortet):**

  ```text
  ENTSCHEIDUNG ERFORDERLICH
  Kategorie: 1 Architekturänderung (Betriebsart des Kerns, Prozessgrenze Kern/Präsentation)
  Kontext: ADR-002 lässt offen, ob der Rust-Kern als natives Node-Modul oder als
      WebAssembly läuft. Spike 1.1 hat beide Varianten mit 10.000 Personen gemessen.
  Optionen:
    A: napi-rs-Modul im Electron-Utility-Prozess, Bild je Frame per MessagePort kopiert
       → Für deine Vision: meiste Rechenreserve für große Flughäfen (1,3–1,5 × schneller,
         alle Kerne mit stabilem Rust); stürzt die Simulation ab, bleibt das Fenster offen
         und der Spielstand rettbar. Preis: je Plattform ein eigener Build, und die
         Bilddaten werden je Frame kopiert (bei 10.000 Personen ~1 ms, unkritisch).
    B: WebAssembly mit mehreren Threads im Web Worker, gemeinsamer Speicher
       → Für deine Vision: einfachster Vertrieb (eine Datei für alle Systeme), keine
         Kopie zur Anzeige. Preis: ein Drittel weniger Rechenreserve und Bau nur mit
         Nightly-Rust (instabile Werkzeugkette, Widerspruch zu ADR-002/„Reproduzierbarkeit").
    C: WebAssembly mit einem Thread (stabiles Rust)
       → Für deine Vision: einfach und stabil, aber nur ein Prozessorkern – bei schwerer
         Simulation bis 3,9 × langsamer; gefährdet das 10.000-Personen-Ziel.
  Empfehlung: A – templates/architektur-heuristiken.md Teil 1, Default-Bias „die einfachere
      Option gewinnt, bis das Gegenteil belegt ist": C ist am einfachsten, aber durch die
      Vision-Anforderung 10.000 Personen (FR-040) und die Messung widerlegt. Zwischen A und B
      ist keine klar einfacher (A: Prozessgrenze + 6 Builds; B: instabile Nightly-Kette);
      den Ausschlag geben gemessene Reserve und stabile Werkzeuge (ADR-002, CLAUDE.md
      Abschnitt 6 „Reproduzierbarkeit"). Übergabekosten von A gemessen tragbar.
  Trade-off: gewinnst Rechenreserve, Absturzsicherheit, stabile Werkzeuge; gibst auf:
      Zero-Copy-Übergabe und den Ein-Datei-Build (CI-Matrix mit 6 Zielen, macOS per lipo).
  DIE FRAGE AN DICH: Ist dir Rechenreserve für sehr große Flughäfen wichtiger als ein
      einfacher Build- und Vertriebsweg?
  Konfidenz: mittel – Vergleich eindeutig, aber auf 4 vCPU ohne GPU gemessen; Windows-
      und macOS-Builds nur nach Dokumentation bewertet.
  Umkehrbarkeit: billig, solange der Kern (sim-core) frei von Bindungscode bleibt –
      der Wechsel betrifft dann nur Brücke und Build (~150 Zeilen je Anbindung im Spike).
  Blockiert Arbeit an: 1.2, 1.3 (und damit 1.4–1.7)
  ```

#### 1.2: Wegweiser-Prinzip mit 10.000 Personen

- **Status:** ERLEDIGT (2026-10-13) – Option A freigegeben, ADR-010; größte Karte: Referenzszenario mit Reserve
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal zwei Sessions, dann Zwischenstand
- **Abhängigkeiten:** 1.1
- **Freigabepflichtig:** ja, für das Ergebnis (Datenmodell und Parallelisierung)
- **Empfohlene Klasse:** Entscheidung – Datenmodell und Parallelisierung sind Architekturentscheidungen; Beförderung auf BELASTBAR möglich.
- **Eingangskriterien:** Betriebsart aus 1.1 entschieden
- **Anforderungen (ab Klasse M):** FR-017
- **Zu tun:**
  - Testraster in Größe des Referenzszenarios mit zwei Etagen, Wänden, einer Sicherheitszone mit Kontrolle und mehreren Zieltypen (Check-in, Sicherheit, Gates, Toiletten, Ausgang).
  - Wegweiser je Ziel berechnen, 10.000 Personen mit wechselnden Zielen folgen lassen.
  - Bauänderung während der Simulation: inkrementelle Aktualisierung, Messung der Verzögerung.
  - Unerreichbarkeit erkennen und Ursache ermitteln.
  - Datenhaltung im Kern (kompakte Datenblöcke, Parallelisierung) festlegen und Determinismus prüfen.
  - Bodenmarkierung als Kachelbewertung in einer einfachen Form erproben.
- **Akzeptanzkriterien:** Wir wissen, wie viel Rechenzeit ein Simulationsschritt mit 10.000 Personen braucht, wie viel Speicher die Wegweiser belegen und wie schnell eine Bauänderung wirkt; Datenmodell und Parallelisierungsstrategie sind entschieden; Determinismus ist bestätigt oder die Abweichung begründet.
- **Betroffene Module:** Takt & Weltzustand, Welt & Bau, Wegfindung, Personen
- **Reifegrad-Wirkung:** Wegfindung `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen); Datenmodell im Kern `[OFFEN]` → `[VORLÄUFIG]`
- **Artefakte:** Spike-Code, Messprotokoll, ADR (ERKENNTNIS), Update `architecture.md`
- **Notizen:** Lehre aus dem Original: keine Prozesse, die sich über die Spielzeit ansammeln; Langzeitlauf mitmessen.

- **Zwischenstand 2026-10-13:** Prototyp und Messprotokoll in `spikes/1.2-wegweiser/README.md` (Rohdaten `results/`). Erkenntnisse zu allen Fragen liegen vor: Schritt 0,2 ms mit 10.000 Personen (4 Threads), Wegweiser 14,5 MB und 0,2–0,4 s Neuberechnung (512² × 2, 29 Ziele, 4 Threads), Unerreichbarkeit mit Ursache erkannt, Determinismus bitgleich bestätigt, Bodenmarkierung als Kostenfaktor erprobt, kein Leistungsabbau über 60.000 Ticks. Offen: Rastergröße des Referenzszenarios und der größten Karte ist nirgends festgelegt (Informationslücke); getestet mit 2 statt 5 Etagen.
- **Entscheidung:** am 2026-10-13 vom Projektinhaber: „A, freigegeben, Referenzszenario mit Reserve" → ADR-010.
- **Nicht abgedeckt (Hinweis, kein Verschieben):** „Suchen ohne Schilder" und Feinwege zum Einzelziel waren nicht Teil von 1.2; sie gehören zu FR-017 und Landeplatz: Schritt 3.3 (dort als vorgeschaltete Erkundung eingetragen).
- **Entscheidungsvorlage (beantwortet):**

  ```text
  ENTSCHEIDUNG ERFORDERLICH
  Kategorie: 4 Datenmodell (Personen im Kern) und 1 Architektur (Parallelisierung,
      Übernahme neuer Wegweiser)
  Kontext: Spike 1.2 zeigt: Personen kosten wenig (0,2 ms je Tick für 10.000),
      teuer ist die Neuberechnung der Wegweiser nach Bauänderungen (0,2–0,4 s bei
      512 × 512 × 2, Sekunden bei 1024 × 1024). Feste Übernahme-Zeitpunkte machen
      den Spielverlauf exakt wiederholbar, lassen das Spiel aber warten.
  Gemeinsam für alle Optionen (Empfehlung, keine Alternative mit Vorteil gemessen):
      Personen als getrennte Felder „heiß/kalt" in eigenem Rust-Code (kein ECS-Paket,
      keine neue Abhängigkeit); Tick in zwei Phasen (Bewegung, Dichtezählung) parallel
      mit rayon; ein Wegweiser je Ziel, 1 Byte je Zelle; Neuberechnung vollständig im
      Hintergrund auf allen Kernen (inkrementelle Reparatur nicht als Standard).
  Optionen:
    A: Neue Wegweiser übernehmen, sobald fertig; für Tests und Wiederholungen ein
       Prüfmodus mit festem Übernahme-Tick
       → Für deine Vision: Bauen ohne Wartezeit bei jeder Größe; Personen finden
         neue Wege nach Bruchteilen einer Sekunde (große Karten: Sekunden). Ein
         normales Spiel ist nicht bitgenau wiederholbar, Tests sind es.
    B: Immer fester Übernahme-Tick
       → Für deine Vision: jedes Spiel exakt wiederholbar; nach großen Umbauten auf
         großen Karten stockt das Spiel bei hohem Tempo kurz (gemessen bis 6,8 s
         bei 1024²) – widerspricht „Bauen ohne Wartezeit".
    C: Hierarchische Wegweiser (Karte in Sektoren, nur betroffene Sektoren neu)
       → Für deine Vision: auch riesige Karten mit fünf Etagen bleiben schnell;
         deutlich mehr Aufwand und Fehlerrisiko; eigener Erkundungsschritt nötig.
  Empfehlung: A – Default-Bias „die einfachere Option gewinnt, bis das Gegenteil
      belegt ist" (templates/architektur-heuristiken.md Teil 1): A ist einfach und
      erfüllt „Bauen ohne Wartezeit"; B verletzt diese Vision-Anforderung; C ist erst
      belegt nötig, wenn die Karten deutlich größer als 512 × 512 werden.
  Trade-off: gewinnst sofortiges Bauen und einfache Technik; gibst bitgenaue
      Wiederholbarkeit normaler Spiele auf (Spielstände bleiben unberührt).
  DIE FRAGE AN DICH: Wie groß soll die größte Karte sein – etwa so groß, dass das
      Referenzszenario (3 Terminals, 2 Bahnen, 17 Gates) mit etwas Reserve passt,
      oder deutlich größer (z. B. ein Drehkreuz mit 100+ Gates)?
  Konfidenz: mittel – Messungen eindeutig, aber Kartengröße und Etagenzahl des
      Ernstfalls unbekannt, 4 vCPU statt Referenzhardware.
  Umkehrbarkeit: billig für A ↔ B (eine Einstellung); C ersetzt nur das Innere des
      Moduls Wegfindung (Schnittstelle „Richtung je Ziel und Zelle" bleibt).
  Blockiert Arbeit an: Abschluss 1.2; 1.3 kann parallel beginnen (nutzt den Kern aus 1.2)
  ```

#### 1.3: Zustandsübergabe und Spielfeld-Darstellung

- **Status:** ERLEDIGT (2026-10-13) – Messungen auf Referenz-PC (Windows) und Mac M3 Pro; Linux nach ADR-011 in 7.2
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session, dann Zwischenstand
- **Abhängigkeiten:** 1.1, 1.2
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Entscheidung – Beförderung von S-1 auf BELASTBAR (Eskalations-Auslöser 4).
- **Eingangskriterien:** Kern aus 1.2 lauffähig
- **Anforderungen (ab Klasse M):** FR-011, FR-040 (Vorarbeit)
- **Zu tun:**
  - Zustandspuffer (S-1) mit Doppel- oder Dreifachpuffer im Kern-Prozess umsetzen, Übertragung per `MessagePort` an den Renderer (ADR-009); Port-Übergabe als Handshake.
  - Übertragungsumfang klären: vollständiges Bild gegen nur Sichtbares (Kamera-Ausschnitt) oder nur Änderungen; Messung auch mit 50.000 Objekten (Spike 1.1: 3,4 ms je Bild bei vollständiger Übertragung).
  - PixiJS-Spielfeld in Electron: Raster, 10.000 Personen als Sprites, Zoom und Verschieben, Etagenwechsel, einfache Detailstufen.
  - Befehlskanal (S-2) in Minimalform: Bauaktion aus dem Spielfeld.
- **Akzeptanzkriterien:** Wir wissen, wie viele Bilder pro Sekunde das Spielfeld mit 10.000 Personen erreicht, wie lange die Übergabe je Bild dauert und ob eine Bauaktion im nächsten Bild sichtbar ist.
- **Betroffene Module:** Brücke, Spielfeld, Plattform
- **Reifegrad-Wirkung:** S-1 `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Spike-Code, Messprotokoll, Update `architecture.md`
- **Notizen:** WebGL2 als Grundlage, WebGPU nicht voraussetzen.
- **Zwischenstand 2026-10-13:** Spike und Messprotokoll in `spikes/1.3-spielfeld/README.md` (Rohdaten `results/`). Brücke mit Dreifachpuffer, kompaktem Bildformat, Filter auf Etage und Kamera-Ausschnitt und Bauaktion (S-2 minimal); PixiJS-Spielfeld mit Kamera, Etagen, zwei Detailstufen; eingebaute Messreihe. Kern aus 1.2 additiv um Bauen an beliebigen Zellen mit Übernahme der Wegweiser „sobald fertig" (ADR-010 A) ergänzt. Testprogramm für drei Plattformen über `.github/workflows/spike-build-1-3.yml` (Lauf 38389783470: alle drei Plattformen grün).
  - **Übergabe:** 0,5–1,1 ms je Bild bei 10.000, 1,1–2,6 ms bei 50.000 Personen (Kontrolle ohne Zeichnen, 4 vCPU); Änderungslisten lohnen nicht (97–99 % bzw. 49–68 % der Personen bewegen sich je Tick). Empfehlung: kompakt + Ausschnitt.
  - **Zeichenart:** `ParticleContainer` nötig; Sprites kosten bei 50.000 Personen schon auf der CPU-Seite 10–19 ms je Bild.
  - **Bauaktion:** sichtbar im **übernächsten** Bild (27–37 ms), nicht im nächsten – Abweichung von `project-context.md` Abschnitt 6; Maßnahmen im Messprotokoll, Entscheidung in 1.7. Personen weichen nach höchstens 150 ms aus, ohne dass das Spiel wartet.
  - **Referenz-PC Windows (2026-10-13, RX 7700 XT):** 10.000 Personen 60 Bilder/s in allen Kombinationen, kein Bild über 20 ms; 50.000 mit `particles` ebenfalls 60, mit `sprites` bis 46 Bilder/s beim Schwenken; Bauaktion auch hier im übernächsten Bild (p50 33,5 ms). Daten: `spikes/1.3-spielfeld/results/referenz-pc-windows-rx7700xt.json`.
  - **Mac M3 Pro (2026-10-13, eingebautes Display 120 Hz):** 10.000 und 50.000 Personen mit `particles` 120 Bilder/s, Bild p99 ≤ 9,4 ms; Sprites bei 50.000 beim Schwenken 80–109; Bauaktion p50 27 ms. Daten: `spikes/1.3-spielfeld/results/mac-m3pro.json`.
  - **Abschluss:** Akzeptanzkriterien beantwortet (Übergabe, Bauaktion im übernächsten Bild statt im nächsten → Entscheidung in 1.7, Bildrate). Linux auf echter Hardware verschoben auf 7.2 (ADR-011). Ursprünglich offen: Bilder pro Sekunde auf echter Grafik. Im Container nur Software-Rendering (SwiftShader), das die CPU mit Simulation und Übergabe teilt – Werte nicht aussagekräftig. Messung mit dem Testprogramm auf Referenz-PC (Windows, Linux per Live-USB) und Mac; Anleitung im Messprotokoll, Abschnitt „Testprogramm herunterladen und starten". S-1 bleibt bis dahin `[VORLÄUFIG]`.

#### 1.4: Großes Tabellen-Panel neben dem Spielfeld

- **Status:** ERLEDIGT (2026-10-13) – gemessen auf Referenz-PC (Windows) und Mac M3 Pro; Linux nach ADR-011 in 7.2
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.3
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Routine – durchspezifizierter Prototyp ohne Architekturentscheidung.
- **Eingangskriterien:** Spielfeld aus 1.3 lauffähig
- **Anforderungen (ab Klasse M):** FR-039
- **Zu tun:** Ein Panel im Stil des Flugmonitors mit mehreren tausend Zeilen, Sortierung und Live-Werten (React, TanStack Table, virtualisiert, gedrosselte Aktualisierung) neben dem laufenden Spielfeld.
- **Akzeptanzkriterien:** Wir wissen, ob das Panel die Bildrate des Spielfelds drückt und welche Aktualisierungsrate tragbar ist.
- **Betroffene Module:** Oberfläche, Brücke
- **Reifegrad-Wirkung:** Oberfläche mit großen Tabellen `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Spike-Code, Messprotokoll
- **Notizen:** –
- **Zwischenstand 2026-10-13:** Spike und Messprotokoll in `spikes/1.4-tabellen/README.md` (Rohdaten `results/`). Spielfeld und Brücke aus 1.3 unverändert; neu `panel-core` (synthetische Flugtabelle mit Live-Werten aus dem Tick, spaltenweise kodiert, 22 Byte je Zeile) und ein Panel mit React 19.2.8 und TanStack Table 9.2.8 (Sortierung), gebündelt mit Vite 8.3.4, eigene Virtualisierung ohne neue Abhängigkeit. Testprogramm über `.github/workflows/spike-build-1-4.yml` (Lauf 38404046104: Pakete und Rauchtest auf allen drei Plattformen grün).
  - **Container (Kontrolle ohne Zeichnen, 4 vCPU):** bis 5.000 Zeilen, sortiert nach einer Live-Spalte, bei jeder Rate bis 60/s 59,5–60 Bilder/s (höchstens 2 Bilder über 20 ms in 4 s), Aktualisierung 4–10 ms. 20.000 sortierte Zeilen drücken die Bildrate ab 4/s (57 Bilder/s; ab 30/s 40), weil TanStack je Aktualisierung 24–30 ms sortiert; unsortiert 60 Bilder/s. Ohne Virtualisierung schon bei 2.000 Zeilen 6–31 Bilder/s → Virtualisierung Pflicht. Maßnahmen für große Tabellen (Sortieren im Kern, seltener umsortieren, eigenes Sortieren auf den Spalten) im Messprotokoll, Entscheidung in 1.7.
  - **Referenz-PC Windows (2026-10-13):** virtualisiertes Panel drückt die Bildrate nicht – bis 5.000 sortierte Zeilen bei jeder Rate bis 60/s, 20.000 sortierte Zeilen bis 30/s (je Aktualisierung rund 20 ms, bei 60/s nur 30 erreicht); ohne Virtualisierung 17–46 Bilder/s schon bei 2.000 Zeilen. Daten: `spikes/1.4-tabellen/results/referenz-pc-windows-rx7700xt.json`.
  - **Mac M3 Pro (2026-10-13, 120 Hz):** bis 5.000 sortierte Zeilen 120 Bilder/s bei jeder Rate; 20.000 sortierte Zeilen 110–120 Bilder/s (Aktualisierung 11–19 ms); ohne Virtualisierung 59–94. Daten: `spikes/1.4-tabellen/results/mac-m3pro.json`.
  - **Abschluss:** Akzeptanzkriterien beantwortet. Das virtualisierte Panel drückt die Bildrate bis 5.000 sortierte Zeilen nicht; tragbar ist „wenige Male pro Sekunde" bis 20.000 Zeilen; Virtualisierung ist Pflicht. Maßnahmen für sehr große sortierte Tabellen (Sortieren im Kern, seltener umsortieren, eigenes Sortieren) zur Entscheidung in 1.7.

#### 1.5: Leistungsnachweis auf den Zielsystemen

- **Status:** IN ARBEIT (seit 2026-10-13)
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Lasttest / Messung
- **Zeitbox (nur ERKUNDUNG):** maximal eine Session
- **Abhängigkeiten:** 1.2, 1.3, 1.4
- **Freigabepflichtig:** ja (Bestätigung oder Verwerfung von ADR-002)
- **Empfohlene Klasse:** Entscheidung – bestätigt oder verwirft ADR-002 (STRATEGISCH).
- **Eingangskriterien:** Durchstich aus 1.1–1.4 lauffähig; Zugang zu Testsystemen: Mittelklasse-PC (Referenz), Rechner mit integrierter Grafik (z. B. Mac des Projektverantwortlichen) (Linux: ADR-011)
- **Anforderungen (ab Klasse M):** FR-040
- **Zu tun:**
  - Rastergröße des Referenzszenarios mit Reserve festlegen (Vorgabe ADR-010) und Wegweiser mit fünf Etagen messen (Speicher, Neuberechnung).
  - Messung im Szenario mit 10.000 Personen bei höchster Spielgeschwindigkeit: Bildrate, Rechenzeit pro Simulationsschritt, Rückstand, Speicher über einen Langzeitlauf.
  - Wiederholung auf integrierter Grafik (Mac). Linux: nur Bau und Start im CI, Leistungsmessung in 7.2 (ADR-011).
  - Ergebnis gegen die Zielwerte aus `project-context.md` Abschnitt 6 bewerten.
- **Akzeptanzkriterien:** Bewertung liegt vor: Ziel erreicht / mit benannten Maßnahmen erreichbar / nicht erreichbar. Entscheidungsvorlage an den Projektverantwortlichen.
- **Betroffene Module:** alle des Durchstichs
- **Reifegrad-Wirkung:** bei Bestehen: Stack, Pattern, NFR Performance `[VORLÄUFIG]` → `[BELASTBAR]`; bei Scheitern: ADR mit Neubewertung (Rückfalloption Godot-Präsentation mit Rust-Kern)
- **Artefakte:** Messprotokoll, ADR (ERKENNTNIS), Update `architecture.md` Abschnitt 9
- **Notizen:** Falls keine Mittelklasse-Referenz verfügbar ist, Messung auf verfügbarer Hardware und Hochrechnung mit Begründung; das Fehlen ist dann als Informationslücke zu melden.
- **Testsysteme (Stand 2026-10-13, Angabe des Projektinhabers):** Referenz-PC (8 Kerne, Mittelklasse-Grafikkarte) und Mac mit integrierter Grafik verfügbar. Linux: Messung auf echter Hardware verschoben auf 7.2, früher durch Mitwirkende (ADR-011; Live-USB auf dem Referenz-PC vom Projektinhaber abgelehnt, weil die Arbeitsrechner nicht neu gestartet werden sollen).
- **Zwischenstand 2026-10-13:** Spike und Messprotokoll in `spikes/1.5-leistung/README.md` (Rohdaten `results/`). Kern aus 1.2 additiv auf fünf Etagen verallgemeinert (U-Bahn, Gepäckausgabe, Halle, Abflugebene, Lounges; 36 Wegweiser), Zwei-Etagen-Pfad bitgleich zu 1.2. Testprogramm mit Langzeitlauf (10 min, 180 Ticks/s als Auslegung von „3×", Panel, Bauaktionen alle 10 s, Speicher je Prozess) über `.github/workflows/spike-build-1-5.yml`.
  - **Container (4 vCPU):** Wegweiser mit fünf Etagen, volle Berechnung: 512² 45 MB / 0,47 s, 768² 101 MB / 0,98 s, 1024² 180 MB / 2,7 s (36 Wegweiser, 4 Threads); mit 64 Wegweisern 80 / 180 / 320 MB und 0,6 / 1,7 / 4,1 s. Speicherspitze während der Neuberechnung etwa doppelt. Simulationsschritt 10.000 Personen auf fünf Etagen 0,25–0,29 ms, höchste Tickrate 3.500–3.900/s – 180 Ticks/s sind unkritisch.
  - **Befund:** 1.3 und 1.4 maßen Bauaktionen mit inkrementeller Reparatur (Voreinstellung aus 1.2), nicht mit der in ADR-010 gewählten vollen Neuberechnung; 1.5 misst die volle.
  - **Offen:** Rastergröße (Entscheidungsvorlage unten); Langzeitlauf auf Referenz-PC und Mac; Bewertung gegen die Zielwerte; Vorlage zu ADR-002.
- **Entscheidungsvorlage Rastergröße (offen):**

  ```text
  ENTSCHEIDUNG ERFORDERLICH
  Kategorie: 4 Datenmodell (Größe des Weltrasters), Folge aus ADR-010
  Kontext: ADR-010 verlangt „Referenzszenario mit Reserve". Das Raster 512 × 512 aus
      Spike 1.2 fasst die Elemente des Referenzszenarios (17 Gates, Check-in, Kontrolle,
      zwei Ebenen) ohne Reserve. Wie viele Meter eine Zelle darstellt, ist nirgends
      festgelegt; die Größe wird deshalb relativ zu diesem Layout gewählt. Kosten mit
      fünf Etagen im Container gemessen (4 Threads; Referenz-PC hat 8 Kerne).
      Gemessen ist das Raster, auf dem Personen gehen (Gebäude, Gehwege). Das Layout
      aus 1.2 enthält keine Bahnen und kein Vorfeld; liegen diese später im selben
      Raster wie im Original, braucht die Karte mehr Fläche oder Wegweiser nur über
      bebauten Bereichen – das klärt die Planung von Phase 2 (Schritt 1.7).
  Optionen:
    A: 512 × 512 Zellen je Etage (Referenzszenario ohne Reserve)
       → Für deine Vision: kleinster Speicher (45–80 MB Wegweiser), Wege passen sich
         nach Umbauten in rund 0,5 s an. Ein Flughafen wie das Referenzszenario passt,
         größer wird es eng.
    B: 768 × 768 (gut doppelte Fläche des Referenzszenarios)
       → Für deine Vision: Platz für das Referenzszenario plus Ausbau (weitere Pier,
         größere Halle); 100–180 MB, Wege passen sich nach rund 1–1,7 s an (auf
         8 Kernen voraussichtlich schneller). Bauen selbst wartet nie (ADR-010).
    C: 1024 × 1024 (vierfache Fläche)
       → Für deine Vision: Richtung Drehkreuz; 180–320 MB, Anpassung 2,7–4,1 s.
         Weicht von ADR-010 („Referenzszenario mit Reserve") ab; sinnvoll erst mit
         sparsamerer Speicherung oder Sektoren (ADR-010 Option C).
  Empfehlung: B – templates/architektur-heuristiken.md Teil 1, Default-Bias „die
      einfachere Option gewinnt, bis das Gegenteil belegt ist": A und B sind gleich
      einfach (eine Zahl), B erfüllt als einzige „mit Reserve" ohne neue Technik.
  Trade-off: gewinnst Ausbaufläche; gibst gut die doppelte Speicher- und Rechenlast
      gegenüber A auf.
  DIE FRAGE AN DICH: Reicht als größte Karte gut die doppelte Fläche des
      Referenzszenarios (3 Terminals, 2 Bahnen, 17 Gates)?
  Konfidenz: mittel – Kosten gemessen, aber Zellgröße in Metern und spätere
      Kartenelemente (Bahnen, Vorfeld, Straßen) im Raster noch nicht festgelegt.
  Umkehrbarkeit: billig bis Phase 3 (eine Einstellung; Spielstände gibt es noch nicht),
      danach teuer (Spielstände und Inhalte hängen an der Größe).
  Blockiert Arbeit an: Bewertung in 1.5; 1.7
  ```

#### 1.6: Verpackung für drei Plattformen

- **Status:** ERLEDIGT (2026-10-13) – Pakete für drei Plattformen aus der CI; Start auf Windows und macOS durch den Projektinhaber, Linux per CI-Rauchtest (ADR-011)
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** Prototyp
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.1
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Routine – Prototyp der Verpackung nach festen Vorgaben; Ergebnis wird in 1.7 bewertet.
- **Eingangskriterien:** Betriebsart aus 1.1 entschieden
- **Anforderungen (ab Klasse M):** FR-045
- **Zu tun:** Builds des Durchstichs für Windows, Linux und macOS (Universal) über GitHub Actions erzeugen; Start auf jedem System prüfen; Verhalten von macOS bei unsignierter Anwendung dokumentieren.
- **Akzeptanzkriterien:** Wir wissen, ob und mit welchem Aufwand alle drei Plattformen aus der CI gebaut werden können, und was ein Spieler unter macOS ohne Signatur tun muss.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** Verpackung `[VORLÄUFIG]` → `[BELASTBAR]` (bei Bestehen)
- **Artefakte:** Workflow-Entwurf, Erkenntnisnotiz
- **Notizen:** Änderungen an der Build-Pipeline sind ab Phase 2 freigabepflichtig (CLAUDE.md 4.7).
- **Zwischenstand 2026-10-13:** Workflow `.github/workflows/spike-build.yml` baut das Spike-1.1-Programm (Electron 44 + napi-Modul) mit electron-builder 26.16.1 für Windows (NSIS, ZIP), macOS Universal (DMG, ZIP, ohne Signatur) und Linux (AppImage, tar.gz) und startet es auf jedem Runner (Lauf 38383032090, alle grün). Befunde im README des Spikes. Windows-PC (2026-10-13, Rückmeldung Projektinhaber): SmartScreen-Warnung beim Start des unsignierten Programms, nach Zulassen lauffähig – wie erwartet. Linux: CI-Rauchtest genügt für 1.6 (ADR-011). macOS (2026-10-13, Projektinhaber): Gatekeeper blockiert das unbeglaubigte Programm; Freigabe über Systemeinstellungen → Datenschutz & Sicherheit → „Dennoch öffnen" (Rechtsklick → „Öffnen" wirkt seit macOS 15 nicht mehr). Verpackung bleibt `[VORLÄUFIG]`: unsigniert, Linux-Sandbox offen (7.2), Signatur-Entscheidung in 7.1. Ursprünglich offen für den Abschluss: Start auf Mac M3 Pro, insbesondere was ein Spieler unter macOS ohne Signatur tun muss – Anleitung im README des Spikes, Abschnitt „Testprogramm herunterladen und starten".

#### 1.7: Auswertung und Entscheidung

- **Status:** OFFEN
- **Phasentyp-Kontext:** ERKUNDUNG
- **Schritt-Art (nur ERKUNDUNG):** sonstiges (Auswertung)
- **Zeitbox (nur ERKUNDUNG):** maximal eine halbe Session
- **Abhängigkeiten:** 1.1–1.6
- **Freigabepflichtig:** ja
- **Empfohlene Klasse:** Entscheidung – Pflichtfrage am Phasenende und ERKENNTNIS-ADR zu ADR-002.
- **Eingangskriterien:** alle Messprotokolle liegen vor
- **Anforderungen (ab Klasse M):** keine (Auswertung)
- **Zu tun:** Erkenntnisse zusammenfassen, ADR-002 bestätigen oder verwerfen (ERKENNTNIS-ADR), Reifegrade nachziehen, Iterations-Reflexion schreiben, Phase 2 im vollen Schrittformat verfeinern.
- **Akzeptanzkriterien:** Entscheidung des Projektverantwortlichen liegt vor; Phase 2 ist ausgeplant.
- **Betroffene Module:** –
- **Reifegrad-Wirkung:** siehe Phasenziel
- **Artefakte:** ADR, Reflexion, verfeinerter Fahrplan
- **Notizen:** –

### Phase 2: Fundament – Typ: UMSETZUNG

**Ziel:** Produktionsreife Grundlage auf Basis der Erkenntnisse aus Phase 1.

**Abschlusskriterium:** leerer, aber vollständig lauffähiger Spielrahmen mit grüner CI auf drei Plattformen.

**Grobe Schritte (Verfeinerung im vollen Format in 1.7):**

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 2.1 | Repository-Struktur, alle CI-Gates laut `project-context.md` Abschnitt 7, Pre-Commit-Hooks, Branch-Schutz, `.editorconfig` | – |
| 2.2 | Simulationskern: Takt, Zeitsteuerung, Befehlskanal, Ereignisstrom, Determinismus-Tests | FR-003 |
| 2.3 | Weltzustand und Datenmodell, Fracht im Schema vorbereitet | FR-046 |
| 2.4 | Datenpaket-Schema (S-4) und Lader | FR-044 |
| 2.5 | Spielstand-Format (S-5) mit Autosave | FR-004 |
| 2.6 | Lokalisierung (Deutsch, Englisch, Sprachpakete), Stilleitfaden und SVG-Pipeline (ADR-008) | FR-006, FR-050 |
| 2.7 | Brücke und Plattformschicht mit Electron-Härtung, Konsole/Debug-Panel | FR-009 |

### Phase 3: Vertikaler Schnitt „Kleiner Flughafen" – Typ: UMSETZUNG

**Ziel:** Ein kleiner Flughafen ist von Anfang bis Ende spielbar.

**Abschlusskriterium:** Szenario 2 der Vision („Der Einsteiger") ist spielbar.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 3.1 | Bauen im Raster (eine Etage), Bauaufträge sofort sichtbar | FR-010, FR-011 |
| 3.2 | Kleines Terminal: Check-in, Sicherheit, Gate, Boarding, Toiletten; Passagierablauf | FR-012, FR-016, FR-019, FR-020, FR-022 |
| 3.3 | Passagierführung (Schilder, Markierungen) und erklärbare Wegprobleme; davor Erkundung „Suchen ohne Schilder" und Feinwege zum Einzelziel (aus 1.2) | FR-017, FR-018, FR-043 |
| 3.4 | Straßenverkehr, Bahn und Rollwege mit Knoten, Verkehrssteuerung; davor Erkundung Blockadevermeidung (ADR-005) | FR-014, FR-027, FR-028 |
| 3.5 | Abfertigung und Gepäck in Grundform | FR-025, FR-032 |
| 3.6 | GA und kleine Linienflüge, Flugplaner, Geld und Bewertung in Grundform | FR-030, FR-036 |

### Phase 4: Härtung des vertikalen Schnitts – Typ: STABILISIERUNG

**Ziel:** Qualität und Leistung des vertikalen Schnitts sichern, bevor in die Breite gebaut wird.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 4.1 | Referenzszenario baubar, Leistungstest als CI-Gate | FR-040 |
| 4.2 | Coverage auf Mindestwerte, Langzeittest, erste Funktionsgleichheits-Prüfung der umgesetzten Inventar-IDs | – |

### Phase 5: Ausbau in die Breite – Typ: UMSETZUNG

**Ziel:** Funktionsumfang des Originals mit den Kern-Modernisierungen abbilden. Wird bei Phasenbeginn in Teilphasen oder Teil-Fahrpläne aufgeteilt; vor Verkehr und Gepäck jeweils eine Erkundung.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 5.1 | Spielstart vollständig: Standort, Kartengröße, Sandbox, Einstellungen | FR-001, FR-002, FR-005 |
| 5.2 | Etagen, Zonen (international), Bauen vollständig | FR-010, FR-012 |
| 5.3 | Landseite: Parken, ÖPNV, Kreuzungen mit wählbarer Vorfahrt | FR-013, FR-015 |
| 5.4 | Terminal vollständig: moderner Check-in, Kontrollspuren, Grenze, Läden, Info | FR-019 bis FR-024 |
| 5.5 | Airside, Gepäck mit dreistufiger Kontrolle, Flüge, Abfertigung vollständig, Startfreigabe | FR-025, FR-026, FR-027, FR-029, FR-030, FR-032 |
| 5.6 | Personal, Wirtschaft, Forschung, Serviceziele, Notfälle, Störungen, Tutorial, Erfolge | FR-007, FR-008, FR-033 bis FR-038 |
| 5.7 | Flugzeugtypen und Airlines inklusive Add-on-Klassen | FR-031 |
| 5.8 | Benchmark mit Empfehlung, Leistungsbewusstsein | FR-041, FR-042 |

### Phase 6: Inhalte, Mods und Atmosphäre – Typ: UMSETZUNG

**Ziel:** Spielgefühl und Erweiterbarkeit.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 6.1 | Mod-Verwaltung; davor Erkundung zu Code-Mods (`project-context.md` Abschnitt 11) | FR-044 |
| 6.2 | Audio, Durchsagen, Musik, ggf. Intro-Video | FR-047, FR-049 |
| 6.3 | Abgleich der Funktionsgleichheit gegen das gesamte Inventar | – |

### Phase 7: Vorbereitung Version 1.0 – Typ: STABILISIERUNG

**Ziel:** Veröffentlichungsreife.

**Abschlusskriterium:** Erfolgskriterien der Vision erfüllt; Release v1.0.0.

| Schritt | Kurztitel | Anforderungen |
|---|---|---|
| 7.1 | Gate vor dem ersten öffentlichen Release (CLAUDE.md Abschnitt 12, `templates/gate-deployment.md`): Sicherheits-Review, Vertretung, Notfall-Handbuch, Signatur-Entscheidung | – |
| 7.2 | Referenzszenario-Nachweis auf allen Plattformen inklusive Linux-Leistungsmessung auf echter Hardware mit gängigen Treibern (ADR-011, früher gern durch Mitwirkende), Langzeitstabilität, Release-Builds v1.0.0; Linux-Pakete mit funktionierender Chromium-Sandbox (z. B. deb mit SUID-`chrome-sandbox` oder AppArmor-Profil, Befund aus 1.6) | FR-040, FR-045 |

### Querschnitt: wiederkehrende Schritte

#### Q.1: Versions-Nachzug (Electron, Rust, Werkzeuge)

- **Status:** OFFEN
- **Phasentyp-Kontext:** STABILISIERUNG (wiederkehrend, unabhängig von der laufenden Phase)
- **Abhängigkeiten:** keine
- **Frist:** 2027-01-02 (zwei Monate vor Lebensende Electron 44 am 2027-03-02); danach je Electron-Linie neu
- **Freigabepflichtig:** ja (neue Linien nach `CLAUDE.md` Abschnitt 15)
- **Empfohlene Klasse:** Entscheidung – Wahl neuer Linien ist eine Entscheidung über externe Abhängigkeiten.
- **Eingangskriterien:** Vorlauf im Ablaufdaten-Register erreicht
- **Anforderungen (ab Klasse M):** FR-045
- **Zu tun:** Ablaufdaten-Register (`project-context.md` Abschnitt 8) abarbeiten: neue Electron-Linie, Rust-Toolchain, TypeScript 7 bei Unterstützung durch typescript-eslint, übrige Werkzeuge; Tabelle zur Bestätigung vorlegen; ADR-002 per Folge-ADR fortschreiben.
- **Akzeptanzkriterien:** Keine fixierte Version ist über ihrem Lebensende; Register und `Verifiziert`-Vermerke aktuell; CI grün auf drei Plattformen.
- **Betroffene Module:** Plattform, Build/CI
- **Reifegrad-Wirkung:** keine
- **Artefakte:** Versions-Tabelle, Folge-ADR, aktualisierte Register
- **Notizen:** Nach dem ersten Lauf wird ein neuer Q.1-Termin mit der nächsten Frist angelegt.

#### Q.2: Neubewertung WebAssembly mit Threads als Betriebsart

- **Status:** OFFEN (wartet auf Eingangskriterium)
- **Phasentyp-Kontext:** ERKUNDUNG (wiederkehrende Prüfung, unabhängig von der laufenden Phase)
- **Schritt-Art (nur ERKUNDUNG):** Vergleichsstudie
- **Zeitbox (nur ERKUNDUNG):** Prüfung des Eingangskriteriums je Q.1-Lauf 15 Minuten; Vergleichsmessung maximal eine Session
- **Abhängigkeiten:** 1.1 (Spike als Messgrundlage)
- **Frist:** erste Nachprüfung 2027-01-02 zusammen mit Q.1, danach bei jedem Q.1-Lauf
- **Freigabepflichtig:** ja, für einen Wechsel der Betriebsart (Architekturänderung)
- **Empfohlene Klasse:** Entscheidung – kann ADR-009 ändern (Architekturentscheidung).
- **Eingangskriterien:** WebAssembly mit Threads ist mit stabilem Rust baubar (Zielfunktion `atomics` stabil und Standardbibliothek ohne `-Z build-std` threadfähig, oder ein gleichwertiger stabiler Weg); bis dahin wird nur das Kriterium geprüft und im Logbuch vermerkt.
- **Anforderungen (ab Klasse M):** FR-040, FR-045
- **Zu tun:**
  - Bei jedem Q.1: Eingangskriterium anhand der Rust-Release-Notes und der wasm-bindgen-Doku prüfen.
  - Ist es erfüllt: Spike `spikes/1.1-betriebsart/` mit dem aktuellen Kern neu messen (wasmN gegen napi), zusätzlich auf Referenzhardware und unter Windows/macOS; Build-Aufwand für Variante A aus Phase 2 dagegenstellen.
  - Ergebnis als Entscheidungsvorlage: bei A bleiben, auf B wechseln oder Hybrid (z. B. WebAssembly für Teilaufgaben im Renderer).
- **Akzeptanzkriterien:** Wir wissen, ob B mit stabilem Rust baubar ist; falls ja, liegen Vergleichswerte und eine begründete Empfehlung vor.
- **Betroffene Module:** Takt & Weltzustand, Brücke, Plattform
- **Reifegrad-Wirkung:** keine, außer bei Wechsel (dann Betriebsart per Folge-ADR neu bewertet)
- **Artefakte:** Logbuch-Vermerk je Prüfung; bei Messung Messprotokoll und ggf. Folge-ADR zu ADR-009
- **Notizen:** Herkunft: Freigabe von ADR-009 („B perspektivisch“). Regel-006 hält den Kern dafür ohne Anbindungscode baubar für `wasm32-unknown-unknown`.

---

<!-- ANCHOR:iterations-reflexion -->
## Iterations-Reflexion

Noch keine abgeschlossene Phase.

---

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- Schritte **ohne Abhängigkeiten zueinander**: 1.6 parallel zu 1.2–1.4 (nach 1.1); Q.1 und Q.2 unabhängig von allen Phasen
- Schritte **mit gemeinsamen Modulen** (Konfliktgefahr): 1.2 und 1.3 (Takt & Weltzustand, Brücke)

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-10-13 – Schritt 1.2 abgeschlossen (ADR-010); 1.5 um Rastergröße und fünf Etagen ergänzt; 3.3 um vorgeschaltete Erkundung „Suchen ohne Schilder"/Feinwege ergänzt (Landeplatz aus 1.2)
- 2026-10-13 – Schritt 1.1 abgeschlossen (ADR-009); Querschnitt Q.2 als Landeplatz für die zurückgestellte WebAssembly-Variante angelegt (zählt nicht zum Schrittplan von Phase 1); 1.3 um Übertragungsumfang ergänzt
- 2026-10-13 – Erstplanung in der Initialisierung (ADR-001 bis ADR-008); Nachzug nach aktuellem Regelwerk: Schritt-IDs für Phasen 2–7 mit Anforderungs-Zuordnung, Querschnitt Q.1

<!-- ANCHOR:archiv-abgeschlossene-phasen -->
## Archiv / abgeschlossene Phasen

Noch keine.
