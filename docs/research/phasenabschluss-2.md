# Phasenabschluss 2: Weiterbauen, umbauen oder neu aufsetzen

<!-- Pflichtfrage am Phasenende (CLAUDE.md Abschnitt 12). Teil 1: Bewertungen der getrennten Instanzen,
     unverändert übernommen (nur die Unterüberschriften tragen zur Unterscheidung (A) bzw. (B)). Teil 2: Stellungnahme der bauenden KI. Teil 3: Vision-Abgleich.
     Teil 4: Onboarding-Re-Validation. Entscheidung und ADR folgen nach Freigabe durch den Projektinhaber. -->

- **Datum:** 2026-10-19
- **Getrennte Instanzen:** zwei Unteragenten ohne Gesprächsverlauf, gleicher Auftrag, nur lesend:
  - **Instanz A:** Modell der Entscheidungs-Klasse (`claude-opus-5-5`), wie beim Phasenabschluss 1.
  - **Instanz B:** anderes Modell (`claude-sonnet-5-5`, Routine-Klasse), damit die Prüfung nicht nur vom Modell der bauenden KI kommt (`CLAUDE.md` Abschnitt 12, „möglichst mit einem anderen Modell“). Die Ausnahme-Klasse wäre ohne Freigabe nicht zulässig gewesen.
- **Quellen der Instanzen:** Code (`core/`, `bridge/`, `app/`, `tools/`, `scripts/`, `content/`, `assets/`, Workflows, Manifeste), `docs/architecture.md`, `docs/decisions.md` mit `docs/decisions/`, `docs/vision.md`, `docs/fahrplan.md` (Phase 2, Phasen 3–7, Querschnitt, Iterations-Reflexion), `docs/research/2.*`. Ausgeschlossen: Logbuch, Archiv, `phasenabschluss-1.md`.

## Teil 1a: Bewertung der Instanz A (Opus 5.5, unverändert)

### Befund (F = belegt, E = Einschätzung) (A)

1. **F – Umfang.** `core/src` hat 13.241 Zeilen in 24 Modulen, die flach nebeneinander liegen (dazu `datapack/` und `savegame/`). Hinzu kommen `core/tests` (2.277 Zeilen), `bridge/src` (530), `app/src/main` (4.439), `app/renderer` (4.884), `tools/assets` (732) und `scripts` (1.218). Die größten Dateien sind `app/src/main/main.ts` (1.133), `core/src/runner.rs` (1.036), `core/src/simulation.rs` (1.030), `core/src/datapack/load.rs` (925), `core/src/flowfield.rs` (896) und `core/src/world.rs` (813). Im Repository stehen 256 Commits.

2. **F – Testabdeckung.** Laut Fahrplan liegt der Kern bei 98,4 % Lines (2.6), TypeScript bei 85,5–86,5 % (2.6, 2.10). Von der Messung ausgenommen sind laut `vitest.config.ts` Z. 14–22 `main.ts`, `core-host.ts`, `preload.cts`, `renderer/main.ts` und `playfield.ts`, zusammen rund 2.200 Zeilen. Darunter ist Logik, die Sicherheit berührt: Dateizugriff auf Spielstände in `main.ts` Z. 697–760 (`openSave`, `readExactly`, `saveFiles`). Sie ist nur über den Rauchtest `e2e/smoke.spec.ts` geprüft. Die DOM-Panels liegen bei 0 % bzw. 13 % (Fahrplan 2.6b).

3. **F – Platzhalter.** Im Code stehen 11 Marker `TODO(fahrplan-ref:`. Ziele: 3.1 viermal (`simulation.rs:630`, `:640`, `savegame/mod.rs:57`, `playfield.ts:11`), 3.2 einmal (`world.rs:35`), 3.4 dreimal (`simulation.rs:681`, `network.rs:6`, `savegame/mod.rs:60`), 4.1 einmal (`scenario.rs:7`), 5.1 zweimal (`savegame/mod.rs:175`, `bridge/src/lib.rs:139`). Alle Ziel-Schritte gibt es in der Grobplanung (`fahrplan.md` Z. 378–401). TODOs ohne Fahrplan-Bezug: keine.

4. **F – Lokale Suppressions.** Es gibt drei `#[allow(clippy::…)]`. Zwei tragen eine Begründung (`world.rs:65`, `frame.rs:133`). `scenario.rs:77` (`type_complexity`) hat keine Begründung, das ist ein kleiner Verstoß gegen CLAUDE.md §15. Kein `eslint-disable`, kein `@ts-ignore`, kein `unsafe`.

5. **F – Phase 2 hat einen Rahmen gebaut, noch keinen Spielkern.** `build`, `buildArea` und `demolish` werden angenommen, wirken aber nicht (`simulation.rs:630–641`). Die Welt enthält keine Objekte. `Simulation::step` ruft fachlich nur `world.advance` auf, und das bewegt nur Personen (`world.rs:384–407`). S-1 überträgt nur Personen (`frame.rs`). Das Raster wird nicht gezeichnet (`playfield.ts:10–11`). Das Abschlusskriterium „leerer Spielrahmen“ ist damit ehrlich erfüllt. Fachliche Last trägt das Fundament aber noch nicht.

6. **F/E – Für die Fachmodule fehlt eine Struktur im Kern.**
   - F: Laut `architecture.md` Z. 104 sprechen die Kern-Module „ausschließlich über den gemeinsamen Weltzustand und interne Ereignisse des Takts“ miteinander.
   - F: Im Code gibt es keine internen Ereignisse; die Suche danach in `core/src` findet nichts. `World` ist eine einzige Struktur (`world.rs:95–103`), und auch die Zielwahl steckt in ihrem Personen-Datensatz (`rng`, `target`, TODO 3.2).
   - F: Der Spielstand bildet jedes Feld von Hand nach. `SavedState` in `savegame/state.rs:23–90` steht neben `World::saved_persons`, `from_saved` mit eigenen Invarianten (`world.rs:420–510`) und `hash_into`. Jede neue Objektart braucht also vier Stellen, die zueinander passen müssen.
   - E: Phase 3 bringt fünf Fachmodule auf einmal (Bau, Personen, Verkehr, Abfertigung, Flug). Ohne festgelegte Reihenfolge der Module im Takt, ohne interne Ereignisse und ohne Spielstand-Teil je Modul werden `World`, `Simulation` und `SavedState` zu Sammelobjekten. Hier erwarte ich den ersten echten Umbau, spätestens in 3.1/3.2.

7. **F/E – Neuberechnung der Wegweiser bei Dauerbau.**
   - F: In `adoption.rs` Z. 30–36 und 85–95 ersetzt eine neue Anfrage die ältere. Die ältere Berechnung läuft aber weiter („The receiver is gone if a newer request replaced this one“) und wird nicht abgebrochen.
   - F: Eine volle Neuberechnung dauert 1,1–1,2 s auf 4 vCPU bzw. 0,29–0,66 s auf der Zielhardware. Gemessen wurde nur eine Bauaktion alle 10 s (`architecture.md` Z. 206).
   - E: Baut der Spieler schneller, als eine Neuberechnung dauert (Ziehen, Vision-Szenario „Großflughafen-Bauer“), werden womöglich lange keine neuen Wegweiser übernommen. Gleichzeitig laufen verworfene Berechnungen von je rund 100 MB parallel weiter. Reproduziert habe ich das nicht.
   - F: `MAX_TARGETS = 255` (`target.rs:9`), und der Speicher wächst linear mit der Zahl der Ziele (64 Ziele = 101 MB). Die Feinwege bis zum Einzelziel sind offen (vor 3.3).

8. **F – Reaktive ADRs in Phase 2:** ADR-020, 025, 026, 028, 029, 030, 031, 032 (8 Stück; gesamt 13 von 35). Einzeln betrachtet:
   - ADR-020 ändert ADR-013 planmäßig; die Neubewertung war in ADR-013 (Konsequenzen) und ADR-016 schon vorgesehen. Reaktiv nur formal.
   - ADR-025 hebt die Einschränkung einer Bibliothek auf einen Teil des Systems auf (formal).
   - ADR-028, 029 und 031 ergänzen S-6 nur additiv, also nach S-6 „Versionierung: Neue Funktionen sind additiv“ zulässig. Sie wurden ausdrücklich „vorsichtshalber“ als reaktiv gezählt (ADR-028, Klassifikation).
   - ADR-030 ändert zwei Konstanten.
   - ADR-026 ist ein echter Fehler im Vorgehen; die getrennte Prüfung hat die Abweichung gefunden.
   - ADR-032 fasst gesammelte Funde zusammen.
   - Gemeinsame Ursache von 026, 028, 029, 031 und 032: Die Verträge aus 2.9 wurden je Schnittstelle geschrieben und nicht gegeneinander gelesen (Reflexion zu ADR-031/032).
   - Der Code von Kern, Takt, Wegweisern oder Brücke war von keinem reaktiven ADR betroffen.

9. **E – Die Zählregel ist zu streng, die Häufung trotzdem nicht bedeutungslos.**
   - Etwa die Hälfte der reaktiven ADRs ist nur formal reaktiv; die Regel „jeder ADR, der einen anderen binnen 14 Tagen ändert“ bläht die Quote auf.
   - Zugleich deckt L-1 eine echte Lücke im Fundament auf. S-1 gilt seit 2026-10-13 als `[BELASTBAR]`, regelte aber nicht, wie die gebaute, ruhende Welt in die Darstellung kommt (`architecture.md` Z. 352). Gerade für ein Aufbauspiel ist das ein Kernfluss. Die Schwäche liegt also im Entwurf der Verträge, nicht im Modul-Code.

10. **F – Das Warnsignal der Quote unterscheidet nichts mehr.**
    - Die Quote liegt seit ADR-015 fast durchgehend über 20 %. In `fahrplan.md` Z. 501–621 stehen 12 Reflexionen mit 10 Maßnahmen, und jede endet mit „kein Umbau“.
    - Die Vorhersagen zum Absinken sind wiederholt nicht eingetreten: „2/10 ab ADR-025“ (Z. 526), „ab ADR-027“ (Z. 536), „ab ADR-029“, „ab ADR-030“, „ab ADR-035“, „ab ADR-036“, „ab ADR-038“, „ab ADR-039“, jetzt „ab ADR-040“ (Z. 620).
    - E: Ein Signal, das immer anschlägt, trägt keine Information und lädt zum Abnicken ein.

11. **F – Offene Reifegrade und ihre Landeplätze.**
    - Weltdarstellung `[OFFEN]` → 3.8 vor 3.1. Die Reihenfolge ist richtig gewählt.
    - Für die Erkundungen „Blockadevermeidung“ (in 3.4) und „Suchen ohne Schilder/Feinwege“ (in 3.3) steht im Fahrplan nur „davor Erkundung“ (Z. 380–381), ohne eigene Schritt-ID. Nach der eigenen Landeplatz-Regel (CLAUDE.md §6) ist das schwach.
    - Phase 3 hat „noch keinen festgehaltenen ursprünglichen Schrittplan“ (`fahrplan.md` Z. 634), obwohl dort schon 3.7 und 3.8 hinzugekommen sind.
    - NFR „Bauaktion im nächsten Bild“: in 2.10 auf dem Referenz-PC 2–3 Bilder, schlechter als 1–2 in 2.7 (ADR-034). Landeplatz 3.1.
    - Leistung auf Linux erst in 7.2, 8-Kern-Hardware „spätestens 4.1“. E: Für eine harte Randbedingung der Vision (Plattform) ist 7.2 spät.

12. **F – Umfang der Architektur-Dokumentation.** `architecture.md` hat 682 Zeilen bzw. rund 100 KB und überschreitet damit die eigene Auslagerungs-Schwelle von 500 Zeilen (Kommentar Z. 4). Abschnitt 4 (Z. 275–510) mischt den Vertragstext mit „Umsetzung und Präzisierungen“ und vollständigen Listen der Fehlercodes, die es auch im Code gibt. E: Die Pflege doppelter Quellen ist teuer und erzeugt selbst Drift. Genau dieser Fehlertyp hat die Reaktiv-Häufung ausgelöst.

13. **F/E – Last der Methodik.**
    - F: Die Dokumente ohne Logbuch umfassen rund 584 KB, das Logbuch 145 KB; Code und Tests ohne erzeugte Typen rund 957 KB. Die 35 ADRs entstanden in 7 Tagen (2026-10-13 bis 2026-10-19). Einzelne Fahrplan-Einträge sind Absätze von über 1.000 Wörtern (2.5, Z. 283–297).
    - F: Der Prozess hat echte Mängel gefunden: getrennte Prüfungen mit 14, 12 und 7 Befunden, darunter die Speicherverstärkung, die zu ADR-030 führte, und die Abweichung hinter ADR-026. Der erzwungene Fehlerfall ist bei allen CI-Gates belegt.
    - E: Prüfungen und erzwungene Fehler rechtfertigen ihren Aufwand. Die Reflexionen zur Reaktiv-Quote und die Ausführlichkeit von Fahrplan und Architektur stehen in keinem guten Verhältnis zum Nutzen.

### Option 1: Weiterbauen (A)

- **Kosten:** sofort keine; Phase 3 beginnt wie geplant mit 3.7 → 3.8 → 3.1.
- **Risiko:**
  - **Mittel bis hoch:** `World`, `Simulation`, `SavedState` und `main.ts` wachsen in Phase 3 ungeordnet weiter (Befund 6, 2). Ein späterer Umbau berührt dann Spielstand-Format und Determinismus-Tests auf breiter Front.
  - Das Problem beim Dauerbau (Befund 7) zeigt sich frühestens im Spieltest von 3.1.
  - Die Reaktiv-Quote bleibt über Phase 3 hinaus über dem Schwellenwert (neue Verträge in 3.1, 3.6, 3.8) und erzeugt weiter Reflexionen ohne Erkenntnis.

### Option 2: Gezielt umbauen (welche Teile genau) (A)

- **Was geändert wird:**
  - **(a) Kernstruktur vor 3.1**, als Entwurf mit ADR, Kategorie 1:
    - festgelegte Reihenfolge der Fachmodule im Takt;
    - Untermodule je Fachmodul in `core/src` statt flacher Dateien;
    - ein Mechanismus für interne Ereignisse, wie ihn `architecture.md` Z. 104 behauptet;
    - Spielstand, Zustands-Prüfsumme und Invarianten je Modul statt zentral in `savegame/state.rs` und `world.rs`.
    - Die Zielwahl wandert aus `Person` in das künftige Modul Personen (TODO 3.2).
  - **(b) Neuberechnung der Wegweiser:** Anfragen zusammenfassen oder überholte Berechnungen abbrechen (`adoption.rs`), dazu ein Test bzw. Benchmark mit Bauaktionen alle 0,25–0,5 s als Akzeptanzkriterium von 3.1.
  - **(c) `app/src/main/main.ts` teilen:** Datei-I/O der Spielstände, Pakete und Einstellungen in testbare Module (Muster `saves.ts`, `settings.ts`), damit die Logik aus Befund 2 von Unit-Tests erfasst wird.
  - **(d) Dokumente:** Abschnitt 4 nach der eigenen Regel in `architecture-schnittstellen.md` auslagern. Vertrag und „Umsetzung“ trennen; Fehlercode-Listen aus den Rust-Typen herleiten statt doppelt pflegen.
  - **(e) Methodik:** per ADR des Menschen eine Auslegungsregel wie Regel-007: Additive Vertragsergänzungen, die die Versionierungsregel des Vertrags ausdrücklich zulässt, zählen nicht als `[REAKTIV]`. Die Erkundungen vor 3.3 und 3.4 bekommen eigene Schritt-IDs. Der ursprüngliche Schrittplan von Phase 3 wird vor dem ersten Schritt festgehalten.
- **Kosten (E):** (a) 1 Entwurfssession plus 1–2 Sessions Umbau, solange noch keine Objekte im Weltzustand stehen; das ist der billigste Zeitpunkt (ADR-020: „Umkehrbarkeit billig bis Phase 3“). (b) und (c) je unter einer Session. (d) und (e) eine Session Dokumentenarbeit.
- **Risiko:** gering bis mittel. Der Umbau berührt das Spielstand-Format; das ist vor 1.0 zulässig (ADR-021). Determinismus-Tests und Rauchtest decken einen Rückschritt ab. Restrisiko: (a) wird ohne echte Fachmodule zu abstrakt entworfen. Deshalb sollte es eng an 3.1/3.2 ausgerichtet sein, nicht allgemein.

### Option 3: Neu aufsetzen (A)

- **Kosten:** sehr hoch. Rund 13.000 Zeilen Kern, rund 9.000 Zeilen TypeScript, gehärtete Plattform (Absenderprüfung, Pfadprüfung, Grenzen) und CI-Gates auf drei Plattformen wären neu zu erstellen und neu zu belegen.
- **Risiko:** hoch. Die gemessene Leistungsbasis (2.7, 2.10: 180 Ticks/s, 60 bzw. 120 Bilder/s, Speicher ohne Anstieg) und die Befunde aus drei getrennten Prüfungen ginge verloren, ohne dass ein Befund auf einen Fehler im Stack zeigt.
- **Belege dagegen:** ADR-014, ADR-034 und `docs/research/2.7-messung.md` bzw. `2.10-messung.md` bestätigen Stack, Betriebsart und Bildübergabe. Kein reaktiver ADR betrifft Kern, Takt, Wegweiser oder Brücke (Befund 8).

### Empfehlung der Prüf-Instanz (A)

**Option 2, eng gefasst: (a) und (b) vor 3.1, (c), (d) und (e) parallel.** Konfidenz: **mittel.** Begründung:

- Der Befund zur Kernstruktur (6) stützt sich auf das Lesen von Dateien. Ob sie in Phase 3 wirklich trägt, ist nicht gemessen.
- Das Problem beim Dauerbau (7) ist aus dem Code abgeleitet, nicht reproduziert.
- Gegen „Neu aufsetzen“ spricht dagegen viel, mit Messungen und Tests belegt.
- „Weiterbauen“ wäre vertretbar, wenn (a) und (b) ausdrücklich als Akzeptanzkriterien in 3.1/3.2 aufgenommen werden; der Unterschied zu Option 2 ist dann vor allem eine Frage der Reihenfolge.

### Hinweise für die Ausplanung von Phase 3 (A)

- Den ursprünglichen Schrittplan von Phase 3 (mindestens 3.1–3.8) vor dem ersten Schritt festhalten; sonst lässt sich Stopp-Kriterium 9 nicht prüfen.
- Den Entwurf zur Kernstruktur (Reihenfolge der Module im Takt, interne Ereignisse, Spielstand je Modul) zusammen mit 3.8 oder direkt danach anlegen, vor 3.1.
- 3.1-Akzeptanz: Dauerbau-Benchmark mit einer Bauaktion alle 0,25–0,5 s; Wegweiser werden innerhalb einer festen Frist übernommen, ohne Speicherspitze über einer Grenze. Dazu die NFR „Bauaktion im nächsten Bild“ mit der Vorschau messen.
- 3.8 entscheidet auch, ob die Zellen in der Darstellung neu übertragen werden, sobald neue Wegweiser übernommen sind. Den Datenweg des statischen Zustands für Spielstand und Darstellung aus derselben Block-Kennung herleiten (`grid` mit geteilten Blöcken über `Arc`).
- Erkundung „Feinwege/Suchen ohne Schilder“ und „Blockadevermeidung“ als eigene Schritte mit ID (z. B. 3.9, 3.10). Dabei `MAX_TARGETS = 255` und den Speicher linear zur Zahl der Ziele gegen ein realistisch bestücktes kleines Terminal (Check-in, Toiletten, Gates, Läden) prüfen.
- Die Zählung der Reaktiv-Quote per ADR präzisieren, bevor in 3.1, 3.6 und 3.8 weitere additive Vertragsergänzungen fällig werden.
- Den Fahrplan-Text je Schritt begrenzen: Ergebnis, Belege, Landeplätze. Details zu Festlegungen gehören in Forschungsnotizen oder ADRs.
- Erwägen, eine Linux-Leistungsmessung (z. B. über Mitwirkende) vor 7.2 zu ziehen, spätestens in Phase 4.
- Kleinkram: die Begründung an `scenario.rs:77` nachtragen.

Gelesene Quellen (Angabe der Instanz): `docs/architecture.md` (vollständig), `docs/decisions.md` Teil A–D, ADR-016, 020, 021, 025, 026, 028–032, 034, `docs/fahrplan.md` (Phase 2 vollständig, Phasen 3–7, Querschnitt, Iterations-Reflexion, Replanning-Historie), `docs/vision.md` §3, §4, §9, `docs/project-context.md` (nur die Zeilen zu den Schwellenwerten), Inhaltsverzeichnis von `docs/research/2.11-sammelpruefung.md`; Code: `core/src/{lib,world,simulation,adoption,target,network,frame,scenario}.rs` (Auszüge), `core/src/savegame/{mod,state}.rs` (Auszüge), `app/src/main/main.ts` (Struktur, Z. 690–760), `app/renderer/playfield.ts` (Kopf), `vitest.config.ts`, `.github/workflows/ci.yml` (Coverage), dazu Zeilenzählungen und Suchen über `core`, `bridge`, `app`, `tools`, `scripts`; `git log --oneline | wc -l`. Nicht gelesen: Logbuch, Archiv, `phasenabschluss-1.md`; nichts ausgeführt außer Lese- und Zählbefehlen.

## Teil 1b: Bewertung der Instanz B (Sonnet 5.5, unverändert)

### Befund (F = belegt, E = Einschätzung) (B)

1. **F – Der Kern ist ein Personen-Kern.** Aufbau, Zustandsumfang und Tests:
   - `core/src/world.rs` (813 Zeilen) kennt Raster, Ziele, Wegenetze, Wegweiser und Personen (`Person`, Z. 36 ff.).
   - Objekte, Fahrzeuge und Flugzeuge existieren im Weltzustand nicht. `core/src/simulation.rs:626-641`: `Build`/`BuildArea` prüfen nur die Inhalts-ID, `Demolish` ist ein No-op. `network.rs` ist eine reine Datenstruktur (238 Zeilen).
   - Der Gesamtumfang ist gering: etwa 27.000 Zeilen Code und Tests, rund 210 Rust- und 183 TS/Skript-Testfälle. Coverage laut Fahrplan: Kern 98,3-98,4 % Lines, TypeScript 85,5-90,5 %.
   - Die Grundmuster sind belegt: Takt, Zwei-Phasen-Tick (bitgleich über Threadzahlen, `core/tests/determinism.rs`), Wegweiser in Blöcken, Dreifachpuffer, Spielstand-Rundlauf mit gleicher Prüfsumme.

2. **F – Leistungsnachweis auf Produktionscode, aber nur für das Gemessene.**
   - `docs/research/2.10-messung.md`: Referenz-PC 59,9-60,0 Bilder/s bei 60 Hz, Mac M3 Pro 119-120 Bilder/s bei 120 Hz. Gemessen mit 10.000 Personen, 20.000 statischen Sprites in Atlas-Stufe 8 px und 64 Zonen.
   - `architecture.md` Abschnitt 9 führt Atlas-Stufen 16/32, große Atlanten, bewegte Sprites und Last durch Fahrzeuge, Flugzeuge, Gepäck und Personal als `[VORLÄUFIG]`. Landeplätze sind 3.8, vor 3.4/5.5, 4.1 und 5.8.
   - Bauaktion bis zum Bild: 2-3 Bilder auf dem PC, 1-3 auf dem Mac. Die Zielgröße „nächstes Bild“ ist weiter nicht erreicht (Landeplatz 3.1).
   - Schrittzeit-Spitzen auf dem Mac: Einzelschritte bis 75 ms, Sekunden-p99 über 4 ms in 11 von 20 Fenstern. Mit 4 statt 8 Hintergrund-Threads ist die Ursache weder bestätigt noch widerlegt. Landeplatz 4.3 liegt hinter dem vertikalen Schnitt.
   - Linux ist nirgends auf echter Hardware gemessen (7.2).

3. **F – Die größte Lücke liegt auf dem kritischen Pfad von Phase 3: S-1 trägt nur Personen.**
   - `core/src/frame.rs:1-25` und `architecture.md` Z. 346-352: Layout-Version 1 hat 6 Byte je Person und keinen Typ. Der ruhende Zustand der Welt (Raster, Wände, Objekte, Zonen) ist `[OFFEN]`.
   - `app/renderer/playfield.ts:11` (TODO 3.1): Das Raster wird nicht gezeichnet.
   - Fahrzeuge und Flugzeuge brauchen mindestens Layout 2, Typ und Ausrichtung. Das ist der „Layout-Version“-Mechanismus im Kopf und im Vertrag vorgesehen, aber eine nicht additive Änderung.
   - E: Das ist der eigentliche Entwurfsrest. Er hat mit 3.8 einen Entwurfsschritt vor 3.1 (`fahrplan.md` Phase 3, Tabelle). Der Landeplatz ist ausreichend und rechtzeitig, sofern 3.8 wirklich als Messung und nicht als Papierentwurf ausgeführt wird.

4. **F – Reaktiv-Quote: Die Zählung ist mechanisch verzerrt, die Substanz ist gemischt.**
   - Die 14-Tage-Regel macht aus einem Entwurf, der in 2.9 vorab entschieden und dann umgesetzt wurde, eine Kette von Änderungen. Der Status von ADR-021 (`decisions.md` Teil A) nennt sechs Fortschreibungen binnen sechs Tagen: ADR-026, 028, 029, 030, 031, 032.
   - Ohne die Regel sähe es anders aus:
     - Inhaltlich unproblematisch: ADR-028, 029, 031 sind je eine additive S-6-Funktion oder ein Argument. ADR-020 war in ADR-013 und ADR-016 als Neubewertung vorgesehen (Reflexion 2026-10-13). ADR-025 weitet nur den Geltungsbereich von `serde_json`.
     - Rein formal reaktiv: ADR-032 ist die vom Projektinhaber gewählte Sammelmaßnahme (10).
     - Substanziell: ADR-026 (eine Vertragsregel, die das Gebaute nicht trug; die getrennte Prüfung fand es), ADR-030 (Spielstand-Obergrenzen ohne Messung) und ADR-020 (Kartengröße in 1.5 vor der Zellgröße festgelegt). Jedes davon ist im Kern ein Messproblem, nicht ein Strukturproblem.
   - Gegenbefund, der nicht kleinzureden ist: `docs/research/2.11-sammelpruefung.md` und ADR-032 nennen bei 81 Sätzen 10 Lücken, 5 Kartenfehler (u. a. doppelte Kennung `ABF` in der Modul-Karte) und 8 redaktionelle Funde, obwohl die Verträge in 2.9 mit getrennter Prüfung BELASTBAR gesetzt waren.
   - E: Das deutet auf ein **Vertragsentwurfs-Problem** (Verträge am Schreibtisch, je Schnittstelle statt quer gelesen), nicht auf ein Fundament-Problem. Dafür sprechen drei Belege: Alle reaktiven ADRs der Phase 2 sind additiv oder verschieben Konstanten. Keine Modulgrenze hat sich verschoben (ADR-032 Konfidenz hoch/billig). Der Kern ist durch 98 % Coverage und Determinismus-Tests belegt.
   - Die Quote ist als Frühwarnung zur Zählregel unbrauchbar, weil sie gleichzeitig strukturelle Warnung und Dokumentations-Aktualität misst. Der Schwellenwert 20 % ist bei 10 ADRs in 2 Tagen nicht erreichbar.

5. **F – Platzhalter und Schulden sind klein und gedeckt.**
   - 11 `TODO(fahrplan-ref:…)`, alle mit Landeplatz in `fahrplan.md`: 3.1 (`simulation.rs:630,640`, `savegame/mod.rs:57`, `playfield.ts:11`), 3.2 (`world.rs:35`), 3.4 (`simulation.rs:681`, `network.rs:6`, `savegame/mod.rs:60`), 4.1 (`scenario.rs:7`), 5.1 (`savegame/mod.rs:175`, `bridge/src/lib.rs:139`).
   - Stichprobe geprüft: 3.1, 3.2, 3.4, 4.1 und 5.1 existieren als Schritte.
   - Suppressions: nur 3 `allow(clippy::…)` mit Begründung (`world.rs:65`, `scenario.rs:77`, `frame.rs:134`). Kein `eslint-disable`, kein `@ts-ignore`. Kein `unsafe` gefunden.
   - Keine freien TODOs ohne Referenz.
   - Auffällig groß: `app/src/main/main.ts` (1.133 Zeilen, 44 Funktionen und Handler, bündelt S-6, S-8, Messung, Logs, Pakete, Spielstände, Einstellungen), `core/src/runner.rs` (1.036), `core/src/simulation.rs` (1.030), `core/src/datapack/load.rs` (925), `core/tests/datapacks.rs` (909), `core/src/flowfield.rs` (896).
   - E: Wartbarkeitsrisiko ist `main.ts` mit den vielen Zuständigkeiten, nicht der Kern.

6. **F – Zwei Blöcke von Phase 3-7 sind strukturell noch nicht belegt.**
   - Verkehr/Blockadevermeidung ist `[VORLÄUFIG]`. Die Erkundung ist als Teil von 3.4 geplant („davor Erkundung“).
   - Wegweiser für Personal auf dem Vorfeld und Querungen sind noch offen (Entwurfsschritt 3.7 vor 3.1).
   - Wegenetze (`network.rs`) sind nur Datenstrukturen ohne Verkehrslogik.
   - Das Spielstandformat (postcard, `savegame/state.rs`) speichert Raster, Ziele, Wegenetze und Personen. Jede neue Objektart (Objekte, Fahrzeuge, Flugzeuge, Bauaufträge) erhöht die Format-Version. Das ist vor 1.0 zugelassen (S-5, ADR-021) und damit billig, solange keine Zusage ab 1.0 gilt (7.2).

7. **F – Methodik-Last.**
   - Dokumente: 4.159 Zeilen in `docs/*.md`, davon Logbuch 1.063 (über dem 800-Zeilen-Trigger, Archivierung beim Phasenwechsel ansteht), Architektur 682, Fahrplan 644. Dazu 35 ADR-Dateien (122 KB gesamt, 28 KB gemessen).
   - Gegen etwa 27.000 Zeilen Code und Tests. In 7 Kalendertagen (13.-19.10.) entstanden 11 Schritte, 35 ADRs und 10 Reflexions-Einträge zur Reaktiv-Quote.
   - Davon sind die 8 Reflexions-Einträge zur Reaktiv-Quote seit ADR-015 (`fahrplan.md` Z. 501-624) zusammen etwa 130 Zeilen. Sie wiederholen dasselbe Ergebnis („kein Umbau“) und liefern 10 Maßnahmen, deren Nutzen nur teilweise belegt ist.
   - Belegter Nutzen von Prüfung und Messung: getrennte Prüfungen fanden echte Mängel (2.4: 14 Befunde, u. a. Speicher wächst mit Ordnergröße; 2.5: Speicherverstärkung → ADR-030). Die Sammelprüfung 2.11 fand 10 Lücken, die sonst einzeln aufgetaucht wären.
   - E: Der Aufwand ist hoch, aber die Funde belegen den Ertrag. Das Missverhältnis liegt im Reflexions- und ADR-Ritual für Fortschreibungen, nicht in den Prüfungen.

### Option 1: Weiterbauen (B)

- **Kosten:** niedrig, nur die in Phase 3 bereits geplante Arbeit (3.7, 3.8, dann 3.1-3.6). Reflexions- und ADR-Ritual weiterführen.
- **Risiko:** mittel. (a) 3.8 muss S-1 oder S-7 für die ruhende Welt entscheiden. Fällt die Messung schlecht aus, wird S-1 nicht additiv geändert (Layout 2) und berührt Brücke, Renderer und Spielfeld (Befund 3). (b) Der Prozess erzeugt weiter reaktive ADRs und dokumentarischen Aufwand (Befund 4, 7). (c) Die Lücken im Leistungsnachweis (16/32 px, bewegte Sprites, Mac-Spitzen) kommen erst in 3.8/4.3 und 4.1 zur Prüfung.

### Option 2: Gezielt umbauen (welche Teile genau) (B)

- **Was geändert wird (Vorschlag, E):**
  1. *Prozess, nicht Code:*
     - Die 14-Tage-Regel im Reaktiv-Zähler enger fassen: Fortschreibungen, die ein vorab geplanter Entwurfsschritt (2.9/2.11) vorgesehen hat, und rein additive Vertragsergänzungen mit Code-Beleg zählen als `[OPERATIV]` oder `[ERKENNTNIS]`. Das ist eine Methodik-Änderung und braucht einen ADR.
     - Dazu die ADR-Last für Fortschreibungen senken: ein Änderungsprotokoll im Vertrag statt je eine ADR-Datei.
     - Reflexions-Einträge zur Quote auf einen Sammel-Eintrag je Phase kürzen.
  2. *Code, klein:* `app/src/main/main.ts` (1.133 Zeilen) vor Phase 3 in Zuständigkeiten teilen (S-6-Handler, Messung, Spielstand/Autosave). Das ist ein lokales Refactoring innerhalb des Moduls Plattform und braucht keinen ADR.
  3. *Vorziehen statt umbauen:* In 3.8 Layout 2 von S-1 (Typ, Fahrzeug/Flugzeug) mit entwerfen, damit der nicht additive Schnitt einmal und nicht zweimal passiert.
- **Kosten:** niedrig bis mittel (eine Session für Zählregel und Reflexionsformat, eine für `main.ts`).
- **Risiko:** niedrig. Eine abgeschwächte Zählregel kann als „Frisieren der Warnung“ wirken. Daher als eigener ADR mit Vision-Frage („soll die Quote Strukturmängel oder Dokumentenänderungen messen?“) und sichtbare Parallelzählung der alten Regel für Phase 3.

### Option 3: Neu aufsetzen (B)

- **Kosten:** sehr hoch. Phase 2 ist in 7 Tagen entstanden, aber die Wiederholung wäre 27.000 Zeilen Code samt Plattformhärtung, Rauchtests auf drei Plattformen und getrennten Prüfungen.
- **Risiko:** hoch. Der Leistungsnachweis (Takt, Wegweiser, S-1, Bildrate) würde verloren und müsste neu gemessen werden.
- **Belege dagegen:**
  - Kein reaktiver ADR betrifft Kern, Brücke, Takt oder Wegfindung inhaltlich (Befund 4).
  - Der Kern ist durch Tests belegt (Befund 1).
  - Alle Platzhalter haben gedeckte Landeplätze (Befund 5).
  - Die Spikes sind Vorlagen, kein Altlast-Code: `spikes/` liegt außerhalb des Workspaces und Phase 2 hat Code neu geschrieben (`fahrplan.md` „Übernahme aus den Spikes“).
  - Nichts im Befund rechtfertigt das.

### Empfehlung der Prüf-Instanz (B)

**Option 2, auf Prozess und `main.ts` begrenzt; der Code-Kern wird weitergebaut.** Konfidenz: **mittel**. Hoch ist sie für „nicht neu aufsetzen“ (Kern, Tests, Messwerte, keine Strukturverschiebung belegt). Niedriger ist sie bei zwei Punkten: Ob S-1 die ruhende Welt tragen kann, ist erst mit 3.8 messbar. Wie schwer die Methodik-Last für die Entwicklungsgeschwindigkeit wirklich wiegt, zeigt erst der Aufwand von Phase 3, nicht die Phase-2-Zahlen.

### Hinweise für die Ausplanung von Phase 3 (B)

- **3.8 echt messen und vor 3.1 festschreiben.** Nach Befund 3 ist es der Punkt mit der größten Umbauwahrscheinlichkeit. Im Entwurf Layout 2 für bewegte Objekte (Typ, Ausrichtung, Fahrzeug, Flugzeug) und Atlas-Stufen 16/32 px gemeinsam messen (ADR-034 lässt beides ausdrücklich offen).
- **Messung „Bauaktion bis zum Bild“ in den Akzeptanzkriterien von 3.1** mit dem vorläufigen Zeichnen als Teil des Schritts, nicht als Nachsatz. Der Referenz-PC liegt jetzt bei 2-3 Bildern.
- **3.4 braucht seine Erkundung zu Blockadevermeidung wirklich vor dem Code** (Phasentyp-Disziplin). Die Mac-Spitzen aus 4.3 (75 ms Einzelschritt) sollten vorgezogen werden, bevor Fahrzeuge die Last erhöhen. Der Landeplatz 4.3 kommt sonst nach der Last, die er erklären soll.
- **Vertragsabgleich vor jedem Schritt, der S-n berührt, als feste Checkliste** (Maßnahmen 7-9 gelten schon, 2.11 hat die Prüfliste). Das ist die wirksamste Gegenmaßnahme gegen Reaktiv-ADRs in Phase 3.
- **Phase 3 besteht aus 8 Schritten (3.1-3.8), die Wucherungsschwelle ist Faktor 2 und +5 Schritte.** Nachprüf-Schritte zählen nicht, die Grenze ist also nicht knapp, aber 3.1-3.6 sind jeweils groß; vorab die Teilaufteilung festlegen.
- **Hygiene:** Logbuch (1.063 Zeilen) beim `[PHASEN-WECHSEL]` verdichten; `fahrplan.md` „Aktueller Stand“ nennt „7 / 10“, `decisions.md` Teil A „6 / 10“ (verschiedene Fenster), beim Sessionende angleichen.

Gelesene Quellen (Angabe der Instanz): `docs/architecture.md` (Abschnitte 0-3, 4 teilweise, 5-10), `docs/decisions.md` (vollständig), `docs/decisions/ADR-020.md`, `ADR-032.md`, `docs/fahrplan.md` (Aktueller Stand, Phase 2 Kopf und Teile der Schritte, Phasen 3-7, Querschnitt, Iterations-Reflexion), `docs/research/2.10-messung.md`, `2.4-getrennte-pruefung.md`; Code: `core/src/world.rs`, `frame.rs`, `network.rs`, `scenario.rs`, `simulation.rs` (Ausschnitt), `savegame/mod.rs`, `savegame/state.rs` (Anfang), `Cargo.toml`, `app/renderer/playfield.ts` und `app/src/main/main.ts` (Anfang), dazu Zeilen-/TODO-/Suppression-Zählungen über `core/ bridge/ app/ tools/ scripts/`. Nicht gelesen: Logbuch, Archiv, `phasenabschluss-1.md`, die Details der Schritte 2.2-2.7 und der übrigen ADRs, `docs/research/2.5/2.6/2.7/2.9/2.11`.

## Teil 2: Stellungnahme der bauenden KI

**Nachgeprüft am Repository (2026-10-19):**

- `core/src/adoption.rs` Z. 31–36 und 89–100 (A7): Eine neue Anfrage ersetzt `pending`; der ältere Auftrag läuft im Pool weiter, sein Ergebnis wird verworfen. Es gibt keinen Abbruch und keine Zusammenfassung von Anfragen. Der Befund trifft zu. Folge bei Dauerbau: Solange schneller gebaut wird, als eine Neuberechnung dauert, wird nie übernommen. Gleichzeitig laufen verworfene Berechnungen und belegen Speicher. Nicht reproduziert.
- `vitest.config.ts` Z. 14–22 (A2): `main.ts`, `core-host.ts`, `preload.cts`, `renderer/main.ts` und `playfield.ts` sind von der Coverage ausgenommen. Der Befund trifft zu.
- `core/src/scenario.rs` Z. 76: `#[allow(clippy::type_complexity)]` hat keinen Begründungs-Kommentar. A4 trifft zu, B5 („alle mit Begründung“) nicht. Das ist ein Verstoß gegen `CLAUDE.md` Abschnitt 15 („Lokale Suppressions“) seit 2.3, unbemerkt von Hooks und Prüfungen.
- `core/src/`: Es gibt keine internen Ereignisse zwischen Kern-Modulen. `core/src/event.rs` ist der äußere Ereignisstrom S-3. Der Satz in `architecture.md` Abschnitt 2 beschreibt also eine Absicht, keinen Zustand (A6).
- `docs/architecture.md` Z. 4: Die eigene Schwelle (Auslagerung ab 500 Zeilen, ADR-001) ist mit 682 Zeilen überschritten. Das ist eine offene Pflicht, kein Vorschlag (A12).
- `docs/fahrplan.md` „Aktueller Stand“ nennt 7 / 10, `decisions.md` Teil A 6 / 10. Das ist Drift seit ADR-035 (B, Hygiene). Behoben in diesem Phasenabschluss.

**Zustimmung:**

- Beide Instanzen schließen Option 3 mit Belegen aus. Dem stimme ich zu: Kein Befund zeigt auf Stack, Pattern, Betriebsart oder Datenmodell.
- Beide empfehlen Option 2 mit Konfidenz mittel. Ich empfehle ebenfalls Option 2, im Zuschnitt unten. Der Unterschied zu Option 1 liegt vor allem in der **Reihenfolge**: Was ohnehin kommt, kommt vor 3.1 statt mitten in 3.1/3.2 (A, Empfehlung).
- **Kernstruktur vor 3.1 (A6 a):** Ich halte das für den wichtigsten Befund. Er ist neu, Instanz B hat ihn nicht. Phase 3 bringt Objekte, Personal, Bauaufträge, Fahrzeuge und Flugzeuge in einen Weltzustand, der heute aus `World` mit Personen besteht. Jede Objektart braucht vier passende Stellen: Feld, Spielstand, Invarianten beim Laden und Prüfsumme. Billiger wird der Schnitt nicht mehr. Zuschnitt: ein Entwurfsschritt mit ADR (Kategorie 1 und 4), ausgerichtet auf das, was 3.1 und 3.2 brauchen. Dazu gehören die Reihenfolge der Fachmodule im Takt, Zustand und Spielstand je Modul, die Weitergabe von Wirkungen zwischen Modulen und die Ablage im Crate. Kein allgemeines Rahmenwerk. Für die internen Ereignisse gilt: entweder bauen oder den Satz in `architecture.md` korrigieren, je nach Entwurf.
- **Dauerbau (A7 b):** gehört als Akzeptanzkriterium in 3.1 mit Benchmark. Die Korrektur (Anfragen zusammenfassen, überholte Berechnungen nicht weiterrechnen) ist klein und liegt im Modul Takt & Weltzustand.
- **S-1 Layout 2 in 3.8 mitentwerfen (B3):** Zustimmung. Den nicht additiven Schnitt an S-1 nur einmal machen, Fahrzeuge und Flugzeuge mitdenken, auch wenn sie erst in 3.4 kommen.
- **`main.ts` teilen (A c, B 2):** Zustimmung. Es ist ein lokales Refactoring im Modul Plattform. Der Grund ist nicht die Länge, sondern dass Dateizugriff auf nicht vertrauenswürdige Spielstände außerhalb der Coverage liegt. Ich schlage einen eigenen kleinen Schritt vor 3.1 vor, weil 3.1 `game.new` dort hinzufügt. Die Begründung an `scenario.rs:76` zieht derselbe Schritt nach.
- **Erkundungen mit eigener Schritt-ID (A11):** Zustimmung. „Davor Erkundung“ ohne ID ist nach `CLAUDE.md` Abschnitt 6 kein gültiger Landeplatz. Das war ein Fehler meiner Planung.
- **Architektur auslagern (A d):** Zustimmung. Abschnitt 4 kommt nach `docs/architecture-schnittstellen.md`, Vertrag und „Umsetzung und Präzisierungen“ werden getrennt. Fehlercode-Listen aus den Rust-Typen herzuleiten wäre eine Änderung an Werkzeugen; das schlage ich erst vor, wenn die Auslagerung Drift zeigt.
- **Fahrplan-Text begrenzen (A):** Zustimmung. Ab Phase 3 hält ein Schritt-Eintrag Ergebnis, Belege und Landeplätze; Herleitungen stehen in `docs/research/`.

**Widerspruch:**

- **Zählregel per Auslegung ändern (A e, B 1):** Die Zählregel steht wörtlich in `CLAUDE.md` Abschnitt 6: „jeder ADR, der einen anderen ADR innerhalb von 14 Tagen … ändert … – unabhängig von Kategorie und Phasentyp“. Diese Datei bleibt projektübergreifend unverändert (Abschnitt 1). Anders als bei Regel-007 gibt es hier keinen Widerspruch zwischen zwei Regeln, den ein ADR auflösen könnte. Eine Auslegung „additiv zählt nicht“ würde die Regel umgehen, und genau davor warnt Instanz B selbst („Frisieren der Warnung“).
  - **Regelkonformer Weg:** `CLAUDE.md` Abschnitt 4 Nr. 5 macht nur Vertragsänderungen freigabepflichtig, die **nicht** rein additiv und rückwärtskompatibel sind. Additive Ergänzungen wie ADR-028, ADR-029 und ADR-031 brauchen danach keinen ADR. Der ADR entstand, weil `architecture.md` Z. 6 jede Änderung an `[BELASTBAR]`-Bestandteilen freigabepflichtig macht.
  - Vorschlag: Diese Projektregel per ADR an Abschnitt 4 Nr. 5 angleichen. Additive, rückwärtskompatible Ergänzungen eines Vertrags kommen dann mit Eintrag im Änderungsprotokoll des Vertrags (neue Datei `architecture-schnittstellen.md`) und im Logbuch, ohne eigenen ADR. Sie werden dem Projektinhaber im Schritt zur Kenntnis vorgelegt.
  - Die Quote misst dann weiter, was `CLAUDE.md` vorsieht. Sie wird nur nicht mehr durch ADRs aufgebläht, die gar nicht nötig waren.
  - Den Vorschlag, die Zählregel selbst zu ändern, gebe ich als Rückmeldung an das Vorlagen-Repo Dev-Templates weiter. Er gehört nicht in dieses Projekt.
- **Reflexionen zur Quote (A10, B7):** Der Befund trifft zu: 12 Reflexionen, jede mit „kein Umbau“, die Vorhersagen zum Absinken lagen wiederholt falsch. Die Reflexion ist aber Pflicht (`decisions.md` Teil A, `CLAUDE.md` Abschnitt 0 Auslöser 5). Wegfallen kann nur die Wiederholung.
  - Vorschlag: Solange die Quote aus demselben, schon reflektierten Muster kommt, gibt es je Phase **eine** Reflexion statt einer je ADR. Diese hier ist die Reflexion für Phase 2.
  - Eine neue Reflexion gibt es nur, wenn ein reaktiver ADR ein neues Muster zeigt, etwa wenn er Code in Kern, Takt, Wegweisern oder Brücke betrifft.
  - Das ist eine Auslegung von „Reflexion ergänzen“, keine Änderung der Zählung. Sie braucht die Zustimmung des Projektinhabers.
- **Linux früher messen (A11):** Mein Widerspruch aus Phasenabschluss 1 gilt weiter. Es gibt keine Mitwirkenden, und eine Linux-Maschine des Projektinhabers ist nicht bekannt. Der Landeplatz bleibt 7.2 (ADR-011). Hat der Projektinhaber einen Linux-Rechner, kann die Messung jederzeit vorgezogen werden. Das frage ich ab.
- **4.3 vorziehen (B):** Zustimmung im Grundsatz: Die Mac-Spitzen sollten erklärt sein, bevor Fahrzeuge die Last erhöhen. Phase 3 ist aber UMSETZUNG und 4.3 eine Ursachensuche. Vorschlag: 4.3 als Erkundungsschritt mit Phasentyp-Kontext ERKUNDUNG (wie 2.8) **vor 3.4** einordnen, unter neuer ID in Phase 3. Er braucht den Mac des Projektinhabers.
- **Abschätzung „mittel bis hoch“ für Option 1 (A):** Das halte ich für zu hoch. Spielstand-Format und Determinismus-Tests decken einen späteren Umbau ab, und vor 1.0 gibt es keine Zusage für alte Spielstände (ADR-021). Das Risiko liegt in den Kosten (Umbau über mehr Code), nicht in der Sicherheit. Meine Einschätzung: mittel.

**Ergänzung aus dem Vision-Abgleich (Teil 3):** eine Lücke (Stillstand im Passagierverkehr ohne Akzeptanzkriterium) und eine Abweichung in Gegenrichtung (Walk-out-Boarding MOD-13 ist „Kandidat“, steckt aber in 3.7 und über ADR-020 in 3.4).

### Vorschlag für Phase 3 bei Option 2 (zur Freigabe; ursprünglicher Schrittplan erst nach der Antwort)

Reihenfolge 3.7 → 3.8 → 3.9 → 3.10 → 3.1 → 3.2 → 3.11 → 3.3 → 3.12 → 3.13 → 3.4 → 3.5 → 3.6. Zusammen 13 Schritte; 3.9 bis 3.13 sind neu, die übrigen bestehen schon.

| Schritt | Kurztitel | Art |
|---|---|---|
| 3.7 | Personengruppen, Begehbarkeit und Querungen (unverändert; Walk-out nach Antwort zu MOD-13) | Entwurf (ERKUNDUNG) |
| 3.8 | Weltdarstellung im Spielfeld, **zusätzlich S-1 Layout 2 für Fahrzeuge und Flugzeuge mitentwerfen** und Atlas-Stufen 16/32 messen | Entwurf mit Messung (ERKUNDUNG) |
| 3.9 | **neu:** Kernstruktur der Fachmodule: Reihenfolge im Takt, Zustand und Spielstand je Modul, Weitergabe von Wirkungen, Ablage in `core/src`; ADR (Kategorie 1, 4); Umbau des bestehenden Codes | Entwurf, dann Umbau (UMSETZUNG) |
| 3.10 | **neu:** `app/src/main/main.ts` nach Zuständigkeiten teilen, Dateizugriff unter Unit-Tests; Begründung an `scenario.rs:76` nachtragen | Refactoring (UMSETZUNG) |
| 3.1 | wie geplant, **zusätzlich**: Dauerbau-Benchmark (eine Bauaktion alle 0,25–0,5 s, Übernahme der Wegweiser in fester Frist, Speichergrenze) und Korrektur in `adoption.rs`; „Bauaktion bis zum Bild“ mit Vorschau gemessen | UMSETZUNG |
| 3.2 | wie geplant, **zusätzlich** Akzeptanz „kein Stillstand im Passagierverkehr“ (Teil 3) | UMSETZUNG |
| 3.11 | **neu (bisher „davor Erkundung“ in 3.3):** Suchen ohne Schilder und Feinwege, dabei `MAX_TARGETS` und Speicher je Ziel im bestückten kleinen Terminal | ERKUNDUNG |
| 3.3 | wie geplant | UMSETZUNG |
| 3.12 | **neu (bisher „davor Erkundung“ in 3.4):** Blockadevermeidung (ADR-005) | ERKUNDUNG |
| 3.13 | **neu (vorgezogen aus 4.3, falls gewünscht):** Schrittzeit-Spitzen auf Apple-Chips | ERKUNDUNG |
| 3.4–3.6 | wie geplant | UMSETZUNG |

Daneben ohne eigenen Schritt (Dokumentation, Autonomiebereich): Abschnitt 4 nach `docs/architecture-schnittstellen.md` auslagern. Das erledige ich mit dem ADR zum Phasenabschluss.

## Teil 3: Vision-Abgleich an der Phasengrenze

Geprüft gegen `docs/vision.md` (Abschnitte 3–7) und `docs/requirements.md` (Abschnitte 3 und 5), dazu die 19 Modernisierungen „Kern“ aus `docs/modernisierung.md`.

| Befund | Art | Vorschlag |
|---|---|---|
| Erfolgskriterium „Kein Stillstand“ nennt Passagier-, Straßen- und Rollverkehr. Straße (FR-014, 3.4) und Rollverkehr (FR-028, 3.4) haben ein Akzeptanzkriterium. Für den **Passagierverkehr** gibt es keines: FR-016 und FR-017 prüfen Ablauf und Schrittzeit, nicht Stillstand. Die Vision nennt Stau vor dem Check-in ausdrücklich als Problem des Originals (Abschnitt 2). | Lücke (teilweise verwaist) | FR-016 um „1 h Spielzeit im Szenario ohne Stillstand, der sich nicht selbst auflöst“ ergänzen; Schritt 3.2, Wiederholung im Referenzszenario 4.1 |
| MOD-13 Walk-out-Boarding ist in `modernisierung.md` als **Kandidat** entschieden (nach der Funktionsgleichheit). ADR-020 lässt Fluggäste auf Vorfeld-Gehwegen „zum Treppenfahrzeug an Stellplätzen ohne Fluggastbrücke“ gehen (3.4); 3.7 nennt „Walk-out-Boarding (MOD-13)“. Das Original (GATE-06) bringt Fluggäste an Außenpositionen mit dem Vorfeldbus. | Abweichung in Gegenrichtung (Kandidat in Kern-Schritten, ohne Entscheidung) | Frage an den Projektinhaber: MOD-13 per ADR in den Kern holen (dann FR) oder Vorfeld-Gehwege in 3.4/3.7 auf Personal und Bus beschränken |
| „Leistungsbewusstsein“ steht im Kernumfang der Vision, FR-042 hat Priorität „Soll“ (bestätigt am 2026-10-13) | Hinweis, kein Bug (Schritt 5.8 vorhanden) | keine Änderung; bei 5.8 beachten |
| Erkundungen „Suchen ohne Schilder/Feinwege“ und „Blockadevermeidung“ ohne eigene Schritt-ID (nur „davor Erkundung“ in 3.3, 3.4) | Landeplatz ohne ID (Regelverstoß `CLAUDE.md` Abschnitt 6) | eigene Schritte 3.11 und 3.12 (Teil 2) |
| Übrige Elemente: Plattformen, Deutsch/Englisch (erledigt 2.6), Sprachpakete (erledigt 2.6), Benchmark und Warnung (5.8), Raster ohne Kurven (3.1, 5.2), Etagen (5.2), Bahnen und Rollweg-Knoten (3.4, 5.5), Gepäck (3.5, 5.5), Straßen und Parkhäuser (3.4, 5.3), Bauen ohne Wartezeit (3.1), Passagierführung und Suchen (3.3), Verkehrsregeln, Vorrang, Verkehrssteuerung (3.4), Vorfahrt umschaltbar (5.3), erklärbare Wegprobleme (3.3), Wartegründe (3.4, FR-028), Atmosphäre (6.2), moderne Abläufe und alle 19 Kern-Modernisierungen (FR-013 bis FR-038), Mods (2.4, 3.6, 6.1), Kartengrößen (3.1 Grundform, 5.1), Trennung Ankunft/Abflug freiwillig (3.2, 5.2), Langzeit (4.2, 7.2), Referenzszenario (4.1, 7.2), Untergrenze (5.8), Funktionsgleichheit (4.2, 6.3), Spielerlebnis und Machbarkeitstest (7.3), Cargo vorbereitet (erledigt 2.3), SVG mit Stilleitfaden (erledigt 2.10), Signatur (7.1), Lizenzen (ADR-007) | Schritt-ID vorhanden | – |
| Cargo spielbar (FR-048), isometrische Sicht (FR-051), Kandidaten | laut Vision nicht im ersten Umfang; in `requirements.md` als bewusst ausgeschlossen geführt (ADR-006) | – |
| `[VERSCHOBEN]`-Zeilen | keine im Fahrplan | – |

## Teil 4: Onboarding-Re-Validation (Abschnitt 16, Trigger 3, Klasse G)

Ergebnis siehe Logbuch-Eintrag `[ONBOARDING-VALIDATION]` vom 2026-10-19.
