# Fahrplan-Archiv – bis Abschluss Bündel #34

<!-- Quell-Dokument: docs/fahrplan.md. Ausgelagert am 2026-09-27 nach CLAUDE.md Abschnitt 14
     (Klasse K: Abschluss eines Schritt-Bündels, ADR-018). Abgedeckter Zeitraum: 2026-08-13 bis 2026-09-24.
     Enthalten: die Schritte vor dem ersten Bündel (S-1 bis S-12 ohne S-7) und das am 2026-09-24
     mit ADR-015 abgeschlossene Bündel „Umsetzung von Issue #34" (S-13 bis S-16, S-19 bis S-22).
     Die [CI-AUSSTEHEND]-Schritte darunter führt S-17 namentlich auf; fällt einer bei der Nachprüfung
     auf [IN ARBEIT] zurück, wird er ins aktive Dokument zurückgeholt. -->

## Schritte

### S-1: Modellklassen-Disziplin verankern

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk, freigegeben durch den Eigentümer
- **Zu tun:** Regel ergänzen, die den Wechsel zwischen Modellklassen an objektiv prüfbare Auslöser bindet statt an Selbsteinschätzung des Modells.
- **Akzeptanzkriterien:** `CLAUDE.md` Abschnitt 0 enthält Eskalations- und Rückstufungs-Auslöser; `CLAUDE.md` bleibt modellneutral; die konkrete Zuordnung erfolgt projektspezifisch.
- **Betroffene Bestandteile:** Regelwerk, Vorlagen
- **Artefakte:** PR [#21](https://github.com/Paddel87/Dev-Templates/pull/21)

### S-2: ANCHOR-Konvention umsetzen

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** nein – Umsetzung einer bereits beschlossenen Konvention
- **Zu tun:** Die in `CLAUDE.md` Abschnitt 2 vorgeschriebenen Sprung-Anker tatsächlich setzen; die fehlende Namensregel ergänzen.
- **Akzeptanzkriterien:** Alle Hauptabschnitte der Pflicht-Dokumente tragen einen eindeutigen Anker; der Grep-nach-`offset`/`limit`-Workflow ist durchgespielt.
- **Betroffene Bestandteile:** Regelwerk, Vorlagen
- **Artefakte:** PR [#22](https://github.com/Paddel87/Dev-Templates/pull/22) – 42 Anker
- **Notizen:** Der Drift war gravierender als er aussah: Ohne Anker war die Vorschrift in ihrer Kernsituation nicht befolgbar, weil Abschnitt 2 gleichzeitig verbietet, einen abgebrochenen Read stillschweigend hinzunehmen.

### S-3: Selbstanwendung einführen

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Strukturänderung am Repo, freigegeben durch den Eigentümer (ADR-001)
- **Eingangskriterien:** ADR-001 liegt vor
- **Zu tun:** Vorlagen nach `templates/docs/` verschieben, Quellort-Referenzen korrigieren, `docs/` als ausgefüllten Arbeitsdokumenten-Satz dieses Repos anlegen, Modellklassen-Zuordnung eintragen.
- **Akzeptanzkriterien:** `templates/docs/` enthält die unbefüllten Vorlagen; `docs/` enthält die sechs Pflicht-Dokumente dieses Repos mit realen Werten; alle Referenzen lösen auf; `templates/projektstart.md` beschreibt den Kopier-Schritt.
- **Betroffene Bestandteile:** Vorlagen, Selbstanwendung, Regelwerk (nur zwei Quellort-Referenzen)
- **Reifegrad-Wirkung:** Bestandteil „Selbstanwendung" entsteht neu als `[VORLÄUFIG]`
- **Artefakte:** ADR-001, `docs/*.md`, `templates/docs/*.md`
- **Verifikation bei Abschluss:** PR [#23](https://github.com/Paddel87/Dev-Templates/pull/23) gemerged. 9/9 Vorlagen byte-identisch zu HEAD, 5/5 Inter-Pflicht-Drift-Anker grün, keine Link-Regression gegenüber dem Stand vor dem Umbau.

### S-4: Markdown-Linter einrichten

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – neue externe Abhängigkeit (`CLAUDE.md` Abschnitt 4, Kategorie 3), freigegeben durch den Eigentümer (Option A: `markdownlint-cli2` über das pre-commit-Framework)
- **Abhängigkeiten:** keine
- **Zu tun:** Einen Markdown-Linter auswählen und konfigurieren. `CLAUDE.md` Abschnitt 15 stuft die Kategorie „Linter" als anwendbar ein, sobald ein etabliertes Tool existiert – für Markdown ist das der Fall, eingerichtet ist bisher keiner.
- **Akzeptanzkriterien:** Linter läuft über alle `*.md`, ist als Pre-Commit-Gate durchgesetzt, meldet 0 Issues auf dem aktuellen Stand.
- **Betroffene Bestandteile:** Vorlagen, Selbstanwendung, Regelwerk (nur Formatierungs-Fixes: Sprachtags an Codeblöcken, Leerzeilen um Listen/Codeblöcke, zwei disambiguierte Vorlagen-Überschriften)
- **Artefakte:** `.markdownlint-cli2.jsonc`, `.pre-commit-config.yaml`, PR [#25](https://github.com/Paddel87/Dev-Templates/pull/25)
- **Notizen:** Erster echter Testfall der Modellklassen-Regel aus S-1 (Eskalations-Auslöser 1, Kategorie 3) – der Eigentümer hat den empfohlenen Modellwechsel aus Zeitgründen nicht vollzogen, siehe `[BEOBACHTUNG]` im Logbuch. Verworfen: direkte npm-Abhängigkeit (Node.js würde zur harten Voraussetzung für ein reines Doku-Repo) und `rumdl` (Reife nicht verifizierbar). Begründung in ADR-002.

### S-5: CI-Pipeline einrichten

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Build-/Deploy-Pipeline (`CLAUDE.md` Abschnitt 4, Kategorie 7), freigegeben durch den Eigentümer (Option A: nur der Linter als CI-Gate, Drift-Checks als eigener Schritt S-7 abgespalten)
- **Abhängigkeiten:** S-4 (erledigt)
- **Zu tun:** GitHub-Actions-Workflow anlegen, der den bestehenden pre-commit-Hook (`markdownlint-cli2`) als zweite, unabhängige Diagnoseschicht ausführt.
- **Akzeptanzkriterien:** Workflow läuft bei Push und Pull Request; rotes Gate blockiert den Merge.
- **Betroffene Bestandteile:** Selbstanwendung, Regelwerk (keine inhaltliche Änderung)
- **Artefakte:** `.github/workflows/ci.yml`, ADR-003, PR [#26](https://github.com/Paddel87/Dev-Templates/pull/26)
- **Verifikation bei Abschluss:** Lokal simuliert (frisches venv, `pip install pre-commit`, `pre-commit run --all-files`) – Hook installiert sich selbst isoliert, lief grün auf dem sauberen Bestand (Exit-Code 0) und rot bei einer absichtlich eingebauten Testdatei (Exit-Code 1, MD040/MD041 korrekt erkannt). Simuliert exakt, was `pre-commit/action` in CI tut.
- **Notizen:** `ENTSCHEIDUNG ERFORDERLICH`-Block deckte eine echte Lücke in diesem Schritt selbst auf: „Zu tun" nannte ursprünglich auch die Automatisierung der Inter-Pflicht-Drift-Checks aus Abschnitt 16, „Akzeptanzkriterien" forderte das nie. Auf Wunsch des Eigentümers abgespalten – siehe S-7. Nebenbefund beim Versions-Check: `templates/github-workflows/ci-minimal.yml` pinnt noch `actions/checkout@v4`, veraltet (aktuell v7.0.1) – nicht mitgefixt, da Vorlage für Ziel-Projekte, kein Bestandteil dieses Schritts. Siehe S-8.

### S-6: Lizenzfrage klären

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Lizenz- und Compliance-Entscheidung (`CLAUDE.md` Abschnitt 4, Kategorie 8), freigegeben durch den Eigentümer (Option B: CC0 1.0 Universal)
- **Zu tun:** Entscheiden, unter welcher Lizenz das Repo steht, und eine `LICENSE`-Datei anlegen.
- **Akzeptanzkriterien:** `LICENSE` existiert; der Verweis in `README.md` zeigt darauf; ADR hält die Wahl fest.
- **Betroffene Bestandteile:** Selbstanwendung (`LICENSE`), Regelwerk (keine inhaltliche Änderung)
- **Artefakte:** `LICENSE`, ADR-004, PR [#27](https://github.com/Paddel87/Dev-Templates/pull/27)
- **Verifikation bei Abschluss:** `LICENSE`-Volltext live von `creativecommons.org` bezogen (121 Zeilen, per Checksumme reproduzierbar), nicht aus dem Trainingsstand rekonstruiert. Abhängigkeitslizenzen (`markdownlint-cli2`, `pre-commit`, beide MIT) live gegen `package.json`/`setup.cfg` verifiziert und mit CC0 kompatibel.
- **Notizen:** Korrektur zur ursprünglichen Formulierung dieses Schritts: „größte Außenwirkung unter den offenen Schritten" war überzogen. Das Repo ist per GitHub-API bestätigt **privat** – es gab nie akute Nutzungsunsicherheit für Dritte, nur eine Inkonsistenz zwischen README-Versprechen und Realität. Verworfen: MIT (reale Grauzone bei kopierten Vorlagen-Inhalten – müsste jedes abgeleitete Projekt einen Copyright-Vermerk mitführen?) und Apache-2.0 (Patent-Klausel gegenstandslos für ein Dokumentations-Repo). Begründung in ADR-004.

### S-8: Aktions-Versionen in Vorlagen-Workflows aktualisieren

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** nein – Versions-Update ohne Breaking Change am Vorlagen-Inhalt selbst (Freigabepflicht nur bei Major-Updates mit Breaking Changes, hier reine Aktualisierung gepinnter Actions). Keiner der sechs Eskalations-Auslöser aus `CLAUDE.md` Abschnitt 0 griff, daher ohne Modellwechsel-Hinweis auf der Routine-Klasse bearbeitet.
- **Zu tun:** Nebenbefund aus S-5: `templates/github-workflows/ci-minimal.yml` (und ggf. `ci-python.yml`, `ci-typescript.yml`) pinnen `actions/checkout@v4`, mittlerweile veraltet. Aktuelle Versionen live verifizieren und nachziehen.
- **Akzeptanzkriterien:** Alle Action-Versionen in `templates/github-workflows/*.yml` gegen die jeweils aktuellen GitHub-Releases verifiziert und aktualisiert.
- **Betroffene Bestandteile:** Vorlagen (nur `templates/github-workflows/*.yml`)
- **Artefakte:** aktualisierte `ci-minimal.yml`, `ci-python.yml`, `ci-typescript.yml`, PR [#28](https://github.com/Paddel87/Dev-Templates/pull/28)
- **Verifikation bei Abschluss:** Fünf Actions live gegen GitHub-Releases verifiziert (`actions/checkout` v4→v7.0.1, `actions/setup-python` v5→v7.0.0, `actions/upload-artifact` v4→v7.0.1, `actions/setup-node` v4→v7.0.0, `pnpm/action-setup` v4→v6.0.10). Verwendete `with:`-Parameter (`node-version`, `cache`, `version`, `name`, `path`, `retention-days`) gegen die aktuellen `action.yml`-Definitionen auf Kompatibilität geprüft, keine Breaking Changes gefunden. YAML-Syntax aller drei Dateien validiert, Diff enthält ausschließlich `uses:`-Zeilen.
- **Notizen:** Betrifft nur `templates/`, nicht die Selbstanwendung dieses Repos. Fund am Rande, nicht umgesetzt: Für `pnpm/action-setup` gibt es ab pnpm v11 einen Nachfolger (`pnpm/setup`) – das wäre ein Werkzeugwechsel, keine reine Versions-Aktualisierung, damit außerhalb des Scopes dieses Schritts. `pnpm/action-setup` v6 ist weiterhin aktiv gepflegt.

### S-9: Modellwechsel-Eskalation zum echten Stopp umbauen

- **Status:** [ERLEDIGT] 2026-08-13
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk, ausdrücklich angefordert durch den Eigentümer
- **Abhängigkeiten:** S-1 (erledigt) – korrigiert die dort eingeführte Regel
- **Zu tun:** Die Eskalations-Empfehlung aus S-1 wirkte im Betrieb nicht: Die vorgeschriebene `Ohne Wechsel`-Zeile ließ die KI sofort weiterarbeiten, sodass der Mensch den Entscheidungspunkt praktisch nie nutzen konnte. Umbau zu einem blockierenden Stopp.
- **Akzeptanzkriterien:** Der Block hält die Arbeit tatsächlich an; beim Wiederanlauf benennt die KI die aktive Modellklasse, bevor sie fortfährt; die Grenzen der Modell-Erkennung sind im Regelwerk offen dokumentiert.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` Abschnitt 0), Selbstanwendung (ADR-005, Regel-001)
- **Artefakte:** ADR-005, Regel-001 (erster Eintrag in `decisions.md` Teil C)
- **Verifikation bei Abschluss:** Voraussetzung der Regel vorab geprüft statt angenommen – in einem Test mit zwei aufeinanderfolgenden Modellwechseln (Entscheidungs-Klasse → Routine-Klasse → Entscheidungs-Klasse) wurden beide korrekt erkannt und benannt. Ohne diese Erkennung wäre ein Stopp mit Bestätigungs-Pflicht nicht umsetzbar gewesen.
- **Notizen:** Erster `[REAKTIV]`-ADR des Projekts (ADR-005) – Auslöser war ein Fehlschlag im Betrieb, nicht geplante Weiterentwicklung. Damit steigt die Reaktiv-Quote von 0/4 auf 1/5 (20 %), weiterhin unter dem Schwellenwert von 30 %. Der Schritt erzeugte außerdem den ersten Eintrag in `decisions.md` Teil C, weil sich aus dem Fehlschlag ein verallgemeinerbares Muster ergab.

### S-10: Vision-Verlust-Lücke schließen (A1 aus dem Pilotprojekt)

- **Status:** [ERLEDIGT] 2026-08-28
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk (drei Abschnitte), ausdrücklich angefordert durch den Eigentümer. Eskalations-Auslöser 1 und 3 aus `CLAUDE.md` Abschnitt 0 griffen; die Session lief auf der Entscheidungs-Klasse, kein Modellwechsel nötig.
- **Abhängigkeiten:** keine
- **Zu tun:** Die drei Regelbausteine A1.1–A1.3 aus dem Vision-Gap-Postmortem des Pilotprojekts EB-Digital in die Vorlage übernehmen. Sie sind dort seit 2026-06-25 produktiv, waren aber nie zurückgeflossen: Der Befund kam als Sammel-Issue [#19](https://github.com/Paddel87/Dev-Templates/issues/19) und wurde dadurch nie einzeln abgearbeitet.
- **Akzeptanzkriterien:** `CLAUDE.md` Abschnitt 6 enthält die harte Regel „Keine Verschiebung ohne Landeplatz"; Abschnitt 7 kennt `[VERSCHOBEN]` mit Pflicht zur Ziel-Schritt-ID; Abschnitt 12 enthält die beiden Vision-Checkpoints (Phasengrenze, Go-Live); alle Stellen, die `vision.md` als „einmalig gelesen" führen, sind auf die neue Ausnahme angepasst; die Schritt-Format-Vorlage kennt den Marker und das Landeplatz-Feld.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` Abschnitte 2, 3, 6, 7, 12), Vorlagen (`templates/docs/fahrplan.md`)
- **Artefakte:** ADR-006, PR [#32](https://github.com/Paddel87/Dev-Templates/pull/32)
- **Verifikation bei Abschluss:** siehe S-11 (gemeinsamer Verifikationslauf) – markdownlint grün, Marker-Aufzählungen in Regelwerk und Vorlage deckungsgleich, keine verbliebene Stelle, die `vision.md` ausnahmslos als „nur zu Projektstart gelesen" führt.
- **Notizen:** Bewusst **nicht** mitgenommen: ein Vision-Konsistenz-Anker in der Drift-Tabelle aus Abschnitt 16. Diese Tabelle prüft Pflicht-Dokumente gegeneinander; ein Vision-Eintrag dort würde genau das Spiegel-Dokument-Muster einführen, gegen das der Checkpoint schützt. Ebenfalls nicht übernommen: die projektspezifische Traceability-Tabelle (A1.4) als Vorlagen-Bestandteil – sie ist in Abschnitt 12 nur als *zulässiges abgeleitetes Werkzeug* erwähnt, nicht als Pflicht, weil sie sonst zur autoritativen Quelle würde.

### S-11: Teil-C-Lücke in der Pflichtlektüre schließen

- **Status:** [ERLEDIGT] 2026-08-28
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung an der Mindest-Lektüre, ausdrücklich angefordert durch den Eigentümer
- **Abhängigkeiten:** keine
- **Zu tun:** `CLAUDE.md` Abschnitt 2 Punkt 5 nennt Teil A (lesen) und schließt Teil B aus – **Teil C kommt im Satz gar nicht vor**. Verbindliche Entscheidungsregeln fielen dadurch strukturell durch die Pflichtlektüre. Im Pilotprojekt betraf das 36 Regeln, in diesem Repo seit ADR-005 eine (`Regel-001`).
- **Akzeptanzkriterien:** Abschnitt 2 Punkt 5 nennt Teil C ausdrücklich als Mindest-Lektüre und grenzt ihn gegen Teil B ab; Abschnitt 14 nimmt Teil C von der ADR-Auslagerung aus; die `decisions.md`-Vorlage trägt die Lesepflicht an **beiden** Stellen, an denen sie die Lektüre beschreibt (Kopfkommentar und Teil-C-Überschrift).
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` Abschnitt 2), Vorlagen (`templates/docs/decisions.md`)
- **Artefakte:** ADR-007, PR [#32](https://github.com/Paddel87/Dev-Templates/pull/32)
- **Verifikation bei Abschluss:** `pre-commit run --all-files` grün (markdownlint-cli2, 0 Findings). Gegenprobe an diesem Repo: `Regel-001` ist der einzige Teil-C-Eintrag und war in der Pflichtlektüre dieser Session nach altem Wortlaut nicht enthalten – nach neuem Wortlaut ist er es. Marker-Sets `CLAUDE.md` Abschnitt 7 ↔ `templates/docs/fahrplan.md` Schritt-Format abgeglichen (7/7 deckungsgleich).
- **Notizen:** Der Fehler ist älter als jede Regel, die er betrifft: Teil C existierte in der Vorlage von Anfang an, die Lese-Anweisung hat ihn nie erfasst. Aufgefallen ist er erst, als das Pilotprojekt genug Regeln angesammelt hatte, dass ihr Fehlen im Alltag weh tat – dieselbe Sichtbarkeits-Latenz wie bei der ANCHOR-Konvention in S-2.

  Bei der Verifikation kam eine **zweite Fundstelle** dazu, die im ursprünglichen Auftrag nicht benannt war: Der Kopfkommentar von `templates/docs/decisions.md` wiederholte denselben Fehler wörtlich („Bei Sessionstart liest Claude nur Teil A"). Sie fiel nur auf, weil die Datei beim Aufteilen der Commits einmal auf den Ausgangsstand zurückgesetzt und dabei ganz gelesen wurde. Eine Regel, die an zwei Stellen steht, muss an beiden korrigiert werden – sonst bleibt die zweite als Widerspruch stehen und gewinnt möglicherweise, weil sie näher am Arbeitsort steht.

### S-12: README-Abschnitt „Für wen es ist" schärfen

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-23
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** nein – Formulierungsarbeit an `README.md` und `docs/project-context.md` Abschnitt 2 ohne Regelwirkung; keine der acht Kategorien aus `CLAUDE.md` Abschnitt 4 berührt. Ausdrücklich angefordert durch den Eigentümer.
- **Abhängigkeiten:** keine
- **Zu tun:** Die Zielgruppen-Beschreibung von einer allgemeinen Aufzählung („Einzelpersonen und kleine Teams", „Nicht-Programmierer, die Wert auf Methodik legen", „Klassen K–V") auf die tatsächliche Zielgruppe schärfen: Menschen mit Produktidee und Fachwissen, ohne Programmierkenntnisse und ohne eingebaute Projektdisziplin. Die beiden Schwächen benennen, die das Regelwerk ausgleicht; den Erprobungsstand ehrlich angeben; benennen, für wen es weniger geeignet ist.
- **Akzeptanzkriterien:** Abschnitt nennt die Zielgruppe konkret; jede Aussage über den Erprobungsstand ist belegbar; keine Regel wird als geltend dargestellt, die nicht in `CLAUDE.md` steht (offene Vorschläge nur mit Verweis auf Issue #34); `docs/project-context.md` Abschnitt 2 stimmt inhaltlich mit dem README-Abschnitt überein; `pre-commit run --all-files` grün.
- **Betroffene Bestandteile:** Selbstanwendung (`docs/project-context.md` Abschnitt 2); zusätzlich `README.md`
- **Artefakte:** `README.md`, `docs/project-context.md`, PR [#35](https://github.com/Paddel87/Dev-Templates/pull/35)
- **Verifikation bei Abschluss:** `pre-commit run --all-files` grün (markdownlint-cli2). Aussagen zum Erprobungsstand gegen das Pilot-Repo geprüft (erster Commit dort 2026-05-07, Klasse G laut dortigem ADR-001). Der Leitsatz „technische Wachsamkeit liegt bei der KI" steht bewusst als *Anspruch* mit Verweis auf Issue #34, nicht als geltende Regel – er ist in `CLAUDE.md` noch nicht verankert. Inter-Pflicht-Drift-Check 5/5 unverändert grün (kein neuer ADR, kein Reifegrad-Wechsel, kein Blocker).
- **CI-Ausnahme (Freigabe des Eigentümers, 2026-09-23):** Der DoD-Punkt „CI-Pipeline läuft grün" ist für diesen Schritt **nicht** erfüllt. Die CI-Läufe 29 und 30 (inkl. einmaliger Wiederholung) auf `a229ae0` endeten nach 3–6 s ohne zugewiesenen Runner (`runner_id: 0`, keine Schritte, keine Logs). Ursache nach Angabe des Eigentümers und ADR-213 im Pilotprojekt: erschöpftes Actions-Kontingent des Kontos; das Pilotprojekt läuft deshalb auf einem eigenen, nur dort registrierten Runner. Entscheidung: **Option A** – Warten auf den Kontingent-Reset, keine Änderung an der Pipeline, daher kein ADR. PR #35 wird als einmalige Ausnahme ohne grüne CI gemergt; der Prüfumfang der CI (markdownlint-cli2) ist lokal über beide Commits nachgewiesen. Nach dem Reset prüft der nächste CI-Lauf auf `main` den Stand nachträglich.
- **Notizen:** Der persönliche Entstehungshintergrund des Eigentümers (zwei Jahre gescheiterter KI-Projekte ohne Regelwerk) ist bewusst **nicht** in die README übernommen, sondern nur verallgemeinert („an denen solche Projekte typischerweise scheitern"). Aufnahme nur auf ausdrücklichen Wunsch.

### S-13: Paket 1 aus #34 – Sicherheit und Betrieb, Stufe 1 (Regeln 9 und 13)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option B (zwei Stufen). Diese Stufe umfasst nur die Punkte 9 und 13; die Punkte 5, 8 und 11 sind nach **S-14** verschoben.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk (`CLAUDE.md`) und an den Vorlagen. Eskalations-Auslöser 1 aus `CLAUDE.md` Abschnitt 0; die Session läuft auf der Opus-Linie (Entscheidungs-Klasse laut Abschnitt 6 von `project-context.md`).
- **Abhängigkeiten:** keine. Triage und Reihenfolge durch den Eigentümer am 2026-09-24 festgelegt ([#34, Triage-Kommentar](https://github.com/Paddel87/Dev-Templates/issues/34#issuecomment-5812135202)).
- **Zu tun:** Die fünf Punkte der Stufe 1/2 aus #34, die das Paket „Sicherheit und Betrieb" bilden, in Regelwerk und Vorlagen übernehmen. Jeder Punkt ist im Pilotprojekt EB-Digital belegt:
  - **Punkt 9 – Secrets in der Ausgabe des Agenten:** `CLAUDE.md` §6 „Secrets niemals im Code oder Log" erweitern um die Terminal-Ausgabe und das Gesprächsprotokoll des Agenten. Der Agent prüft Secrets nur auf Vorhandensein; ein ausgegebenes Secret gilt als kompromittiert und bekommt einen Rotations-Schritt mit Frist.
  - **Punkt 13 – Belegen durch erzwungenen Fehler:** neue harte Regel in §6. Ein Schutzmechanismus (Alarm, Backup, Überwachung, Gate) wird erst `[BELASTBAR]`, wenn ein absichtlich herbeigeführter Fehler vollständig durchlief.
  - **Punkt 5 – Sicherheitsgrundriss und Deploy-Gate:** `templates/projektstart.md` Modus 2 um einen Pflicht-Sicherheitsgrundriss ergänzen (Bedrohungsmodell, Sicherheitsniveau, Rubriken Host/Netz/Secrets/Backups). `templates/docs/architecture.md` um diese Rubriken ergänzen. `templates/docs/fahrplan.md`: Security-Review aus dem STABILISIERUNG-Akzeptanzformat vor das erste Deployment ziehen. Neues **Gate vor dem ersten öffentlichen Deployment** in `CLAUDE.md` §12, analog zum Go-Live-Gate.
  - **Punkt 8 – Unabhängige Prüfung:** Teil des Gates: sicherheits- und datenschutzrelevante Änderungen werden von einer getrennten Instanz geprüft (eigene Session oder anderes Modell); vor Go-Live externer Blick oder dokumentiertes Restrisiko mit ADR.
  - **Punkt 11 samt Ergänzung – Betreiber, KI-Abhängigkeit, Kontingent:** Teil des Gates: benannte Vertretung oder dokumentierter Verzicht; Notfall-Handbuch ohne KI; bei unbeaufsichtigtem Handeln der KI feste Befehlsliste ohne löschende oder Secret-lesende Befehle und Probelauf; Konto und Zurücksetz-Zeitpunkt des Kontingents in `templates/docs/project-context.md`; kritische Termine gegen das Kontingent-Fenster planen.
- **Akzeptanzkriterien:** Jeder der fünf Punkte hat eine konkrete Fundstelle in `CLAUDE.md` oder `templates/`; das neue Gate steht in §12 neben dem Go-Live-Gate und verweist auf die Punkte; keine neue Regel beruht auf ungeprüfter Technik (kein Hook); Grep-Gegenprobe über die berührten Begriffe (Lehre aus S-10); `pre-commit run --all-files` grün; ADR angelegt.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §6, §12), Vorlagen (`templates/projektstart.md`, `templates/docs/architecture.md`, `templates/docs/fahrplan.md`, `templates/docs/project-context.md`)
- **Artefakte:** ADR-008, PR [#36](https://github.com/Paddel87/Dev-Templates/pull/36)
- **Verifikation bei Abschluss:** Die Akzeptanzkriterien gelten für diese Stufe nur für die Punkte 9 und 13 (Freigabe Option B); die Punkte 5, 8 und 11 samt Gate in §12 sind S-14 zugeordnet. Fundstellen: `CLAUDE.md` §6 (zwei neue harte Regeln), `templates/docs/architecture.md` (Ausnahme in der Beförderungsregel). Kein Hook, keine neue Technik. Grep-Gegenprobe im Logbuch 2026-09-24 12:35. `pre-commit run --all-files` grün (markdownlint-cli2, Hook installiert). Inter-Pflicht-Drift-Check 5/5 grün: ADR-008 verweist auf S-13 und S-14, beide existieren; Reaktiv-Quote 1/8 stimmt mit Teil B überein; kein Reifegrad-Wechsel, kein Blocker.
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist für diesen Schritt **nicht** erfüllt – aus demselben Grund wie bei S-12 (erschöpftes Actions-Kontingent, kein Runner). PR #36 wird als zweite dokumentierte Ausnahme ohne grüne CI gemergt; der Prüfumfang der CI ist lokal nachgewiesen. Nach dem Reset prüft der nächste CI-Lauf auf `main` den Stand nachträglich.
- **Notizen:** Die CI dieses Repos bekommt bis zum Reset des Actions-Kontingents keinen Runner (S-12, PR #35). Ein PR vor dem Reset braucht dieselbe dokumentierte Ausnahme oder wartet.

### S-14: Paket 1 aus #34 – Sicherheit und Betrieb, Stufe 2 (Gate vor dem ersten öffentlichen Deployment)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option A (einheitliches, blockierendes Gate); Nebenfragen mit Voreinstellung: DoD-Zeile für die unabhängige Prüfung, ein PR.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – im Grundsatz freigegeben mit Option B zu S-13; die ausgearbeitete Fassung wurde am 2026-09-24 als Entwurf vorgelegt (Logbuch 14:00) und mit Option A freigegeben.
- **Abhängigkeiten:** S-13
- **Zu tun:** Die Punkte 5, 8 und 11 (samt Kontingent-Ergänzung) aus #34 umsetzen, wie in S-13 „Zu tun" beschrieben: Sicherheitsgrundriss in Modus 2, Rubriken in der Architektur-Vorlage, Security-Review vor das erste Deployment, unabhängige Prüfung, Vertretung und Notfall-Handbuch, Befehlsliste und Probelauf bei unbeaufsichtigtem Handeln der KI, Konto und Zurücksetz-Zeitpunkt des Kontingents – gebündelt als neues Gate in `CLAUDE.md` §12.
- **Akzeptanzkriterien:** wie S-13 für die Punkte 5, 8, 11; das Gate verweist auf die Regel aus ADR-008 zum erzwungenen Fehlerfall.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §3, §9, §12), Vorlagen (`templates/projektstart.md`, `templates/docs/architecture.md`, `templates/docs/fahrplan.md`, `templates/docs/project-context.md`, `templates/docs/onboarding-runbook.md`); zusätzlich `README.md`
- **Artefakte:** ADR-009, PR [#37](https://github.com/Paddel87/Dev-Templates/pull/37)
- **Verifikation (Stand Umsetzung):** Die acht Prüfpunkte decken die Punkte 5, 8, 11 samt Ergänzung und den bisher landeplatzlosen dritten Vorschlag aus Punkt 9 ab; Prüfpunkt 5 und 8 verweisen auf die Regel aus ADR-008. Kein Hook, keine neue Technik. Grep-Gegenprobe im Logbuch 2026-09-24 14:40. `pre-commit run --all-files` grün. Inter-Pflicht-Drift-Check 5/5 grün: ADR-009 verweist auf S-14, der existiert; Reaktiv-Quote 1/9 stimmt mit Teil B überein; kein Reifegrad-Wechsel, kein Blocker. Nicht erprobt: das Gate an einem echten Projekt (Konfidenz mittel, ADR-009).
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist **nicht** erfüllt – derselbe Grund wie bei S-12 und S-13 (erschöpftes Actions-Kontingent, kein Runner). PR #37 wird als dritte dokumentierte Ausnahme ohne grüne CI gemergt; der Prüfumfang der CI ist lokal nachgewiesen. Nach dem Reset prüft der nächste CI-Lauf auf `main` den Stand von #35, #36 und #37 nachträglich.

### S-15: Paket 2 aus #34 – Stille Fehler, Stufe 1 (Punkte 7, 10, 14)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option B (zwei Stufen). Diese Stufe umfasst die Punkte 7, 10 und 14; Punkt 15 ist nach **S-16** verschoben.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk (`CLAUDE.md` §0, §9, §12) und an den Vorlagen, Eskalations-Auslöser 1 aus `CLAUDE.md` Abschnitt 0.
- **Abhängigkeiten:** keine fachliche; Paket 1 (S-13, S-14) ist abgeschlossen. Reihenfolge laut Triage des Eigentümers ([#34](https://github.com/Paddel87/Dev-Templates/issues/34#issuecomment-5812135202)).
- **Zu tun:** Die vier Punkte des Pakets „Stille Fehler" in Regelwerk und Vorlagen übernehmen. Jeder Punkt ist im Pilotprojekt EB-Digital belegt:
  - **Punkt 7 – Warnungen im CI-Protokoll:** Warnungen färben den Lauf rot, wo die Werkzeuge es können (Defaults in `templates/github-workflows/` und `templates/pre-commit/`). Wo nicht: festgehaltener Bestand mit Obergrenze, die nur sinken darf – ohne eigenes Zählskript, über die eingebauten Mittel der Werkzeuge (z. B. Obergrenze für Linter-Warnungen, benannte Ausnahmeliste für Test-Warnungen, jede Ausnahme mit Fahrplan-Schritt). Jede Abkündigung wird ein Fahrplan-Schritt mit Frist. `CLAUDE.md` §9: „CI grün" heißt grün ohne neue Warnungen über der Obergrenze.
  - **Punkt 10 – Ablaufdaten-Register:** neuer Abschnitt in `templates/docs/project-context.md` (Was, Ablaufdatum, Vorlauf, Fahrplan-Schritt); Prüfung beim Sessionende in `CLAUDE.md` §12. Auch für Dev-Templates selbst anlegen (z. B. monatlicher Reset des Actions-Kontingents).
  - **Punkt 14 – Versionswahl:** `templates/projektstart.md` Schritt 2a umkehren – die KI belegt jede Version mit Quellen, der Mensch bestätigt die Tabelle. Auswahlregel „ausgereifte Linie" (Mindestreife als Zahl in `project-context.md`, tragende Abhängigkeiten unterstützen die Linie, Unterstützungsfenster reicht über die Projektdauer). Jeder `Verifiziert`-Stempel bekommt einen Eintrag im Ablaufdaten-Register.
  - **Punkt 15 samt Ergänzung – Modellwahl nach Aufgabe:** In `CLAUDE.md` §0 vier Klassen statt zwei, weiterhin ohne Modellnamen (Mechanik, Routine, Entscheidung, Ausnahme); Zuordnung zu Modellen und Bezugsmodell (Abo oder API, knappe Ressource) in `project-context.md`. Jeder Fahrplan-Schritt trägt eine empfohlene Klasse. Die KI stellt das aktive Modell über die Laufzeitumgebung fest und vergleicht es je Schritt mit der Empfehlung. Routinearbeit über einen Unteragenten mit fest eingestelltem Modell, sofern das Werkzeug das kennt. Die Mechanik-Klasse wird erst nach einem Probelauf aktiv. Ein mechanischer Hook nur nach einem eigenen Erkundungsschritt. Die veraltete Modellzuordnung in `docs/project-context.md` Abschnitt 6 dieses Repos wird dabei nachgezogen.
- **Akzeptanzkriterien:** Jeder Punkt hat eine konkrete Fundstelle in `CLAUDE.md` oder `templates/`; `CLAUDE.md` bleibt frei von Modell- und Werkzeugnamen (Grep aus `docs/project-context.md` Abschnitt 6); keine Regel beruht auf ungeprüfter Technik; Grep-Gegenprobe über die berührten Begriffe; `pre-commit run --all-files` grün; ADR angelegt.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §0, §9, §12), Vorlagen (`templates/projektstart.md`, `templates/docs/project-context.md`, `templates/docs/fahrplan.md`, `templates/github-workflows/`, `templates/pre-commit/`); Selbstanwendung (`docs/project-context.md`)
- **Artefakte:** ADR-010, PR [#38](https://github.com/Paddel87/Dev-Templates/pull/38)
- **Verifikation (Stand Umsetzung):** Die Akzeptanzkriterien gelten für diese Stufe für die Punkte 7, 10 und 14. Fundstellen: `CLAUDE.md` §9 (Bedeutung von „grün"), §12 Punkt 8 (Ablaufdaten-Register), §15 („Warnungen und Abkündigungen", „Versionswahl"); `templates/projektstart.md` Schritt 2a; `templates/docs/project-context.md` §3, §7, §8; `templates/docs/fahrplan.md` (Feld „Frist"); CI- und Pre-Commit-Vorlagen. Kein Zählskript, nur eingebaute Mittel der Werkzeuge. Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md` leer. YAML der Vorlagen parst. Grep-Gegenprobe im Logbuch 2026-09-24 16:10. `pre-commit run --all-files` grün. Inter-Pflicht-Drift-Check 5/5 grün: ADR-010 verweist auf S-15 und S-16, beide existieren; Reaktiv-Quote 1/10 stimmt; kein Reifegrad-Wechsel, kein Blocker.
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist **nicht** erfüllt – derselbe Grund wie bei S-12 bis S-14. PR #38 wird als vierte dokumentierte Ausnahme ohne grüne CI gemergt; der Prüfumfang der CI ist lokal nachgewiesen. Nachträgliche Prüfung: S-17.
- **Notizen:** Die CI dieses Repos hat weiterhin keinen Runner (S-12). Ein PR vor dem Reset braucht dieselbe dokumentierte Ausnahme oder wartet.

### S-16: Paket 2 aus #34 – Stille Fehler, Stufe 2 (Punkt 15: Modellwahl nach Aufgabe)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option C (Abgabe an Unteragenten, sonst Warnung und Bilanz); Nebenfrage mit Voreinstellung: Probelauf als Teil von S-16.
- **Empfohlene Klasse:** Entscheidung – Änderung an `CLAUDE.md` §0 (`[STRATEGISCH]`-ADR).
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – im Grundsatz freigegeben mit Option B zu S-15 (2026-09-24). Der ausgearbeitete Entwurf wird vor der Umsetzung vorgelegt, zusammen mit der offenen Frage, ob die KI bei Arbeit oberhalb der empfohlenen Klasse anhält oder nur warnt (`Regel-001`).
- **Abhängigkeiten:** S-15
- **Zu tun:** Punkt 15 samt Ergänzung „Modell erkennen und vor Fehlnutzung warnen" aus #34 umsetzen, wie in S-15 „Zu tun" beschrieben: vier Klassen in `CLAUDE.md` §0 ohne Modellnamen, Zuordnung und Bezugsmodell in `project-context.md`, empfohlene Klasse je Fahrplan-Schritt, Abgleich je Schritt, Unteragent für Routinearbeit, Probelauf vor Aktivierung der Mechanik-Klasse, Hook nur nach eigenem Erkundungsschritt. Veraltete Modellzuordnung in `docs/project-context.md` Abschnitt 6 dieses Repos nachziehen.
- **Akzeptanzkriterien:** wie S-15 für Punkt 15; `CLAUDE.md` bleibt frei von Modellnamen.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §0, §2, §12), Vorlagen (`templates/docs/project-context.md`, `templates/docs/fahrplan.md`, `templates/docs/logbuch.md`), Selbstanwendung (`docs/project-context.md`, `docs/decisions.md` Regel-001); zusätzlich `README.md`
- **Artefakte:** ADR-011, PR [#39](https://github.com/Paddel87/Dev-Templates/pull/39)
- **Verifikation (Stand Umsetzung):** Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md` leer. Probelauf mit erzwungenem Fehler (Kopie des Repos, zwei eingebaute Drift-Fehler: Reaktiv-Quote 2/10 statt 1/10, ADR-Verweis auf S-96 statt S-16): Routine-Klasse (Sonnet 5) fand beide mit Beleg, Mechanik-Klasse (Haiku 4.5) fand beide beim Zählen; Schreibprobe der Routine-Klasse (README-Eintrag zu #38) korrekt, aber lückenhaft. Ergebnisse in `docs/project-context.md` Abschnitt 6, Details im Logbuch 2026-09-24 17:40. Ersparnis der Abgabe nicht belegt → S-19. `pre-commit run --all-files` grün. Inter-Pflicht-Drift-Check 5/5 grün: ADR-011 verweist auf S-16 und S-19, beide existieren; Reaktiv-Quote 1/11 stimmt.
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist **nicht** erfüllt – derselbe Grund wie bei S-12 bis S-15. PR #39 wird als fünfte dokumentierte Ausnahme ohne grüne CI gemergt. Nachträgliche Prüfung: S-17.

### S-19: Erkundung – Ersparnis der Abgabe messen und Hook-Angaben prüfen

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – Befunde im Logbuch 20:50 und in `docs/project-context.md` Abschnitt 6; PR [#42](https://github.com/Paddel87/Dev-Templates/pull/42) als achte dokumentierte CI-Ausnahme gemergt (Anweisung des Eigentümers; Nachprüfung S-17)
- **Phasentyp-Kontext:** ERKUNDUNG innerhalb der STABILISIERUNG (Spike)
- **Schritt-Art:** Spike; Zeitbox: eine Session
- **Freigabepflichtig:** nein für die Messung; eine daraus folgende Regeländerung ist es.
- **Empfohlene Klasse:** Routine – Messen und Dokumentieren; eine Regeländerung danach läuft über die Entscheidungs-Klasse.
- **Abhängigkeiten:** S-16
- **Zu tun:** (1) An einer typischen Routineaufgabe messen, ob die Abgabe an einen Unteragenten gegenüber der Arbeit im geladenen Kontext der Entscheidungs-Klasse Kontingent spart – mit den Zählwerten, die die Laufzeitumgebung liefert, getrennt nach Cache-Anteil, falls verfügbar. (2) Prüfen, welche Angaben ein Hook des eingesetzten Werkzeugs erhält (Modellname, Kontingent) und ob er Text ins Gespräch einspielen kann – Voraussetzung für eine mechanische Warnung (#34 Punkt 15, Ergänzung Nr. 5).
- **Akzeptanzkriterien:** wissensbasiert – beide Fragen mit Beleg beantwortet oder als nicht messbar begründet; Ergebnis in `docs/project-context.md` Abschnitt 6 und im Logbuch.
- **Ergebnis:** (1) Direkte Messung nicht möglich – der Kostenzähler der Sitzungsabfrage ändert sich innerhalb einer Antwort nicht und wird verzögert nachgetragen. Ersatz: Modellrechnung aus Token-Zahlen und Listenpreisen. Befund: Abgabe an die Routine-Klasse spart bei Lesearbeit kaum, weil Cache-Lesen bei Opus 5.5 und Sonnet 5 gleich viel kostet; der größte Hebel ist die Kontextgröße der Hauptsitzung. (2) Hooks erhalten das Modell nur optional beim Sessionstart und bei Modellwechseln, aber keine Kosten oder Limits; die Statuszeile erhält Modell, Kosten und – bei Pro/Max – 5-Stunden- und 7-Tage-Limit. Eine mechanische Warnung zum Wochenkontingent ist damit dokumentiert möglich (Statuszeile schreibt, Hook liest), aber unerprobt.
- **Folgeentscheidung:** S-22.

### S-22: Folgerungen aus S-19 – Regel zur Abgabe, Sitzungsgröße, Kontingent-Warnung (Vorschlag)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – PR [#43](https://github.com/Paddel87/Dev-Templates/pull/43) als neunte dokumentierte CI-Ausnahme gemergt (Anweisung des Eigentümers; Nachprüfung S-17). Kontingent-Warnung bewusst inaktiv, bis ein lokaler Probelauf besteht.
- **Freigabe:** 2026-09-24, Option A (alle drei Teile, Probelauf innerhalb des Schritts); Nebenfragen mit Voreinstellung: Grenze 200.000 Token, Warnschwellen 80 und 95 %.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung an `CLAUDE.md` §0 (ADR-011); eine Hook-Konfiguration wäre zusätzlich Werkzeug-Einstellung
- **Empfohlene Klasse:** Entscheidung – Regeländerung, `[STRATEGISCH]`-ADR.
- **Abhängigkeiten:** S-19
- **Zu tun:** Drei Folgerungen aus S-19 zur Entscheidung vorlegen: (a) Abgabe-Regel in §0 schärfen – sie lohnt vor allem an die Mechanik-Klasse und bei ausgabelastiger Arbeit, kaum bei Lesearbeit an die Routine-Klasse; (b) Sitzungsgröße als Hebel: neue Session je Paket bzw. Arbeitsblock, damit nicht jeder Aufruf einen großen Kontext liest; (c) Kontingent-Warnung über Statuszeile und Hook als Erkundung mit Probelauf, bevor sie Regel wird.
- **Akzeptanzkriterien:** Entscheidung des Eigentümers mit ADR; bei (c) ein erzwungener Probelauf nach ADR-008, bevor die Warnung als belastbar gilt.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §0, §12), Vorlagen (`templates/docs/project-context.md`, neu `templates/werkzeuge/claude-code/`, `templates/README.md`), Selbstanwendung (`docs/project-context.md`); zusätzlich `README.md`
- **Artefakte:** ADR-014, PR [#43](https://github.com/Paddel87/Dev-Templates/pull/43)
- **Verifikation (Stand Umsetzung):** Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md` leer. Kontingent-Warnung: Skript mit erzwungenem Fehlerfall geprüft – Schwelle 0 % erzeugt die Warnung; Gegenproben still bei 12 %, Warnung bei 85 % (Schwelle 80) und 97 % (Schwelle 95), still bei abgelaufenem Fenster, fehlender Datei und fehlenden Limit-Angaben, Exit-Code 0 bei kaputter Eingabe; `py_compile` ohne Fehler. Zustellweg (Statuszeile → Datei → Hook → Gespräch) in dieser Umgebung nicht prüfbar – die Warnung bleibt deshalb inaktiv (`docs/project-context.md` Abschnitt 6), wie in der Freigabe vorgesehen. `pre-commit run --all-files` grün.

### S-20: Paket 3 aus #34 – Roter Faden (Punkte 1 und 4)

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option B (Bewertung durch eine getrennte Instanz); Nebenfragen mit Voreinstellung: Faktor 2 und mindestens +5 Schritte; automatische `[REAKTIV]`-Klassifikation nur für Architekturentscheidungen.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Regelwerk (`CLAUDE.md` §6, §12, §14 bzw. §16) und an den Vorlagen.
- **Empfohlene Klasse:** Entscheidung – Freigabe-Vorlage und `[STRATEGISCH]`-ADR (Eskalations-Auslöser 1 und 3).
- **Abhängigkeiten:** keine fachliche; Reihenfolge laut Triage des Eigentümers. Nutzt die getrennte Instanz aus ADR-009, falls Option B gewählt wird.
- **Zu tun:**
  - **Punkt 1 – Auslöser zum Neuplanen:** Jede Phase (bei Klasse K: jedes Schritt-Bündel) hält beim Anlegen ihren ursprünglichen Schrittplan als Zahl fest. Übersteigt die Zahl der Schritte diesen Plan um einen festen Faktor, folgt ein STOPP mit Neuplanung und Vision-Abgleich, bevor ein weiterer Schritt angelegt wird.
  - **Punkt 4 – Pflichtfrage „Weiterbauen, gezielt umbauen oder neu aufsetzen?":** an jeder Phasengrenze und beim Auslöser aus Punkt 1 ein eigener Entscheidungsblock mit den Kosten jeder Option, belegt mit konkreten Stellen im Code; nicht verschiebbar. Jede Architekturentscheidung, die in einer STABILISIERUNG fällt, zählt zur Reaktiv-Quote, unabhängig vom Etikett.
- **Akzeptanzkriterien:** beide Punkte mit Fundstelle in `CLAUDE.md` und `templates/`; Auslöser als Zahl prüfbar; der Umbau-Block steht neben dem Vision-Abgleich in §12; die Zählregel für die Reaktiv-Quote ist in der Drift-Prüfung (§16) nachprüfbar; Grep-Gegenprobe; `pre-commit run --all-files` grün; ADR angelegt.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §6, §8, §12, §16), Vorlagen (`templates/docs/fahrplan.md`, `templates/docs/decisions.md`, `templates/docs/project-context.md`), Selbstanwendung (`docs/project-context.md`, `docs/fahrplan.md`); zusätzlich `README.md`
- **Artefakte:** ADR-012, PR [#40](https://github.com/Paddel87/Dev-Templates/pull/40)
- **Verifikation (Stand Umsetzung):** Fundstellen: `CLAUDE.md` §6 (Reaktiv-Klassifikation in der STABILISIERUNG), §8 Kriterium 9 (Phasen-Wucherung), §12 „Weiterbauen, umbauen oder neu aufsetzen – Pflichtfrage", §16 (Zeile „Phasenumfang", erweiterte Reaktiv-Zeile); Vorlagen: Phasenkopf mit „Ursprünglicher Schrittplan" und „Pflichtfrage am Phasenende", `[REAKTIV]`-Definition, Wucherungs-Schwelle. Erste Anwendung auf dieses Repo: Bündel #34 mit 8 von höchstens 10 Schritten, keine Auslösung. Grep-Gegenprobe im Logbuch 2026-09-24 18:30. `pre-commit run --all-files` grün. Inter-Pflicht-Drift-Check 6/6 grün (neuer Anker „Phasenumfang": Bündel #34 bei 8 von höchstens 10 Schritten).
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist **nicht** erfüllt – derselbe Grund wie bei S-12 bis S-16. PR #40 wird als sechste dokumentierte Ausnahme ohne grüne CI gemergt. Nachträgliche Prüfung: S-17.

### S-21: Paket 4 aus #34 – Projektstart (Punkte 3, 6, 12) zusammen mit #20

- **Status:** [ERLEDIGT] – CI nachgeprüft am 2026-10-12 (S-17, grün); vorher [CI-AUSSTEHEND] (bis 2026-09-27 als [ERLEDIGT] geführt; umgestellt nach ADR-017, Nachprüfung S-17) 2026-09-24 – mit CI-Ausnahme (siehe unten)
- **Freigabe:** 2026-09-24, Option D (eigene Anforderungs-Vorlage ab Klasse M, Modus 1.5 für G und V); Nebenfragen mit Voreinstellung: Schutzbedarf und Kostenrahmen in Modus 2 für alle Klassen, Teil D gestaffelt.
- **Phasentyp-Kontext:** STABILISIERUNG
- **Freigabepflichtig:** ja – Änderung am Projektstart-Verfahren (`templates/projektstart.md`), an der Pflichtlektüre (`CLAUDE.md` §2) und neue Vorlagen.
- **Empfohlene Klasse:** Entscheidung – Freigabe-Vorlage und `[STRATEGISCH]`-ADR.
- **Abhängigkeiten:** Optionswahl zu [#20](https://github.com/Paddel87/Dev-Templates/issues/20) (mit diesem Schritt vorgelegt).
- **Zu tun:**
  - **#20 / Punkt 3 – Anforderungsschicht zwischen Vision und Architektur:** gemäß gewählter Option; Staffelung nach Klasse wie im Nachtrag zu #20 (Anforderungen Pflicht ab M, Business-Analyse Pflicht ab G; neue Artefakte erst ab G und nur als Übersicht in der Mindest-Lektüre). Zusätzlich aus #34: Das Ergebnis von Modus 1 wird im Repo festgehalten, einschließlich **bewusst ausgeschlossener Anwendungsfälle** mit Begründung.
  - **Punkt 6 – Schutzbedarf als Obergrenze:** In Modus 2 wird der Schutzbedarf (z. B. nach dem Standard-Datenschutzmodell) per ADR festgelegt, mit der Frage an den Menschen „Wie schlimm wäre es für deine Nutzer, wenn diese Daten nach außen gelangen?". Jede spätere Datenschutz- oder Sicherheitsmaßnahme nennt die Anforderung dieses Niveaus, die sie erfüllt; alles andere wird als optional vorgelegt. Ergänzt das Sicherheitsniveau aus ADR-009.
  - **Punkt 12 – Kosten:** Kostenrahmen als Vision-Frage in Modus 2 („Was darf das monatlich kosten?"); optionales Kostenregister ab Klasse M (laufende Kosten, KI-Verbrauch, Obergrenze, Auslöser für eine Entscheidung).
- **Akzeptanzkriterien:** jede Änderung mit Fundstelle in `templates/` bzw. `CLAUDE.md`; Klassen-Staffelung in `templates/projektstart.md` §2.2 als einzigem Ort (Nachtrag #20); Pflichtlektüre-Budget für K/M unverändert; Grep-Gegenprobe; `pre-commit run --all-files` grün; ADR angelegt.
- **Betroffene Bestandteile:** Regelwerk (`CLAUDE.md` §2, §3, §6, §12, §16), Vorlagen (`templates/projektstart.md`, neu `templates/docs/requirements.md`, `templates/docs/vision.md`, `templates/docs/project-context.md`, `templates/docs/architecture.md`, `templates/docs/decisions.md`, `templates/docs/fahrplan.md`, `templates/README.md`), Selbstanwendung (`docs/project-context.md`); zusätzlich `README.md`
- **Artefakte:** ADR-013, PR [#41](https://github.com/Paddel87/Dev-Templates/pull/41); Issue #20 geschlossen
- **Verifikation (Stand Umsetzung):** Klassen-Staffelung steht an einem Ort, `templates/projektstart.md` §2.2 (plus Initialisierungshinweis der neuen Vorlage). Pflichtlektüre für K und M unverändert; ab G nur „Übersicht" von `requirements.md`. Neue Vorlage mit sechs Sprung-Ankern. Grep-Gegenprobe im Logbuch 2026-09-24 19:40. `pre-commit run --all-files` grün. Inter-Pflicht-Drift-Check 6/6 grün (Anker „Anforderung → Schritt" für dieses Repo nicht anwendbar, Klasse K).
- **CI-Ausnahme (Anweisung des Eigentümers, 2026-09-24):** Der DoD-Punkt „CI-Pipeline läuft grün" ist **nicht** erfüllt – derselbe Grund wie bei S-12 bis S-20. PR #41 wird als siebte dokumentierte Ausnahme ohne grüne CI gemergt. Nachträgliche Prüfung: S-17.
