# Fahrplan – Dev-Templates

<!-- Arbeitsdokument von Dev-Templates selbst (Selbstanwendung, ADR-001).
     Die Vorlage für Ziel-Projekte liegt unter templates/docs/fahrplan.md.
     Klasse-K-Default: flache Schrittliste ohne Phasenstruktur,
     Phasentyp pro Schritt notiert. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

- **Stand vom:** 2026-10-12
- **Phasentyp:** STABILISIERUNG – die Methodik wird gehärtet. Neue Regelbereiche laufen als eigene UMSETZUNG-Schritte (ADR-017; Bündel #34 hat das verletzt, Prüfbericht W2).
- **Aktiver Schritt:** keiner. S-17 (CI-Nachlauf) am 2026-10-12 abgeschlossen; alle früheren `[CI-AUSSTEHEND]`-Schritte sind `[ERLEDIGT]`.
- **Archiv:** Schritte vor dem ersten Bündel und das Bündel „Umsetzung von Issue #34" (S-13 bis S-22, abgeschlossen am 2026-09-24 mit ADR-015) liegen seit 2026-09-27 in [`docs/archiv/selbstanwendung/fahrplan-bis-buendel-34.md`](fahrplan-bis-buendel-34.md) (ADR-018). Punkt 2 aus #34 (Betriebsmodus) hat den Landeplatz S-27.
- **Aktuelles Schritt-Bündel:** „Umbau nach Prüfbericht" (ADR-015). Ursprünglicher Schrittplan: 4 Einheiten = 4 Schritte (S-23 bis S-26, je ein Umbau-Block), festgehalten am 2026-09-24 beim Anlegen. Wucherungs-Schwelle: mehr als 9 Schritte (Faktor 2 und mindestens +5). Bis zum Abschluss keine neue Regel außerhalb der Blöcke.
- **Nächster Schritt:** Pause am 2026-10-12 aufgehoben. Als Nächstes S-26 (letzter Block des Bündels, freigabepflichtig). Daneben zur Entscheidung: S-28 (CI ohne `pre-commit/action`, Frist 2026-12-31) und der veraltete PR #31 zu S-7. S-7 und S-18 warten auf Anforderung.
- **Reaktiv-Quote über der Schwelle** (40 % seit ADR-018): Reflexion unten unter „Iterations-Reflexion".
- **Offene STOPP-Situationen:** keine. Der Stopp vom 2026-09-24 22:15 (Kriterium 9, Bündel #34) ist mit ADR-015 aufgelöst: Pflichtfrage durch getrennte Instanz beantwortet, Vision-Abgleich im Prüfbericht Abschnitt 4, Neuplanung als neues Bündel.

<!-- ANCHOR:schritte -->
## Schritte

### S-7: Inter-Pflicht-Drift-Checks automatisieren

- **Status:** [OFFEN]
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – neues Hilfsskript plus CI-Erweiterung (`CLAUDE.md` Abschnitt 4, Kategorie 7)
- **Empfohlene Klasse:** Entscheidung – freigabepflichtig (Eskalations-Auslöser 1); die spätere Umsetzung des freigegebenen Skripts wäre Routine.
- **Abhängigkeiten:** S-5 (erledigt)
- **Zu tun:** Option B aus dem S-5-`ENTSCHEIDUNG ERFORDERLICH`-Block: ein Hilfsskript (vermutlich Python, `scripts/check-drift.py`), das die fünf Anker aus `CLAUDE.md` Abschnitt 16 prüft (ADR→Fahrplan, Reifegrad-Konsistenz, Bestandteils-Liste, Blocker-Referenz, Reaktiv-Quote), plus CI-Job.
- **Akzeptanzkriterien:** Skript erfüllt die Pflichtkategorien A–H aus `CLAUDE.md` Abschnitt 15 (Header, Voraussetzungs-Deklaration, Plattform-Matrix, Idempotenz-Aussage); eigener Tooling-Inventar-Eintrag in `architecture.md`; CI-Job führt es aus. **Zusätzlich: das Skript braucht eigene Tests für seine Erkennungslogik** – siehe Notiz unten.
- **Notizen:** Bewusst von S-5 abgespalten, kein Anhängsel. Bis dahin bleibt die Drift-Prüfung Sessionende-Disziplin.

  **Anforderung aus S-9 (2026-08-13):** Der in dieser Session wiederholt benutzte Ad-hoc-Prüfcode hat eine belegte Falsch-Positiv-Klasse: Er zählte eine Erwähnung von `[REAKTIV]` im **Fließtext** eines ADR als Klassifikations-Tag und meldete dadurch eine Reaktiv-Quote von 2/5 statt korrekt 1/5 – also eine scheinbare Schwellenwert-Überschreitung, die keine war. Ein Prüfskript, das falsch Alarm schlägt, ist gefährlicher als keines: Es erzeugt Gewöhnung an ignorierte Warnungen. Konsequenz für S-7: Die Prüfungen müssen strukturell verankert sein (nur `- **Tags:**`-Zeilen auswerten, nicht das ganze Dokument), und das Skript selbst braucht Tests mit bewusst konstruierten Fällen – sowohl echte Drift als auch Prosa-Erwähnungen, die keine sein dürfen.

  **Anforderung aus S-10/S-11 (2026-08-28):** Zwei Befunde dieser Session hätte kein bestehendes Gate gefangen – die Folgewirkung eines Patches auf zwei andere Abschnitte und dieselbe Regel, die in einer Vorlage zweimal in widersprüchlichem Wortlaut stand. Der Linter prüft Form, der Drift-Check die fünf definierten Anker; **semantische Widersprüche zwischen zwei Stellen desselben Regelwerks fängt bislang nichts**. Realistische Zwischenstufe für S-7 statt einer Volllösung: eine Begriffs-Konkordanz für eine kurze Liste von Kernbegriffen (z. B. `vision.md`, `Teil C`, die Status-Marker) – wo ein Begriff mehrfach mit einer Lese- oder Pflicht-Aussage auftaucht, werden die Fundstellen zur manuellen Sichtung ausgegeben, statt sie automatisch zu bewerten. Ein Werkzeug, das Fundstellen zeigt, ist ehrlicher als eines, das Widerspruchsfreiheit behauptet.

### S-17: CI-Nachlauf auf `main` nach dem Reset des Actions-Kontingents

- **Status:** [ERLEDIGT] 2026-10-12
- **Verifikation bei Abschluss:** Lauf 121 auf `main` (Commit `4e46c4e`, enthält PR #35 bis #48) neu gestartet, Versuch 2 am 2026-10-12 grün: alle vier Hooks bestanden. Protokoll vollständig gelesen. Einzige Plattform-Warnung: „Node.js 20 is deprecated … actions/cache@v4", aus `pre-commit/action@v3.0.1` (neueste Version). Als benannte Ausnahme im Warnungs-Bestand (`docs/project-context.md` Abschnitt 7) mit Landeplatz S-28. Sonst nur Hinweise ohne Warnungsrang: Node-`DeprecationWarning`s innerhalb von `actions/cache@v4`, gleiche Ursache, und der `git init`-Hinweis zum Namen des Start-Branches. Lokal `pre-commit run --all-files` ebenfalls grün. Zwölf Schritte von `[CI-AUSSTEHEND]` auf `[ERLEDIGT]`.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** nein
- **Empfohlene Klasse:** Routine – CI-Lauf auslösen und Protokoll lesen, kein Eskalations-Auslöser.
- **Abhängigkeiten:** Reset des Actions-Kontingents (Ablaufdaten-Register, `docs/project-context.md` Abschnitt 8)
- **Frist:** erster Werktag nach dem Reset – vermutlich 2026-09-29 (Angabe des Eigentümers, unbestätigt) –, spätestens 2026-10-31 (ADR-017)
- **Zu tun:** Einen CI-Lauf auf `main` auslösen (ein Push oder manuell) und das Protokoll vollständig lesen – Status **und** Hinweise der CI-Plattform. Damit werden die ohne grüne CI gemergten PRs #35 bis #46 nachträglich geprüft.
- **Akzeptanzkriterien:** Lauf grün, Protokoll gelesen, Befund im Logbuch. Danach die Schritte mit Status `[CI-AUSSTEHEND]` – namentlich S-12, S-13, S-14, S-15, S-16, S-19, S-20, S-21, S-22 (im Archiv `docs/archiv/selbstanwendung/fahrplan-bis-buendel-34.md`), S-23, S-24, S-25 sowie jeder spätere – auf `[ERLEDIGT]` setzen. Bei Rot fallen die betroffenen Schritte auf `[IN ARBEIT]` zurück (`CLAUDE.md` Abschnitt 9), und ein Fix-Schritt mit Frist entsteht.
- **Notizen:** Bisher stand dieser Nachlauf nur als „offen geblieben" in Logbuch-Einträgen – ohne Schritt-ID und damit ohne gültigen Landeplatz. Angelegt mit Einführung des Ablaufdaten-Registers (S-15).

### S-28: CI ohne `pre-commit/action` – Abkündigung von Node 20 in `actions/cache@v4`

- **Status:** [WARTET-AUF-FREIGABE]
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung an `.github/workflows/ci.yml` (`CLAUDE.md` Abschnitt 4, Kategorie 7)
- **Empfohlene Klasse:** Entscheidung für die Freigabe (Eskalations-Auslöser 1); die Umsetzung danach ist Routine.
- **Abhängigkeiten:** keine
- **Frist:** 2026-12-31 (Nachprüf-Datum im Ablaufdaten-Register; ein hartes Lebensende von `actions/cache@v4` ist nicht veröffentlicht)
- **Anlass:** CI-Lauf vom 2026-10-12 (S-17) warnt: „Node.js 20 is deprecated. The following actions target Node.js 20 but are being forced to run on Node.js 24: actions/cache@v4." Die Aktion stammt aus `pre-commit/action@v3.0.1`; eine neuere Version gibt es nicht (Release-Seite, 2026-10-12). GitHub hat Node 20 am 2026-09-23 von den Runnern entfernt (GitHub-Changelog vom 2025-09-19); der Lauf funktioniert, weil die Aktion zwangsweise auf Node 24 läuft.
- **Zu tun:** `pre-commit/action` durch eigene Schritte ersetzen: `pip install pre-commit` mit fester Version, Cache über eine aktuelle, Node-24-fähige Version von `actions/cache` (Version nach `CLAUDE.md` §15 „Versionswahl" belegen), dann `pre-commit run --show-diff-on-failure --all-files`.
- **Akzeptanzkriterien:** CI-Lauf grün ohne Plattform-Warnung; benannte Ausnahme aus dem Warnungs-Bestand und Zeile im Ablaufdaten-Register entfernt; ADR zur Pipeline-Änderung.
- **Betroffene Bestandteile:** Selbstanwendung

### S-18: Beispielversionen in den CI- und Pre-Commit-Vorlagen nach der Versionsregel prüfen (Vorschlag)

- **Status:** [OFFEN] – Vorschlag, wartet auf Anforderung durch den Eigentümer
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** nein, solange nur Beispielwerte in Vorlagen betroffen sind; Major-Sprünge bei eigenen Abhängigkeiten wären es.
- **Empfohlene Klasse:** Routine – Versionen mit Quellen belegen nach der Regel aus ADR-010, kein Eskalations-Auslöser.
- **Abhängigkeiten:** S-15
- **Zu tun:** Die vorbelegten Werte gegen die Regel „ausgereifte Linie" (ADR-010) prüfen und mit Quelle belegen: `templates/pre-commit/typescript.yaml` pinnt Prettier auf `v4.0.0-alpha.8` (eine Vorabversion); `templates/github-workflows/ci-typescript.yml` nennt `NODE_VERSION: "20"` und `PNPM_VERSION: "9"`, die Python-Vorlagen `3.12`. Alle sind als `TBD` gekennzeichnet und werden in Modus 2 ersetzt – als Beispiele sollten sie der eigenen Regel trotzdem nicht widersprechen.
- **Akzeptanzkriterien:** jeder Wert mit Quelle belegt oder bewusst als Platzhalter ohne konkrete Zahl gesetzt; `pre-commit run --all-files` grün.

### S-23: Umbau U2 – Unterbrechungen bündeln

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-27 – PR [#45](https://github.com/Paddel87/Dev-Templates/pull/45) als elfte dokumentierte CI-Ausnahme gemergt (Anweisung des Eigentümers; Befund W1 wird in S-24 geklärt, Nachprüfung S-17)
- **Freigabe:** 2026-09-27, Teilfragen 1B, 2A, 3D, 4B (Empfehlung war 1A, 2C, 3B, 4B).
- **Artefakte:** ADR-016, PR [#45](https://github.com/Paddel87/Dev-Templates/pull/45)
- **Verifikation (Stand Umsetzung):** Unterbrechungsarten 13 → 12. Grep-Gegenprobe: „weiter hier" kommt in `CLAUDE.md` nicht mehr vor; §12 Punkt 4 nennt die Kontextgröße weiterhin. `pre-commit run --all-files` grün.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung an `CLAUDE.md` §0, §4, §12
- **Empfohlene Klasse:** Entscheidung – Regeländerung mit `[STRATEGISCH]`-ADR.
- **Abhängigkeiten:** ADR-015
- **Zu tun:** Prüfbericht Abschnitt 5.2 Block U2: Modellwechsel-Stopp und Freigabe-Block zu einem Block zusammenlegen (A3); Nebenfragen je Session höchstens einmal gesammelt vorlegen; Obergrenze für Grundsatzentscheidungen je Tag oder Session prüfen (3.5); festlegen, ob die Pflichtfrage eine Freigabe im Sinne von Abschnitt 4 ist.
- **Akzeptanzkriterien:** jede Änderung mit Fundstelle; Liste der Unterbrechungsarten aus Prüfbericht 3.5 vorher und nachher gezählt; Grep-Gegenprobe; `pre-commit run --all-files` grün; ADR.
- **Betroffene Bestandteile:** Regelwerk

### S-24: Umbau U3 – Widersprüche auflösen

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] 2026-09-27 – PR [#46](https://github.com/Paddel87/Dev-Templates/pull/46) auf ausdrückliche Anweisung des Eigentümers ohne CI-Lauf gemergt (erster Anwendungsfall von `CLAUDE.md` Abschnitt 9, „Einzige Ausnahme"). Nachprüfung: S-17, Frist 2026-10-31.
- **Artefakte:** ADR-017, PR [#46](https://github.com/Paddel87/Dev-Templates/pull/46)
- **Verifikation (Stand Umsetzung, 2026-09-27) – Fundstelle je Befund:**
  - **W1:** `CLAUDE.md` §7 (Status `[CI-AUSSTEHEND]`) und §9 („Einzige Ausnahme"); zehn Schritte umgestellt, S-17 mit Frist 2026-10-31.
  - **W2:** `docs/project-context.md` §6 („Phasentyp bei Arbeit am Regelwerk"), Phasen-Kopf dieses Fahrplans, S-27 auf UMSETZUNG.
  - **W3:** `CLAUDE.md` §6 (Korrektur-Regel, 14 Tage) und §16 (Drift-Anker); Selbstausnahme in `docs/project-context.md` §6 aufgehoben; ADR-014 und ADR-016 rückwirkend `[REAKTIV]`. Quote 3/10.
  - **W4, W5:** `CLAUDE.md` §8 Kriterium 9 („Gezählt wird nach Inhalt"); Vorlage `templates/docs/fahrplan.md` Feld „Ursprünglicher Schrittplan". Das laufende Bündel erfüllt die Regel bereits (4 Schritte, beim Anlegen festgehalten).
  - **W6:** Angaben in Commit d24e254. Linter, Formatter, Security-Regeln und 8 Tests in Commit 3d110d9, Type-Checker begründet nicht eingerichtet (ADR-017). Reifegrad des Regelwerks → S-26.
  - **W7:** Suche über die ganze Datei leer für `CLAUDE.md` (Dateiname ausgenommen) und `templates/projektstart.md`. Übrige Vorlagen ohne Treffer, außer `AGENTS.md` Zeile 4 („nicht Claude Code … OpenAI Codex"). Diese Zeile bleibt: Sie erklärt als Einstiegsdatei den Zweck für andere Werkzeuge und fällt nicht unter die Regel.
  - **W8:** Commit d24e254.
  - `pre-commit run --all-files` grün (4 Hooks), auch in einem frischen Worktree; erzwungene Fehler für Linter und Test belegt.
- **Freigabe:** 2026-09-27, Teilfragen 1A, 2A, 3B, 4A, 5A, 6A (alle wie empfohlen) → ADR-017.
- **Stand der reinen Korrekturen (2026-09-27):**
  - **W6 Angaben:** `docs/project-context.md` Abschnitte 1, 3, 5, 6 und 7 nennen jetzt die Entwicklungswerkzeuge und das Python-Skript. `docs/architecture.md` Abschnitt 10 führt das Skript mit Schwellen-Prüfung, Abschnitt 9 mit Reifegrad VORLÄUFIG. Der Skript-Kopf hat jetzt eine Idempotenz- und eine Reproduzierbarkeits-Aussage statt „nicht anwendbar". Die `.pyc`-Datei war schon in 72dc87b entfernt.
  - **W6 Reifegrad des Regelwerks:** nicht hier, sondern in S-26 (U4). Ein Vermerk steht in `docs/architecture.md` Abschnitt 9.
  - **W8:** In `templates/projektstart.md` die Schrittnummern korrigiert (Architektur = Schritt 4, Stack = Schritt 2), dazu Abschnitt 2.1 an Schritt 4 angeglichen. In `CLAUDE.md` Abschnitt 16 den Pfad der README-Vorlage korrigiert. Feldname „Betroffene Bestandteile" als Namenskonvention in `docs/project-context.md` Abschnitt 4 festgehalten.
  - **W7 Suche über die ganze Datei (Nachholung):** `CLAUDE.md` 7 Treffer (Zeile 3 „Claude Code", Zeilen 134, 162, 214, 218, 253, 306 „Claude" als Handelnder). `templates/projektstart.md` Zeile 13 („derzeit Opus", „Claude Code", „200K Kontext") und zusätzlich Zeile 138 („Claude Code (1M Kontext)"), die der Prüfbericht nicht nennt. Die Bereinigung ist freigabepflichtig.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** teilweise – die reinen Korrekturen der Selbstanwendung und der Verweisfehler (W6-Angaben, W8) nein; Fertig-Status (W1), Phasentyp für Regelarbeit (W2), Reaktiv-Regel (W3), Bündel-Zählung (W4, W5) und Namen im Regelwerk (W7) ja.
- **Empfohlene Klasse:** Entscheidung für die freigabepflichtigen Teile; Routine für die Korrekturen.
- **Abhängigkeiten:** S-23
- **Zu tun:** Prüfbericht Abschnitt 5.2 Block U3 vollständig. Dazu: Neutralitäts-Suche künftig über die ganze Datei (W7), Linter und Werkzeug-Eintrag für das Python-Skript (W6).
- **Akzeptanzkriterien:** jeder Befund W1 bis W8 mit Erledigungs-Fundstelle oder begründetem Verzicht per ADR; `pre-commit run --all-files` grün.
- **Betroffene Bestandteile:** Regelwerk, Vorlagen, Selbstanwendung

### S-25: Umbau U1 – Regelwerk verschlanken und begrenzen

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] 2026-09-27 – PR [#47](https://github.com/Paddel87/Dev-Templates/pull/47) auf ausdrückliche Anweisung des Eigentümers ohne CI-Lauf abgeschlossen (`CLAUDE.md` §9, „Einzige Ausnahme"). Nachprüfung: S-17, Frist 2026-10-31.
- **Freigabe:** 2026-09-27, Teilfragen 1B (70.000 Byte; am selben Tag vor dem Merge gestrichen, siehe ADR-018), 2B, 3A, 4A, 5A, 6A wie empfohlen; Nebenfrage abweichend: Messung sofort per Test-Session; Einordnung `[REAKTIV]` bestätigt → ADR-018
- **Artefakte:** ADR-018, PR [#47](https://github.com/Paddel87/Dev-Templates/pull/47); Regel-Liste `docs/pruefberichte/2026-09-27-regelliste-s25.md` (+ `.txt` mit Suchbegriffen); neu `templates/wirtschaftlichkeit.md`, `templates/gate-deployment.md`, `docs/archiv/selbstanwendung/fahrplan-bis-buendel-34.md`
- **Verifikation (Stand Umsetzung, 2026-09-27):**
  - `CLAUDE.md` 81.519 → rund 69.700 Byte, 714 → 679 Zeilen. Die Grenze von 70.000 Byte ist gestrichen (ADR-018, Revision von Teilfrage 1); das Akzeptanzkriterium „unter der neuen Grenze" entfällt damit.
  - Regel-Liste 238 von 238 abgehakt; Wortvergleich jeder der 48 geänderten Regelzeilen (Logbuch).
  - Neutralitäts-Suche über die ganze Datei leer für `CLAUDE.md`, `templates/projektstart.md` und die zwei neuen Anhänge.
  - ANCHOR-Kommentare in `CLAUDE.md` vorher und nachher identisch.
  - `docs/fahrplan.md` 63.399 Byte vorher; nachher siehe Logbuch.
  - Kontextgröße nach der Mindest-Lektüre, je eine Test-Session mit gleichem Auftrag (Sitzungsabfrage `context_usage.used_tokens`, 2026-09-27): vorher (`main`) 134.726 Token, nachher (Branch) 133.615 Token – **−1.111 Token (−0,8 %)**. Kosten der Lesesitzung laut Sitzungsabfrage 1,30 $ → 1,02 $ (Cache-Schreiben 147.653 → 107.087 Token). Befund: Die Kontextgröße je Aufruf ändert sich kaum. Der feste Systemkontext (Werkzeug-Definitionen, rund 85.000 Token) überwiegt, und die Lesetiefe schwankt zwischen den Läufen. Je Lauf gibt es nur eine Messung.
  - `pre-commit run --all-files` grün.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Größengrenze für `CLAUDE.md`, Auslagerung des Kostenteils aus Abschnitt 0, Archivierungs-Auslöser für Klasse K
- **Empfohlene Klasse:** Entscheidung – Umbau des Regelwerks.
- **Abhängigkeiten:** S-24
- **Zu tun:** Prüfbericht Abschnitt 5.2 Block U1; je Regel höchstens ein Satz Begründung im Regeltext (Stellungnahme zu B4). **Vorher** eine Liste aller Regeln anlegen, **nachher** abhaken (Gegenmittel gegen stillen Regelverlust, Lehre aus ADR-007). Pflichtlektüre einer frischen Session vorher und nachher messen (Stellungnahme zu B2). **Rückmeldung aus einem zweiten Projekt (2026-09-26):** Der Eigentümer erprobt das Regelwerk in einem neuen Projekt; die Grenze der Sessiongröße von 200.000 Token (ADR-014) wird dort zunehmend lästig. Einzubeziehen hier und in S-23 (Unterbrechungen): Grenze als Setzung ohne Messung (Prüfbericht 3.6), Anteil der Pflichtlektüre an der Grenze (B2), Prüfzeitpunkt nach jedem Schritt.
- **Akzeptanzkriterien:** Regel-Liste vollständig abgehakt; gemessene Kontextgröße nach der Mindest-Lektüre vorher/nachher; `CLAUDE.md` unter der neuen Grenze; `pre-commit run --all-files` grün; ADR.
- **Betroffene Bestandteile:** Regelwerk, Vorlagen

### S-26: Umbau U4 – ehrlicher Reifegrad und Leitsatz

- **Status:** [OFFEN]
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Reifegrad-Rückstufung per ADR, Verankerung des Leitsatzes in `CLAUDE.md` §0
- **Empfohlene Klasse:** Entscheidung – Reifegrad-Änderung und Regeltext.
- **Abhängigkeiten:** S-25
- **Zu tun:** Prüfbericht Abschnitt 5.2 Block U4: neue Regelteile (Gate, vier Klassen, Sessiongröße, Wucherung, Anforderungsschicht) als eigene Bestandteile `[VORLÄUFIG]` in `docs/architecture.md`; Leitsatz aus #34 in Abschnitt 0; jede Regel, die Wachsamkeit beim Menschen ablegt, einzeln prüfen. Erprobung an einem zweiten Projekt der Klasse K oder M als Bedingung für `[BELASTBAR]` festhalten.
- **Akzeptanzkriterien:** Reifegrade mit ADR; Leitsatz-Fundstelle; Liste der geprüften Regeln aus Prüfbericht Abschnitt 4 (V8); `pre-commit run --all-files` grün.
- **Betroffene Bestandteile:** Regelwerk, Selbstanwendung

### S-27: Punkt 2 aus #34 – Betriebsmodus (Grundsatzentscheidung)

- **Status:** [OFFEN] – Landeplatz für Punkt 2 aus Issue #34; beginnt erst nach Abschluss des Bündels „Umbau nach Prüfbericht" (ADR-015)
- **Phasentyp-Kontext:** UMSETZUNG – neuer Regelbereich (ADR-017)
- **Freigabepflichtig:** ja – ändert das Phasenmodell
- **Empfohlene Klasse:** Entscheidung – Grundsatzentscheidung am Regelwerk.
- **Abhängigkeiten:** S-26
- **Zu tun:** Betriebsmodus nach dem ersten produktiven Deployment als vierter Phasentyp oder parallele Spur entscheiden (#34 Punkt 2). Nicht Teil des Umbau-Bündels.
- **Betroffene Bestandteile:** Regelwerk, Vorlagen

---

<!-- ANCHOR:iterations-reflexion -->
## Iterations-Reflexion

### Reflexion nach Überschreiten der Reaktiv-Quote (2026-09-27)

- **Auslöser:** ADR-018 ist `[REAKTIV]` (verschiebt Regeltext aus ADR-009, -011, -014, -016, alle jünger als 14 Tage). Quote 4/10 = 40 % über ADR-009 bis ADR-018, Schwelle 30 % (`CLAUDE.md` §0 Auslöser 5; Modell war bereits Entscheidungs-Klasse).
- **Befund:** Alle vier reaktiven ADRs der letzten zehn (014, 016, 017, 018) korrigieren Regeln aus dem Bündel #34 vom 2026-09-24. Die Quote misst damit genau, was der Prüfbericht festgestellt hat: Sieben Grundsatzentscheidungen an einem Tag waren zu schnell, ihre Form trug nicht.
- **Methodik-Refactoring nötig?** Es läuft bereits: das Bündel „Umbau nach Prüfbericht" (ADR-015). Ein zusätzliches Refactoring wäre dieselbe Arbeit noch einmal.
- **Folgerung:** Keine neue Regel bis zum Abschluss des Bündels (unverändert, ADR-015). S-26 ändert Reifegrade und verankert den Leitsatz; ein weiterer Korrektur-ADR innerhalb von 14 Tagen nach ADR-009 bis ADR-018 hielte die Quote über der Schwelle. Die Quote sinkt erst, wenn nicht-reaktive ADRs dazukommen oder die reaktiven aus dem Zehner-Fenster fallen. S-27 (UMSETZUNG, neuer Regelbereich) ist nicht reaktiv.
- **Beobachtung:** Der Eigentümer folgte in sechs von sieben Punkten der Empfehlung, bei der Nebenfrage nicht. Das ist ein Datenpunkt für S-26.

### Reflexion nach S-1 bis S-3 (2026-08-13)

- **Gelernt:** Zwei der drei Schritte dieser Session waren Nachweis-Arbeit an bereits *beschlossenen*, aber nicht *umgesetzten* Regeln (ANCHOR-Konvention, Selbstanwendung). Die Methodik neigt dazu, Regeln schneller zu formulieren als umzusetzen.
- **Kippende Annahmen:** Die Annahme, `docs/` sei ein sinnvoller Ort für die Vorlagen, hielt genau so lange, bis das Repo zum ersten Mal einen eigenen Wert eintragen sollte.
- **Reifegrad-Änderungen:** Selbstanwendung neu als `[VORLÄUFIG]`.
- **Neu erkannte Bedarfe:** S-4 und S-5 – ohne Linter und CI bleibt jede Konvention davon abhängig, dass sie jemand von Hand prüft. Genau daran ist die ANCHOR-Konvention gescheitert.
- **ADRs aus diesem Abschnitt:** ADR-001

<!-- ANCHOR:parallelisierbarkeit -->
## Parallelisierbarkeit

- **Ohne Abhängigkeiten zueinander:** S-7, S-18 und S-28. Das Umbau-Bündel (S-25, S-26) und S-27 laufen nacheinander.

<!-- ANCHOR:replanning-historie -->
## Replanning-Historie

- 2026-08-13 – Fahrplan erstmals angelegt im Zuge der Selbstanwendung (ADR-001). Die Schritte S-1 und S-2 sind rückwirkend erfasst, weil sie vor Einführung dieses Dokuments abgeschlossen wurden; ihre Artefakte sind über die PR-Nummern belegt.

<!-- ANCHOR:archiv-abgeschlossene-phasen -->
## Archiv / abgeschlossene Phasen

- [`docs/archiv/selbstanwendung/fahrplan-bis-buendel-34.md`](fahrplan-bis-buendel-34.md) – ausgelagert am 2026-09-27 (ADR-018): S-1 bis S-6, S-8 bis S-16, S-19 bis S-22. Bilanz: Schritte vor dem ersten Bündel (Einführung der Selbstanwendung, Linter, CI, Lizenz, Modellwechsel-Stopp, Vision-Lücke, Teil C, README) und Bündel #34 (Sicherheit und Betrieb, stille Fehler, Modellklassen, roter Faden, Anforderungsschicht; abgeschlossen mit ADR-015, Pflichtfrage: gezielt umbauen).
