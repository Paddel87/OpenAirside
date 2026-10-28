# Bewertung nach ADR-046: Weiterbauen, umbauen oder neu aufsetzen

<!-- Pflichtfrage nach CLAUDE.md Abschnitt 12, Anlass Vision-Pivot ADR-046, vor dem ersten Schritt der
     neu geplanten Phase 3 (3.2). Dieses Dokument enthält nur die Bewertung der getrennten Instanz.
     Stellungnahme der bauenden KI, Vorlage als ENTSCHEIDUNG ERFORDERLICH und ADR folgen getrennt;
     die Entscheidung fällt der Projektinhaber. -->

- **Datum:** 2026-10-29
- **Getrennte Instanz:** eigene Session ohne den Gesprächsverlauf der Umsetzung, nur lesend; Modell `claude-opus-5-5` (Entscheidungs-Klasse nach `docs/project-context.md` Abschnitt 6). Dasselbe Modell hat 3.1c gebaut; `CLAUDE.md` Abschnitt 12 wünscht „möglichst“ ein anderes. Eine zweite Bewertung mit `claude-sonnet-5-5` wie beim Phasenabschluss 2 wäre möglich.
- **Stand:** Branch `docs/adr-046-neuausrichtung` auf `6e52a2b` (PR #40, noch nicht gemergt), Code gleich `main` `df6da46`.
- **Quellen:** `docs/vision-2.md`, `docs/decisions/ADR-046.md`, `docs/architecture.md` (Abschnitte 1, 2, 3, 7, 8, 9), `docs/architecture-schnittstellen.md` (Grundlagen, S-1, S-7, Auszüge S-4, S-5), `docs/decisions.md` Teil A und C, `docs/requirements.md`, `docs/fahrplan.md` (Aktueller Stand, Phase 3, Phasen 4–7, letzte Reflexion), Code in `core/src` (`build/`, `wayfinding/`, `persons/`, `world.rs`, `internal.rs`, `display.rs`, `frame.rs`, `savegame/`, `scenario.rs`), `app/renderer`, `package.json`. Nicht gelesen: Logbuch, Archiv, frühere Phasenabschlüsse außer der Gliederung von `phasenabschluss-2.md`.
- **Kennzeichnung:** Befunde sind durchnummeriert; F = belegt mit Stelle, E = Einschätzung.

## 1. Befund

### Umfang und Qualität des Bestands

- **Befund 1 (F) – Umfang.** `core/src` 20.250 Zeilen Rust, `core/tests` 3.773, `core/benches` 816, `bridge/src` 565, `app/src/main` 3.470 und `app/renderer` 5.213 Zeilen TypeScript (ohne Tests). 279 Rust-Tests, 34 Vitest-Dateien, zwei Playwright-Rauchtests (`e2e/smoke.spec.ts`, `e2e/world.spec.ts`).
- **Befund 2 (F) – Prüfdichte.** Coverage Kern 98,17 % Zeilen, TypeScript 81,25 % (3.1c, `fahrplan.md`). Drei Benchmarks als CI-Gate: Takt (`core/benches/tick.rs`), Wegweiser (`flowfield.rs`), Dauerbau (`continuous_build.rs`). Spielstand mit Rundlauf und Zustands-Prüfsumme (`core/tests/savegame_roundtrip.rs`, `construction.rs`).
- **Befund 3 (F) – Leistungsreserve des Takts.** 10.000 Personen: Tick Mittel 0,066 ms, p99 0,127 ms (3.1a, CI), Gate bei 1 ms bzw. 4 ms. Der Takt hat damit rund das Zehnfache an Reserve für neue Logik je Person.
- **Befund 4 (F) – Leistungsreserve der Wegweiser ist knapp.** Neuberechnung beider Feldsätze Median 1.935 ms auf dem Runner, Grenze 4.000 ms (3.1c). Der Spike 3.7 maß für 79 Ziele 2,6 s auf 4 vCPU. `vision-2.md` Abschnitt 9 nennt das selbst als Risiko.
- **Befund 5 (F) – Platzhalter.** Neun `TODO(fahrplan-ref: …)`: 3.1 einmal (`core/src/savegame/mod.rs:57`), 3.2 einmal (`core/src/persons/mod.rs:41`), 3.4 viermal (`simulation.rs:694`, `build/access.rs:69`, `build/network.rs:6`, `savegame/mod.rs:60`), 4.1 einmal (`scenario.rs:7`), 5.1 zweimal (`savegame/mod.rs:175`, `bridge/src/lib.rs:139`). Alle Ziele existieren. Der Marker 3.1 ist überholt: Schritt 3.1 ist erledigt, die Grenze `MAX_OBJECTS` wird in `build/objects.rs:21` durchgesetzt; die Konstante in `savegame/mod.rs:58` ist eine zweite, ungenutzte Kopie.
- **Befund 6 (F) – Suppressions.** Elf `#[allow(clippy::…)]`, alle mit Begründung. Sechsmal `too_many_arguments` an Bau-Befehlen (`world.rs:179`, `:237`, `build/orders.rs:330`, `:386`, `:431`, `build/objects.rs:380`). E: Mit Bauphasen kommt ein weiterer Parameter dazu; ein Befehls-Datensatz statt langer Parameterlisten wäre billiger, bevor die Liste weiter wächst.
- **Befund 7 (F) – Reaktiv-Quote.** 4 / 10 (ADR-037 bis ADR-046), Schwellenwert 20 %. Die reaktiven ADRs der Phase 3 (036, 037, 038, 042, 045) entstanden laut Iterations-Reflexion aus zu knapp beschriebenen Schritten, nicht aus einem tragenden Fehler des Fundaments; jede Reflexion verneint die Frage „Architektur-Refactoring nötig?“. Maßnahmen (12) und (13) richten sich auf die Vorlagen vor dem Code.

### Was der Pivot im Code berührt

- **Befund 8 (F) – Der gebaute Code ist fast vollständig vision-neutral.** Takt und Zeitmodell, Brücke, S-1 bis S-8, Spielstand, Datenpakete, Raster mit Flächenarten und Zugangsprofilen, Wegweiser mit zwei Feldsätzen, Bauaufträge, Anlieferstellen und Bauarbeiter, Weltdarstellung. Nichts davon setzt Funktionsgleichheit oder ein Element voraus, das ADR-046 streicht. Die Reste der alten Vision im Code sind Kommentare: `core/src/build/target.rs:14` und `:22` („ADR-006“, „cargo extension after feature parity“), erzeugt auch in `app/src/generated/LoadKind.ts:5`.
- **Befund 9 (F) – Die Architektur ist nicht nach der neuen Vision fortgeschrieben.** PR #40 ändert `docs/architecture.md` und `docs/architecture-schnittstellen.md` nicht (`git diff --stat origin/main...HEAD`). Die Modul-Einträge tragen noch Verantwortungen, die ADR-046 streicht oder umdeutet: Flugbetrieb „Hubschrauber (DLC-Umfang), später Cargo“ und „Datenmodell erweiterbar für Cargo (ADR-006)“; Wirtschaft & Management „Forschung und Programme, Bewertungen und Serviceziele, Auszeichnungen“ (FR-037 verworfen, Bewertung über FR-063); Ereignisse „Tiere“ (verworfen); Personen „Gewerkschaft“, „Lehre aus dem Original“; Oberfläche „Forschung, Bewertungen“; Inhalte & Mods „Forschungsbaum“; Abschnitt 7 „Fracht als spätere Erweiterung (ADR-006)“. Die Reifegrad-Übersicht steht auf dem 2026-10-28.
- **Befund 10 (F) – Neue Verantwortungen der Vision haben kein Modul.** In Abschnitt 3 fehlt ein Besitzer für: Messung von Wartezeit, Dichte, Herkunft und Ziel sowie Engpass-Diagnose (S2, FR-057, FR-058; Spielfeld zeigt Overlays nur an, Regel-002 verlangt die Rechnung im Kern); Kennzahlen und Zusagen über alle Module (FR-038, FR-063; Wirtschaft nennt „alle Fachmodule liefern Kennzahlen“ ohne Weg); Läden mit Bestand und Lieferungen (FR-023, in keinem Modul); Beiladefracht (FR-048, nur „später Cargo“ in Flugbetrieb); Bauphasen und Sperrungen (FR-053, fällt natürlich an Welt & Bau, steht dort aber nicht); Szenarien und Startformen, Lektionen (FR-067, FR-007). Ein neues Modul ist Kategorie 2 nach `CLAUDE.md` Abschnitt 4.
- **Befund 11 (F) – Kleine Drift im neu geplanten Fahrplan.** 3.2 nennt den `TODO(fahrplan-ref: 3.2)` „in `world.rs`“, er steht in `core/src/persons/mod.rs:41`. 3.5 verlangt „Inventar-IDs nach Regel-004“, die aufgehoben ist. 3.6 verweist für das Markenrecht auf `docs/vision.md` statt `vision-2.md` und nennt AIR-01, MOD-01. Phase 3 nennt „Die Einsteigerin“, 3.6 „Szenario ‚Einsteiger‘“.

### Was dem Bestand für die Kernschleife fehlt

- **Befund 12 (F) – Personen warten nie und beeinflussen einander nicht.** `step_person` (`core/src/persons/mod.rs:429–461`) bewegt jede Person unabhängig entlang ihres Wegweisers. Am Ziel wählt sie sofort ein neues Ziel per Zufall (`:432–437`). Es gibt keine Bedienzeit, keine Kapazität eines Schalters, keine Warteschlange und keine Dichte. Ein Stau an der Sicherheitskontrolle, den Szenario „Die Einsteigerin“ und das Abschlusskriterium von Phase 3 voraussetzen, kann so nicht entstehen. Ebenso fehlt jede Größe, aus der ein Wartezeit-Overlay rechnen könnte.
- **Befund 13 (F) – Passagiere sehen das aktuelle Raster nicht, nur ihren Feldsatz.** `step_person` erhält nur den `FieldSet`; `FieldSet::step_target` prüft gegen die Topologie seines Schnappschusses (`core/src/wayfinding/flowfield.rs:527–547`). Eine neue Wand oder Sperrung wirkt für Fluggäste erst mit der Übernahme der Neuberechnung, auf dem Runner bis rund 2 s Echtzeit, bei 3× also rund 2 Spielminuten. Bauarbeiter prüfen dagegen das aktuelle Raster (`architecture.md`, Bauarbeiter). ADR-010 verbietet, Spiellogik vom Übernahmezeitpunkt abhängig zu machen. Für Sperrungen (FR-053) ist damit offen, wann eine Sperrung für Fluggäste gilt, ohne dass sie durch gesperrte Flächen laufen oder stehen bleiben; der Fahrplan nennt das nur als Notiz aus 3.1a.
- **Befund 14 (F) – Personen haben keine Identität.** `Person` (`core/src/persons/mod.rs:43–55`) hält Lage, Richtung, Ziel als `u8`, Fortschritt und Zufallszustand. Flug, Ticket, Gruppe, Ankunftszeit, Bedürfnisse fehlen (`TODO(fahrplan-ref: 3.2)`). Herkunft-Ziel-Overlays, Umsteiger, Mindestumsteigezeit und Zusagen gegenüber Airlines brauchen diese Felder. `Pool<T>` (`core/src/pool.rs`) ist der vorgesehene Baustein für kalte Felder.
- **Befund 15 (F) – Objekte haben keinen Betriebszustand.** Der Objektzustand in S-1 ist nur der Bauzustand 0/1/2 (`architecture.md`, Welt & Bau, ADR-045). Läden mit Lager, Schalter mit Besetzung oder Spuren mit Technikstufe brauchen einen Zustand je Objekt.
- **Befund 16 (F) – Kein Weg für Overlay-Daten zur Präsentation.** S-1 Layout 2 hat feste Abschnitte (Personen, Fahrzeuge, Flugzeuge, Gepäck, Welt; `core/src/frame.rs:28–38`); ein neuer Abschnitt ist nicht additiv und erhöht `protocol`. S-7 kennt Tabellen, Details, Wörterbuch, Katalog. Eine Abfrage für Rasterwerte (Dichte, Wartezeit je Block) wäre als neue `kind` additiv nach Regel-008.
- **Befund 17 (F) – Bestandsflughafen als Startform hat kein Datenformat.** Szenarien sind Rust-Code (`core/src/scenario.rs`: `openHall`, `referenceTerminal`, `furnishedTerminal`). `scenario.rs:7` plant das Referenzszenario für 4.1 „from data packs and a reference save game“. FR-067 verlangt den Bestandsflughafen schon in 3.2. Regel-003 (Inhalte sind Daten) verlangt ein Datenformat, also S-4 oder S-5.
- **Befund 18 (F) – Gewählte Oberflächen-Technik ist nicht im Programm.** ADR-002 und `project-context.md` Abschnitt 3 legen React 19.2.8, Tailwind 4.3.3 und TanStack Table 9.2.8 fest; `package.json` hat als Abhängigkeit nur `pixi.js`. Panels sind direkter DOM-Code (`app/renderer/build-panel.ts`, `saves-panel.ts`, `settings-panel.ts`). Der Reifegrad „Oberfläche mit großen Tabellen `[BELASTBAR]`“ stützt sich auf Spike 1.4 mit React. Kein Fahrplan-Schritt nennt die Einführung. E: Die neue Vision ist oberflächenlastig (Verträge, Kennzahlen, Diagnose, Lektionen, Flugplaner); der Zeitpunkt der Einführung sollte vor 3.2 feststehen.
- **Befund 19 (F) – 3.2 berührt ein Modul, das es nicht nennt.** 3.2 setzt FR-019 (Check-in), FR-020 (Sicherheitskontrolle) und FR-022 (Gates, Boarding) um. Diese liegen nach `architecture.md` im Modul Abfertigung & Gepäck. Die Liste „Betroffene Module“ von 3.2 nennt es nicht. Ohne Klärung greift Stopp-Kriterium 5 (fremde Modulgrenze) im Schritt selbst, oder die Abfertigung landet in Personen.
- **Befund 20 (E) – 3.2 ist der breiteste Schritt der Phase.** Neun Anforderungen, sechs Module, vier neue Mechaniken (Bauphase, Overlay, Lektion, Bestandsflughafen) und die erste Passagier-Logik mit Bedürfnissen. Die Neuplanung hat keine Schritte hinzugefügt, aber 3.2, 3.3 und 3.6 inhaltlich stark erweitert. Die Zählung nach Kriterium 9 erfasst diesen Zuwachs nicht. Das Muster der Reflexionen zu ADR-042 und ADR-045 (Lücken in knapp beschriebenen Schritten) trifft hier stärker zu als bei 3.1.

## 2. Trägt der Bestand die fünf Säulen?

| Säule | Was trägt (Beleg) | Was fehlt | Bewertung |
|---|---|---|---|
| S1 Umbau im laufenden Betrieb: Bauphasen mit Sperrungen | Aufträge als Objektart mit Arbeitsmenge (`build/orders.rs`), Bauarbeiter mit Anlieferstelle und Ladung (`persons/workers.rs`), Dauerbau-Gate mit längster Lücke 1.995 ms bei Grenze 8.000 ms (ADR-041), Zugangsprofil × Flächenart als Tabelle (`build/access.rs`), zwei freie Bits im Flag-Byte (`build/grid.rs:25–38`: belegt sind Bits 0–5) für ein Merkmal „gesperrt“ | Bauphase als Bündel von Aufträgen; Sperr-Semantik gegenüber veralteten Feldsätzen und ADR-010 (Befund 13); Umgang mit Personen in einer neu gesperrten Fläche; Störwirkung (braucht Zufriedenheit, Befund 14); Lkw-Logistik erst mit Verkehr (3.4) | trägt; eine Entwurfsfrage mit Wirkung auf ADR-010 |
| S2 Ströme lesen: Overlays und Engpass-Diagnose | Personen in zwei Phasen mit vertauschbaren Zählungen (`persons/mod.rs:307–340`, Zählung je Etage als Vorbild für Zählung je Zelle oder Block); Unerreichbarkeit im Wegweiser (`UNREACHABLE`), Ursachen nach ADR-037 für 3.3 vorgesehen; S-7 mit Drosselung auf 4 Abfragen/s | Warteschlangen und Bedienzeit (Befund 12) als Quelle von Wartezeit und Ursache „Kapazität“; Identität der Personen für Herkunft und Ziel (Befund 14); Modul für Messung und Diagnose (Befund 10); Übertragungsweg der Rasterwerte (Befund 16); Vergleich vor/nach braucht gespeicherte Messstände | trägt nicht von selbst; das Fundament verhindert nichts, aber die zentrale Größe „Wartezeit“ hat noch kein Modell |
| S3 Wellen und Umsteiger | Zeitmodell mit Kalender und Nachtsprung (`time.rs`, ADR-020); Objektarten über `Pool<T>`; Ereignis-Warteschlange zwischen Modulen (`internal.rs`) mit Regel „Erzeuger vor Verbraucher“ | Flugbetrieb ohne Code, Flugplan, Wellen; Person mit Flug und Anschluss (Befund 14); Platz von Flugbetrieb und Abfertigung in der Reihenfolge der Stufen (3.4–3.6) | trägt; nichts gebaut, nichts im Weg |
| S3 Beiladefracht | `LoadKind::Freight` an Zielen (`build/target.rs`, FR-046); Abschnitt Gepäck in S-1 mit Typ (8 Byte) für bewegte Ladung; Ziele nach Ladungsart als Wegweiser | Ladung als Objektart, Frachthalle, Umschlag am Stellplatz; Gepäckmodell (Einzelstücke oder Ströme) ist laut `architecture.md` noch eine offene Erkundung; NFR „bewegte Sprites“ `[VORLÄUFIG]` | trägt; Leistungsfrage offen wie für Gepäck |
| S5 Verträge mit messbaren Zusagen | S-7-Tabellen mit Sortierung im Kern (große Tabellen gemessen in 1.4); Wörterbuch `{ key }`/`{ text }` (ADR-032, L-6) | Wirtschaft & Management ohne Code; Messung der Zusagen braucht dieselben Messgrößen wie S2 (Wartezeit, Bodenzeit, Pünktlichkeit, Umsteigezeit); Modul-Zuschnitt Verträge zwischen Flugbetrieb und Wirtschaft unklar (beide nennen Verträge bzw. Gebühren) | trägt; Kennzahlen sollten mit S2 aus einer Quelle kommen |
| S5 Läden mit Bestand | Muster der Bauarbeiter (Anlieferstelle → Ziel, Ladung, Rückweg; `persons/workers.rs`) ist direkt übertragbar auf Lieferungen an Läden; `Pool<T>` für Lager | Betriebszustand je Objekt (Befund 15); kein Modul (Befund 10); Lieferung über Landseite braucht Verkehr (3.4) | trägt; Zuschnitt offen |

**Leistung über alle Säulen (E):** Der Takt hat Reserve für Warteschlangen, Zählungen und kalte Felder (Befund 3). Eng wird die Neuberechnung der Wegweiser (Befund 4): Die neue Vision bringt mehr Ziele (Schalter je Airline, Läden, Umsteigeschalter, Lager) und häufigere Neuberechnungen (jede Bauphase, jede Sperrung). `MAX_TARGETS = 255` (`build/target.rs:11`, Ziel als `u8` in `Person`) und die Grenze von 4 s prüft 3.11 erst nach 3.2.

## 3. Optionen

### Option 1: Weiterbauen

3.2 wie geplant: Vorlage nach Maßnahme (13), dann Umsetzung.

- **Kosten:** am geringsten vorab; keine Änderung an Code oder Architektur vor 3.2.
- **Risiko:** hoch für die Planung, gering für den Code. Die Vorlage zu 3.2 muss mindestens fünf Architekturfragen klären, die der Fahrplan nicht als Entscheidung vorsieht: Warteschlangen-Modell (Befund 12), Sperr-Semantik gegen ADR-010 (Befund 13), neues Modul für Messung und Diagnose (Befund 10), Übertragungsweg der Overlays (Befund 16), Datenformat des Bestandsflughafens (Befund 17), dazu die Modulgrenze zur Abfertigung (Befund 19). Mehrere davon sind Kategorie 1, 2, 4 oder 5 und fielen in einer UMSETZUNG-Phase unvorhergesehen, wären also nach `CLAUDE.md` Abschnitt 6 wahrscheinlich `[REAKTIV]`. Bei 4 / 10 hält das die Quote über dem Schwellenwert, und jeder davon ändert Code in Welt & Bau oder Wegfindung, ist also nach Regel-009 ein neues Muster mit Stopp.
- **Belege:** Befunde 9–19; Muster der Reflexionen zu ADR-042 und ADR-045.

### Option 2: Gezielt umbauen

Kein Umbau des Codes, sondern der Planung und der Architektur vor 3.2. Die Teile:

1. **Architektur nach `vision-2.md` fortschreiben** (`docs/architecture.md` Abschnitte 2, 3, 7, 9): gestrichene Verantwortungen entfernen (Befund 9); neue Verantwortungen einem Modul zuweisen (Befund 10). Zu entscheiden ist vor allem, ob Messung, Kennzahlen und Engpass-Diagnose ein eigenes Kern-Modul werden, das S2 und S5 aus einer Quelle bedient, und wo Läden und Fracht liegen. Kategorie 1 und 2, als ADR.
2. **Entwurfsschritt vor 3.2** (Phasentyp-Kontext ERKUNDUNG, wie 3.7 und 3.8), Ergebnis als ADR mit Messung:
   - Warteschlangen und Bedienstellen: Kapazität, Bedienzeit, Reihenfolge deterministisch (etwa seriell wie die Bauarbeiter oder nach Personen-Index), Wartezeit je Person, Kosten bei 10.000 Personen.
   - Sperrungen: wann eine Sperrung für Fluggäste gilt, solange der neue Feldsatz rechnet, ohne ADR-010 zu verletzen (etwa Vorlauf der Bauphase in Spielzeit, Prüfung des aktuellen Rasters im Schritt der Personen, Räumen der Fläche).
   - Kalte Felder der Personen mit Identität (Flug, Gruppe, Zeitstempel) und Betriebszustand von Objekten.
   - Übertragungsweg der Rasterwerte: S-7-Abfrage (additiv nach Regel-008) oder Abschnitt in S-1 (`protocol` 3).
   - Startform Bestandsflughafen als Datenpaket-Inhalt oder Spielstand (S-4 oder S-5).
   - Zahl der Ziele und Neuberechnungs-Takt bei Bauphasen gegen die Grenze von 4 s, also der Teil von 3.11, der 3.2 betrifft.
3. **3.2 in Stufen teilen** (wie ADR-040 für 3.1), etwa: Szenario mit Passagierablauf, Bedienstellen und Wartezeit; Bauphase mit Sperrung und Umleitung; Overlay und erste Lektion. Modulliste um Abfertigung & Gepäck ergänzen oder Abfertigung bewusst in Grundform anders zuordnen (Befund 19).
4. **Einführung von React festlegen** (Befund 18): Schritt mit ID, vor der ersten großen Oberfläche der neuen Vision, oder per ADR verwerfen.
5. **Kleine Bereinigung im Code**, Routine: ungenutzte Konstante und überholten Marker in `core/src/savegame/mod.rs:57–58` entfernen; Kommentare in `core/src/build/target.rs:14`, `:22` (und erzeugt `LoadKind.ts`) auf ADR-046 und FR-048 umstellen; Drift im Fahrplan aus Befund 11 beheben.

- **Kosten (E):** ein bis zwei Sessions für Architektur und Entwurfsschritt, dazu Messläufe; eine Session für Bereinigung und Fahrplan. Der Schrittplan der Phase wächst um einen Entwurfsschritt (15 → 16, Schwelle 26). Kein Code wird verworfen.
- **Risiko:** gering. Die Entscheidungen fallen geplant statt in der Umsetzung; sie bauen auf vorhandenen Bausteinen auf (`Pool<T>`, interne Ereignisse, Zugangsprofile, Feldsätze je Gruppe, Muster der Bauarbeiter). Restrisiko: Der Entwurfsschritt zeigt, dass Warteschlangen mit Wegweisern nicht flüssig zusammengehen (etwa Dichte, die Wege verändern müsste). Das wäre eine Grundsatzfrage zu ADR-004 und würde dort früh statt mitten in 3.2 auffallen.
- **Belege:** Befunde 9–20; Abschnitt 2.

### Option 3: Neu aufsetzen

Neuer Kern, neue Präsentation oder beides, von der neuen Vision her entworfen.

- **Kosten:** sehr hoch. Rund 20.000 Zeilen Kern, 9.000 Zeilen TypeScript, 279 Rust-Tests, drei Benchmark-Gates und die gemessenen Reifegrade auf Referenz-PC und Mac (ADR-014, ADR-034, ADR-038, ADR-044) wären neu zu bauen bzw. neu zu messen. Phasen 1 und 2 sowie 3.7–3.15 und 3.1 entsprechen dem Großteil der bisherigen Projektzeit.
- **Risiko:** hoch, ohne erkennbaren Gewinn. Kein Befund zeigt eine Grundsatzentscheidung, die der neuen Vision widerspricht. Die tragenden Entscheidungen – Kern/Hülle-Trennung (ADR-003), Wegweiser statt Einzelweg-Suche (ADR-004), Raster in Blöcken (ADR-020), Personengruppen mit Zugangsprofilen (ADR-037), Bauaufträge mit Bauarbeitern (ADR-045) – sind Voraussetzungen der Säulen S1 und S2, nicht Hindernisse. Ein Neuaufbau müsste dieselben Fragen aus Abschnitt 2 ebenfalls beantworten.
- **Belege:** Befunde 1–3, 8; Abschnitt 2.

## 4. Einschätzung der getrennten Instanz

Diese Einschätzung ist keine Entscheidung. Sie fasst zusammen, wohin die Belege zeigen.

- **Das Fundament trägt die neue Vision.** Der Pivot ändert Spielentwurf und Maßstab, nicht die technischen Voraussetzungen. ADR-046 sagt „der gebaute Code ist kaum betroffen“; die Durchsicht bestätigt das (Befund 8).
- **Nicht nachgezogen sind Architektur und der Zuschnitt von 3.2.** Die Kernschleife der neuen Vision hängt an einer Größe, die es im Bestand nicht gibt: Wartezeit an Bedienstellen (Befund 12). Daran hängen Engpass, Overlay, Diagnose, Zusagen und Lektion. Dazu kommt die Sperr-Semantik, die ADR-010 berührt (Befund 13).
- **Option 2 in der beschriebenen Form** (Architektur, Entwurfsschritt, Stufen für 3.2, React-Zeitpunkt, Bereinigung) passt nach den Belegen am besten. Option 1 ist zulässig, verschiebt dieselben Entscheidungen aber in die Umsetzung von 3.2 und erhöht die Reaktiv-Quote. Für Option 3 findet die Instanz keinen Beleg.
- **Konfidenz:** hoch für „kein Neuaufbau“ (breite Belege); mittel für den Zuschnitt von Option 2, weil das Warteschlangen-Modell noch nicht gemessen ist.

## 5. Offene Fragen, die nur der Projektinhaber beantworten kann

1. Soll vor 3.2 ein eigener Entwurfsschritt laufen, auch wenn er die Phase um einen Schritt verlängert?
2. Wie streng soll eine Sperrung wirken: Dürfen Fluggäste eine frisch gesperrte Fläche noch kurz durchqueren, oder muss die Bauphase einen Vorlauf haben, in dem die Umleitung sichtbar vorbereitet wird? (Spielgefühl, berührt ADR-010.)
3. Soll das Spiel Warteschlangen sichtbar zeigen (Personen stehen in Reihe) oder genügt eine gemessene Wartezeit mit Overlay? (Aufwand im Kern und in der Darstellung sehr verschieden.)
4. Wann soll die Oberfläche auf React umgestellt werden, oder bleibt sie bei direktem DOM-Code?
