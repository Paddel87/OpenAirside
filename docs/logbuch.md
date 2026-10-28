# Logbuch

<!-- Chronologischer Flugschreiber von OpenAirside. Ereignisbasierte Einträge, neueste oben.
     Zweck:
       1. Nahtlose Fortsetzung in neuer Session: was war zuletzt los, womit ging es zu Ende?
       2. Wiederfindbarkeit kleiner Lösungen
       3. Selbst-Beobachtung des Projekts

     Abgrenzung zu anderen Dokumenten:
       - fahrplan.md: Was tun wir? (Plan)
       - decisions.md: Warum so? (Begründung)
       - architecture.md: Wie ist es gebaut? (Zustand)
       - blockers.md: Was hindert uns aktuell? (offene Probleme)
       - CHANGELOG.md: Was hat sich für Nutzer geändert? (extern, versionsorientiert)
       - logbuch.md: Was ist während der Arbeit passiert? (intern, chronologisch) -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

Phase 1 abgeschlossen (ADR-016), Detail-Einträge in `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) abgeschlossen am 2026-10-19 mit „gezielt umbauen“ (ADR-036), Detail-Einträge in `docs/archiv/logbuch-phase-2.md`. Phase 3 (Vertikaler Schnitt „Kleiner Flughafen“) läuft: ausgeplant mit 13 Schritten, jetzt 15. 3.7, 3.8, 3.14, 3.15 und 3.9 sind erledigt, 3.10 und Q.4 sind erledigt, 3.1a ist erledigt und gemergt (PR #35), 3.1b ist im Code fertig und wartet auf Messungen und Grafik-Abnahme des Projektinhabers (`docs/research/3.1b-messung.md`), danach 3.1c.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-28 18:00 (UTC) – [ADR-ANGELEGT]

- **ADR:** ADR-045 „Bauaufträge mit Bauarbeitern, Anlieferstelle, Gelände in neuen Karten“, freigegeben per Auswahl („A: Auftrag“, „A: Anlieferstelle“, „A: Gelände“, „Übernehmen“).
- **Tag:** `[REAKTIV]`; Reaktiv-Quote 5 / 10, Reflexion mit Maßnahme (13) in `fahrplan.md` „Iterations-Reflexion“, vom Projektinhaber übernommen.
- **Auslöser:** Vorlage 3.1c nach Maßnahme (12). 3.1c jetzt `[IN ARBEIT]`.

### 2026-10-28 17:50 (UTC) – [BEOBACHTUNG]

- **Modellwechsel bestätigt:** Projektinhaber hat nach dem Stopp auf `claude-opus-5-5` gewechselt; `get_session` („self“) meldet `claude-opus-5-5` → Entscheidungs-Klasse aktiv, passt zur Vorlage mit `ENTSCHEIDUNG ERFORDERLICH`.
- **Vorlage 3.1c** (`docs/research/3.1c-vorlage.md`) nach Maßnahme (12): Je Teilaufgabe die gelesenen Daten mit Fundstelle. Funde: (1) Bau-Befehle ändern ihre Bedeutung (S-2 „Wirkung im Schritt der Annahme“) → nicht additiv; (2) ein neues Spiel ist vollständig Leere (`World::new`, `Cell::VOID`), Bauarbeiter könnten nirgends gehen; (3) die Topologie der Wegweiser ist für alle Profile gleich (Blöcke mit irgendeiner begehbaren Zelle) – begehbares Gelände würde jedes Fluggast-Feld über das ganze Erdgeschoss ziehen, deshalb eine Topologie je Feldsatz; (4) Gruppen-Nummern in S-1 nirgends festgelegt; (5) Arbeitsmenge und Anlieferstelle fehlen in S-4.
- **Einstufung:** `[REAKTIV]` (Schritt 3.1 sah außer dem CI-Gate keine Freigabe vor), Reaktiv-Quote danach 5 / 10; neues Muster nach Regel-009 (Code in Welt & Bau, Wegfindung, Personen) → Reflexion mit Stopp, Entwurf in der Vorlage Abschnitt 7 mit Maßnahme (13).
- **Risiko:** Wegweiser-Benchmark mit dem Satz der Bauarbeiter auf dem Runner nahe der Grenze 4 s (bisher 2,4–3,2 s, Satz der Bauarbeiter ≈ 0,6 s auf 4 vCPU nach 3.7).

### 2026-10-28 17:20 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-26 14:30 UTC; danach PR #37 (Q.5) und PR #36 (3.1b) gemergt, `main` auf `568af2c`, sauber. 3.1a, 3.1b und Q.5 `[ERLEDIGT]`; offen daneben 3.1b-N1 (Grafik-Abnahme, Zeitpunkt nennt der Projektinhaber).
- **Geplant für diese Session:** 3.1c Bauaufträge und Bauarbeiter (FR-011), Branch `feat/3-1c-bauauftraege-und-bauarbeiter` von `main` angelegt.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Schritt 3.1, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht); dazu `docs/research/3.1-vorlage.md` Abschnitte 1–4, ADR-037 und ADR-042 (Vertiefung für 3.1c).
- **Vorabprüfung:** Welt & Bau (Grundraster, Bauen mit Wirkung seit 3.1a), Datenmodell im Kern, S-1, S-2, S-5, S-7 `[BELASTBAR]`; Eingangskriterien von 3.1 erfüllt. Befund: Bauauftrag, Bauarbeiter (Gruppe, Anlieferstelle, Anforderung und Entlassung), Fortschritt und Zustand „im Bau“ sind in keinem ADR und keinem Vertrag festgelegt (ADR-040 und der Fahrplan nennen nur Stichworte; ADR-042 hat die Sofortwirkung aus 3.1a ausdrücklich bis 3.1c offen gelassen); die Umsetzung berührt Datenmodell (Kategorie 4, Spielstand-Format), Modulbeziehung Welt & Bau ↔ Personen (Kategorie 1) und S-1/S-5 (Kategorie 5).
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` laut `get_session` („self“) 17:19 UTC → Routine-Klasse. Kontingent: `get_session` liefert hier keine `rate_limit_info`, nicht feststellbar.
- **Vorhaben und Klasse:** Schritt 3.1 empfiehlt Entscheidung für die Bewertung, Routine für die Umsetzung. Die Vorlage vor dem Code mit `ENTSCHEIDUNG ERFORDERLICH` ist Eskalations-Auslöser 1 (`CLAUDE.md` Abschnitt 0): Stopp mit Modellwechsel auf die Entscheidungs-Klasse, bevor die Vorlage entsteht.


- **Merges nach Freigabe des Projektinhabers** („Merge du beide, erst #37, dann #36“): PR #37 (Q.5, ADR-043) gemergt als `56bdc8c`; `main` in den Branch von 3.1b geholt. Konflikte nur in `docs/decisions.md` (Teil A: Zeilen ADR-043 und ADR-044, Reaktiv-Quote jetzt 4 / 10 über ADR-035 bis ADR-044, gesamt 17 / 44) und `docs/logbuch.md` (Einträge beider Branches nach Zeit geordnet).
- **3.1b `[ERLEDIGT]`:** DoD erfüllt, CI grün auf `1dd8082` (39 Checks); Merge-Stand läuft durch die neue CI (Code aus `main` enthalten, daher voller Lauf). Grafik-Abnahme bleibt als 3.1b-N1 offen.

### 2026-10-26 15:05 (UTC) – [PROBLEM-GELÖST]

- **Q.5 erledigt:** Nachweise auf PR #37: (1) Code-Push `0c59ece` nur als PR-Lauf, 23 Checks grün; (2) Doku-Push `f5e6269` auf grünem Stand in rund 20 s, Pflicht-Checks des Rulesets bestanden, PR mergebar; (3) erzwungener Fehlerfall: roter Rust-Test `ff1cfab`, Doku-Push `adcfc91` darauf → volle Pipeline, rot; Revert `312c9c6`, danach `c79c1b1` grün. Dieser Eintrag selbst ist der erste Doku-Commit, der nach dem grünen Stand nur den Doku-Weg nehmen soll.

### 2026-10-26 15:00 (UTC) – [REIFEGRAD-WECHSEL]

- **Nachmessung 3.1b eingetroffen:** Referenz-PC 14:50 UTC, Mac 14:51 UTC (Pakete aus Release-Lauf 40339206366 auf `2b36e3d`, Messergebnis Version 3); Rohdateien `docs/research/3.1b-ergebnisse/referenz-pc-nachmessung.json`, `mac-nachmessung.json`; Auswertung `docs/research/3.1b-messung.md` 5.3–5.5. Keine Störung gemeldet, alle Fenster gewertet.
- **Ergebnis gegen die vorab festgelegten Schwellen (4a):** `previewFramesMax` 1 in allen 20 Fenstern auf beiden Rechnern; Referenz-PC 59,9–60 Bilder/s, p99 17,3–17,8 ms, 0 späte Antworten, 180 Ticks/s; Speicher 641 MB (PC) bzw. 841 MB (Mac, bei s = 40). Das Bild von 160,5 ms aus der ersten Messung kam nicht wieder.
- **ADR-044:** „NFR Bauaktion im nächsten Bild“ `[VORLÄUFIG]` → `[BELASTBAR]`; „Speicher der Weltdarstellung“ `[BELASTBAR]` (seit 14:25 UTC) bestätigt; Auflage Windows aus 3.15 erfüllt. Entscheidungs-Klasse aktiv (Opus 5.5).
- **Befund ohne Kriterium:** Mac einmal 179 Ticks/s (Fenster 8), ein Verzögerungsereignis (Fenster 19, 6 Ticks), Schritt bis 39,2 ms; Muster der Erkundung 3.13. Bauaktion bis zum Bild am Mac einmal 5 Bilder (32,8 ms, Schwelle 100 ms eingehalten, Ziel 1–2 Bilder verfehlt).
- **ADR-Nummer:** ADR-043 trägt die CI-Trigger (Q.5, Branch `ci/q-5-ci-trigger`, PR #37); die Reifegrade erhalten ADR-044.

### 2026-10-26 14:45 (UTC) – [ADR]

- **ADR-043 (Q.5):** CI-Trigger nur für `main` und PRs, Code-Jobs nicht bei reinen Doku-Änderungen; Option B, freigegeben vom Projektinhaber per Auswahl. Anlass: rund 270 CI-Jobs für acht Pushes auf PR #36, davon sieben reine Doku.
- **Fund beim Entwurf:** Der Ruleset „main-schutz“ (öffentlich lesbar über die API) verlangt 19 Checks namentlich, darunter die Matrix-Jobs je Betriebssystem. Ein auf Job-Ebene übersprungener Matrix-Job meldet nicht unter seinem vollen Namen; deshalb überspringen die Matrix-Jobs ihre Schritte. Zweiter Fund: Scheitert der Entscheidungs-Job, würden abhängige Jobs übersprungen und als bestanden gelten; deshalb laufen die Code-Jobs, außer die Entscheidung ist ausdrücklich `false`.
- **Lokal geprüft:** `scripts/ci-changes.test.mjs` (10 Tests, darunter Doku auf rotem, laufendem und unlesbarem Stand); echter Diff im Repo: zwei Doku-Commits → `false`, Bereich mit Code → `true`, ohne Token (HTTP 401) → `true`.
- **Modell:** Entscheidungs-Klasse (Opus 5.5, laut `get_session`), passt zur Empfehlung von Q.5.

### 2026-10-26 14:30 (UTC) – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung 2026-10-26 13:50 bis 14:30 UTC (Session ab 2026-10-25 20:54 UTC).
- **Bearbeitet:** 3.1b: Auswertung der Messungen Referenz-PC und Mac, Grafik-Abnahme verschoben (3.1b-N1), Reifegrade, Bildzählung der Vorschau im Messwerkzeug (Option C).
- **Erreicht:** Commits `c7cab0c`, `868fd07`, `b027de8`, `ee59a4d`, `aa57ae0`, `2b36e3d`, `b4c2086` (gepusht, PR #36). „Speicher der Weltdarstellung“ `[BELASTBAR]`. Lokal grün: Vitest Renderer (18 Dateien, 177 Tests), `tsc`, ESLint; `measure 1` im Container mit Version 3. CI grün auf `aa57ae0`; Lauf auf `ee59a4d` rot nur im Artefakt-Upload (`ENOTFOUND`), PR-Kommentar. Release-Lauf 40339206366 auf `2b36e3d` für die Nachmessung gestartet.
- **Offen geblieben:** Nachmessung auf beiden Rechnern durch den Projektinhaber (zugesagt „nachher“); danach Auswertung nach `docs/research/3.1b-messung.md` Abschnitt 4a, ADR-043 zu den Reifegraden, `[ERLEDIGT]` für 3.1b, Merge nur nach Freigabe. Grafik-Abnahme 3.1b-N1, Zeitpunkt nennt der Projektinhaber.
- **Nächster Schritt:** Messdateien auswerten.
- **Modell-Bilanz:** bis 14:07 UTC Routine-Klasse (Sonnet 5.5, laut `get_session`), danach Entscheidungs-Klasse (Opus 5.5, Wechsel durch den Projektinhaber nach Stopp). Schritt 3.1b empfiehlt Entscheidung für die Bewertung; die Auswertung lief vor dem Wechsel auf Routine (nur Eintragen der Zahlen, keine Bewertung), die Bewertung auf Entscheidung. Keine Teilarbeit an Unteragenten abgegeben (Probelauf offen).
- **Kontextgröße:** 292.231 von 1.000.000 Token laut `get_session`, unter der Grenze von 400.000.

### 2026-10-26 14:25 (UTC) – [REIFEGRAD-WECHSEL]

- **Modellwechsel:** Stopp „Modellwechsel erforderlich“ (Beförderung nach `[BELASTBAR]`) aufgehoben; laut `get_session` eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse.
- **Speicher der Weltdarstellung:** `[VORLÄUFIG]` → `[BELASTBAR]`. Kriterium aus ADR-038 (höchstens +256 MB gegenüber 2.10) auf beiden Rechnern eingehalten: Referenz-PC höchstens 671 MB (Schwelle 811), Mac höchstens 849 MB (Schwelle 930).
- **Nicht befördert:** „NFR Bauaktion im nächsten Bild“ bleibt `[VORLÄUFIG]` (Zeit bis zur Vorschau im Maximum über einem Bildtakt; Messgröße sagt nicht, in welchem Bild die Vorschau stand). „NFR Performance unter Windows“ bleibt `[BELASTBAR]` bis zur Nachmessung: Fenster 9 (59,4 Bilder/s) fällt zeitlich auf einen Bildschirm-Standby (Abschaltung nach 5 Minuten, Angabe des Projektinhabers; 300 s nach der letzten Eingabe ≈ 289 s Fensterzeit, Fenster 9 von 271 bis 301 s).
- **Entscheidung des Projektinhabers (Option C von A/B/C):** 3.1b bleibt `[IN ARBEIT]`; Bildzählung der Vorschau ins Messwerkzeug (`previewFramesMax`, Messergebnis Version 3), neue Pakete, Nachmessung auf beiden Rechnern mit Bildschirm-Abschaltung aus. Schwellen vorab festgelegt in `docs/research/3.1b-messung.md` Abschnitt 4a. Container-Prüfung: `measure 1` mit Version 3, `previewFramesMax` 1 in beiden Fenstern.
- **CI auf `ee59a4d`:** „TS Tests & Coverage (ubuntu-latest)“ rot nur im Schritt `upload-artifact` (`ENOTFOUND`), alle 473 Tests grün; PR-Kommentar, Neustart abgelehnt, solange der Lauf lief.

### 2026-10-26 14:20 (UTC) – [BEOBACHTUNG]

- **Messungen 3.1b eingetroffen:** Referenz-PC (Windows, 13:48 UTC) und Mac (14:03 UTC), Rohdateien in `docs/research/3.1b-ergebnisse/`, Auswertung gegen die Schwellen in `docs/research/3.1b-messung.md` Abschnitt 5 (Commits `c7cab0c`, `868fd07`). Nur Zahlen aus den Dateien, keine Schätzung. Auffällig: Referenz-PC Fenster 9 mit 59,4 Bildern/s und einem Bild von 160,5 ms; Mac `ticksPerSecMin` 162 (Fenster 16, längster Schritt 110,5 ms) und 179 (Fenster 10), 7 Verzögerungsereignisse, ohne Wirkung auf das Bild; Vorschau der Bauaktion am Referenz-PC im Median meist ein Bildtakt (16,6–17,7 ms), im Maximum bis 28,6 ms. Notizen zum Zuschauen: Referenz-PC: Bildschirm einmal kurz im Standby (siehe Messdokument 5.1); Mac: keine.
- **Grafik-Abnahme verschoben (Antwort des Projektinhabers):** „Die Grafiken würde ich gerne zu einem anderen Zeitpunkt abnehmen. Die können ja auch noch mal komplett durchgetauscht werden.“ Die Verschiebung hat ihren Landeplatz im neuen Fahrplan-Schritt 3.1b-N1 `[OFFEN]` (Abschnitt 6 der Harten Regeln); die Testgrafiken aus 2.10 bleiben im Spiel. Vorlage 3.1 und Messdokument entsprechend vermerkt.
- **Reifegrade nicht entschieden:** Die Beförderung von „Speicher der Weltdarstellung“, „NFR Bauaktion im nächsten Bild“ und „NFR Performance unter Windows“ nach `[BELASTBAR]` ist ein Eskalations-Auslöser (Entscheidungs-Klasse, `CLAUDE.md` Abschnitt 0, Punkt 4). Aktive Klasse: Routine (Sonnet 5.5). Die Entscheidung wartet auf einen Wechsel.

### 2026-10-25 21:50 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 20:54 bis 21:50 UTC.
- **Bearbeitet:** 3.1b: S-1 Layout 2 mit `worldAck`, `game.new`, Weltdarstellung (Z-1), Bau-Werkzeug, möbliertes Szenario und Messreihe, Rauchtest über vier Sitzungswechsel, Dokumente, Messanleitung.
- **Erreicht:** Commits `0634e65`, `9edb236`, `0b2c685`, `7c97219`, `a21643d` auf `feat/3-1b-darstellung-und-bedienung` (gepusht; CI nachgetragen 21:46 UTC: grün auf `9b730e8`, Läufe 40236635139, 40236700118 und Security 40236700122; PR #36 eröffnet). Lokal grün: Rust-Tests (Kern 167 Lib-Tests, alle Integrationstests), clippy, Vitest 480+ Tests, ESLint, Prettier, `tsc`, Rauchtests `smoke` und `world` unter xvfb, `measure 1` im Container. 3.1b bleibt `[IN ARBEIT]`.
- **Offen geblieben:** Release-Pakete starten (Workflow „Release“ auf dem Branch, die KI hat ihn nicht gestartet); Messungen auf Referenz-PC und Mac und Grafik-Abnahme (Wand A/B, Boden A/B, Sitzbank) durch den Projektinhaber nach `docs/research/3.1b-messung.md`; danach Auswertung, Reifegrade, PR (nicht eröffnet: keine Anweisung dazu in dieser Session), `[ERLEDIGT]`. Coverage der Kern-Änderung nicht gemessen (CI-Gate prüft).
- **Nächster Schritt:** Ergebnisse des Projektinhabers auswerten; danach 3.1c.
- **Aktive Klasse und Bilanz:** Routine (Sonnet 5.5 laut `get_session` 20:54 und 21:45 UTC); 0 Schritte oberhalb der Empfehlung; nichts abgegeben (Probelauf der Unteragenten-Klassen offen).
- **Kontextgröße:** laut `get_session` nicht feststellbar (meldet 0 von 1.000.000); die Session war sehr umfangreich, die nächste Arbeit sollte in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung und Drift-Check:** Register geprüft, kein Eintrag am Vorlauf. Keine neuen ADRs; Reaktiv-Quote unverändert 4 / 10; Fahrplan-Stand, README und Runbook nachgezogen; Landeplatz der 253-Typen-Grenze in 6.1 eingetragen; kein CHANGELOG vorhanden (Version 0.0.0). Onboarding-Pfad: Quick-Start-Befehle unverändert, daher keine Neu-Validierung des Klons; die Rauchtests liefen im Arbeitsbaum.

### 2026-10-25 21:40 (UTC) – [PROBLEM-GELÖST]

- **Reibungen in 3.1b, je mit Lösung:**
  1. `pre-commit` war im neuen Container nicht installiert und der Hook hat drei Commits wegen ESLint und `tsc` gestoppt (`prefer-function-type` bei `game.new`: In einem Typ-Literal ist `new(…)` eine Konstruktor-Signatur, kein Methodenname; Lösung: Eigenschaft `readonly new: (…) => …`; `no-unnecessary-condition` bei einem `let`, das nur einmal gesetzt wird: Lösung: `const` vor die Handler gezogen). `pip install pre-commit` und `pre-commit install` im Container.
  2. Der Budget-Test des Welt-Senders baute sein Muster aus `floor_kind`-Nummern, die ohne Typtabelle alle zu „fehlt“ übersetzt werden und damit gleichförmig sind; Muster stattdessen aus Zone, Kostenfaktor und Flächenart.
  3. `Build::build_floor` über Leerzellen **pflastert** sie (Semantik von `buildArea`). Das möblierte Terminal setzt den Boden deshalb je Lauf begehbarer Zellen einer Zeile; die Wände folgen auf den Leerzellen daneben.
  4. `npm ci --ignore-scripts` lädt das Electron-Paket nicht; `node node_modules/electron/install.js` holt es über den Proxy. Der Test der Messreihe mit Playwright muss auf `data-session` warten, nicht auf `data-ready` (der Port kommt später); sonst meldet die Konsole „measure: not connected“.
  5. Ein neuer Stempel je Block (`Block::stamp`, Änderungszähler des Rasters beim Ändern) ersetzt das Festhalten gesendeter Blöcke im Sender; sonst hätte jede Änderung eines gesendeten Blocks ihn kopieren müssen (einer der Posten für den Speicher auf dem Mac in 3.8).

### 2026-10-25 21:40 (UTC) – [BEOBACHTUNG]

- **3.1b umgesetzt** (Commits `0634e65`, `9edb236`, `0b2c685`, `7c97219`). Entscheidungen im Rahmen von ADR-038 und ADR-040 („Technisch, Entscheidung der KI“), die den Wortlaut des Entwurfs präzisieren, ohne eine Kategorie aus Abschnitt 4 zu berühren:
  1. **Kopf und Abschnitt von Layout 2:** Kopf zu 48 Byte mit Welt-Revision und den Zahlen der Fahrzeuge, Flugzeuge und Gepäckstücke (je `u32`); die Zahl der Welt-Blöcke und die Bytes des Abschnitts stehen im 16-Byte-Kopf des Welt-Abschnitts, der immer da ist. Beschrieben in `architecture-schnittstellen.md` (S-1, Umsetzung 3.1b).
  2. **Typen in Welt-Blöcken** sind Katalog-Index plus 1 (0 keiner, `0xFFFF` unbekannt oder über 16 Bit); Objekte gehören zum Block ihres Ankers.
  3. **`worldAck`** quittiert die Welt-Revision des zuletzt **gezeichneten** Bildes; ohne Quittung bietet der Kern dieselben Blöcke wieder an. Ein Bild einer fremden Sitzung oder ein Handler, der wirft, quittiert nichts.
  4. **Kachelblatt:** höchstens 253 Boden- und Wandtypen, darüber Ersatzkachel; Landeplatz Schritt 6.1 (Grafiken in Mods).
  5. **Zonentönung** nur für Zone 1 bis 3 (Luftseite, Kontrolle, Ausgang); die Landseite bleibt ungetönt, bis 3.2 Zonen als Spielerwerkzeug einführt.
  6. **Bedienung:** Ist ein Werkzeug gewählt, baut die linke Maustaste; rechte und mittlere Taste verschieben die Ansicht; R dreht, Esc beendet das Werkzeug. Der Katalog wird mit dem ersten Bild einer Sitzung abgefragt (S-7 antwortet aus dem jüngsten übergebenen Bild).
  7. **Neues Szenario `furnishedTerminal`** (S-2 Debug-Aktion, additiv nach Regel-008): Das Referenz-Terminal mit Boden, Wänden und 20.000 Bänken (ohne Sperrwirkung) für die Messung; ein Feld der Halle bleibt frei für die Bauaktionen der Messreihe. Das Szenario `referenceTerminal` und seine Benchmarks bleiben unverändert.
  8. **Messreihe, Ergebnisformat 2:** echte Befehle `buildArea` (Wand) und `demolish`, Zeit bis zur Zeichnung der Vorschau und bis zum Bild mit dem geänderten Block, Erstübertragung der Welt (Ausschnitt, ganz), Zahl abgelehnter Bauaktionen.
- **Container-Prüfung nach Maßnahme (11):** `e2e/world.spec.ts` (neues Spiel 1536, möbliertes Szenario, neues Spiel 2048, neues Spiel 1536, Ablehnung mit Hinweis) und die vollständige Messreihe `measure 1` über das echte Programm: Welt in einem Bild (4.988 Blöcke, 0,84 MB, 20.000 Objekte), 26 Datentexturen (26,1 MB), Bauaktionen 6 von 6 angenommen, 179–180 Ticks/s. Bildraten ohne Aussage (SwiftShader). Alle bisherigen Rauchtests unverändert grün.
- **Abgrenzung zu ADR-038:** Der Entwurf nennt „Kopf 48 Byte“ mit „Zahl der Welt-Blöcke, Bytes des Welt-Abschnitts“ im Kopf; die Einzelheit steht jetzt im Abschnittskopf (gleiche Information, ein Feld weniger im Hauptkopf). Kein Vertragsbruch, weil Layout 2 vorher nirgends umgesetzt war.

### 2026-10-25 20:55 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 19:55 UTC, danach `[PROBLEM-GELÖST]` 20:05 UTC (unbeständiger Test, CI auf `2b386a4` grün). 3.1a ist auf `main` gemergt (PR #35, Merge-Commit `63070a1`); 3.1b `[OFFEN]`.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Schritt 3.1, Architektur 1/2/9, Decisions Teil A und C, Requirements-Übersicht, aktive Blocker: keine); Vertiefung folgt: `docs/research/3.1-vorlage.md` (gelesen), ADR-038, ADR-040, ADR-042, `docs/research/3.8-weltdarstellung.md`, `docs/architecture-schnittstellen.md` (S-1, S-2, S-6, S-7, S-8).
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` laut `get_session` 20:54 UTC → Routine-Klasse; passt zur Empfehlung für die Umsetzung (Bewertungen sind in ADR-038/040/042 entschieden). Kontextgröße 0 von 1.000.000 (frische Session).
- **Branch:** `feat/3-1b-darstellung-und-bedienung` vom aktuellen `main` (`63070a1`).
- **Vorhaben:** 3.1b nach Fahrplan und Vorlage; Messungen auf Referenz-PC und Mac sowie die Grafik-Abnahme macht der Projektinhaber (Messwerkzeug und Anleitung bereitstellen, dann anhalten). Freigabepflichtige Entscheidung außerhalb von ADR-038/040/042 oder Eskalations-Auslöser → Stopp.

### 2026-10-25 20:05 (UTC) – [PROBLEM-GELÖST]

- **CI rot auf `1f7a35c` (Läufe 40228076332 und 40228097329):** `core/tests/build.rs`, Test `the_buildings_survive_a_save_game_and_go_on_identically`, letzter Vergleich der Zustands-Prüfsumme des ganzen Weltzustands schlug auf Linux, Windows und in der Coverage fehl. Ursache: Die Prüfsumme enthält den Zustand der Wegfindung (offene Neuberechnung, Übernahme); die beiden Simulationen laufen im Modus „sobald fertig“ und übernehmen zu verschiedenen Zeitpunkten. Der Lauf davor (`160628b`) war zufällig grün. Es ist ein Fehler des Tests, nicht des Kerns: Der Rundlauf selbst (Prüfsumme direkt nach dem Laden) bleibt unverändert geprüft. Der Test vergleicht danach nur den Beitrag von Welt & Bau und die Objekte. Behoben in `2b386a4`; die Läufe auf diesem Stand sind grün (Push 40231648278, PR 40231652631, Security 40231652556).

### 2026-10-25 19:55 (UTC) – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung 19:00 bis 19:55 UTC (Session ab 18:55 UTC).
- **Bearbeitet:** 3.1a Teilaufgaben 3, 4, 6 nach ADR-042 und die Kosten der Profil-Prüfung; Pflichtlektüre, `[SESSIONSTART]`.
- **Erreicht:** Commits `0229cad`, `0c68b9a`, `b14eb98`, `d8ee3ff`, `160628b` (Datenpaket `effect`/`blocks`; Bodenbelag und Wandart; Objekte, Bau-Befehle, Spielstand; schnellere Neuberechnung; Dokumente). CI-Lauf 40227300244 auf `160628b`: alle 17 Jobs grün; Runner-Werte: Neuberechnung 2357 ms (Grenze 4000), Dauerbau längster Abstand 2700 ms und letzter Stand 4584 ms (Grenze 8000), 229 MB. 3.1a `[ERLEDIGT]`. PR „feat: 3.1a Kern – Bauen mit Wirkung und Dauerbau“ eröffnet.
- **Offen geblieben:** Merge des PR nach Freigabe des Projektinhabers; 3.1b, 3.1c. Die Senkung der Profil-Prüfung ist nur im Container belegt (−15 %), die Runner-Werte schwanken (3219 → 2357 ms zwischen zwei Läufen); 3.1c prüft die Reserve erneut mit dem Feldsatz der Bauarbeiter.
- **Nächster Schritt:** PR mergen (nach Freigabe), dann 3.1b (Darstellung und Bedienung; Layout 2 von S-1 mit Umsetzungstabelle Typtabelle → Katalog-Index). Empfohlene Klasse laut Fahrplan 3.1: Entscheidung für Bewertungen, Umsetzung Routine.
- **Aktive Klasse und Bilanz:** Routine (Sonnet 5.5 laut `get_session` 19:00 und 19:55 UTC); 0 Schritte oberhalb der Empfehlung (Umsetzung = Routine); nichts abgegeben (Probelauf der Unteragenten-Klassen offen, nichts Abgabefähiges).
- **Kontextgröße:** 459.863 Token laut `get_session` – **über der Grenze von 400.000**; nächste Arbeit in neuer Session beginnen.
- **Ablaufdaten-Prüfung und Drift-Check:** Register geprüft, kein Eintrag am Vorlauf. ADR-042 → 3.1a (existiert), Reaktiv-Quote unverändert 4 / 10, Anforderungen FR-010/FR-011 bleiben `OFFEN` bis 3.1 vollständig, Blocker keine, Phasenumfang unverändert (Stufen zählen als ein Schritt), README „Nächste Schritte“ und Konsolenbeschreibung nachgezogen.

### 2026-10-25 19:40 (UTC) – [PROBLEM-GELÖST]

- **Kosten der Profil-Prüfung gesenkt (3.1a):** Im inneren Schritt der Neuberechnung (`dijkstra` in `wayfinding/flowfield.rs`) wird jetzt zuerst die billige Frage „ist der Weg kürzer“ gestellt und erst danach die Eckenregel und die Zugangsregeln; die meisten Nachbarn einer Zelle sind schon abgeschlossen. Ergebnis unverändert (Tests gegen die dichte Referenz, Zustands-Prüfsumme). Messung im selben Container, abwechselnd alt/neu mit je einem Binärpaar aus `cargo bench --no-run`, vier Paare: alt 2010 / 1872 / 1921 / 1789 ms, neu 1512 / 1640 / 1749 / 1571 ms, Mittel **1898 → 1618 ms (−15 %)**, in jedem Paar schneller. Die Maschine dieser Session ist langsamer als die der Vorsession (alt 1727–1926 ms statt 1462 ms), die Werte sind daher nur untereinander vergleichbar. Zwei frühere Versuche (Tabelle je Flag-Byte, Prüfungen der Zielzelle herausgezogen) hatten nichts gebracht; die Reihenfolge der Prüfungen war die Ursache. Der Wert auf dem Runner steht nach dem CI-Lauf des PR im Fahrplan.
- **Reibung (Test):** Der Vergleich der Zustands-Prüfsumme vor und nach einem abgelehnten Befehl schlug fehl, weil die vorherige Bauaktion im selben Schritt die Neuberechnung anstieß (`dirty` der Wegfindung). Der Test vergleicht jetzt nur den Beitrag von Welt & Bau. Außerdem baut `Simulation::new` eine offene Halle, nicht eine leere Welt; die Bau-Tests nehmen `World::new`.

### 2026-10-25 19:35 (UTC) – [BEOBACHTUNG]

- **3.1a Teilaufgaben 3, 4, 6 umgesetzt** (Commits `0229cad`, `0c68b9a`, `b14eb98`, `d8ee3ff`). Entscheidungen im Rahmen von ADR-042 („Technisch, Entscheidung der KI“), die vom Wortlaut des Entwurfs abweichen oder ihn präzisieren, ohne eine Kategorie aus Abschnitt 4 zu berühren:
  1. **Typtabelle statt Katalog-Index im Kern.** Bodenbelag und Wandart je Zelle sowie der Typ jedes Objekts sind Nummern (≥ 1) einer Tabelle voller IDs im Modul Welt & Bau, die beim ersten Bauen eines Typs wächst. Damit ist die Tabelle im Spielstand genau das, was ADR-042 für die Datei verlangt, und der Kern braucht keine Umrechnung aller Blöcke beim Speichern und Laden. S-1 (3.1b) rechnet beim Zusammenstellen der Blöcke über eine Umsetzungstabelle auf den Katalog-Index um (`Catalog::index_of` und `id_at` liegen mit Tests bereit). Vermerkt in `architecture.md` (Modul Welt & Bau, Datenmodell, verworfene Alternativen) und im Hinweis von S-1.
  2. **Handle = Seriennummer** (aufsteigend, nie wiederverwendet) statt Slot und Generation: Datensätze liegen in Handle-Reihenfolge, die Suche ist eine Halbierung, ein veraltetes Handle findet nichts. Der Weg über `geometry::Handle` für die Präsentation (S-7) bleibt für 3.1b offen und braucht keine Änderung des Kerns.
  3. **Kein Feld „Zustand“** im Objekt: Kein Schritt schreibt oder liest es, ein Platzhalter ohne Schritt wäre eine stille Verschiebung (`CLAUDE.md` Abschnitt 6). S-1 sendet 0, bis ein Schritt Zustände einführt (Hinweis in `architecture-schnittstellen.md` und im Fahrplan für 3.1b).
  4. **`at` = Zelle mit den kleinsten Koordinaten des Fußabdrucks**, Drehung 1 und 3 tauschen Breite und Tiefe. Der Vertrag S-2 sagte dazu nichts; steht jetzt in S-2. Wer eine andere Lage des Ankers will (Mitte), ändert es mit dem Bau-Werkzeug in 3.1b.
  5. **Sperrende Objekte setzen `WALL` ohne Wandart.** So bleiben Wegweiser, Zugangsregeln und Personen unverändert; ob eine Zelle eine Wand ist, sagt die Wandart (≠ 0), ob ein Objekt darauf steht, das Belegungsverzeichnis. Beim Abriss eines sperrenden Objekts wird `WALL` auf den Zellen außerhalb des Rahmens wieder entfernt; innerhalb des Rahmens wird die Zelle Leere.
  6. **Boden über Wandzellen:** die Wandzelle bleibt unverändert (auch ihr Bodenbelag). Boden über einem sperrenden Objekt ändert Bodenbelag, Flächenart und Kostenfaktor, die Zelle bleibt gesperrt.
- **Landeplatz:** Personen auf Zellen, die durch Bauen unbegehbar werden, bleiben stehen (Richtung „unerreichbar“); das Verhalten legt 3.2 mit den Personen fest (Notiz im Fahrplan, Schritt 3.2).
- **Werkzeuge im Container:** Node 22, `npm ci --ignore-scripts`, `pre-commit`, `cargo-llvm-cov` 0.9.1 nachinstalliert; `check:types` und `check:schema` laufen auch unter Node 22 und sind unverändert bzw. aktualisiert.

### 2026-10-25 19:00 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 18:25 UTC, `[BEOBACHTUNG]` 18:45 UTC (erster CI-Lauf mit Dauerbau-Schritt grün). ADR-042 freigegeben, Code für 3.1a Teilaufgaben 3, 4, 6 nicht begonnen.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Schritt 3.1, Architektur 1/2/9, Decisions Teil A und C, Requirements-Übersicht, aktive Blocker: keine); Vertiefung: ADR-042, `docs/research/3.1a-objekte-vorlage.md`.
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` laut `get_session` 19:00 UTC → Routine-Klasse; passt zur Empfehlung (Entscheidungen gefallen, ADR-042). Kontingent: 5-Stunden-Fenster `allowed`. Kontextgröße laut `get_session` 0 von 1.000.000 (frische Session).
- **Branch:** `feat/3-1a-bauen-mit-wirkung-dauerbau` auf `8258e70`, weitergeführt (Anweisung des Projektinhabers).
- **Vorhaben:** Teilaufgaben 3, 4, 6 nach ADR-042 und die Kosten der Profil-Prüfung; danach CI und PR „feat: 3.1a Kern – Bauen mit Wirkung und Dauerbau“. Eine freigabepflichtige Entscheidung außerhalb von ADR-042 → Stopp.

### 2026-10-25 18:45 (UTC) – [BEOBACHTUNG]

- **Erster CI-Lauf mit Dauerbau-Schritt** (Lauf 40219644818, Job „Kern-Benchmark“, grün): Tick Mittel 0,072 ms, p99 0,145 ms; Wegweiser-Neuberechnung **3219 ms** (Grenze 4000 ms), Schritt p99 0,46 ms; Dauerbau 120 Bauaktionen, **8 Übernahmen** beim Bauen, längster Abstand **3645 ms** (Grenze 8000), letzter Stand nach **5250 ms** (Grenze 8000), Spitzen 1 Neuberechnung / 3 Feldsätze / 2 Schnappschuss-Kopien, 223 MB, Schritt p99 0,65 ms. Die Frist aus ADR-041 trägt auf dem Runner, der Spielraum beim letzten Stand ist 2,75 s.
- **Korrektur einer eigenen Aussage:** Im `[SESSIONENDE]` 17:54 UTC, in `architecture.md` und im Commit `d86e3c1` stand „kein Rückgang, Neuberechnung 1457 ms“. Das war nur gegen alte Dokumentwerte verglichen. Abwechselnde Messung im Container: **1234 ms vor, 1462 ms nach `d86e3c1`, rund +18 %** durch die Prüfung der Zugangsprofile im inneren Schritt. Auf dem Runner bleiben 0,78 s Abstand zur Grenze von 4 s; 3.1c (Feldsatz der Bauarbeiter, +0,57 s auf 4 vCPU nach ADR-037) und mehr Ziele (3.2) verkleinern ihn.
- **Zwei Optimierungsversuche** (Tabelle „begehbar je Flag-Byte“, Prüfungen der Zielzelle aus dem inneren Schritt gezogen) brachten nur 1435 ms, im Rauschen; verworfen, nicht committet. Ein Lauf ohne Profil-Prüfung ergab 1283–1329 ms, die Kosten liegen also im inneren Schritt.
- **Landeplatz:** 3.1a (noch offen) bekommt die Teilaufgabe „Kosten der Profil-Prüfung senken oder begründet hinnehmen“, mit Messung vor 3.1c; Eintrag im Fahrplan.

### 2026-10-25 18:25 (UTC) – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung 18:05 bis 18:25 UTC nach dem `[SESSIONENDE]` 17:54 UTC (gesamt 17:25 bis 18:25 UTC).
- **Bearbeitet:** Vorlage `docs/research/3.1a-objekte-vorlage.md`, Entscheidung, ADR-042, Reflexion mit Maßnahme (12).
- **Erreicht:** ADR-042 freigegeben (1A, 2A, 3A, Festlegungen übernommen); Fahrplan, Decisions Teil A, Architektur Abschnitt 8 nachgezogen.
- **Offen geblieben:** 3.1a Teilaufgaben 3, 4, 6 (Code), dann PR für 3.1a und CI-Lauf mit dem Dauerbau-Schritt.
- **Nächster Schritt:** neue Session, Routine-Klasse genügt: 3.1a Teilaufgaben 3, 4, 6 nach ADR-042 auf dem Branch `feat/3-1a-bauen-mit-wirkung-dauerbau`.
- **Aktive Klasse und Bilanz:** Entscheidung (Opus 5.5 nach `/model`), passend zur Empfehlung (Auslöser 1); 0 Schritte oberhalb der Empfehlung; nichts abgegeben.
- **Kontextgröße:** 439.802 Token bei Wiederanlauf laut `get_session` – **über der Grenze von 400.000**; nächste Arbeit in neuer Session beginnen.
- **Ablaufdaten-Prüfung und Drift-Check:** keine Änderung gegenüber 17:54 UTC; ADR-042 in Teil A und als Datei, Reaktiv-Quote in Teil A und Fahrplan gleich (4 / 10).

### 2026-10-25 18:20 (UTC) – [ADR-ANGELEGT]

- **ADR-042** Wirkung der Objekttypen (Feld `effect` im Objekttyp), Möbel sperren ihre Zellen, Abriss entfernt alles Berührte; Festlegungen aus Abschnitt 4 der Vorlage übernommen. `[REAKTIV]`, Reaktiv-Quote 4 / 10 (ADR-033 bis ADR-042). Freigabe: „1A, 2A, 3A, Übernehmen“.
- **Regel-009:** neues Muster (Code in Welt & Bau) → Reflexion in `fahrplan.md` „Iterations-Reflexion“ mit Maßnahme (12), vom Projektinhaber mit „Übernehmen“ zur Kenntnis genommen; kein Umbau.

### 2026-10-25 18:10 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 3.1a, Vorlage `docs/research/3.1a-objekte-vorlage.md`: Wirkung der Objekttypen (Frage 1: Feld `effect` im Objekttyp, Empfehlung A), Sperren durch Möbel (Frage 2, A), Abriss mit Rahmen (Frage 3, A); Festlegungen ohne eigene Frage in Abschnitt 4.
- **Klassifikation:** `[REAKTIV]` (in der Planung von 3.1 nicht als Entscheidung vorgesehen); Regel-009 neues Muster (Code in Welt & Bau) → Reflexion mit Stopp, Entwurf in Abschnitt 6 der Vorlage.

### 2026-10-25 18:05 (UTC) – [SESSIONSTART]

- **Anlass:** Fortsetzung derselben Session nach `STOPP – MODELLWECHSEL ERFORDERLICH`; der Projektinhaber hat per `/model` gewechselt („Modellwechsel durchgeführt, weiter mit Entscheidung zu 3.1a“).
- **Modell:** eingestellt bei Anlage `claude-sonnet-5-5`, umgestellt (`user_switched_model`) und bedient `claude-opus-5-5` laut `get_session` → Entscheidungs-Klasse. Stopp aufgehoben.
- **Kontext:** 439.802 Token laut `get_session`, über der Grenze von 400.000; Empfehlung, die Umsetzung nach der Entscheidung in einer neuen Session zu beginnen.
- **Vertiefung:** `templates/architektur-heuristiken.md`, `architecture.md` Abschnitt 8, `3.8-weltdarstellung.md` (Objekte je Block), `core/src/pool.rs`.

### 2026-10-25 17:54 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 17:25 bis 17:54 UTC.
- **Bearbeitet:** 3.1a Teilaufgaben 1, 2, 5, 7 und 8 (Reihenfolge der Umsetzung 1, 5, 2, 8, 7). Pflichtlektüre, Branch (`feat/3-1a-bauen-mit-wirkung-dauerbau` auf `b1ef134`, Sessionende-Commits der Vorsession übernommen nach `project-context.md` Abschnitt 10).
- **Erreicht:** sieben Code-Commits (`a454f70`, `67d6bfe`, `05c2b4c`, `d86e3c1`, `b19d25b`, `9eb36c9` als Korrektur zu `b19d25b`, `8370f91`) und Dokumentation:
  - **Dauerbau (ADR-041):** Benchmark `core/benches/continuous_build.rs` als Schritt im Job „Kern-Benchmark“; gegen den unkorrigierten Code reproduziert (Ableitung aus ADR-036 bestätigt), nach der Korrektur innerhalb Frist und Grenze (Zahlen im Eintrag `[PROBLEM-GELÖST]` unten). Der CI-Lauf des neuen Schritts steht aus (Frist 8 s auf dem Runner nicht gemessen, Konfidenz mittel laut ADR-041).
  - **Flächenart, Personalzone, Zugangsprofil (ADR-037):** im Flag-Byte bzw. am Ziel; Wegweiser pro Profil berechnet; Neuberechnung im Wegweiser-Benchmark unverändert (1457 ms), Tick-Mittel 0,093 ms.
  - **`{ key }` in `ParamValue` (L-5)** und **S-7 `catalog` (L-2)** im Kern, in der Plattform-Prüfung und im Renderer-Client; TypeScript-Typen erzeugt.
  - Dokumente nachgezogen: `architecture.md` (Wegfindung, Welt & Bau, CI-Test), `architecture-schnittstellen.md` (S-7, `ParamValue`, Änderungsprotokoll), `fahrplan.md` (Stand 3.1a).
- **Offen geblieben:** 3.1a Teilaufgaben 3 (Objekte im Weltzustand), 4 (Wirkung von `build`, `buildArea`, `demolish`, `core/tests/build.rs`) und 6 (Objekte im Spielstand, Vertragsabgleich S-5). **Grund: Information fehlt** (`CLAUDE.md` Abschnitt 6, „Keine stillen Annahmen“; Stopp-Kriterium 1) – die Festlegungen stehen im Eintrag `[BEOBACHTUNG]` „Offene Festlegungen“ unten. Es sind Datenmodell (Kategorie 4) und Schnittstellen S-4/S-2/S-5 (Kategorie 5, additiv); damit steht ein `ENTSCHEIDUNG ERFORDERLICH`-Block an, Pflicht-Eskalation auf die Entscheidungs-Klasse (`CLAUDE.md` Abschnitt 0, Auslöser 1).
- **Nächster Schritt:** Modellwechsel auf die Entscheidungs-Klasse (`/model`), dann den Entscheidungsblock zur Wirkung der Objekte vorlegen und nach Freigabe 3.1a Teilaufgaben 3, 4, 6 umsetzen (Routine-Klasse genügt wieder). Danach PR für 3.1a (Titel `feat: 3.1a Kern – Bauen mit Wirkung und Dauerbau`), CI-Lauf beobachten, insbesondere den Dauerbau-Schritt.
- **Aktive Klasse und Bilanz:** Routine (Sonnet 5.5) die ganze Session; 0 Schritte oberhalb der Empfehlung (Empfehlung für die Umsetzung: Routine); nichts abgegeben (Probelauf der Unteragenten-Klassen steht aus, nichts Abgabefähiges nach `templates/wirtschaftlichkeit.md` angefallen). Beim Auslöser 1 wurde nicht auf der Routine-Klasse weitergearbeitet.
- **Kontextgröße:** nicht feststellbar (`get_session` meldet `used_tokens` 0); Gesamtverbrauch laut Token-Zähler der Sitzung rund 0,43 Mio. Token. Falls der Kontext über der Grenze von 400.000 liegt: neue Session empfohlen. Hinweis dazu in der Abschlussmeldung.
- **Ablaufdaten-Prüfung:** Register geprüft, kein Eintrag hat seinen Vorlauf erreicht (frühester: Electron 44, Lebensende 2027-03-02, Vorlauf ab 2027-01-02). Node 22 im Container (Werkzeug nur lokal, nicht die festgelegte Version 26.11.1), `npm ci --ignore-scripts` und `pre-commit` 4.6.2 wurden für die lokalen Prüfungen installiert und der pre-commit-Hook mit `pre-commit install` eingerichtet.
- **Drift-Check:** `fahrplan.md` „Aktueller Stand“ nannte Reaktiv-Quote 6 / 10 (ADR-030 bis ADR-039), `decisions.md` 4 / 10 (ADR-032 bis ADR-041): auf 4 / 10 angeglichen. ADR → Schritt (ADR-040/041 → 3.1a), Anforderung → Schritt (FR-010/FR-011 → 3.1, Status OFFEN bis 3.1 vollständig), Blocker (keine), Phasenumfang (Stufen zählen als ein Schritt): ohne Befund. README: „Nächste Schritte“ (3.1a, 3.1b, 3.1c) unverändert gültig, Status-Block unverändert.
- **Stimmung/Beobachtung:** Der Dauerbau-Fehler war so schwer, wie abgeleitet: Das letzte Ergebnis wird nie übernommen, weil die Warteschlange der Aufträge die jüngste Anfrage hinter 100 überholten einreiht.

### 2026-10-25 17:53 (UTC) – [BEOBACHTUNG]

**Offene Festlegungen zu 3.1a Teilaufgaben 3, 4 und 6** (für die Entscheidungs-Klasse; weder Vorlage 3.1 noch ADR-037 oder ADR-038 treffen sie):

1. **Wirkung eines Objekttyps auf das Raster.** Der Objekttyp in S-4 hat nur `name` und `placement` (`area` oder `single` mit Breite und Tiefe). Nirgends steht, was `floor_concrete` (Boden), `wall_concrete` (Wand), eine Zone, eine Treppe oder `bench` im Raster bewirken: Zelle begehbar oder `WALL`, Flächenart und Personalzone, Zone (Land, Luft, Kontrolle, Ausgang), Kostenfaktor, Treppe, Bodenbelag und Wandart (Katalog-Index für S-1, 3.8). Ein Zusatzfeld im Objekttyp wäre eine additive Änderung von S-4 (Modformat und JSON-Schema); die Ordnung „Fläche setzt Zellwerte“ gegen „Objekt belegt Zellen“ ist offen.
2. **Objekte im Weltzustand.** `Pool<T>` und das Merkmal `Kind` sind da (3.9), `MAX_OBJECTS` 1.000.000 nur als Konstante. Offen: Speicherung je Block (so 3.8, „Katalog-Index, Lage im Block, Drehung, Zustand“) oder als `Pool`; Bezug auf den Typ im Spielstand (volle ID gegen Katalog-Index; Pakete können sich ändern); Überlappung und Grenzen (nur Erdgeschoss in 3.1, Rand der Karte, Bau auf Leere/Gelände, Belegung); Drehung für `single`; Handle und Abfragbarkeit.
3. **Abriss.** `demolish { area }`: entfernt Objekte, deren Fußabdruck die Fläche berührt oder nur vollständig enthaltene? Was geschieht mit Zellen von Boden und Wand (zurück zu Leere)? Kein Geld in 3.1 (Wirtschaft erst 3.6): Erstattung BAU-04 und `notEnoughMoney` entfallen bis dahin – zu bestätigen.
4. **Ablehnungscodes** der Bau-Befehle (S-2 „fachliche Codes der Module“, additiv): z. B. `blocked` (Zelle belegt), `notBuildable` (außerhalb des Baubaren), `zoneNotAllowed`; der Objektname als `{ key }`-Parameter (L-5, jetzt vorhanden).
5. **Dauer der Wirkung.** In 3.1a wirkt jeder angenommene Bau sofort im Weltzustand; die Bauaufträge mit Fortschritt durch Bauarbeiter (FR-011, Planungsmodus BAU-02) folgen in 3.1c – zu bestätigen, dass 3.1a die Sofortwirkung ohne Auftrag liefert und 3.1c sie auf Aufträge umstellt.

Die Eingangsbedingung (Regel-007: berührte Bestandteile `[BELASTBAR]`, Datenmodell, S-2, S-4, S-5) ist erfüllt; die offenen Punkte sind Auslegung innerhalb der Verträge, deshalb kein Reifegrad-Rückschritt.

### 2026-10-25 17:36 (UTC) – [PROBLEM-GELÖST]

- **Dauerbau reproduziert und behoben (3.1a Teilaufgaben 1 und 5).** Gegen den unkorrigierten Code (nur mit Zählern, `a454f70`): 120 Bauaktionen alle 0,25 s, **0 Übernahmen in 210 s** (Zeitüberschreitung 180 s), bis zu **110 Neuberechnungen und 111 Schnappschuss-Kopien gleichzeitig**, 8 Feldsätze, **810 MB** Prozessspeicher. Ursache: Jedes `GridChanged` startete einen Auftrag, `Adoption` übernimmt nur die jüngste Revision, der Pool arbeitet in der Reihenfolge der Aufträge – die jüngste Anfrage steht immer hinter allen überholten, die zu Ende rechnen.
- **Korrektur (`67d6bfe`):** `Wayfinding::step` merkt Änderungen als `dirty` und startet höchstens eine Neuberechnung zugleich; die Folgeanfrage startet mit der Übernahme. `dirty` in Spielstand (Format 3) und Prüfsumme. Danach: **13 Übernahmen** beim Bauen, längster Abstand **2,4 s**, letzter Stand nach **3,6 s**, **1** Neuberechnung, **3** Feldsätze, **2** Schnappschuss-Kopien, **223 MB**, Schritt p99 1,1 ms.
- **Abweichung vom Entwurf:** Die Vorlage nennt als Ort der Korrektur auch `core/src/adoption.rs` („überholte Anfragen nicht weiterrechnen“). Eine überholte Anfrage entsteht jetzt gar nicht mehr, deshalb bleibt `Adoption` unverändert; seine Ersetzung gilt nur noch für ausdrückliche Anfragen (`Simulation::request_recompute`, Benchmarks und Tests).
- **Einschränkung:** CI-Runner sind langsamer (Neuberechnung 2,5 s statt 1,5 s): Frist 8 s für den Abstand sind dort rechnerisch 2 × 2,5 s = 5 s; Spielraum vorhanden, aber nicht gemessen.

### 2026-10-25 17:48 (UTC) – [BEOBACHTUNG]

- **Lücke in der eigenen Prüfung:** Die Änderung von `ParamValue` (`b19d25b`) ließ `no-base-to-string` in `app/renderer/platform.ts` anschlagen, weil ich ESLint nur auf die geänderten Dateien statt auf das ganze Projekt laufen ließ. Behoben in `9eb36c9`. Seither läuft vor jedem Commit `npx eslint . --max-warnings 0`, `tsc --noEmit` und die gesamte Vitest-Suite; der pre-commit-Hook ist eingerichtet.
- **Umgebung:** Container ohne `node_modules` und ohne `pre-commit`; beides nachinstalliert (Node 22.22.0 statt 26.11.1, nur für lokale Prüfungen; CI nutzt 26.11.1).

### 2026-10-25 17:25 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 17:19 UTC (Vorlage 3.1 freigegeben, ADR-040/041). Code für 3.1a nicht begonnen.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3 mit Schritt 3.1, Vorlage `docs/research/3.1-vorlage.md`, Architektur 1/2/9, Decisions Teil A und C, aktive Blocker: keine).
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` laut `get_session` 17:25 UTC → Routine-Klasse; passt zur Empfehlung für die Umsetzung von 3.1a (Entscheidungen gefallen, ADR-040/041). Kontingent: 5-Stunden-Fenster `allowed`.
- **Branch:** `feat/3-1a-bauen-mit-wirkung-dauerbau` auf `b1ef134` (Sessionende-Commits der Vorsession übernommen nach `project-context.md` Abschnitt 10, `main` = `f4341c6` ist Vorfahre).
- **Vorhaben:** 3.1a, Teilaufgabe 1 zuerst: Dauerbau-Benchmark gegen den heutigen Code (Reproduktion, ADR-041), dann Teilaufgaben 2–8 nach Vorlage Abschnitt 2.

### 2026-10-25 17:19 (UTC) – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung 17:06 bis 17:19 UTC nach dem `[SESSIONENDE]` 17:05 UTC (gesamt 15:58 bis 17:19 UTC). Zeiten der Einträge dieser Fortsetzung nach den Commit-Zeiten berichtigt (`[ENTSCHEIDUNG-VORGELEGT]` war mit 17:35 geschätzt).
- **Bearbeitet:** `[SESSIONENDE]` der 3.9-Session nachgetragen; Pflichtlektüre Architektur 1/2/9 und Decisions A/C nachgeholt; Vorlage 3.1 (`docs/research/3.1-vorlage.md`) vorgelegt und freigegeben.
- **Erreicht:** ADR-040 (3.1 in drei Stufen 3.1a/3.1b/3.1c) und ADR-041 (Dauerbau-Benchmark im Job „Kern-Benchmark“, Frist 8 s, gezählte Grenze), beide freigegeben vom Projektinhaber am 2026-10-25 („E1 A und E2 A freigeben“). Reaktiv-Quote 4 / 10 (ADR-032 bis ADR-041), weiter über der Schwelle; Reflexion nach Regel-009 gilt. Gate in `project-context.md` Abschnitt 7 eingetragen, README „Nächste Schritte“ nachgezogen.
- **Offen geblieben:** Umsetzung 3.1a; Code nicht begonnen.
- **Nächster Schritt:** 3.1a in neuer Session (Anweisung „weiter in neuer Session“), zuerst Dauerbau-Benchmark gegen den heutigen Code (Reproduktion, ADR-041), dann Teilaufgaben nach Vorlage Abschnitt 2. Empfohlene Klasse für die Umsetzung: Routine (die Entscheidungen sind gefallen); neuer Branch vom aktuellen `main` bzw. Übernahme der Sessionende-Commits dieses Branches nach `project-context.md` Abschnitt 10.
- **Aktive Klasse und Bilanz:** Teil 1 (bis 17:05) Routine (Sonnet 5.5), 0 Schritte oberhalb der Empfehlung; Teil 2 (ab 17:06) Entscheidung (Opus 5.5 nach `/model`), Vorlage 3.1 passend zur Empfehlung. Abgabe: Faktensammlung für die Vorlage an einen Unteragenten (Explore, gleiches Modell, kein günstigeres, da Probelauf offen), Ergebnis geprüft und in die Vorlage übernommen.
- **Kontextgröße:** 426.947 Token laut `get_session` – **über der Grenze von 400.000**; Empfehlung: nächste Arbeit in neuer Session beginnen (vom Projektinhaber bereits so angewiesen).
- **Ablaufdaten-Prüfung:** keine Änderung gegenüber 17:05 UTC.
- **Drift-Check:** ADR-040/041 in Teil A und B, Fahrplan 3.1 mit Stufen und ADR-Verweisen, `project-context.md` Gate-Liste, README; Phasenumfang unverändert (Stufen zählen als ein Schritt).

### 2026-10-25 17:18 (UTC) – [ADR-ANGELEGT]

- **ADR-040** Schritt 3.1 in drei Stufen (`[OPERATIV]`, Methodik) und **ADR-041** Dauerbau-Benchmark als Schritt im CI-Job „Kern-Benchmark“ (`[OPERATIV]`, Kategorie 7); beide nicht `[REAKTIV]`. Freigabe: „E1 A und E2 A freigeben, weiter in neuer Session“.

### 2026-10-25 17:12 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 3.1, Vorlage `docs/research/3.1-vorlage.md`.
- **E1:** Aufteilung in 3.1a (Kern: Bauen mit Wirkung, Dauerbau, Format 3, `catalog`), 3.1b (S-1 Layout 2, Spielfeld, Bau-Werkzeug, `game.new`, Messungen, Grafik-Abnahme), 3.1c (Bauarbeiter); Empfehlung A.
- **E2:** Dauerbau-Benchmark als Schritt im Job „Kern-Benchmark“ (Kategorie 7, Muster ADR-023), Frist 8 s zwischen Übernahmen (2 × Grenze der Neuberechnung), Grenze gezählt (1 laufende + 1 wartende Neuberechnung, ≤ 3 Feldsätze, ≤ 2 Schnappschüsse); Empfehlung A, Konfidenz mittel.
- **Faktenlage:** von einem Unteragenten (Explore, gleiches Modell) aus Fahrplan, ADR-036 bis ADR-039, Schnittstellen, Research und Code zusammengetragen; Befunde (u. a. Korrektur auch in `wayfinding/mod.rs`, abweichender Bezug der Speicherschwelle in 3.8) in Abschnitt 1 und 4 der Vorlage.
- **Klassifikation:** in der Phasenplanung vorgesehen (Fahrplan 3.1 nennt Kategorie 7) → nicht `[REAKTIV]`.

### 2026-10-25 17:06 (UTC) – [SESSIONSTART]

- **Anlass:** Fortsetzung derselben Claude-Code-Session nach dem `[SESSIONENDE]` 17:05 UTC; der Projektinhaber hat per `/model` gewechselt und angewiesen: „Modellwechsel durchgeführt, Sessionende nachholen, dann weiter mit 3.1“ (statt neuer Session).
- **Modell:** eingestellt laut `get_session` `claude-sonnet-5-5` (Sessionanlage), aktuell `claude-opus-5-5` (`user_switched_model`), bedient `claude-opus-5-5` → Entscheidungs-Klasse. Passt zur Empfehlung von 3.1.
- **Erledigt vorab:** `[SESSIONENDE]` der 3.9-Session nachgetragen (2026-10-22 01:18 UTC).
- **Pflichtlektüre:** project-context vollständig (diese Session), Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und 3.1, Architektur 1/2/9, Decisions Teil A und C (jetzt nachgeholt), aktive Blocker: keine.
- **Vorabprüfung 3.1:** Eingangskriterien erfüllt (Weltdarstellung `[BELASTBAR]` seit ADR-038, Kernstruktur 3.9 erledigt); Reaktiv-Quote 6 / 10 über der Schwelle, Reflexion nach Regel-009 gilt (Anlass ADR-038).
- **Kontext:** 370.649 Token bei Wiederanlauf (Grenze 400.000); Hinweis an den Projektinhaber gegeben.
- **Branch:** `claude/admiring-mayer-u95tfm` auf `main` (`f4341c6`) plus Sessionende-Commit `cd5d8ab`.

### 2026-10-25 17:05 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 15:58 bis 17:05 UTC (rund 1 Stunde 10 Minuten).
- **Bearbeitet:** Orientierung („was ist als nächstes dran?“), 3.10 (`main.ts` nach Zuständigkeiten geteilt), Q.4 (Audit-Advisories: `undici`, `fast-uri`, Electron 44.9.3), PR [#34](https://github.com/Paddel87/OpenAirside/pull/34) eröffnet und bei grüner CI gemergt (`f4341c6`).
- **Erreicht:** 3.10 und Q.4 `[ERLEDIGT]`; CI auf dem PR komplett grün (17 Jobs, CodeQL, beide wöchentlichen Audits), Rauchtest auf drei Plattformen. Phase 3: 3.7, 3.8, 3.14, 3.15, 3.9, 3.10 erledigt.
- **Offen geblieben:** Ein `[SESSIONENDE]` zur 3.9-Session (22:30 UTC am 2026-10-21) fehlte; am 2026-10-25 auf Anweisung nachgetragen (unten, 2026-10-22 01:18 UTC). Probeläufe für Mechanik- und Routine-Klasse (`project-context.md` Abschnitt 6) bleiben offen.
- **Nächster Schritt:** 3.1 Bauen im Raster, Bauaufträge sofort sichtbar, Dauerbau, in neuer Session auf Anweisung des Projektinhabers („Weiter mit 3.1 in neuer Session“). Empfohlene Klasse laut Fahrplan: Entscheidung (Bewertung der NFR „Bauaktion im nächsten Bild“, Frist und Grenze des Dauerbau-Benchmarks in der Vorlage vor dem Code, ggf. neues CI-Gate = Kategorie 7); Umsetzung ist Routine und abgebbar. Neuer Branch vom aktuellen `main` (`f4341c6`); der Branch dieser Session ist gemergt.
- **Aktive Klasse und Bilanz:** Routine-Klasse (Sonnet 5.5 laut `get_session`, bedient und eingestellt gleich). Schritte oberhalb der Empfehlung: 0 (3.10 und Q.4 sind Routine). Abgegebene Teilarbeiten: keine, Probelauf offen.
- **Kontextgröße:** 364.016 Token (Grenze 400.000, nicht überschritten); neue Session für 3.1 trotzdem sinnvoll, da 3.1 groß ist.
- **Ablaufdaten-Prüfung:** Electron 44 (Lebensende 2027-03-02, Vorlauf ab 2027-01-02) noch nicht erreicht, Q.1 offen mit Frist 2027-01-02; keine neuen Einträge fällig. Drift-Check: Fahrplan, Architektur (Modul Plattform), README, `project-context.md` abgeglichen; Archivierungs-Trigger nicht erreicht (Logbuch unter 800 Zeilen).
- **Beobachtung:** Der Container-npm (10.9.4) schreibt Lockfiles anders als das npm zu Node 26 (entfernt `libc`-Felder); Lockfile-Änderungen im Container deshalb gezielt vornehmen oder im Container Node 26 nutzen.

### 2026-10-25 16:45 (UTC) – [GELÖST]

- **CI grün:** Push-Lauf 40212258539 auf `7197730` (Q.4 und 3.10) erfolgreich, auch der Job „TS Dependency-Audit & Lizenzen“. Damit sind 3.10 und Q.4 `[ERLEDIGT]`.
- **Q.4:** Electron 44.9.3, `undici` 7.30.1/6.29.1, `fast-uri` 3.1.9; Lockfile gezielt angepasst, weil npm 10.9.4 die `libc`-Felder entfernt hätte (Container-npm älter als das npm zu Node 26). Spike-Ordner bleiben bei 44.7.0.

### 2026-10-25 16:30 (UTC) – [BEOBACHTUNG]

- **3.10 umgesetzt (`6d62de4`, `615e63e`):** `main.ts` von 1133 auf 522 Zeilen, kein Dateizugriff mehr; acht neue Module mit Tests (Details und Belege im Fahrplan, Schritt 3.10). Verhalten von S-6 und S-8 unverändert; Rauchtest lokal (Linux, Xvfb, Electron 44) grün, vorher Kern, Atlanten, Hauptprozess und Oberfläche frisch gebaut.
- **Reibung, gelöst:** Die ersten Service-Tests schlugen fehl, weil `save`, `load`, `delete`, `preview` und `setActive` bei ungültigem Argument synchron warfen statt abzulehnen; die ursprünglichen `async function` hatten das verdeckt. Die Methoden sind wieder `async`.
- **Fund, behoben:** Wird die Antwort-Promise von `packs.setActive` abgelehnt (Kern-Ende, Zeitgrenze), solange die Auswahl-Datei noch geschrieben wird, war sie ohne Handler (unbehandelte Ablehnung im Hauptprozess). Kleine Härtung in `packs-service.ts`; der Aufrufer sieht dasselbe Ergebnis.
- **Werkzeuge im Container:** `npm ci` unter Node 22.22.0 (Projekt verlangt ≥ 26.11.1) lief ohne Fehler für Lint, Typen, Tests, Build und Rauchtest; `check:types` und `check:schema` nicht ausgeführt (brauchen Node 26, nicht berührt). pre-commit installiert.
- **CI rot, nicht durch diesen Schritt:** Der Job „TS Dependency-Audit & Lizenzen“ ist seit dem ersten Push dieser Session rot (auch beim reinen Logbuch-Commit `2f76ec1`, Lauf 40209477968). `npm audit --audit-level=high` meldet `undici` (high, 2 Pfade: `node_modules/undici`, `node-gyp/node_modules/undici`), dazu `fast-uri` (moderate) und eine Electron-Meldung; auf `main` war der Lauf am 2026-10-21 grün, die Advisories sind neu. Lockfile unverändert durch diese Session. Behebung ist ein eigener Schritt (Vorschlag an den Projektinhaber).
- **Modell-Arbeit:** Umbau und Tests (Routine) liefen auf der Routine-Klasse (Sonnet 5.5), keine Abgabe, Probelauf offen.

### 2026-10-25 16:00 (UTC) – [SESSIONSTART]

- **Anlass:** Frage des Projektinhabers „Neue Session, was ist als nächstes dran?“; bisher nur Orientierung, keine Umsetzung.
- **Letzter Stand:** 3.9 erledigt (ADR-039, PR #33 gemergt, `ec5f349`). Ein `[SESSIONENDE]`-Eintrag zur 3.9-Session fehlt; letzter vorhandener ist 2026-10-21 01:17 UTC, danach nur `[BEOBACHTUNG]` 23:20 UTC.
- **Modell:** eingestellt `claude-sonnet-5-5`, bedient `claude-sonnet-5-5` laut `get_session` 16:00 UTC → Routine-Klasse. Passt zur Empfehlung von 3.10 (Routine).
- **Pflichtlektüre:** vollständig (project-context, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Schritt 3.10, Blocker: keine aktiven; Architektur und Decisions für die reine Orientierung nicht nötig).
- **Branch:** `claude/admiring-mayer-u95tfm` auf dem Stand von `main` (`ec5f349`), vom Werkzeug vergeben.

### 2026-10-22 01:18 (UTC) – [SESSIONENDE] (nachgetragen am 2026-10-25)

- **Hinweis:** Nachgetragen in der Session vom 2026-10-25 auf Anweisung des Projektinhabers („Sessionende nachholen“). Quellen: Commits `3431e91` bis `ec5f349` und der `[BEOBACHTUNG]`-Eintrag 2026-10-21 23:20 UTC. Was diese Quellen nicht hergeben, steht als „nicht überliefert“.
- **Session-Dauer:** 2026-10-21 22:30 UTC (Sessionstart) bis 23:10 UTC (letzter Commit `3790d2c`); Merge von PR #33 am 2026-10-22 01:18 UTC (`ec5f349`).
- **Bearbeitet:** 3.9 Kernstruktur der Fachmodule: Vorlage (`3431e91`), ADR-039 (`fd2e6e4`, Option B gegen Empfehlung A), Umbau (`383f545`), Dokumente (`3308f96`), Abschluss (`3790d2c`).
- **Erreicht:** 3.9 `[ERLEDIGT]` (ADR-039), Zustands-Prüfsumme vor und nach dem Umbau gleich (`f8580b54e3165422`), CI auf PR #33 grün, gemergt.
- **Offen geblieben:** nicht überliefert über den Fahrplan hinaus; nächster Schritt laut Fahrplan war 3.10.
- **Nächster Schritt:** 3.10 (erledigt am 2026-10-25).
- **Aktive Klasse und Bilanz:** Entscheidungs-Klasse (Opus 5.5 laut `[SESSIONSTART]`); Umbau (Routine) lief oberhalb seiner Empfehlung, keine Abgabe (laut `[BEOBACHTUNG]`). Zahl der Schritte oberhalb der Empfehlung: 1 Teilarbeit (Umbau in 3.9; der Schritt selbst ist Entscheidung).
- **Kontextgröße:** nicht überliefert (zum Zeitpunkt nicht erfasst).
- **Ablaufdaten-Prüfung:** nicht überliefert; bei der Prüfung am 2026-10-25 war kein Eintrag fällig.

### 2026-10-21 23:20 (UTC) – [BEOBACHTUNG]

- **3.9 umgesetzt:** Kern nach ADR-039 umgebaut (`383f545`), Dokumente nachgezogen (`3308f96`); PR #33 nach Anweisung „Mach daraus ein PR und merge ihn bei grün“ eröffnet.
- **Prüfsumme wertgleich:** Basislauf vor und nach dem Umbau `f8580b54e3165422` (Tick 590), alle Zwischenwerte gleich; der temporäre Test wurde nicht committet.
- **Einziger angepasster Testwert:** `savegame_roundtrip.rs` nahm Format 2 als „zu neu“ an; jetzt 3 als zu neu, 0 und 1 als `migrationFailed`.
- **Kleine Abweichung im Detail:** Zwei `setWalls` in derselben Befehlsgruppe berechneten früher die erste Anfrage mit dem Zwischenstand; jetzt beide in der Stufe Wegfindung mit dem Endstand. Revisionen, übernommenes Ergebnis und Spielstand sind gleich, weil eine neuere Anfrage die ältere ersetzt.
- **Zeitstempel berichtigt:** `[ENTSCHEIDUNG-VORGELEGT]` und `[ADR-ANGELEGT]` trugen geschätzte Zeiten (23:10, 23:25); nach den Commit-Zeiten 22:35 und 22:42.
- **Werkzeuge im Container:** pre-commit installiert; Node 26.11.1 mit Prüfsumme nach `~/node26` für `check:types`; cargo-llvm-cov 0.9.1.
- **Modell-Arbeit:** Umbau (Routine) lief auf der Entscheidungs-Klasse; keine Abgabe, Probelauf offen.

### 2026-10-21 22:42 (UTC) – [ADR-ANGELEGT]

- **ADR-039** Kernstruktur der Fachmodule mit interner Ereignis-Warteschlange, `[STRATEGISCH]`, nicht reaktiv. Freigabe des Projektinhabers per Auswahl „B freigeben“ (gegen die Empfehlung A; keine Begründung genannt). Reaktiv-Quote 7 / 10 → 6 / 10 (ADR-030 bis ADR-039).
- **Drift behoben:** In `decisions.md` Teil B fehlte der Listeneintrag zu ADR-038; nachgetragen.
- **Basis vor dem Umbau:** temporärer Test mit Referenz-Terminal 256², 3.000 Personen, Prüfmodus (Übernahme nach 20 Ticks), `setWalls` bei Schritt 50 und 300, Pause 120–130 mit `spawnPersons` 500: Prüfsumme nach 600 Schritten `f8580b54e3165422` (Tick 590), Zwischenwerte je 100 Schritte in der Session festgehalten. Muss nach dem Umbau gleich sein.

### 2026-10-21 22:35 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt:** 3.9 Kernstruktur der Fachmodule (Kategorie 1 und 4; Kategorie 5 nur für Rust-Pfade im Kern-Crate).
- **Vorlage:** `docs/research/3.9-kernstruktur.md`. Optionen: A Fachmodule `build/`, `wayfinding/`, `persons/` mit eigenem Zustand, Spielstand-Teil, Prüfung und Prüfsumme, feste Reihenfolge Welt & Bau → Wegfindung → Personen, Wirkungen als Rückgabewerte, Baustein `Pool<T>` für Objektarten (Empfehlung); B wie A mit interner Ereignis-Warteschlange; C nur Ordnerablage.
- **Vertragsabgleich S-5 (Maßnahmen 7–9):** keine Abweichung vom Vertrag; Format-Version 1 → 2 nach der Versionierungsregel von S-5, Prüfsumme bleibt wertgleich.
- **Klassifikation:** geplanter Entwurf aus ADR-036, ändert keinen ADR jünger als 14 Tagen (der Satz zu internen Ereignissen stammt aus der Initialisierung von `architecture.md`, nicht aus einem ADR) → nicht `[REAKTIV]`.

### 2026-10-21 22:30 (UTC) – [SESSIONSTART]

- **Anlass:** Schritt 3.9 Kernstruktur der Fachmodule, in neuer Session nach Empfehlung des letzten `[SESSIONENDE]` (Kontext 379.648 Token) und Anweisung des Projektinhabers („Weiter mit 3.9 in neuer Session“).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 22:30 UTC → Entscheidungs-Klasse. Passt zur Empfehlung von 3.9 (Entwurf: Entscheidung, Eskalations-Auslöser 1).
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session`.
- **Pflichtlektüre:** vollständig (project-context, Logbuch ab `[SESSIONENDE]` 2026-10-21 01:17 UTC von `origin/docs/grafik-abnahme`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions A/C, Blocker, Requirements-Übersicht).
- **Branch:** `refactor/3-9-kernstruktur-fachmodule` vom aktuellen `main` (`89aa217`); Sessionende-Commit `7fcc3bb` übernommen (`85f809a`) nach `project-context.md` Abschnitt 10.

### 2026-10-21 01:17 (UTC) – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung nach dem `[SESSIONENDE]` 2026-10-20 23:15 UTC, rund 2 Stunden bis 01:17 UTC (mit Pausen).
- **Bearbeitet:** Fragen des Projektinhabers zu 3.9 und zu den Grafiken; Abnahme der Spielgrafiken als Regel (`docs/stilleitfaden.md` Abschnitt 7) mit Akzeptanzkriterium in 3.1, 3.2, 3.4, 3.5; zehn Probeentwürfe mit Übersichtsseite (privates Artifact); Entwurf der Lackierungen in 3.6 eingetragen; PR #32 nach Anweisung („Mach daraus ein PR und merge ihn“) eröffnet und gemergt, Merge-Commit `89aa217`, 39 Prüfungen grün auf `308aa64`.
- **Erreicht:** Ablauf für Grafik-Entwürfe festgelegt; Lackierungen haben einen Landeplatz (3.6).
- **Offen geblieben:** Wahl des Projektinhabers zu den Probeentwürfen (spätestens vor 3.1); Ausrichtung von Fahrzeugen und Flugzeugen bei Drehung 0 (3.4). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `docs/grafik-abnahme` und geht mit dem PR des nächsten Schritts nach `main`.
- **Nächster Schritt:** 3.9 Kernstruktur der Fachmodule, Entwurf als `ENTSCHEIDUNG ERFORDERLICH` vor dem Code (Entscheidungs-Klasse).
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 01:16 UTC). Seit dem letzten Sessionende kein Fahrplan-Schritt bearbeitet, nur Dokumentationspflege (Routine); abgegeben: keine Teilarbeiten.
- **Kontextgröße:** 379.648 Token laut `get_session` 01:16 UTC, unter der Grenze von 400.000, aber nahe daran. Empfehlung: 3.9 in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 01:16 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt.
- **Archivierung und Größen:** keine Trigger; Logbuch rund 490 Zeilen (Grenze 800).
- **Drift-Prüfung:** README unverändert gültig (Phase, Reife, Nächste Schritte); keine ADRs, Anforderungen oder Blocker geändert; Modulnamen in 3.6 (Spielfeld ergänzt) stimmen mit der Modul-Karte; Phase 3 bei 15 Schritten.

### 2026-10-21 (UTC) – [BEOBACHTUNG]

- **Probeentwürfe:** Rückmeldung des Projektinhabers „vielversprechend“, noch keine Wahl.
- **Frage Lackierungen und Logos:** beantwortet. Detail je Stufe (Flugzeug 608 × 576 px bei 32 px, 152 × 144 px bei 8 px); von oben sichtbar sind Rumpfdach, Leitwerk als Strich, Triebwerke, Flügelspitzen; reale Airline-Logos sind Marken. Vorschlag Grundgrafik mit Farbflächen, Logo auf dem Dach, Einfärben im Spielfeld. Auf Antwort „Ja, trag das bei 3.6 ein“ als Entwurfspunkt mit Akzeptanzkriterium und Freigabepflicht (Kategorie 1 und 4) in 3.6 eingetragen; Spielfeld zu den betroffenen Modulen ergänzt. Kein neuer Schritt, Phase 3 bleibt bei 15.

### 2026-10-21 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Anlass:** Frage des Projektinhabers, ob man sich Entwürfe von Bauelementen und Verkehrsmitteln zeigen lassen kann. Befund: Eine Abnahme der Spielgrafiken durch den Projektinhaber war nirgends vorgesehen; die bisherigen Grafiken (Bank, Boden, Wand, Ersatzgrafik, Symbol) waren Testmaterial der Pipeline aus 2.10.
- **Vorgeschlagen und angenommen** („Ja, nimm das so auf“): Übersichtsseite mit zwei bis drei Varianten je Objekt in den drei Atlas-Stufen vor dem Commit; Regel in `docs/stilleitfaden.md` Abschnitt 7; Akzeptanzkriterium in 3.1, 3.2, 3.4, 3.5. Keine Kategorie aus `CLAUDE.md` Abschnitt 4, Dokumentationspflege.
- **Probeentwürfe:** Wand, Boden, Check-in-Schalter, Bus und Flugzeug in je zwei Varianten (`docs/research/grafik-probeentwuerfe/`), alle bestehen `checkSvg`. Übersichtsseite als privates Artifact. Wahl des Projektinhabers steht aus.
- **Offen:** Ausrichtung von Fahrzeugen und Flugzeugen bei Drehung 0 (Entwürfe: Front nach rechts); Festlegung mit 3.4.
- **Branch:** `docs/grafik-abnahme` vom aktuellen `main` (`421781b`), Sessionende-Vermerk `c29a0fd` übernommen (`c3a2ce0`) nach `project-context.md` Abschnitt 10.

### 2026-10-20 23:15 (UTC) – [SESSIONENDE]

- **Session-Dauer:** rund 50 Minuten seit `[SESSIONSTART]` 22:24 UTC, davon etwa 10 Minuten Messlauf des Projektinhabers.
- **Bearbeitet:**
  - Schritt 3.15: Code, Unit-Tests, Regel in Code und Architektur.
  - Zähler später Antworten im Messwerkzeug, nach Vorlage und Antwort „Zähler ergänzen“.
  - Release-Lauf 39508715951 (der erste, 39508278718, wurde abgebrochen) und Messung auf dem Referenz-PC.
  - Beförderung nach Vorlage und Antwort „Befördern“.
  - PR #31 nach Anweisung („PR eröffnen und bei grün mergen“) eröffnet und gemergt, Merge-Commit `421781b`, 39 Prüfungen grün auf `1826398`.
- **Erreicht:**
  - 3.15 erledigt ohne ADR: 0 von 21.668 Antworten spät, Bildabstand p99 17,3–17,5 ms.
  - „NFR Performance: dasselbe unter Windows“ ist `[BELASTBAR]`, Auflage Nachmessung in 3.1.
- **Offen geblieben:**
  - Warum der GPU-Prozess unter D3D11 zeitweise einen Takt im Flush wartet, bleibt ungeklärt. Ohne synchrone Abfragen ist das nicht sichtbar.
  - Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `fix/3-15-gpu-name-einmal-lesen` und geht mit dem PR des nächsten Schritts nach `main`.
- **Nächster Schritt:** 3.9 Kernstruktur der Fachmodule, Entwurf als `ENTSCHEIDUNG ERFORDERLICH` vor dem Code (Entscheidungs-Klasse).
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 23:14 UTC). Ein Schritt oberhalb seiner Empfehlung (3.15: Routine; die Beförderung selbst ist Entscheidung, Auslöser 4). Abgegeben: keine Teilarbeiten, kein bestandener Probelauf.
- **Kontextgröße:** 291.608 Token laut `get_session` 23:14 UTC, unter der Grenze von 400.000.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 23:14 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt (Electron 44: Vorlauf ab 2027-01-02).
- **Archivierung und Größen:** keine Trigger. Logbuch rund 460 Zeilen (Grenze 800), `project-context.md` 375 Zeilen (Budget 600).
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Architektur-Reife, Nächste Schritte).
  - Reifegrad in `architecture.md` Abschnitt 9 passt zum Ergebnis 3.15.
  - Kein neuer ADR; Reaktiv-Quote unverändert 7 / 10.
  - Modulnamen in 3.15 (Spielfeld, Benchmark & Leistung) stimmen mit der Modul-Karte überein.
  - Keine Anforderung neu oder geändert; keine Blocker, kein `[BLOCKIERT]`-Schritt.
  - Phase 3: 15 Schritte bei Plan 13, unter der Wucherungs-Schwelle von 26. Der Zähler in 3.15 ist kein eigener Schritt.
- **Mini-Reibungen:**
  - Prettier formatierte die Messdatei beim Commit um. Inhalt per JSON-Vergleich gleich; der erste Commit-Versuch schlug wie in 3.14 fehl.
  - Ein Heredoc im selben Befehl wie eine Pipe brach den Commit ab; Commit-Text danach aus einer Datei.
- **Beobachtung:** Beim Anlegen eines Schritts mit Messkriterium prüfen, ob das Werkzeug auf `main` die Größe überhaupt misst. Das Kriterium aus 3.15 stammte aus dem Spike-Prototyp.

### 2026-10-20 23:05 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteil:** NFR Performance: Weltdarstellung mit Atlas-Stufen 16/32 px und großen Atlanten unter Windows („dasselbe unter Windows“), `[VORLÄUFIG]` → `[BELASTBAR]`.
- **Grundlage:** Welt-Nachmessung am Prototyp mit der Korrektur (3.14, `9d01469`) und Messung auf dem Stand für `main` (3.15, `41786ec`, ohne Welt). Vorgelegt mit der Einschränkung, dass `main` noch keine Weltdarstellung hat; Antwort des Projektinhabers: „Befördern“. Gleicher Maßstab wie macOS in ADR-038.
- **Auflage:** 3.1 misst die Weltdarstellung auf `main` auf dem Referenz-PC gegen die Schwellen aus 3.15 (Akzeptanzkriterium dort ergänzt); sonst zurück auf `[VORLÄUFIG]`.
- **Modellklasse:** Entscheidungs-Klasse (`claude-opus-5-5`, Auslöser 4 erfüllt). Kein ADR: keine Kategorie aus `CLAUDE.md` Abschnitt 4, Beförderung durch Messung belegt (`CLAUDE.md` Abschnitt 6, Validierung).

### 2026-10-20 23:04 (UTC) – [BEOBACHTUNG]

- **Messung 3.15 auf dem Referenz-PC:** `measure 6`, Paket aus Release-Lauf 39508715951 (`41786ec`), fertig 22:57 UTC. 0 von 21.668 Antworten spät, Bildabstand p99 17,3–17,5 ms, 59,9–60,0 Bilder/s, Rundlauf p99 0,7–1,0 ms, keine langen Aufgaben; Speicher ab 40 s 507–540 MB. Alle Schwellen erfüllt. Datei `docs/research/3.15-ergebnisse/referenz-pc-measure.json`, Notiz `docs/research/3.15-messung.md`.
- **CI:** Lauf 39508710449 auf `41786ec` grün (17 Jobs). Der Release-Lauf brauchte dank Cache nur 4 Minuten; Jobs und Artefakte geprüft, Rauchtest des Pakets auf drei Plattformen grün.
- **Frage des Projektinhabers „beide Betriebssysteme?“:** nur Windows; macOS ist seit ADR-038 belastbar, die Änderung entfernt nur Arbeit.

### 2026-10-20 22:31 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Fund:** Das Messwerkzeug auf `main` zählt keine späten Antworten; die Zählung (`late`, `latency`) gab es nur im Spike `erkundung/3-14-messaufbau`. Das Akzeptanzkriterium „unter 0,1 % späte Antworten“ aus 3.15 war damit nicht prüfbar (Rundlauf p99 ≤ 3 ms belegt nur „unter 1 %“). Beim Anlegen von 3.15 übersehen.
- **Vorgelegt:** Zähler ergänzen (empfohlen) oder Kriterium auf die vorhandenen Werte abschwächen. **Antwort:** „Zähler ergänzen“.
- **Umgesetzt:** `dea2e1f`, je Fenster `answers` und `rttLate` (Rundlauf länger als Bildabstand p50), additiv im Messbericht; zwei Unit-Tests. Der Release-Lauf 39508278718 auf `7ef3673` wurde abgebrochen, weil sein Paket den Zähler nicht hätte.
- **Weitere synchrone WebGL-Abfragen:** im eigenen Code keine außer dem GPU-Namen (Suche nach `getParameter`, `getExtension`, `getError`, `readPixels`, `getContextAttributes`, `toDataURL`, `extract` in `app/`). PixiJS 8.20.1 fragt nur beim Anlegen von Kontext, Shader-Programmen und Texturen (`GlContextSystem`, `generateProgram`, `applyStyleParams`); `readPixels` nur über `extract`, vom Spielfeld nicht genutzt.

### 2026-10-20 22:24 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-20 21:40 UTC (Session zu 3.14, Kontext 408.994 Token über der Grenze von 400.000; neue Session vom Projektinhaber gewählt: „Weiter mit 3.15 in neuer Session“). Merge-Vermerk `33d8a19` nach `project-context.md` Abschnitt 10 von `erkundung/3-14-spaete-bilder-referenz-pc` auf den Branch dieser Session `fix/3-15-gpu-name-einmal-lesen` vom aktuellen `main` (`fddad76`) übernommen (`1e68f1b`); der alte Branch wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant:** Schritt 3.15 (UMSETZUNG, Fehlerbehebung im Spielfeld): Getter `gpu` liest einmal, Suche nach weiteren synchronen WebGL-Abfragen im laufenden Betrieb, Regel als Kommentar und in `architecture.md`, Unit-Test; Paket aus dem Release-Workflow für die Messung `measure 6` auf dem Referenz-PC.
- **Vorabprüfung:** keine Grundsatzentscheidung berührt; Spielfeld ist `[BELASTBAR]` im umgesetzten Umfang; NFR Darstellung unter Windows `[VORLÄUFIG]`, Beförderung nach Messung (Auslöser 4, Entscheidungs-Klasse). Reaktiv-Quote 7 / 10, Maßnahmen (5) bis (11) gelten.
- **Modus/Werkzeug:** Claude Code in der Cloud (Container), Rechte-Modus „auto“; Pre-Commit-Hook installiert (`pre-commit` per pip, `.git/hooks/pre-commit` vorhanden). Node 26.11.1 nach `~/node26` geholt, Prüfsumme gegen `SHASUMS256.txt` bestätigt.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: `get_session` 22:23 UTC) – Entscheidungs-Klasse. 3.15 empfiehlt Routine: der Schritt läuft oberhalb seiner Empfehlung; keine Abgabe, weil kein Probelauf bestanden ist.

### 2026-10-20 21:46 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #30 (Schritt 3.14) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `fddad76`. Vorher grün auf `e908c87`: 39 Prüfungen ohne Fehler. Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `erkundung/3-14-spaete-bilder-referenz-pc` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-20 21:40 (UTC) – [SESSIONENDE]

- **Session-Dauer:** rund 4 Stunden seit `[SESSIONSTART]` 17:37 UTC, davon gut 3 Stunden Messläufe des Projektinhabers.
- **Bearbeitet:**
  - Schritt 3.14: Vorlage vor der Messung mit zwei Antworten (5 Läufe, Rückgabe im Chat).
  - Messprototyp `erkundung/3-14-messaufbau` (`6808a35`, `9d01469`), geprüft nach Maßnahme (11); Release-Läufe 39471593590 und 39483411097.
  - Lauf 2 und drei Zeitspuren ausgewertet: Ursache gefunden. Nachmessung beider Läufe mit der Korrektur.
- **Erreicht:**
  - 3.14 erledigt ohne ADR. Ursache: synchrone Abfrage des GPU-Namens alle 250 ms in der Leistungsanzeige.
  - Mit der Korrektur 0–0,01 % späte Antworten und Bildabstand p99 17,3–18,1 ms.
  - Neuer Schritt 3.15 (Korrektur auf `main`) nach Antwort des Projektinhabers.
  - Reifegrad „NFR Darstellung unter Windows“ bleibt `[VORLÄUFIG]` bis 3.15.
- **Offen geblieben:**
  - PR für `erkundung/3-14-spaete-bilder-referenz-pc` nach `main`, nur auf Anweisung.
  - Der Spike-Branch `erkundung/3-14-messaufbau` bleibt als Aufzeichnung und wird nicht gemergt.
  - Ungeklärt bleibt, warum der GPU-Prozess unter D3D11 zeitweise einen Takt im Flush wartet. Ohne synchrone Abfragen ist das nicht sichtbar.
  - AMD-Treiberversion und Update-Verlauf hat der Projektinhaber nicht nachgereicht; für das Ergebnis nicht mehr nötig.
- **Nächster Schritt:** 3.15 GPU-Namen nur einmal lesen, in einer neuen Session; danach 3.9.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 21:37 UTC). 0 Schritte oberhalb der Empfehlung (3.14: Entscheidung). Abgegeben: keine Teilarbeiten.
- **Kontextgröße:** 408.994 Token laut `get_session` 21:37 UTC, **über der Grenze von 400.000**. Empfehlung: die nächste Arbeit in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 21:37 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt.
- **Archivierung und Größen:** keine Trigger. Logbuch rund 400 Zeilen (Grenze 800), `project-context.md` 375 Zeilen (Budget 600), `architecture.md` 470 Zeilen.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Architektur-Reife, Nächste Schritte); „Aktueller Stand“ im Logbuch berichtigt (nannte noch 3.7).
  - Kein neuer ADR; Reaktiv-Quote unverändert 7 / 10.
  - Reifegrad passt zum Ergebnis 3.14.
  - Modulnamen in 3.14 und 3.15 stimmen mit der Modul-Karte überein.
  - Keine Anforderung neu oder geändert; keine Blocker.
  - Phase 3: 15 Schritte bei Plan 13, unter der Wucherungs-Schwelle von 26.
- **Mini-Reibungen:**
  - Prettier formatiert die Messdateien beim Commit um. Der Inhalt wurde jeweils per Vergleich als gleich belegt; der erste Commit-Versuch schlägt dadurch jedes Mal fehl.
- **Beobachtung:** Die Selbst-Nachricht und die Zeitspur hätten die Ursache schon in 3.8 sichtbar gemacht. Bei späten Bildern zuerst den Hauptthread in der Zeitspur prüfen, bevor man Grafikwege vergleicht.

### 2026-10-20 21:37 (UTC) – [BEOBACHTUNG]

- **3.14, Nachmessung Lauf 2 (ohne Welt, Paket `9d01469`):** 0 späte von 21.691 Antworten, Rundlauf p99 0,7–1,1 ms, Bildabstand p99 17,3–18,1 ms, 59,8–60,0 Bilder/s. Zusammen mit Lauf 1 ist die Ursache belegt, alle Schwellen sind erfüllt. Die Läufe 3–5 entfallen. Ergebnis: `docs/research/3.14-spaete-bilder.md` Abschnitt 14.
- **Mini-Reibung:** Der erste Versuch von Lauf 2 (21:28 UTC) lieferte keine Messdatei, vermutlich zu früh nachgesehen. Der Lauf wurde neu gestartet.
- **Kontextgröße:** 408.994 Token laut `get_session` 21:37 UTC, über der Grenze von 400.000.

### 2026-10-20 21:25 (UTC) – [BEOBACHTUNG]

- **3.14, Nachmessung Lauf 1 (mit Welt, Paket `9d01469`):** 2 späte von 21.636 Antworten (0,01 %), Rundlauf p99 0,8–1,1 ms, Bildabstand p99 17,4–18,1 ms, 59,8–60,0 Bilder/s, keine langen Aufgaben. Alle Schwellen der Vorlage sind erfüllt; der Projektinhaber sah es flüssig. Die Ursache aus Abschnitt 12 (synchrone Abfrage des GPU-Namens alle 250 ms) ist damit für die Weltdarstellung belegt. Lauf 2 (ohne Welt) folgt. `docs/research/3.14-spaete-bilder.md` Abschnitt 13.

### 2026-10-20 19:26 (UTC) – [PROBLEM-GELÖST]

- **3.14, Ursache der späten Bilder (vorbehaltlich Nachmessung):** Lauf 2 (ohne Welt) und die Zeitspuren aus Lauf 1 und 2 zeigen bei jeder späten Antwort denselben Ablauf. Der Hauptthread des Renderers wartet in einem Zeitgeber auf `GLES2::GetString` → `CommandBufferHelper::Finish`, also auf den GPU-Prozess. Dessen WebGL-Flush dauert in diesen Phasen rund einen Bildtakt. Die späten Antworten folgen im 250-ms-Raster. Auslöser ist die Leistungsanzeige: Sie liest alle 250 ms `playfield.gpu`, und der Getter ruft `getParameter(UNMASKED_RENDERER_WEBGL)` synchron auf. Dieser Getter steht auch auf `main`. Kern-Prozess (H3) und Zeichnen (PixiJS-Ticker p99 unter 1 ms) sind ausgeschlossen. Einzelheiten: `docs/research/3.14-spaete-bilder.md` Abschnitt 12, Messdatei `docs/research/3.14-ergebnisse/`.
- **Nachmessung:** Spike-Commit `9d01469` liest den Namen nur einmal; Probelauf im Container mit drei Sitzungswechseln fehlerfrei; Release-Lauf gestartet.

### 2026-10-20 19:17 (UTC) – [BEOBACHTUNG]

- **Sichtbeobachtung des Projektinhabers, 3.14 Lauf 2 (ohne Weltdarstellung, `measure 6`):** „wieder das gleiche Ruckeln“, angezeigtes Fenster 10 (Index 9, Kreisen). In 2.10 trat auf demselben Rechner ohne Welt kein Ruckeln auf. Hinweis darauf, dass die Ursache nicht an der Weltdarstellung hängt, sondern am heutigen Stand von Rechner, Treiber oder Paket. Bestätigung steht mit der Messdatei aus.

### 2026-10-20 18:05 (UTC) – [BEOBACHTUNG]

- **Messprototyp 3.14:** `erkundung/3-14-messaufbau` (`6808a35`, von `f23e154`), Release-Lauf 39471593590 per „Run workflow“ gestartet. Prüfung nach Maßnahme (11) im Container bestanden: drei Sitzungswechsel in einem Programmstart, Zeitstempel beider Seiten in allen drei Messdateien, vier Zeitspuren zu 1,9–3,0 MB, Auswertung mit `analyze.py` gelesen. Einzelheiten und Messanleitung: `docs/research/3.14-spaete-bilder.md` Abschnitte 10 und 11.
- **Mini-Reibungen:** (1) Unter Linux gibt es im Container keinen Ordner „Downloads“; Electron legt die Dateien dann im Heimatordner ab, das `ls` im Probelauf schlug fehl (nur Prüfbefehl). (2) Mit `--use-angle=gl` fällt PixiJS im Container ohne Meldung auf Canvas zurück und die Weltdarstellung wirft einen Fehler; im Prototyp nun Abbruch mit Meldung. (3) Die Liste der späten Antworten ist auf 400 begrenzt; die erste Auswertung rechnete den Anteil aus der Liste statt aus den Fenstern (behoben). (4) `scripts/assets.test.mjs` schlägt auf dem Spike-Branch fehl (512 Testgrafiken aus 3.8 ohne Objekt), wie schon in 3.8; ohne Bedeutung für die Messung.

### 2026-10-20 17:44 (UTC) – [BEOBACHTUNG]

- **Antworten zur Vorlage 3.14:** Frage 1 A (fünf Läufe), Frage 2 A (Dateien im Chat). Festgehalten in `docs/research/3.14-spaete-bilder.md` Abschnitt 9. Weiter mit dem Messprototyp auf `erkundung/3-14-messaufbau`.

### 2026-10-20 17:41 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.14, Vorlage vor der Messung:** `docs/research/3.14-spaete-bilder.md`.
- **Neuer Befund aus den Dateien von 3.8:** Die späten Antworten liegen in der Übersicht bei genau einem oder zwei Bildtakten (16,8–18,1 bzw. 32,4–33,9 ms), beim Kreisen 2,3 ms darunter (14,0–14,4 bzw. 30,4–31,2 ms), weil die Anforderung dort später nach dem Takt hinausgeht. Die Antwort wartet also auf den Takt des Renderers; das spricht gegen den Kern-Prozess (H3) und gegen das Zeitgeber-Raster von Windows (15,6 ms) und für Renderer bzw. Grafikweg (H1, H2, H4).
- **Messaufbau:** Spike-Branch `erkundung/3-14-messaufbau` vom Stand `f23e154`; Zeitstempel je Bild auf beiden Seiten, Selbst-Nachricht im Renderer, Zeitspur über Electron `contentTracing`, Umschalter `--use-angle` nur im Prototyp; keine neue Abhängigkeit, keine Änderung an `main`.
- **Vorgelegt:** zwei Fragen (Umfang der Messung: 5 Läufe empfohlen; Rückgabe der Dateien).

### 2026-10-20 17:37 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-20 17:25 UTC (Session zu 3.8, Kontext 605.223 Token über der Grenze von 400.000; neue Session vom Projektinhaber gewählt: „Weiter mit 3.14 in neuer Session“). Merge-Vermerk `7cd0d3e` nach `project-context.md` Abschnitt 10 von `erkundung/3-8-weltdarstellung-spielfeld` auf den Branch dieser Session `erkundung/3-14-spaete-bilder-referenz-pc` vom aktuellen `main` (`4b8a679`) übernommen (`0ed265d`); der alte Branch wird nicht weiterverwendet. Branch und Übernahme liefen vor diesem Eintrag, damit er auf dem Arbeits-Branch steht.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Geplant:** Schritt 3.14 (ERKUNDUNG, Messung): Vorlage vor der Messung, Messaufbau mit Zeitstempeln auf beiden Seiten, Prüfung nach Maßnahme (11) im Container, Pakete an den Projektinhaber.
- **Vorabprüfung:** 3.14 hat keine Eingangskriterien außer dem Referenz-PC; NFR Darstellung unter Windows `[VORLÄUFIG]` ist der Erkundungsgrund. Reaktiv-Quote 7 / 10 über dem Schwellenwert, Maßnahmen (5) bis (11) gelten.
- **Modus/Werkzeug:** Claude Code in der Cloud (Container), Rechte-Modus „auto“; Pre-Commit-Hook installiert (`pre-commit` 4.x per pip, `.git/hooks/pre-commit` vorhanden). Node im Container 22.22.0, Node 26.11.1 bei TypeScript-Arbeit nachzuholen.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: `get_session` 17:36 UTC) – Entscheidungs-Klasse; 3.14 empfiehlt Entscheidung.

### 2026-10-20 17:34 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #29 (Schritt 3.8, ADR-038) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `4b8a679`. Vorher grün auf `ac00324`: Push-Lauf 39466020245 und die PR-Läufe CI 39466633289 und Security 39466633371 (39 Prüfungen). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `erkundung/3-8-weltdarstellung-spielfeld` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-20 17:25 (UTC) – [SESSIONENDE]

- **Session-Dauer:** rund 2 Stunden 35 Minuten seit `[SESSIONSTART]` 14:53 UTC.
- **Bearbeitet:**
  - Schritt 3.8: Vorlage vor der Messung mit drei Antworten; Prototyp auf `erkundung/3-8-messaufbau`; zwei Release-Läufe (39449159828, 39459607051).
  - Fehler im W-B-Prototyp gefunden und behoben.
  - Sechs Messläufe des Projektinhabers ausgewertet; Vorlage zur Entscheidung; ADR-038 mit Reflexion nach Regel-009.
- **Erreicht:**
  - 3.8 erledigt; Weltdarstellung `[BELASTBAR]`.
  - S-1 Layout 2 und `protocol` 2 entschieden; erste Ergänzung nach Regel-008 (S-7-Katalog-Index).
  - Neuer Schritt 3.14; Maßnahme (11) gilt.
- **Offen geblieben:**
  - PR für `erkundung/3-8-weltdarstellung-spielfeld` nach `main`, nur auf Anweisung. Der Branch enthält auch die übernommenen Sessionende-Commits der Session 3.7.
  - Der Spike-Branch `erkundung/3-8-messaufbau` bleibt als Aufzeichnung bestehen und wird nicht gemergt.
- **Nächster Schritt:** 3.14 Späte Bilder auf dem Referenz-PC (Messung mit Zeitstempeln, Projektinhaber), in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 17:25 UTC). 0 Schritte oberhalb der Empfehlung (3.8: Entscheidung). Abgegeben: keine Teilarbeiten; Probelauf für Routine und Mechanik bleibt offen.
- **Kontextgröße:** 605.223 Token laut `get_session` 17:25 UTC, **über der Grenze von 400.000**. Empfehlung: die nächste Arbeit in einer neuen Session beginnen.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 17:25 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung und Größen:** keine Trigger. Logbuch rund 320 Zeilen (Grenze 800), `project-context.md` 375 Zeilen (Budget 600), `architecture.md` 470 Zeilen (eigene Schwelle 500). Phase 3 läuft.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Architektur-Reife, Nächste Schritte).
  - ADR-038 verweist auf vorhandene Schritte (3.1, 3.2, 3.4, 3.5, 3.14, 4.1, 5.1).
  - Reifegrade passen zu ADR-038.
  - Modulnamen in 3.8 und 3.14 stimmen mit der Modul-Karte überein.
  - Keine Anforderung neu oder geändert (FR-010 vorbereitend).
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 7 / 10 (ADR-029 bis ADR-038) stimmt mit den Tags, gesamt 16 / 38.
  - Phase 3: 14 Schritte bei Plan 13, unter der Wucherungs-Schwelle.
- **Mini-Reibungen:**
  - Zwei Zeitstempel geschätzt statt mit `date -u` geholt (17:35 statt 17:25), vor dem Commit korrigiert. Wiederkehrend, siehe Reflexion Phase 2.
  - Prettier formatiert die Messdateien in `docs/research/` beim Commit um; Inhalt jeweils per Vergleich als gleich belegt.
- **Beobachtung:** Die Sichtbeobachtungen des Projektinhabers (Ruckeln in der Nahansicht, Figuren über Möbeln, Fenster 12) deckten sich mit den Messwerten bzw. mit Grenzen des Prototyps und halfen beim Zuordnen der späten Bilder.

### 2026-10-20 17:25 (UTC) – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Weltdarstellung im Spielfeld. **Wechsel:** `[OFFEN]` → `[BELASTBAR]`. **Auslöser:** ADR-038 (Messung 3.8). Neu `[VORLÄUFIG]`: Speicher der Weltdarstellung (3.1). NFR Darstellung geteilt: macOS mit Weltdarstellung, Stufen 16/32 und 512 Grafiken `[BELASTBAR]`, Windows `[VORLÄUFIG]` (3.14). Eingetragen in `architecture.md` Abschnitt 9.

### 2026-10-20 17:25 (UTC) – [ADR-ANGELEGT]

- **ADR-038** `[REAKTIV]` „Ruhende Welt als Blöcke im Bild, Zeichnung mit Datentextur, S-1 Layout 2“: freigegeben vom Projektinhaber per Auswahl, alle Empfehlungen:
  - Frage 1: W-A + Z-1.
  - Frage 2: Speicher in 3.1 nachmessen.
  - Frage 3: neue Erkundung 3.14.
  - Frage 4: Kenntnis der Reflexion und Maßnahme (11).
- **Klassifikation:** reaktiv nach der 14-Tage-Regel (S-1 aus ADR-021 und ADR-032). Reaktiv-Quote 7 / 10 (ADR-029 bis ADR-038). Neues Muster nach Regel-009 (Code in der Brücke): Reflexion mit Stopp in `fahrplan.md`, zur Kenntnis genommen.
- **Nachgezogen:**
  - `decisions.md` (Teil A, Status ADR-021 und ADR-032, Quote).
  - `architecture-schnittstellen.md`: S-1 Layout 2 und ruhende Welt, Grundlagen `protocol` 2, S-7 Katalog-Index; Änderungsprotokoll mit der ersten Ergänzung nach Regel-008.
  - `architecture.md`: Modul Spielfeld, Abschnitt 6 und 9.
  - Fahrplan: 3.8 erledigt, neuer Schritt 3.14, Landeplätze in 3.1, 3.2, 3.4, 3.5, 4.1, 5.1; Reflexion, Replanning.
  - README, Aufzeichnung `spikes/3.8-weltdarstellung/README.md`.
- **Schritt 3.8:** erledigt (Phasentyp-Kontext ERKUNDUNG, Akzeptanz wissensbasiert; zwei Sessions Zeitbox, eine gebraucht). Phase 3: 14 Schritte bei Plan 13.

### 2026-10-20 17:22 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.8, Messung abgeschlossen:** sechs Läufe (Referenz-PC und Mac je W-A + Z-1, W-A + Z-2, W-B + Z-1), Dateien in `docs/research/3.8-ergebnisse/`, Auswertung in der Vorlage Abschnitte 14–17.
- **Mac W-B + Z-1:** sichtbar 32 ms, ganz 174 ms; Bauaktion rund ein Bild nach dem Tick; 120 Bilder/s; Speicher 913–1.257 MB (mehr als W-A).
- **Vorgelegt** (Vorlage Abschnitt 18): W-A + Z-1 mit Layout 2, Katalog-Index additiv in S-7, Reifegrade; offene Punkte späte Bilder Referenz-PC und Speicher Mac; Klassifikation `[REAKTIV]` (14-Tage-Regel, Quote 7 / 10), neues Muster nach Regel-009 (Code in der Brücke) mit Reflexion und Vorschlag Maßnahme (11).

### 2026-10-20 17:20 (UTC) – [BEOBACHTUNG]

- **Ergebnis Referenz-PC W-B + Z-1** (`docs/research/3.8-ergebnisse/referenz-pc-wb-z1.json`; Vorlage Abschnitt 16):
  - Erstübertragung: sichtbar 74 ms, ganz 348 ms.
  - Bauaktion bis zum Welt-Bild: Median 40 ms, höchstens 73 ms, 3–4 Bilder; rund ein Bild nach dem Tick der Annahme (W-A: dasselbe Bild).
  - Speicher wie W-A.
- **Späte Bilder auch bei W-B** ab Fenster 8 (Rundlauf p99 14–31 ms), vorher nicht. Der Welt-Abschnitt im Bild ist als Ursache widerlegt. Ursache auf dem Referenz-PC weiter offen, vermutlich Renderer unter Windows.

### 2026-10-20 16:48 (UTC) – [BEOBACHTUNG]

- **Ergebnisse Mac W-A** (`docs/research/3.8-ergebnisse/mac-m3pro-wa-z1.json`, `mac-m3pro-wa-z2.json`; Vorlage Abschnitt 15):
  - 119,8–120,0 Bilder/s, Bildabstand p99 bis 11,7 ms, Rundlauf p99 bis 2,5 ms.
  - Erstübertragung ganz 123 bzw. 141 ms; Bauaktion bis zum Welt-Bild höchstens 32 ms.
  - Speicher über der Schwelle, Z-1 Spitze +339 MB, Z-2 +620 MB gegenüber 2.10.
- **Sichtbeobachtung Projektinhaber, Lauf `world wb z1` (Referenz-PC, neues Paket, noch laufend):** bis Fenster 11 kein Ruckeln erkennbar, ab Fenster 12 der Anzeige (Index 11, Nahansicht) Ruckeln. Frage, ob Personen über Möbel laufen dürfen: im Prototyp ja, weil die 20.160 Objekte nur Darstellungsdaten sind und der Kern sie nicht kennt (Objekte im Weltzustand und ihre Wirkung auf die Begehbarkeit kommen mit 3.1).
- **Sichtbeobachtung Projektinhaber (Referenz-PC):** Ruckeln vor allem in der Nahansicht beim Kreisen; deckt sich mit Bildabstand p99 bis 47 ms in den kreisenden Ansichten.
- **Folgerung:** Die späten Bilder des Referenz-PCs sind dort an den Rechner gebunden, nicht an W-A. Der Engpass ist der Speicher. Z-2 scheidet aus.

### 2026-10-20 16:45 (UTC) – [BEOBACHTUNG]

- **Schritt 3.8, Fehler im Prototyp (W-B):** Auf dem Mac blieb `world wb z1` ohne Welt. Ursache: Das erste Bild einer neuen Sitzung kommt vor `sessionStarted`; der Renderer fragte die angekündigten Blöcke noch mit der alten Sitzung ab, der Kern verwarf sie (Vertrag S-7), alle vier Abfrageplätze blieben belegt. Im Container lief es zufällig durch (Reihenfolge der Nachrichten). Behoben in `f23e154` (Abfrage erst mit der neuen Sitzung, hängende Abfragen neu stellen); Container-Probe danach: sichtbarer Ausschnitt 1,1 s, ganze Welt 1,6 s. Neuer Release-Lauf 39459607051. Lehre für den Vertrag: W-B müsste den Sitzungswechsel ausdrücklich regeln.
- **Ergebnisse Referenz-PC W-A** (`docs/research/3.8-ergebnisse/referenz-pc-wa-z1.json`, `referenz-pc-wa-z2.json`; Auswertung in der Vorlage Abschnitt 14):
  - Erstübertragung: sichtbar 0,14 s bzw. im ersten Bild, ganz 0,23 bzw. 0,27 s.
  - Bauaktion bis zum Welt-Bild höchstens 57 ms, in allen 122 Bauaktionen im selben Bild wie der Tick.
  - Bildrate 58,1–60,0 bzw. 58,9–60,0 Bilder/s, Bildabstand p99 bis 47 ms (2.10: 17,3–19,7 ms). Ursache: Rundlauf der Bildanforderung p99 14–31 ms statt 1–3 ms. Herkunft offen, Kern-Prozess oder Renderer.
  - Speicher: Z-1 +90 bis +161 MB, Z-2 bis +369 MB gegenüber 2.10.

### 2026-10-20 15:32 (UTC) – [BEOBACHTUNG]

- **Schritt 3.8, Prototyp:** auf dem nicht gemergten Spike-Branch `erkundung/3-8-messaufbau` (`257a498`, `8f557d2`, `74ceec9`). Kern allein: 4.988 Blöcke des Referenz-Terminals auf Karte 2048² = 2,76 MB, Kodierung 100 ms; W-A in 3 Bildern (langsamstes 47 ms im Kern-Prozess). Container-Probelauf aller drei Kombinationen über SwiftShader: ganze Welt nach 0,74–0,96 s, Bauaktion bis zum Welt-Bild mit W-B 1–2 Bilder später als mit W-A. Bildraten ohne Aussage.
- **Antworten zum Messplan (15:05 UTC, per Auswahl):** Übersicht als Einstellung im Grafikmenü mit Voreinstellung „vereinfacht“; ganze Welt nach höchstens 3 s, sichtbarer Ausschnitt nach höchstens 1 s; drei Läufe je Rechner. Festgehalten in der Vorlage Abschnitt 11.
- **Release-Lauf** 39449159828 auf dem Spike-Branch grün auf drei Plattformen (Bau und Rauchtest der Pakete; Artefakte openairside-Windows, -macOS, -Linux); Messanleitung in `docs/research/3.8-weltdarstellung.md` Abschnitt 13.
- **Reibungen:** (1) PixiJS 8.20.1 setzt `#version 300 es` bei eigenem `GlProgram` nicht selbst; ohne die Zeile scheitert `texelFetch` beim Übersetzen. (2) Ein Block ohne zweiten Halter wird von `Arc::make_mut` an Ort und Stelle geändert; die Erkennung geänderter Blöcke über die Identität braucht deshalb, dass der Sender die übergebenen Blöcke selbst hält. (3) Der Grafik-Prüfer meldet die 512 Testgrafiken (kein Objekt im Basispaket); betrifft nur den Spike-Branch, der Release-Workflow prüft das nicht.

### 2026-10-20 15:01 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.8, Vorlage vor der Messung:** `docs/research/3.8-weltdarstellung.md`. Kandidaten für den Weg der ruhenden Welt (W-A Blöcke im Bild gegen den quittierten Stand, W-B Abruf über S-7) und für das Zeichnen (Z-1 Datentextur mit Shader, Z-2 vorgerenderte Blöcke; Z-3 Sprite je Zelle ohne Messung verworfen: 6,55 Mio. Zellen). Entwurf S-1 Layout 2 (Kopf 48 Byte; Personen 8 Byte mit Gruppe nach ADR-037, Fahrzeuge 12, Flugzeuge 16, Gepäck 8; Welt-Abschnitt nur bei W-A).
- **Vertragsabgleich (Maßnahmen 7–9):** 12 Sätze geprüft. Funde: V-1 Layout 2 erhöht `protocol` auf 2; V-4 W-B bräche „Wirkung im Bild von `tick`“ und das vorläufige Zeichnen; V-5 (Maßnahme 8) der Katalog in S-7 nennt keine feste Reihenfolge, auf die S-1 per Index verweisen kann; V-7 W-B bräuchte eigene Grenzen mit Herkunft.
- **Frage 3 (neue Wegweiser):** im Entwurf beantwortet – die Darstellung zeigt keine Wegweiser und erhält nach `waypointsAdopted` nichts neu; die Messung prüft es mit.
- **Vorgelegt:** drei Fragen (Einzelheiten in der weiten Übersicht, Wartezeit der Erstübertragung, Umfang des Messplans). Messaufbau: Prototyp auf dem nicht gemergten Spike-Branch `erkundung/3-8-messaufbau`, Pakete aus dem bestehenden Release-Workflow.

### 2026-10-20 14:53 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-20 14:52 UTC (Session zu 3.7, Kontext 492.293 Token über der Grenze von 400.000; neue Session vom Projektinhaber gewählt). Merge-Vermerk `8adb074` und Sessionende `3f813e3` nach `project-context.md` Abschnitt 10 von `scp/compassionate-shannon-wz4apd` auf den Branch dieser Session vom aktuellen `main` (`b8b3106`) übernommen (`b23c396`, `6b56ab1`); der alte Branch wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 14:53 UTC → Entscheidungs-Klasse.
- **Werkzeug:** Claude Code in der Cloud; Branch `erkundung/3-8-weltdarstellung-spielfeld` nach `CLAUDE.md` Abschnitt 11.
- **Vorhaben:** Schritt 3.8 Weltdarstellung im Spielfeld und S-1 Layout 2 (Vergleichsstudie mit Messung, Zeitbox zwei Sessions). Empfohlene Klasse: Entscheidung (Auslöser 1, Kategorie 5) – aktive Klasse passt.
- **Vorabprüfung:** Eingangskriterium „3.7 entschieden“ erfüllt (ADR-037); Abhängigkeit 3.7 erledigt. Reaktiv-Quote 7 / 10 über dem Schwellenwert, Reflexion zu ADR-037 liegt vor (Regel-009). Vor dem Vertrag: Vertragsabgleich nach Maßnahmen (7) bis (9).

### 2026-10-20 14:52 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 50 Minuten seit `[SESSIONSTART]` 14:02 UTC.
- **Bearbeitet:**
  - Archivierte Session geprüft: „Weiter mit 3.7“ kam an, die Session wurde 11 Sekunden später archiviert, ohne Ausführung.
  - Schritt 3.7: Messung `spikes/3.7-personengruppen/`, Vorlage, Einwand des Projektinhabers zu Frage 1, ADR-037, Reflexion nach Regel-009.
  - PR #28 eröffnet und nach grüner CI gemergt (`b8b3106`).
- **Erreicht:**
  - 3.7 erledigt.
  - Personengruppen, Flächenarten, Zugangsprofile und Querungen entschieden; Landeplätze in 3.1, 3.2, 3.3, 3.4, 3.5, 3.11, 3.12, 4.1 und 5.6.
  - Maßnahme (10) gilt.
- **Offen geblieben:** Merge-Vermerk `8adb074` und dieser Eintrag liegen auf `scp/compassionate-shannon-wz4apd` (`project-context.md` Abschnitt 10). Die nächste Session übernimmt sie auf ihren Branch vom aktuellen `main`.
- **Nächster Schritt:** 3.8 Weltdarstellung im Spielfeld und S-1 Layout 2, in einer neuen Session; der Projektinhaber hat die neue Session gewählt.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 14:50 UTC). 0 Schritte oberhalb der Empfehlung (3.7: Entscheidung). Abgegeben: keine Teilarbeiten; der Probelauf für Routine und Mechanik bleibt offen.
- **Kontextgröße:** 492.293 Token laut `get_session` 14:50 UTC, **über der Grenze von 400.000**. Empfehlung: die nächste Arbeit in einer neuen Session beginnen; so umgesetzt.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 14:50 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung und Größen:** keine Trigger. Logbuch rund 200 Zeilen (Grenze 800), `project-context.md` 375 Zeilen (Budget 600), `architecture.md` 467 Zeilen (eigene Schwelle 500). Phase 3 läuft noch.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Architektur-Reife, Nächste Schritte, Datum).
  - ADR-037 verweist auf vorhandene Schritte (3.1–3.5, 3.11, 3.12, 4.1, 5.6).
  - Reifegrade passen zu ADR-037 (Datenmodell fortgeschrieben, neue Zeile Aufgabenfelder `[VORLÄUFIG]`).
  - Modulnamen in 3.7 stimmen mit der Modul-Karte überein.
  - FR-052 → 3.7, 3.4.
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 7 / 10 (ADR-028 bis ADR-037) stimmt mit den Tags, gesamt 15 / 37.
  - Phase 3: 13 Schritte bei Plan 13.
- **Beobachtung:** Der Einwand des Projektinhabers (Henne-Ei bei den Sicherheitsbeamten) fand einen Fehler in der ersten Empfehlung, den weder Messung noch Inventar gezeigt hätten. Fragen mit Vision-Bezug lohnen sich auch dann, wenn die Empfehlung klar wirkt.

### 2026-10-20 14:49 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #28 (Schritt 3.7, ADR-037) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `b8b3106`. Vorher grün auf `49312e5`: Push-Lauf 39441426518 und die PR-Läufe CI 39441613876 und Security 39441613790 (39 Prüfungen, CodeQL ohne Meldung). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `scp/compassionate-shannon-wz4apd` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-20 14:40 (UTC) – [ADR-ANGELEGT]

- **ADR-037** `[REAKTIV]` „Personengruppen mit Zugangsprofilen, Flächenarten und Querungen“: freigegeben vom Projektinhaber per Auswahl.
  - Frage 1: zuerst Einwand gegen A. Wenn auch Sicherheitsbeamte durch die Kontrolle müssen, kommt der erste Beamte einer Schicht nie an seinen Arbeitsplatz (Henne-Ei-Problem). Danach A2: Kontrolle für alle außer Sicherheitsbeamten und Bauarbeitern.
  - Frage 2 A: Vorfeldpersonal zu Fuß und per Fahrzeug.
  - Frage 3 A: Walk-out-Querungen geregelt im Wechsel.
  - Frage 4 A: Bauarbeiter überall außer auf Bahnen in Betrieb.
  - Technischer Entwurf wie vorgelegt, mit fünftem Profil für Sicherheitsbeamte.
- **Klassifikation:** reaktiv nach der 14-Tage-Regel (ändert ADR-020 nach 7 Tagen). Reaktiv-Quote 7 / 10 (ADR-028 bis ADR-037). Nach Regel-009 neues Muster: Reflexion mit Stopp in `fahrplan.md` („Ergänzung Reaktiv-Quote, Anlass ADR-037“), zur Kenntnis genommen; Maßnahme (10).
- **Nachgezogen:**
  - `decisions.md` (Teil A, Reaktiv-Quote, Liste) und Status von ADR-020.
  - `architecture.md`: Module Welt & Bau, Wegfindung, Verkehr, Personen; Datenmodell; verworfene Alternativen; Reifegrade. Neue Zeile „Aufgabenfelder“ `[VORLÄUFIG]` (5.6); Datenmodell bleibt `[BELASTBAR]`, fortgeschrieben.
  - `requirements.md` FR-052.
  - Fahrplan: 3.7 erledigt; Landeplätze in 3.1, 3.2, 3.3, 3.4, 3.5, 3.11, 3.12, 4.1, 5.6.
  - README; Vorlage um die Antworten ergänzt.
- **Schritt 3.7:** erledigt (Phasentyp-Kontext ERKUNDUNG, Akzeptanz wissensbasiert). Phase 3 weiter 13 Schritte bei Plan 13; die Landeplätze ergänzen bestehende Schritte und legen keine neuen an.

### 2026-10-20 14:31 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.7:** Messung `spikes/3.7-personengruppen/` (Produktions-Kern als Pfad-Abhängigkeit, Referenz-Terminal 768² × 5 in Welt 2048² mit Vorfeld, 17 Stellplätzen, 6 Walk-out-Gehwegen und 4 Personalräumen; Container, 4 vCPU). Herleitung und Vorlage: `docs/research/3.7-personengruppen.md`.
- **Ergebnisse:**
  - Begehbarkeit für Vorplatz, Gehwege und Stellplätze kostet 10 % mehr Zellen (1,50 → 1,65 Mio.). Ein gemeinsamer Feldsatz für Fluggäste und Personal mit 79 Zielen braucht 130 MB und 2,6 s (heute 36 Ziele: 54 MB, 1,0 s). Getrennte Sätze kosten dasselbe.
  - Bauarbeiter über das ganze Erdgeschoss: 5,1 MB je Feld. Ein Satz für alle Gruppen bräuchte 444 MB und 9–11 s. Zwei Felder (Anlieferstelle, alle Baustellen) kosten 10 MB und 0,6 s.
  - Einzelweg-Suche je Fahrt (A*, gleiche Kosten wie die Wegweiser, 1.000 von 1.000 gleich): im Terminal Median 18–61 ms, p99 170–272 ms; Bauarbeiter p99 1,3–1,7 s. Für Personal ungeeignet.
- **Vorgelegt:** vier Fragen (Personal und Kontrolle, Vorfeldpersonal, Walk-out-Querung, Bauarbeiter und Bahnen) und der technische Entwurf (Flächenart und Personalzone je Zelle, Profil je Ziel, ein Feldsatz für Fluggäste und Personal, einer für Bauarbeiter). Klassifikation `[REAKTIV]` (ändert ADR-020 nach 7 Tagen); nach Regel-009 neue Reflexion mit Stopp, Entwurf in der Vorlage Abschnitt 8.
- **Orientierung am Original:** Wechsel in den Sicherheitsbereich nur über eine Kontrolle, Sicherheitsbeamte und Vertragskräfte ausgenommen (airportceo.wiki.gg/wiki/Security); Vorfeldagenten über Gehwege und Zebrastreifen (forum.apoglabs.com).

### 2026-10-20 14:10 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** Sessionstart in einem frischen Cloud-Container.
- **Symptom:** `.git/hooks/pre-commit` fehlte, `pre-commit` war nicht installiert. Der Commit `aad31a3` (Sessionstart, nur Markdown) lief deshalb ohne Hooks.
- **Lösung:** `pip install --user pre-commit`, `pre-commit install` nach Runbook Schritt 3. Die Hooks liefen danach über die beiden Dateien des Commits, alle grün. `node_modules` fehlt im Container und Node ist 22.22.0 statt 26.11.1; für Markdown und Rust in 3.7 nicht nötig, vor TypeScript-Änderungen `npm ci` mit Node 26.
- **Wiederkehrgefahr:** hoch bei jeder neuen Cloud-Session. Vor dem ersten Commit `ls .git/hooks` prüfen.

### 2026-10-20 14:02 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-19 20:03 UTC, danach Merge PR #27 (`119d2b1`) mit Vermerk `d2e6c70` auf `docs/phasenabschluss-2`. Der Vermerk ist nach `project-context.md` Abschnitt 10 auf den Branch dieser Session vom aktuellen `main` übernommen (`0162671`); `docs/phasenabschluss-2` wird nicht weiterverwendet. Die Session „Phasenabschluss Phase 2“ erhielt am 2026-10-20 13:50 UTC „Weiter mit 3.7“ und wurde 11 Sekunden später versehentlich archiviert, ohne dass etwas ausgeführt wurde (Transkript über `list_events` geprüft); 3.7 beginnt hier.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 14:01 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Werkzeug:** Claude Code in der Cloud; vom Werkzeug vergebener Branch `scp/compassionate-shannon-wz4apd` (Typ und Schritt-ID im PR-Titel, `CLAUDE.md` Abschnitt 11).
- **Vorhaben:** Schritt 3.7 Personengruppen, Begehbarkeit und Querungen (Entwurf mit Messung, ADR in Ergänzung von ADR-020). Empfohlene Klasse: Entscheidung (Auslöser 1) – aktive Klasse passt.
- **Vorabprüfung:** Eingangskriterium „Datenmodell im Kern `[BELASTBAR]`“ erfüllt (`architecture.md` Abschnitt 9); Abhängigkeit 2.3 erledigt. Reaktiv-Quote 6 / 10 über dem Schwellenwert, Reflexion für Phase 2 liegt vor (Regel-009).

### 2026-10-19 20:10 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #27 (Phasenabschluss 2) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `119d2b1`. Vorher grün auf `62998bd`: Push-Lauf 39316204108 und die PR-Läufe CI 39316460887 und Security 39316460888 (39 Prüfungen, CodeQL ohne Meldung). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `docs/phasenabschluss-2` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 20:03 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 23 Minuten seit `[SESSIONSTART]` 19:40 UTC.
- **Bearbeitet:** Phasenabschluss 2: Vision-Abgleich, Pflichtfrage mit zwei getrennten Instanzen, Stellungnahme, Vorlage und Freigabe, ADR-036, Onboarding-Re-Validation, Archivierung und `[PHASEN-WECHSEL]`, Phase 3 im vollen Format.
- **Erreicht:** Phase 2 abgeschlossen. Phase 3 hat einen ursprünglichen Schrittplan von 13 Schritten; Regel-008 und Regel-009 gelten. Abschnitt 4 der Architektur ist ausgelagert, FR-052 neu, FR-016 ergänzt.
- **Offen geblieben:** PR für `docs/phasenabschluss-2` nach `main`, nur auf Anweisung. Rückmeldung an das Vorlagen-Repo Dev-Templates zur Zählregel der Reaktiv-Quote: im Fahrplan vermerkt, nicht gesendet, liegt außerhalb dieses Repos.
- **Nächster Schritt:** 3.7 Personengruppen, Begehbarkeit und Querungen, in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 20:03 UTC). 0 Schritte oberhalb der Empfehlung (Phasenabschluss: Entscheidung). Abgegeben: zwei Bewertungen an Unteragenten als getrennte Instanzen (Opus 5.5 und Sonnet 5.5). Das war kein Sparzweck; der Probelauf für die Abgabe an Routine/Mechanik bleibt offen.
- **Kontextgröße:** nicht feststellbar – `get_session` meldet `used_tokens: 0`. Kein Hinweis zur Sessiongröße.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 20:03 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung:** Fahrplan Phase 2 und Logbuch Phase 2 ausgelagert. Logbuch jetzt rund 130 Zeilen, Architektur 461 Zeilen (unter der Schwelle von 500), `project-context.md` 375 Zeilen.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Nächste Schritte, Verweise auf `architecture-schnittstellen.md`).
  - ADR-036 verweist auf vorhandene Schritte (3.1–3.13, 4.1, 4.3).
  - Reifegrade passen zu ADR-036 (keine Änderung).
  - Modulnamen der Phase-3-Schritte stimmen mit der Modul-Karte überein.
  - Jede Muss-Anforderung hat einen Schritt (FR-052 → 3.7, 3.4).
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036) stimmt mit den Tags.
  - Phase 3: 13 Schritte bei Plan 13.

### 2026-10-19 20:02 (UTC) – [PHASEN-WECHSEL]

- **Phase 2 → Phase 3:** Phase 2 (UMSETZUNG, Fundament) mit 11 von 11 Schritten abgeschlossen; Phase 3 (UMSETZUNG, vertikaler Schnitt) mit ursprünglichem Schrittplan von 13 Schritten ausgeplant (ADR-036). Detail-Einträge von Phase 2: [`docs/archiv/logbuch-phase-2.md`](archiv/logbuch-phase-2.md); Fahrplan: [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md). Die Einträge der Session 1.7 stehen jetzt in [`docs/archiv/logbuch-phase-1.md`](archiv/logbuch-phase-1.md).
- **Reflexion Phase 2:**
  - Gelernt: Fundament mit Reserve – Schritt mit 10.000 Personen 0,13–0,56 ms, Bild an der Grenze des Bildschirms auf Referenz-PC und Mac (2.7, 2.10), Spielstand bitgleich nach Laden auch während einer Neuberechnung (2.5). Getrennte Prüfungen fanden in jedem Schritt mit Kategorie 6 echte Mängel; die Sammelprüfung 2.11 fand 10 Vertragslücken auf einmal.
  - Gekippte Annahmen: Verträge aus 2.9 trugen das Gebaute nicht vollständig (ADR-026, ADR-028 bis ADR-032); „Kern-Module über Weltzustand und interne Ereignisse“ ist noch nicht gebaut (3.9); Neuberechnung im Hintergrund ohne Zusammenfassen der Anfragen trägt Dauerbau nicht (3.1); Linux bleibt ohne Messung auf echter Hardware (7.2).
  - Reifegrade: Takt & Weltzustand, Welt & Bau (Grundraster), Brücke, Plattform, Inhalte & Mods (Lader), Lokalisierung, Grafik-Pipeline `[BELASTBAR]` durch Umsetzung; S-2 bis S-7 bestätigt, S-8 neu; NFR Darstellung im gemessenen Umfang (ADR-034); Weltdarstellung `[OFFEN]` (3.8).
  - ADRs der Phase: ADR-018 bis ADR-034 (dazu ADR-035 im Querschnitt, ADR-036 Phasenabschluss); reaktiv ADR-019, ADR-020, ADR-025, ADR-026, ADR-028 bis ADR-032 – formal durch die 14-Tage-Regel oder additive Vertragsergänzungen, keiner änderte Code in Takt, Welt & Bau, Wegfindung oder Brücke. Maßnahmen: Regel-008, Regel-009.
  - Neue Erkundungen: 3.11 Suchen ohne Schilder und Feinwege, 3.12 Blockadevermeidung, 3.13 Schrittzeit-Spitzen auf Apple-Chips; Entwurf 3.9 Kernstruktur.
  - Mini-Reibungen, die wiederkehren können: Zeitangaben geschätzt statt mit `date -u` geholt (zweimal); Typ-Check des Pre-Commit-Hooks über den ganzen Arbeitsbaum; Prettier formatiert JSON in `docs/research/`; Suppression ohne Begründung (`scenario.rs:76`) blieb in Hooks und Prüfungen unbemerkt.
- **Pflichtfrage:** „gezielt umbauen“ (A), zwei getrennte Instanzen (Opus 5.5, Sonnet 5.5), `docs/research/phasenabschluss-2.md`.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ → FR-016 mit 3.2 und 4.1; MOD-13 Walk-out in den Kern → FR-052 mit 3.7 und 3.4; Erkundungen mit eigener ID 3.11, 3.12; keine `[VERSCHOBEN]`-Zeilen. Keine verwaisten Elemente mehr.
- **Onboarding-Validierung:** siehe `[ONBOARDING-VALIDATION]` 19:56.

### 2026-10-19 20:01 (UTC) – [ADR-ANGELEGT]

- **ADR-036** `[REAKTIV]` „Phasenabschluss 2 – gezielt umbauen vor dem vertikalen Schnitt“: freigegeben vom Projektinhaber per Auswahl (A; MOD-13 in den Kern; FR-016 ergänzen; 4.3 als 3.13 vorziehen; kein Linux-Rechner). Reaktiv, weil er den Landeplatz 4.3 aus ADR-034 (heute) nach 3.13 verlegt. Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036), Reflexion für Phase 2 im Fahrplan nach Regel-009.
- **Nachgezogen:** `decisions.md` (Teil A, Regel-008, Regel-009), Abschnitt 4 nach `docs/architecture-schnittstellen.md` mit Änderungsprotokoll und Ankern, `architecture.md` (Abschnitte 0, 2, 4, 9, offene Fragen), `requirements.md` (FR-016, FR-052, Schritt-IDs 3.11/3.12, Übersicht 52 Anforderungen), `modernisierung.md` (MOD-13 Kern), Fahrplan (Phase 3 im vollen Format, 4.1, 4.3, Reflexion, Replanning), README.

### 2026-10-19 19:56 (UTC) – [ONBOARDING-VALIDATION]

- **Phasen-Abschluss 2 (Trigger 3, Klasse G):** frischer Worktree von `origin/main` (`d4e457f`) im Container (Linux, 4 vCPU), Runbook Schritte 2 bis 5 wörtlich: `npm ci`, `pre-commit run --all-files` (alle Hooks grün), `npm run build`, Start `npx electron . --no-sandbox` unter `xvfb-run` (40 s ohne Absturz, nur GPU-Meldungen der Software-Grafik), dann alle Gates: rustfmt, clippy, `cargo test` (alle grün, u. a. 134 Kern-Unit-Tests, 26 Datenpaket-, 10 Spielstand-Fälle), cargo-deny, ESLint, Prettier, tsc, Vitest 356 Tests mit 86,6 % Lines, Lizenzen, `check:types`, `check:schema`, `check:graphics`, Benchmarks (Tick: Mittel 0,17 ms, p99 0,47 ms; Wegweiser: Neuberechnung Median 2,9 s bei Grenze 4 s, Schritt p99 1,8 ms), Rauchtest grün (17,1 s).
- **Fund und behoben:** Runbook Schritt 4 nannte für das Basispaket „3 entries“; seit 2.6 zählen die Texte der Sprachdateien mit. Text auf „N entries“ mit Erklärung umgestellt (`a0603e7`).
- **Einschränkungen:** Electron kam aus dem Cache der Container-Einrichtung (`~/.cache/electron`), der Download beim ersten Start ist damit nicht frisch belegt. Neuberechnung 2,9 s liegt über den früher lokal gemessenen 1,1–1,2 s (anderer Container, kein Gate verletzt; Beobachtung für 3.1). Zweite Plattform (Klasse G): Windows und macOS über die CI des Stands (PR #26 auf `0cf1724`, Lauf 39311921712 grün auf drei Plattformen), nicht von Hand.

### 2026-10-19 19:51 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Pflichtfrage Phasenende 2:** zwei getrennte Instanzen ohne Gesprächsverlauf, Logbuch ausgeschlossen: A Opus 5.5 (wie Phase 1), B Sonnet 5.5 (anderes Modell). Beide empfehlen „gezielt umbauen“, Konfidenz mittel, und schließen Neuaufsetzen mit Belegen aus. A: Kernstruktur für die Fachmodule vor 3.1, Dauerbau der Wegweiser, `main.ts` teilen, Architektur auslagern, Zählregel. B: Prozess (Zählregel, Reflexionen), `main.ts`, S-1 Layout 2 in 3.8. Volltext und Stellungnahme: `docs/research/phasenabschluss-2.md`.
- **Nachgeprüft:** `core/src/adoption.rs` (ältere Neuberechnung läuft weiter, wird verworfen), `vitest.config.ts` (Ausnahmen), `core/src/scenario.rs:76` (Suppression ohne Begründung, Verstoß seit 2.3; B hatte sie als begründet gezählt), keine internen Ereignisse im Kern, `architecture.md` über der eigenen Schwelle von 500 Zeilen.
- **Widerspruch der Stellungnahme:** Die Zählregel steht wörtlich in `CLAUDE.md` Abschnitt 6 und ist im Projekt nicht auslegbar. Stattdessen soll die Projektregel „jede Änderung an `[BELASTBAR]` ist freigabepflichtig“ an `CLAUDE.md` Abschnitt 4 Nr. 5 angeglichen werden (additiv ohne ADR) und es soll eine Reflexion je Phase geben.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ ohne Akzeptanzkriterium (FR-016 ergänzen, 3.2); MOD-13 Walk-out (Kandidat) steckt in 3.7 und über ADR-020 in 3.4 – Frage an den Projektinhaber; Erkundungen vor 3.3/3.4 ohne Schritt-ID (Regelverstoß, neue Schritte 3.11/3.12). Keine `[VERSCHOBEN]`-Zeilen.
- **Archivierung vorgezogen (unabhängig von der Antwort):** Fahrplan Phase 2 nach `docs/archiv/fahrplan-phase-2.md` mit Phasen-Bilanz; Logbuch-Einträge von Phase 2 nach `docs/archiv/logbuch-phase-2.md`, die Session 1.7 (2026-10-13 18:00–19:05) nach `docs/archiv/logbuch-phase-1.md`. `[PHASEN-WECHSEL]` mit Reflexion folgt nach der Antwort.
- **Drift behoben:** Fahrplan „Aktueller Stand“ nannte 7 / 10, `decisions.md` 6 / 10 (seit ADR-035); jetzt 6 / 10.

### 2026-10-19 19:40 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 18:53 UTC; danach Merge PR #25 (19:11), ADR-035 und Q.3 (19:31), Merge PR #26 (19:37, `d4e457f`). Der Abschluss-Commit `836ec95` nach `project-context.md` Abschnitt 10 auf den neuen Branch `docs/phasenabschluss-2` vom aktuellen `main` übernommen (`e1c2262`); `ci/q-3-rustup-toolchain-install` wird nicht weiterverwendet. Die vorige Session endete mit über 500.000 Token Kontext (Grenze 400.000).
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 19:40 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Vorhaben:** Phasenabschluss Phase 2 nach `CLAUDE.md` Abschnitt 12: Vision-Re-Derivations-Pass (mit `requirements.md`), Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“ mit Bewertung durch eine getrennte Instanz, Onboarding-Re-Validation (Trigger 3, Klasse G), Archivierung und `[PHASEN-WECHSEL]`. Empfohlene Klasse: Entscheidung (Auslöser 1 und 3) – aktive Klasse passt.

---

<!-- ANCHOR:eintragstypen -->
## Eintragstypen (Übersicht)

| Typ | Wann | Pflicht? |
|---|---|---|
| `[SESSIONSTART]` | Zu Beginn jeder Session | Ja |
| `[SESSIONENDE]` | Vor Sessionabschluss | Ja |
| `[PROBLEM-GELÖST]` | Nach Behebung eines Problems, das Reibung war | Empfohlen, alle Mini-Probleme erfassen |
| `[PROBLEM-OFFEN → BLOCKER]` | Wenn ein Problem zum Blocker eskaliert | Ja, mit Verweis auf `blockers.md` |
| `[BLOCKER-AUFGELÖST]` | Wenn ein Blocker gelöst wurde | Ja, mit Verweis auf den ursprünglichen Logbuch- und Blocker-Eintrag |
| `[REIFEGRAD-WECHSEL]` | Bei jeder Reifegrad-Änderung in `architecture.md` | Ja |
| `[ADR-ANGELEGT]` | Bei Anlage eines neuen ADR | Ja |
| `[BEOBACHTUNG]` | Wenn etwas auffällt, das später nützlich sein könnte | Optional, KI proaktiv |

### Format der Einträge

```text
### YYYY-MM-DD HH:MM – [TYP]

- **Feld:** Inhalt
```

Felder je Typ:

- `[SESSIONSTART]`: Letzter Stand, Geplant für diese Session, Vorabprüfung (Reifegrade, Eingangskriterien), Modus/Werkzeug, Modell (eingestellt und bedient) mit Modellklasse und Quelle (`CLAUDE.md` Abschnitt 0)
- `[SESSIONENDE]`: Session-Dauer, Bearbeitet, Erreicht, Offen geblieben, Nächster Schritt, Aktive Klasse mit Zahl der Schritte oberhalb ihrer Empfehlung und abgegebenen Teilarbeiten, Kontextgröße (mit Hinweis bei Überschreitung der Grenze aus `project-context.md` Abschnitt 6), Ablaufdaten-Prüfung, Stimmung/Beobachtung (optional)
- `[PROBLEM-GELÖST]`: Kontext, Symptom, Ursache, Lösung, Aufwand, Lerneffekt (optional), Wiederkehrgefahr (optional)
- `[PROBLEM-OFFEN → BLOCKER]`: Kontext, Symptom, Versuche bisher, Eskaliert nach (Blocker-Nr.)
- `[REIFEGRAD-WECHSEL]`: Bestandteil, Wechsel, Auslöser, Datum in `architecture.md` Abschnitt 9 nachgetragen
- `[ADR-ANGELEGT]`: ADR, Tag, Auslöser

<!-- ANCHOR:hinweise-zur-pflege -->
## Hinweise zur Pflege

- **Neueste Einträge oben.**
- **Zeitstempel ist Pflicht.** Format: `YYYY-MM-DD HH:MM` (24h, lokale Zeitzone).
- **Detailtiefe lieber zu hoch als zu niedrig.** Wenn unsicher, ob etwas eingetragen werden soll: eintragen.
- **Verweise statt Duplikate** zu ADRs, Blockern und Fahrplan-Schritten.
- **Keine sensiblen Daten.**
- **CI-Ergebnisse:** Bei Problemen aus der CI wird festgehalten, was im Log stand, nicht nur die Ampelfarbe.

<!-- ANCHOR:archivierung -->
## Archivierung

Bei mehr als 800 Zeilen: alte Einträge nach `docs/archiv/logbuch-YYYY-MM.md` auslagern; im aktiven Logbuch bleiben die letzten 4–8 Wochen plus alle Einträge zu offenen Blockern. Auslagerung ist Sessionende-Aktion.
