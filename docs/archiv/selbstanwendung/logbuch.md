# Logbuch – Dev-Templates

<!-- Arbeitsdokument von Dev-Templates selbst (Selbstanwendung, ADR-001).
     Die Vorlage für Ziel-Projekte liegt unter templates/docs/logbuch.md.
     Klasse K: [SESSIONSTART] und [SESSIONENDE] sind Pflicht, andere Typen optional. -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

Die letzten Einträge geben den aktuellen Stand wieder. Bei Sessionbeginn wird mindestens der letzte `[SESSIONENDE]`-Eintrag plus alles danach gelesen.

Dieses Logbuch beginnt mit der Session, in der die Selbstanwendung eingeführt wurde. Frühere Arbeit an der Methodik ist nicht rückwirkend erfasst – sie ist über die Patch-Wellen in `README.md` und die PR-Historie belegt.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-12 – [GELÖST] S-17 CI-Nachlauf grün

- **Vorgehen:** Die CI hat keinen manuellen Auslöser (`workflow_dispatch`). Deshalb den letzten Lauf auf `main` (Lauf 121, Commit `4e46c4e`) neu gestartet statt einen Leer-Commit zu pushen. Er prüft genau den Stand mit PR #35 bis #48.
- **Ergebnis:** Versuch 2 grün, alle vier Hooks bestanden. Lokal ebenfalls grün.
- **Protokoll gelesen:** eine Plattform-Warnung (Node 20 abgekündigt, `actions/cache@v4` aus `pre-commit/action@v3.0.1`, dort die neueste Version). Nach `CLAUDE.md` §9 ist ein Lauf mit Warnung über dem Bestand nicht grün; der Bestand stand auf null. Deshalb benannte Ausnahme mit Obergrenze 1 im Warnungs-Bestand, Landeplatz S-28 (freigabepflichtig, Kategorie 7), Zeile im Ablaufdaten-Register, Nachprüfung 2026-12-31.
- **Folge:** zwölf Schritte `[CI-AUSSTEHEND]` → `[ERLEDIGT]` (S-12 bis S-16, S-19 bis S-25).
- **Beobachtung:** Das Kontingent war am 2026-10-12 wieder verfügbar; das vermutete Reset-Datum 29.09. ist damit weder bestätigt noch widerlegt.

### 2026-10-12 – [SESSIONSTART] Pause aufgehoben, S-17

- **Anlass:** Der Eigentümer hebt die Pause der Regelwerks-Arbeit auf (2026-10-12). Davor in dieser Session: Blick auf den offenen PR #31 (S-7, Stand 2026-08-13). Er ist nicht mehr mergebar: Konflikte in 8 Dateien, ADR-006 und S-10 sind auf `main` anders belegt, S-7 hat seitdem eine zusätzliche Anforderung. Entscheidung über #31 steht aus.
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Sitzungsabfrage) – Entscheidungs-Klasse.
- **Mindest-Lektüre:** vollständig nach `CLAUDE.md` Abschnitt 2.
- **Vorhaben:** S-17 (CI-Nachlauf auf `main`, Frist 2026-10-31). Empfohlene Klasse Routine; der Schritt läuft oberhalb seiner Empfehlung, eine Abgabe lohnt nicht (ein Auslöser, ein Protokoll).

### 2026-09-27 – [SESSIONENDE] Abschluss, Regelwerks-Arbeit pausiert

- **Stand:** PR #47 (S-25, ADR-018) gemergt. S-25 ist `[CI-AUSSTEHEND]`. Die Arbeit am Regelwerk ist auf Wunsch des Eigentümers pausiert.
- **Rückmeldung des Eigentümers:** „Ich wollte doch nur kurz das Regelwerk ändern." Der Aufwand stand in keinem Verhältnis zur Änderung. Selbst gewählt waren die Regel-Liste mit Wortvergleich, zwei Test-Sessions, das Nachkürzen auf 70 KB und eine lange Offen-Liste. Nicht selbst gewählt: die Pflichten der Selbstanwendung wie Freigabe, ADR, Reflexion und Drift-Prüfung. Datenpunkt für S-26: Beleg für die Last aus Prüfbericht 3.5.
- **Offen:** alles im Fahrplan („Aktueller Stand"). S-17 wartet auf den Reset, S-26 und S-27 auf die Wiederaufnahme, S-7 und S-18 auf Anforderung. Dazu Issue #33: kein Fahrplan-Schritt, und der Verweis in `docs/project-context.md` Abschnitt 2 passt vermutlich nicht zum Titel (ungeprüft).
- **Geplanter Check-in gelöscht:** Den Check-in für den 29.09. (S-17) habe ich wegen der Pause gelöscht. S-17 bleibt mit Frist im Fahrplan.
- **Drift und Größen-Check:** seit dem letzten Sessionende unverändert. Die README trägt #47.
- **Modell-Bilanz:** `claude-opus-5-5` (Entscheidungs-Klasse), keine Schritte oberhalb der Empfehlung, nichts abgegeben.
- **Kontextgröße:** 493.941 Token laut Sitzungsabfrage, über der Grenze von 400.000. Hinweis: die nächste Arbeit in einer neuen Session beginnen.

### 2026-09-27 – [BEOBACHTUNG] CI auf PR #47 und Reset-Datum

- **CI auf PR #47:** Alle Läufe scheitern nach etwa 3 Sekunden, ohne zugewiesenen Runner und ohne Protokoll. Der Diff wurde nie geprüft; Ursache ist das erschöpfte Actions-Kontingent. Das steht einmal als Kommentar im PR.
- **Ursache bestätigt:** Das Repo ist laut GitHub-API privat, private Repos verbrauchen Actions-Minuten. Der Eigentümer hatte es kurz für öffentlich gehalten und das richtiggestellt.
- **Reset-Datum:** vermutlich 2026-09-29 (Angabe des Eigentümers, unbestätigt). Eingetragen im Ablaufdaten-Register und in S-17. An diesem Tag ist ein Check-in geplant, der den CI-Lauf auf PR #47 prüft.

### 2026-09-27 – [ENTSCHEIDUNG] S-25 als `[CI-AUSSTEHEND]`, PR #47

- **Anweisung des Eigentümers:** S-25 als `[CI-AUSSTEHEND]` abschließen und PR anlegen. Die Bedingungen aus `CLAUDE.md` §9 sind erfüllt: Alle übrigen DoD-Punkte sind erfüllt, `pre-commit run --all-files` läuft lokal grün, und der Nachprüf-Schritt S-17 (Frist 2026-10-31) führt S-25 jetzt namentlich auf.
- **PR:** [#47](https://github.com/Paddel87/Dev-Templates/pull/47). README „Methodik-Stand" um den Eintrag ergänzt; damit ist die offene Drift aus dem Sessionende-Eintrag behoben.

### 2026-09-27 – [ENTSCHEIDUNG] Größengrenze für `CLAUDE.md` gestrichen

- **Anlass:** Der Eigentümer sieht nach der Messung (−0,8 % Kontext je Aufruf) keinen Sinn in weiterem Kürzen: Der Aufwand steht in keinem Verhältnis zum Nutzen.
- **Entscheidung** (Frage-Antwort-Werkzeug, beide Punkte wie empfohlen): Die Grenze von 70.000 Byte wird gestrichen. Die schon umgesetzte Verschlankung bleibt, nichts wird zurückgebaut. Die Revision steht in ADR-018 (Teilfrage 1, „am selben Tag vor dem Merge revidiert"), kein neuer ADR; die Reaktiv-Quote bleibt bei 4/10.
- **Umgesetzt:** Budget-Zeile in `CLAUDE.md` §2 entfernt, Einleitungssatz auf den vorherigen Wortlaut zurückgesetzt (69.719 Byte, 679 Zeilen). Nachgezogen in ADR-018, Teil A, Fahrplan S-25, `docs/architecture.md` und der Regel-Liste (R-063).
- **Folge:** Der Hinweis „nur 66 Byte Luft" im Sessionende-Eintrag oben ist überholt. S-26 braucht für den Leitsatz keine Kürzung.

### 2026-09-27 – [SESSIONENDE] S-25

- **Bearbeitet:** S-25 (U1). Regel-Liste angelegt, sieben Fragen vorgelegt und freigegeben, ADR-018 angelegt und umgesetzt, Gegenprobe 238/238, Fahrplan archiviert, Reflexion zur Reaktiv-Quote.
- **Stand:** S-25 `[IN ARBEIT]`, nur der CI-Lauf fehlt. Branch `claude/page-25-continue-4et86m` gepusht, kein PR angelegt (der Eigentümer wurde gefragt).
- **Messung nachher (Nebenfrage):** zwei Test-Sessions mit gleichem Auftrag, danach archiviert. Vorher (`main`) 134.726 Token, nachher 133.615 Token (−0,8 %); Kosten 1,30 $ → 1,02 $. Die Kontextgröße je Aufruf hängt fast nur am festen Systemkontext. Der Befund widerspricht der Erwartung aus dem Prüfbericht (B2) und steht in ADR-018. Mein eigener Ersatzwert „vorher" (rund 130.000 über den Token-Zähler) lag nahe am Messwert der Test-Session.
- **README-Synchronisation:** Verzeichnisbaum um die zwei Anhänge ergänzt. Der Eintrag unter „Methodik-Stand" folgt mit der PR-Nummer; bis dahin ist das eine bewusst offene Drift.
- **Inter-Pflicht-Drift-Check:** ADR-018 verweist auf S-17, S-25, S-26, alle vorhanden (S-17 und S-25 im aktiven Fahrplan). Die ADR-Verweise auf archivierte Schritte (z. B. ADR-013 → S-21) sind im Archiv auffindbar. Reaktiv-Quote in Teil A 5/18 und 4/10, stimmt mit den Tags in Teil B überein. Modul-Liste unverändert (drei Bestandteile). Blocker: keine. Phasenumfang: Bündel „Umbau nach Prüfbericht" bei 3 von höchstens 9 Schritten. Reifegrade unverändert (S-26).
- **Größen-Check:** `CLAUDE.md` 69.934 Byte ≤ 70.000; `docs/project-context.md` 169 Zeilen ≤ 600. Logbuch unter 1.600 Zeilen.
- **Ablaufdaten-Register:** Für das Actions-Kontingent gilt die Frist 2026-10-31 (S-17), das Reset-Datum ist weiter offen. Keine neuen Einträge.
- **Modell-Bilanz:** eingestellt und bedient `claude-opus-5-5` (Sitzungsabfrage) = Entscheidungs-Klasse, wie empfohlen. 0 Schritte oberhalb der Empfehlung, nichts abgegeben (die Regel-Liste ist keine Aufgabenart mit bestandenem Probelauf). Zusätzlich zwei Test-Sessions auf der Entscheidungs-Klasse für die Messung (zusammen 2,31 $ Listenpreis).
- **Kontextgröße:** 320.749 Token laut Sitzungsabfrage, unter der Grenze von 400.000.
- **Nächster Schritt:** Anweisung des Eigentümers zu CI und PR, danach S-26 (U4) in einer neuen Session. Achtung: `CLAUDE.md` hat nur noch 66 Byte Luft.

### 2026-09-27 – [GELÖST] S-25 Umsetzung und Gegenprobe

- **Freigabe:** über das Frage-Antwort-Werkzeug, in zwei Runden (höchstens vier Fragen je Runde). 1B, 2B, 3A, 4A, 5A, 6A wie empfohlen, `[REAKTIV]` bestätigt; Nebenfrage abweichend: Nachher-Messung sofort per Test-Session.
- **Reibung – Schätzung zu hoch:** Die Vorlage nannte die 70.000 Byte „erreichbar mit den Teilfragen 2 bis 4 plus §17". Tatsächlich brachten Begründungen, Kostenteil und Gate zusammen rund 7,4 KB (81.519 → 74.105). Den Rest (bis 69.934) brachte Kürzen ohne Regelwirkung: Wiederholungen, Beispiele, Zusatzbegründungen. Lehre: Bytes je Maßnahme vor der Vorlage ausrechnen, nicht schätzen. Die Luft beträgt nur 66 Byte.
- **Gegenprobe:** Suchbegriffe aus der `.txt`-Liste gegen `CLAUDE.md` und die zwei Anhänge; 238/238 zugeordnet. Für jede der 48 Regelzeilen, deren Wortlaut sich geändert hat, ein Wortvergleich (gestrichene gegen neue Wörter). Befund: Gestrichen wurden nur Beispiele, Begründungen und Wiederholungen. Eine Definition ändert sich: Die DoD verweist für „Quick-Start-relevant" auf §17. Die Liste in §17 löste die DoD schon vorher aus; die engere Aufzählung in §9 widersprach ihr (in ADR-018 festgehalten). Der Satz „Diagnose-Heuristik" in §6 fällt weg, sein Wortlaut steht im Anhang von ADR-018. ANCHOR-Kommentare vorher und nachher identisch.
- **Mini-Reibung:** Die Suche nach Modellnamen mit `grep -v 'CLAUDE\.md'` filterte alle Zeilen weg, weil `grep -n` den Dateinamen voranstellt. Richtig: den Dateinamen im Inhalt vor der Suche entfernen (`sed 's/CLAUDE\.md//g' | grep`). Ergebnis dann leer.
- **Doppelung vermieden:** Die Sessiongröße stand zunächst im Anhang und in §0. Sie bleibt nur in §0, weil sie bei jedem Sessionende gilt.
- **Fahrplan:** 63.399 → 20.462 Byte. 19 Schritte archiviert in `docs/archiv/selbstanwendung/fahrplan-bis-buendel-34.md`, S-17 führt die `[CI-AUSSTEHEND]`-Schritte jetzt namentlich auf.
- **Nebenbefund behoben:** Die Fahrplan-Vorlage nannte als Archiv-Auslöser „>500 Zeilen nach `fahrplan-YYYY-MM.md`", `CLAUDE.md` §14 dagegen abgeschlossene Phasen. Die Vorlage ist an §14 angeglichen.

### 2026-09-27 – [ADR-ANGELEGT] ADR-018

- **Umbau U1**, `[REAKTIV]` (verschiebt Regeltext aus ADR-009, -011, -014, -016). Reaktiv-Quote 4/10 = 40 %, über der Schwelle von 30 %. Die Reflexion steht im Fahrplan („Reflexion nach Überschreiten der Reaktiv-Quote"). Eskalations-Auslöser 5: Das Modell war bereits Entscheidungs-Klasse, ein Wechsel war nicht nötig.

### 2026-09-27 – [ENTWURF-VORGELEGT] S-25

- **Regel-Liste vorher:** `docs/pruefberichte/2026-09-27-regelliste-s25.md` mit 238 Einträgen (214 Regeln, 13 zur Wirtschaftlichkeit, 11 Begründungen); je Eintrag ein wörtlicher Suchbegriff in der gleichnamigen `.txt`-Datei, maschinell geprüft: jeder genau einmal in `CLAUDE.md`. Eine Abdeckungsprüfung (jede Textzeile außerhalb von Überschriften, Tabellen und Codeblöcken einem Eintrag zugeordnet) ergab nur Listenpunkte, die zu einem erfassten Eintrag gehören, und fünf Nachträge (R-234 bis R-238).
- **Messwerte für die Vorlage:** `CLAUDE.md` 81.519 Byte, 714 Zeilen. Größte Abschnitte: §0 11.981 Byte, §12 10.811, §2 8.465, §15 8.143, §6 7.860, §16 7.691. Kosten-Teil in §0 (Einträge K) 7.373 Byte, Begründungen (Einträge B) 4.882 Byte, Gate in §12 4.631 Byte. `docs/fahrplan.md` 63.399 Byte, davon rund 16 KB `[ERLEDIGT]`-Schritte und rund 30 KB `[CI-AUSSTEHEND]`-Schritte des abgeschlossenen Bündels #34.
- **Vorgelegt:** sechs Teilfragen (Größengrenze, Begründungen, Kosten-Anhang, Gate-Anhang, Fahrplan-Archiv bei Klasse K, Wert der Sessiongröße) und eine Nebenfrage (Nachher-Messung). Offen gelegt: Die Verlagerung aus ADR-009, -011, -014 und -016 (alle jünger als 14 Tage) macht den neuen ADR nach meiner Lesart `[REAKTIV]`; die Quote stiege auf 4/10 = 40 % über die Schwelle von 30 %.

### 2026-09-27 – [SESSIONSTART] S-25

- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` (Quelle: Sitzungsabfrage `get_session`) → Entscheidungs-Klasse. Empfehlung für S-25: Entscheidung, also eingehalten.
- **Mindest-Lektüre:** `docs/project-context.md` vollständig; Logbuch ab `[SESSIONENDE] S-24`; Fahrplan „Aktueller Stand" und offene Schritte; `docs/architecture.md` Abschnitte 1, 2, 9; `docs/decisions.md` Teil A und C; `docs/blockers.md` „Aktive Blocker" (keine). Klasse K: `requirements.md` entfällt.
- **Kontextgröße nach der Mindest-Lektüre (Messung „vorher" für S-25):** Die Sitzungsabfrage meldet während der laufenden Antwort `used_tokens: 0` und taugt dafür nicht. Ersatzquelle: der Token-Zähler der Laufzeit im Gespräch. Vor dem ersten Lesen rund 85.000 Token (Systemkontext mit Werkzeugen und `CLAUDE.md`), nach der Mindest-Lektüre rund 130.000 Token. Die Lektüre der Pflicht-Dokumente selbst kostete also rund 45.000 Token, dazu kommt `CLAUDE.md` im Systemkontext.
- **Ziel der Session:** S-25 (U1) – Regel-Liste anlegen, Freigabe der Teilfragen einholen.

### 2026-09-27 – [SESSIONENDE] S-24

- **Bearbeitet:** S-24 (U3). Die reinen Korrekturen liefen ohne Freigabe. Sechs Teilfragen wurden entschieden (1A, 2A, 3B, 4A, 5A, 6A), ADR-017 angelegt und umgesetzt. Der Schritt steht auf [IN ARBEIT]; es fehlt nur der CI-Lauf.
- **README-Synchronisation:** Der Eintrag für den PR zu S-24 folgt mit der PR-Nummer. Sonst gibt es keine Drift: Der Aufbau des Repos hat sich nur um `ruff.toml` und `.gitignore` geändert, der Quick-Start nicht.
- **Inter-Pflicht-Drift-Check:** ADR-017 verweist auf S-17, S-24, S-26 und S-27, alle vier existieren. Die Reaktiv-Quote in Teil A (4/17, 3/10) stimmt mit den Tags in Teil B überein. Nach der neuen Korrektur-Regel gibt es sonst keinen Korrektur-ADR innerhalb von 14 Tagen: ADR-008 bis ADR-013 und ADR-015 ändern laut Kategorie-Feld keinen anderen ADR. Blocker: keine aktiven. Phasenumfang: Das Bündel „Umbau nach Prüfbericht" hat 4 von höchstens 9 Schritten.
- **Ablaufdaten-Register:** Für das Actions-Kontingent ist die Frist 2026-10-31 eingetragen, das Reset-Datum ist weiter offen. ruff v0.16.9 steht nicht im Register, weil die Linie 0.x kein Lebensende ausweist. Nachprüfung beim nächsten Hinweis im CI-Protokoll.
- **Modell-Bilanz:** `claude-opus-5-5` (Entscheidungs-Klasse), Empfehlung für S-24 Entscheidung, also eingehalten. Nichts abgegeben: Die Korrekturen waren kurze Arbeit im geladenen Kontext.
- **Kontextgröße:** rund 800.000 Token laut Laufzeitabfrage, über der Grenze von 400.000. Hinweis nach ADR-016: Die nächste Arbeit (S-25) in einer neuen Session beginnen.
- **Nächster Schritt:** S-25, in einer neuen Session. S-24 mit PR #46 als erster Anwendungsfall von `[CI-AUSSTEHEND]` gemergt.

### 2026-09-27 – [GELÖST] Neue Testdatei von `--all-files` nicht erfasst

- **Problem:** `pre-commit run --all-files` lief grün, der Hook beim Commit meldete dann S603 in der neuen Testdatei.
- **Ursache:** `--all-files` prüft nur Dateien, die git kennt. Die Testdatei war noch nicht im Index.
- **Lösung:** Eine begründete Zeilen-Suppression (fester Aufruf des eigenen Skripts), danach Zeilenlänge korrigiert. Lehre: Vor dem Prüflauf `git add` ausführen, oder den Hook beim Commit als eigentlichen Nachweis werten.

### 2026-09-27 – [ADR-ANGELEGT] ADR-017

- **ADR:** ADR-017 zu Umbau U3. Der ADR ist nach seiner eigenen Korrektur-Regel `[REAKTIV]`, weil er ADR-012 ändert. ADR-014 und ADR-016 wurden rückwirkend umgestellt.
- **Reaktiv-Quote:** 3 von 10 über die letzten zehn ADRs, genau an der Grenze von 30 %. Der nächste Korrektur-ADR löst Stopp und Reflexion aus.
- **Entscheidung:** In allen sechs Teilfragen folgte der Eigentümer der Empfehlung (bei S-23 in einer von vier).

### 2026-09-27 – [GELÖST] S-24 reine Korrekturen

- **W6:** Angaben zu Abhängigkeiten, Sprachen, Plattformen und Code-Standards in `docs/project-context.md` berichtigt. Skript im Werkzeug-Verzeichnis (`docs/architecture.md` Abschnitt 10) und in der Reifegrad-Übersicht (VORLÄUFIG) eingetragen. Skript-Kopf um die Aussagen zu Idempotenz und Reproduzierbarkeit ergänzt; das Skript hat jetzt 71 statt 67 Zeilen, `ast.parse` ist fehlerfrei.
- **W8:** Schrittnummern in `templates/projektstart.md` (3 Stellen) und README-Vorlagen-Pfad in `CLAUDE.md` korrigiert, Feldname als Konvention festgehalten.
- **W7 nachgeholt:** Die Neutralitäts-Suche lief diesmal über die ganze Datei. Neu gefunden wurde `templates/projektstart.md` Zeile 138 („Claude Code (1M Kontext)").
- **Mini-Reibung:** Beim Eintragen des Skripts in `docs/architecture.md` zuerst ADR-013 als Herkunft genannt, richtig ist ADR-014 (Kontingent-Warnung). Vor dem Commit korrigiert.
- `pre-commit run --all-files`: grün.

### 2026-09-27 – [SESSIONSTART] Fortsetzung

- **Anlass:** Eigentümer sagt „Weiter" nach dem Sessionende-Eintrag. Der Hinweis zur Sessiongröße (ADR-016) war gegeben; fortgesetzt wird in derselben Session.
- **Modell:** laut Laufzeitabfrage eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` → Entscheidungs-Klasse. Kontext rund 780.000 Token.
- **Schritt:** S-24 (U3). Empfohlene Klasse: Entscheidung für die freigabepflichtigen Teile, Routine für die Korrekturen. Keine Abgabe der Korrekturen: Das ist kurze Lesearbeit im bereits geladenen Kontext, und die Routine-Klasse liest aus dem Cache zum selben Preis (ADR-014).

### 2026-09-27 – [SESSIONENDE]

- **Bearbeitet:** S-23 (U2) – vorgelegt in vier Teilfragen, entschieden 1B/2A/3D/4B, umgesetzt, [ERLEDIGT] mit elfter CI-Ausnahme (PR #45).
- **README-Synchronisation:** Methodik-Stand um #45 ergänzt.
- **Inter-Pflicht-Drift-Check:** ADR-016 verweist auf S-23 und S-25, beide existieren; Reaktiv-Quote 1/16; Phasenumfang: Bündel „Umbau nach Prüfbericht" 4 von höchstens 9 Schritten. Bekannte offene Befunde W6/W8 bleiben für S-24.
- **Ablaufdaten-Register:** Reset-Datum des Actions-Kontingents weiterhin offen.
- **Modell-Bilanz:** `claude-opus-5-5` (Entscheidungs-Klasse), Empfehlung für S-23 Entscheidung – eingehalten. Keine Abgabe.
- **Kontextgröße:** rund 774.000 Token laut Laufzeitabfrage – über der neuen Grenze von 400.000. Hinweis nach ADR-016: nächste Arbeit in einer neuen Session beginnen.
- **Nächster Schritt:** S-24 (U3), reine Korrekturen zuerst.

### 2026-09-27 – [ADR-ANGELEGT]

- **ADR:** ADR-016 – Umbau U2: Sessiongröße nur noch Hinweis; übrige Unterbrechungen bleiben. Reaktiv-Quote 1/16.
- **Entscheidung über ein Frage-Antwort-Werkzeug** (vom Eigentümer gewünscht): 1B, 2A, 3D, 4B. Erstmals weicht der Eigentümer in drei von vier Punkten von der Empfehlung ab.
- **Grep-Gegenprobe:** „weiter hier" nicht mehr in `CLAUDE.md`; „Sessiongröße" in §0 (neu gefasst), §12 Punkt 4 (unverändert gültig), Vorlage `project-context.md` (Vorgabe 400.000).

### 2026-09-27 – [ENTWURF-VORGELEGT]

- **Abweichung Sessiongröße:** Eigentümer sagt erneut ausdrücklich „weiter hier" (Session weit über 600.000 Token). Vermerkt nach ADR-014.
- **Schritt:** S-23 (Umbau U2) – vorgelegt als **vier getrennte Teilfragen** statt einer Gesamtoption, damit jede einzeln entschieden wird (Befund 3.5 und Stellungnahme: acht von acht Entscheidungen folgten der Empfehlung). Jede Teilfrage enthält das stärkste Argument gegen die eigene Empfehlung.
- **Teilfragen:** (1) Modellwechsel-Stopp und Freigabe zu einem Block, Pflichtfrage als Freigabe; (2) Umgang mit Nebenfragen; (3) Schutz gegen bloßes Abnicken; (4) Hinweis zur Sessiongröße (Rückmeldung aus dem zweiten Projekt).
- **Unterbrechungsarten laut Prüfbericht 3.5:** 13 vor dem Umbau.
- **Modell:** `claude-opus-5-5`, Entscheidungs-Klasse; Empfehlung für S-23: Entscheidung – eingehalten.

### 2026-09-26 – [BEOBACHTUNG]

- **Erste Rückmeldung aus einem zweiten Projekt:** Der Eigentümer testet das neue Regelwerk in einem neuen Projekt. Die Grenze der Sessiongröße von 200.000 Token (ADR-014) ist dort „zunehmend lästig". Das stützt Prüfbericht 3.5 (Unterbrechungen) und 3.6 (Grenze als Setzung ohne Messung). Bei S-25 und S-23 vermerkt. Der Grenzwert ist nach ADR-014 projektbezogen (`docs/project-context.md` Abschnitt 6) und kann im neuen Projekt sofort angepasst werden, ohne das Regelwerk zu ändern.

### 2026-09-26 – [ENTSCHEIDUNG]

- **PR #44** (Prüfbericht, Stellungnahme, ADR-015, Umbau-Plan) auf Anweisung des Eigentümers als zehnte dokumentierte Ausnahme ohne grüne CI gemergt – trotz Befund W1, auf den im Vorschlag ausdrücklich hingewiesen wurde. Keine neue Session begonnen; der Merge gehört noch zum Abschluss des Stopps vom 2026-09-24.

### 2026-09-24 23:30 – [SESSIONENDE]

- **Dauer:** ca. 21:15–23:30
- **Bearbeitet:** S-22 abgeschlossen (PR #43); Stopp wegen Phasen-Wucherung im Bündel #34; Pflichtfrage mit getrennter Instanz; ADR-015 (gezielt umbauen); neues Bündel „Umbau nach Prüfbericht" mit S-23 bis S-26; Landeplatz S-27 für Punkt 2 aus #34.
- **Erreichter Stand:** Stopp aufgelöst. Prüfbericht und Stellungnahme in `docs/pruefberichte/`. `.pyc`-Datei entfernt. Alles im Session-Branch; kein PR für den Stand seit #43.
- **README-Synchronisation:** keine nutzersichtbare Änderung an Regelwerk oder Vorlagen seit #43; Methodik-Stand folgt mit dem nächsten PR.
- **Inter-Pflicht-Drift-Check:** ADR-015 verweist auf S-23 bis S-27, alle angelegt; Reaktiv-Quote 1/15; Phasenumfang: neues Bündel 4 von höchstens 9. Der Befund W6/W8 (Feld „Betroffene Bestandteile" statt „Betroffene Module", Abhängigkeitsangaben) bleibt bis S-24 bekannt offen.
- **Ablaufdaten-Register:** unverändert; Reset-Datum weiterhin offen.
- **Modell-Bilanz:** `claude-opus-5-5` (Entscheidungs-Klasse), alle Schritte mit Empfehlung Entscheidung. Keine Abgabe.
- **Sessiongröße:** weit über der Grenze (über 600.000 Token); Ausnahme „weiter hier" des Eigentümers seit 22:15. Nächster Schritt S-23 ausdrücklich in einer neuen Session.
- **Offen geblieben:** PR für den Stand seit #43 (Frage an den Eigentümer, wegen Befund W1 nicht ungefragt als Ausnahme); S-17, S-18; lokaler Probelauf der Kontingent-Warnung.
- **Nächster Schritt:** neue Session – S-23 (U2) als Entscheidung vorlegen.

### 2026-09-24 23:25 – [ENTSCHEIDUNG]

- **Pflichtfrage Bündel #34:** Eigentümer wählt B – gezielt umbauen (ADR-015). Stopp aufgelöst; neues Bündel S-23 bis S-26, Landeplatz S-27 für Punkt 2.

### 2026-09-24 23:10 – [PRUEFBERICHT]

- **Bericht der getrennten Instanz:** `docs/pruefberichte/2026-09-24-pflichtfrage-buendel-34.md` (Branch `pruefung/buendel-34`, in den Session-Branch übernommen). Empfehlung: gezielt umbauen (Blöcke U1 bis U4, Reihenfolge U2, U3, U1, U4), keine neue Regel bis dahin, Punkt 2 aus #34 danach. Konfidenz mittel. Lief auf derselben Modell-Linie (`claude-opus-5-5`).
- **Nachprüfung durch die bauende Session:** Größe `CLAUDE.md`, `.pyc`-Datei, Abhängigkeits- und Skript-Angaben, Modell- und Werkzeugnamen – alle bestätigt. `.pyc` als Bugfix entfernt.
- **Stellungnahme:** `docs/pruefberichte/2026-09-24-stellungnahme-buendel-34.md` – Zustimmung zur Empfehlung, zwei eigene Versäumnisse bestätigt (W3, W4), Ergänzung: in allen acht Entscheidungen des Tages folgte der Eigentümer der Empfehlung. Einschränkungen zu B2 (Token-Schätzung unsicher) und B4 (ein Satz Begründung im Regeltext behalten).

### 2026-09-24 22:30 – [ENTSCHEIDUNG]

- **Zum STOPP 22:15:** Eigentümer wählt Auflösung A – die getrennte Instanz ist eine neue Session, die er selbst startet; der Auftragstext kommt von dieser Session. Vision-Quelle für Dev-Templates festgelegt: `docs/project-context.md` Abschnitt 2 plus Leitsatz aus #34 (dort eingetragen).
- **Bewusste Grenze des Auftrags:** Die getrennte Instanz liest `docs/logbuch.md` nicht. Das Logbuch trägt die Sicht der bauenden Session und würde die Unabhängigkeit schwächen. Sie schreibt nur ihren Bericht auf einem eigenen Branch.

### 2026-09-24 22:15 – [STOPP]

- **Abweichung Sessiongröße:** Der Eigentümer hat nach dem Hinweis auf die Grenze (rund 618.000 von 200.000 Token) ausdrücklich „weiter hier" gesagt. Die Session läuft deshalb weiter (ADR-014, Ausnahme).
- **Stopp-Kriterium 9 beim nächsten Schritt:** Punkt 2 aus #34 wäre Schritt 11 im Bündel #34 (Schwelle mehr als 10). Kein neuer Schritt angelegt; STOPP-Block im Fahrplan.
- **Befund:** Dev-Templates hat keine `docs/vision.md`. Der Vision-Abgleich, den Kriterium 9 verlangt, hat damit keine Quelle – das ist bei der Selbstanwendung (ADR-001) nie festgelegt worden.

### 2026-09-24 22:05 – [SESSIONENDE]

- **Dauer:** ca. 21:15–22:05
- **Bearbeitet:** S-22 – vorgelegt, Option A freigegeben, umgesetzt, [ERLEDIGT] mit CI-Ausnahme.
- **Erreichter Stand:** Abgabe-Kriterium, Regel „Sessiongröße", Kontingent-Warnung (Skript geprüft, inaktiv); ADR-014. PR [#43](https://github.com/Paddel87/Dev-Templates/pull/43) als neunte dokumentierte Ausnahme ohne grüne CI gemergt.
- **README-Synchronisation:** Methodik-Stand um #43 ergänzt; Dateibaum im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 6/6 grün plus ein Anker nicht anwendbar; ADR-014 verweist auf S-19 und S-22, beide existieren; Reaktiv-Quote 1/14. Kleine Teilarbeit im geladenen Kontext, nicht abgegeben.
- **Phasenumfang:** Bündel #34 bei 10 Schritten, Schwelle mehr als 10.
- **Ablaufdaten-Register:** unverändert; Reset-Datum des Actions-Kontingents weiterhin offen.
- **Modell-Bilanz:** Laufzeitabfrage: `claude-opus-5-5` (Entscheidungs-Klasse). S-22: Empfehlung Entscheidung, eingehalten. Keine Abgabe an Unteragenten.
- **Sessiongröße (erste Anwendung, ADR-014):** rund 618.000 Token laut Laufzeitabfrage – über der Grenze von 200.000. Nach dem Abschluss von S-22 beginnt die KI in dieser Session keinen neuen Schritt; der Eigentümer wurde um eine neue Session gebeten.
- **Offen geblieben:** Punkt 2 aus #34 (Betriebsmodus); Abschluss des Bündels #34 mit Vision-Abgleich und Pflichtfrage; S-17, S-18; lokaler Probelauf der Kontingent-Warnung durch den Eigentümer; S-7-Nebenbefund; Kommentar in Zeile 3 von `CLAUDE.md`.
- **Nächster Schritt:** neue Session – Entscheidungsvorlage zu Punkt 2 (Betriebsmodus).

### 2026-09-24 21:50 – [ADR-ANGELEGT]

- **ADR:** ADR-014 – Abgabe nur, wo sie spart; Grenze der Sessiongröße; Kontingent-Warnung nach Probelauf
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Freigabe des Eigentümers zu S-22, Option A. Reaktiv-Quote 1/14 (7 %).
- **Probelauf der Warnung:** in einem temporären Verzeichnis mit dokumentierter Beispiel-Eingabe der Statuszeile; Schwelle 0 % erzwungen → Warnung erschien; Gegenproben (12 %, 85 %, 97 %, abgelaufenes Fenster, fehlende Datei, fehlende Limit-Angaben, kaputte Eingabe) wie erwartet. Kleine Korrektur nach dem ersten Lauf: Wochentag war englisch („Fri"), jetzt deutsch. Die Hooks dieser Cloud-Session gehören der verwalteten Umgebung (`launcher-settings.json`); in sie wurde bewusst nicht eingegriffen. Der Zustellweg bleibt deshalb unerprobt, die Warnung inaktiv.
- **Grep-Gegenprobe:** „Rückstufungs-Hinweis", „Sessionende", „Kontingent" in `CLAUDE.md`: Der letzte Satz von „Was diese Regel nicht leistet" verweist jetzt auf den neuen Unterabschnitt; §12 Punkt 4 nennt die Kontextgröße. Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md`: keine Werkzeug- oder Modellnamen. Werkzeugnamen stehen nur unter `templates/werkzeuge/`.

### 2026-09-24 21:20 – [ENTWURF-VORGELEGT]

- **Schritt:** S-22 – Folgerungen aus S-19.
- **Optionen:** A – alle drei Teile in S-22: Abgabe-Regel schärfen, Sessiongröße als Regel, Kontingent-Warnung mit Probelauf innerhalb von S-22 (empfohlen). B – Regeln jetzt, Kontingent-Warnung als eigener Erkundungsschritt (löst Stopp-Kriterium 9 aus, weil Bündel #34 bei 10 Schritten steht). C – nur die Sessiongröße.
- **Kein neuer Schritt angelegt:** Die Vorlage nutzt den bestehenden Schritt S-22; Kriterium 9 greift nicht.
- **Modell:** `claude-opus-5-5`, Entscheidungs-Klasse; Empfehlung für S-22: Entscheidung – eingehalten. Kontext der Hauptsitzung rund 580.000 Token – die Empfehlung aus S-19 (neue Session) wurde vom Eigentümer nicht aufgegriffen; kein Regelverstoß, weil die Regel noch nicht gilt.

### 2026-09-24 21:15 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 21:05, PR #42 gemergt (`main` = `ec0e8f6`).
- **Modell:** `claude-opus-5-5`, Entscheidungs-Klasse (Laufzeitabfrage 21:05).
- **Geplant:** S-22 zur Entscheidung vorlegen.

### 2026-09-24 21:05 – [SESSIONENDE]

- **Dauer:** ca. 20:15–21:05
- **Bearbeitet:** S-19 (Erkundung) – [ERLEDIGT]; S-22 als Vorschlag angelegt.
- **Erreichter Stand:** Befunde zu Ersparnis der Abgabe, Messmethode, Hooks und Statuszeile in Logbuch (20:50) und `docs/project-context.md` Abschnitt 6. PR [#42](https://github.com/Paddel87/Dev-Templates/pull/42) als achte dokumentierte Ausnahme ohne grüne CI gemergt.
- **README-Synchronisation:** Methodik-Stand um #42 ergänzt.
- **Inter-Pflicht-Drift-Check:** 6/6 grün plus ein Anker nicht anwendbar (Klasse K) – geprüft durch den Routine-Unteragenten, Ergebnis gegen die eigenen Änderungen dieser Session gegengelesen.
- **Phasenumfang:** Bündel #34 bei 10 Schritten, Schwelle mehr als 10 – der nächste neue Schritt löst Kriterium 9 aus.
- **Ablaufdaten-Register:** unverändert; Reset-Datum weiterhin offen.
- **Modell-Bilanz:** Laufzeitabfrage: `claude-opus-5-5` (Entscheidungs-Klasse). S-19: Empfehlung Routine, lief oberhalb (Warnung zu Beginn ausgegeben; der Vergleich mit der Entscheidungs-Klasse war Teil der Messung). An Unteragenten abgegeben: Drift-Prüfung (Routine) und Doku-Recherche (Werkzeug-Hilfeagent). Kontext der Hauptsitzung jetzt rund 575.000 Token – Empfehlung: nächstes Paket in einer neuen Session beginnen (Befund S-19).
- **Offen geblieben:** S-22, Punkt 2 aus #34, S-17, S-18, S-7-Nebenbefund, Kommentar in Zeile 3 von `CLAUDE.md`.
- **Nächster Schritt:** Entscheidung, ob zuerst S-22 oder Punkt 2 (Betriebsmodus) – in einer neuen Session.

### 2026-09-24 20:50 – [ERKENNTNIS]

- **Schritt:** S-19 (Erkundung).
- **Messmethode:** Der Kostenzähler der Sitzungsabfrage (`usage.cost_usd`) änderte sich zwischen zwei Abfragen in derselben Antwort nicht (gleicher Zeitstempel). Zwischen 11:23 und 11:27 sprang er von 308,37 auf 368,21 $, davon rund 155 Mio. Token Cache-Lesen – das ist mit einer einzelnen Antwort nicht erklärbar. Der Zähler wird also verzögert und gebündelt nachgetragen. **Korrektur:** Die in den Sessionende-Einträgen 17:55, 18:45 und 20:00 genannten Zählerstände je Schritt sind deshalb keine belastbaren Messwerte.
- **Preise (Listenpreis 2026-06-24, Referenz des Werkzeugs):** Opus 5.5 – Eingabe 4 $, Ausgabe 20 $, Cache-Lesen 0,20 $; Sonnet 5 – 2 $ / 10 $ / 0,20 $; Haiku 4.5 – 1 $ / 5 $ / 0,10 $; Cache-Schreiben 1,25-fach (5 Minuten). Cache-Lesen kostet bei Opus 5.5 und Sonnet 5 gleich viel.
- **Messung Unteragent:** Drift-Prüfung auf der Routine-Klasse (Sonnet 5): 13 Werkzeugaufrufe, 135.936 Token, 61 s, Ergebnis 6/6 grün plus ein Anker nicht anwendbar – korrekt. Modellrechnung: etwa 0,6 $ plus ein Aufruf der Hauptsitzung. Dieselbe Prüfung im geladenen Kontext (rund 510.000 Token) mit 3–4 Aufrufen: etwa 0,3–0,4 $. Ob die gemeldete Token-Zahl den Endkontext oder die Summe meint, ist nicht dokumentiert; die Folgerung hält in beiden Lesarten nicht für eine Ersparnis.
- **Hooks und Statuszeile (Doku, nicht erprobt):** Hooks – Modell nur optional bei `SessionStart`, `PreModelSwitch`/`PostModelSwitch` mit `from_model`/`to_model`, keine Kosten- oder Limitfelder, Text ins Gespräch über stdout bei `SessionStart`, `UserPromptSubmit`, `UserPromptExpansion`, `PostModelSwitch`. Statuszeile – `model.id`, `cost.total_cost_usd`, Kontext-Token und für Pro/Max `rate_limits.five_hour` und `rate_limits.seven_day` (Prozent, Zurücksetz-Zeitpunkt).
- **Qualitätsbefund zur Abgabe:** Der Recherche-Unteragent meldete die Felder der Statuszeile als „nicht dokumentiert". Die Gegenprüfung in der Doku zeigte das Gegenteil. Bestätigt die Pflicht aus §0, abgegebene Ergebnisse vor der Verwendung zu prüfen.
- **Folgerung:** S-22 als Vorschlag angelegt. Bündel #34 damit bei 10 Schritten – an der Wucherungs-Schwelle (mehr als 10 löst den Stopp aus).

### 2026-09-24 20:15 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 20:00, PR #41 gemergt (`main` = `6265b75`), #20 geschlossen.
- **Modell:** `claude-opus-5-5` (Laufzeitabfrage 20:00), Entscheidungs-Klasse.
- **Geplant:** S-19 (Erkundung). Empfohlene Klasse Routine; läuft oberhalb der Empfehlung, weil der Vergleich mit der Entscheidungs-Klasse Teil der Messung ist – Warnung zu Beginn ausgegeben.

### 2026-09-24 20:00 – [SESSIONENDE]

- **Dauer:** ca. 19:00–20:00
- **Bearbeitet:** S-21 (Paket 4 aus #34 mit #20) – vorgelegt, Option D gewählt, umgesetzt, [ERLEDIGT] mit CI-Ausnahme.
- **Erreichter Stand:** Neue Vorlage `requirements.md`, Modus 1.5 für G/V, Regel „Schutzbedarf ist Obergrenze", Kostenrahmen, Teil D für Geschäftsentscheidungen; ADR-013. PR [#41](https://github.com/Paddel87/Dev-Templates/pull/41) als siebte dokumentierte Ausnahme ohne grüne CI gemergt; #20 mit Verweis auf ADR-013 geschlossen. Pakete 1–4 aus #34 sind umgesetzt.
- **README-Synchronisation:** Methodik-Stand um #41 ergänzt; Überblick und Dateibaum im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 6/6 grün; der neue Anker „Anforderung → Schritt" ist für dieses Repo nicht anwendbar (Klasse K).
- **Phasenumfang:** Bündel #34 bei 9 Schritten, Schwelle mehr als 10.
- **Ablaufdaten-Register:** unverändert; Reset-Datum des Actions-Kontingents weiterhin offen.
- **Modell-Bilanz:** Laufzeitabfrage: `claude-opus-5-5` (Entscheidungs-Klasse). S-21: Empfehlung Entscheidung, eingehalten. Nachführen der Dokumente: Routine, als kleine Teilarbeit nicht abgegeben. Keine Abgabe an Unteragenten. Sitzungszähler: 304,97 → 308,37 $ Listenpreis-Äquivalent.
- **Offen geblieben:** Punkt 2 aus #34 (Betriebsmodus); Abschluss des Bündels #34 mit Vision-Abgleich und Pflichtfrage; S-17, S-18, S-19; S-7-Nebenbefund; Kommentar in Zeile 3 von `CLAUDE.md`.
- **Nächster Schritt:** Entscheidungsvorlage zu Punkt 2 (Betriebsmodus).

### 2026-09-24 19:40 – [ADR-ANGELEGT]

- **ADR:** ADR-013 – Anforderungsschicht zwischen Vision und Architektur, Schutzbedarf als Obergrenze, Kostenrahmen
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Wahl des Eigentümers zu S-21: Option D. Reaktiv-Quote 1/13 (8 %).
- **Grep-Gegenprobe:** „sechs Dokumente" (Zweck von Modus 2) und „sieben Dokumente" (Klassen M, G in §2.2) durch „übrige Pflicht-Dokumente" bzw. „alle Pflicht-Dokumente" ersetzt – Zählungen veralten bei jedem neuen Dokument. Kopf und Initialisierungshinweis der Entscheidungs-Vorlage von drei auf vier Teile angepasst. Vorbereitungs-Absatz in Modus 2 nennt `requirements.md` als Datei ab Klasse M. Dateibäume in `README.md` und `templates/README.md` ergänzt. Die Zeile zu #22 im Methodik-Stand („sechs Pflicht-Dokumente") ist historisch und bleibt.
- **Bewusst nicht eingeführt:** eine neue Freigabe-Kategorie für Anforderungs-Änderungen (in #20 als Lücke genannt). Das Streichen einer Muss-Anforderung ist über die Landeplatz-Regel bereits ein Descope mit ADR; eine neunte Kategorie hätte den Eskalations-Auslöser „alle acht Kategorien" in §0 mitgeändert.

### 2026-09-24 19:05 – [ENTWURF-VORGELEGT]

- **Schritt:** S-21 – Paket 4 aus #34 (Punkte 3, 6, 12) zusammen mit #20.
- **Optionen:** A – in vorhandene Dokumente einweben. B – eigene Anforderungs- und BA-Vorlagen, gestaffelt nach Klasse. C – eigener Zwischenschritt „Modus 1.5" für alle Klassen. D – B für alle Klassen ab M, zusätzlich Modus 1.5 für G und V (empfohlen; entspricht der Tendenz in #20, gestützt durch die Pilot-Belege aus #34 Punkt 3).
- **Wucherungs-Prüfung (Kriterium 9) beim Anlegen:** Bündel #34 jetzt 9 Schritte bei Plan 5, Schwelle mehr als 10 – keine Auslösung.
- **Modell:** `claude-opus-5-5` (Laufzeitabfrage 18:45), Entscheidungs-Klasse; Empfehlung für S-21: Entscheidung – eingehalten.

### 2026-09-24 19:00 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 18:45, PR #40 gemergt (`main` = `40e9f5e`).
- **Modell:** `claude-opus-5-5`, Entscheidungs-Klasse (Laufzeitabfrage 18:45).
- **Geplant:** Paket 4 mit #20 als S-21 zur Entscheidung vorlegen.

### 2026-09-24 18:45 – [SESSIONENDE]

- **Dauer:** ca. 18:05–18:45
- **Bearbeitet:** S-20 (Paket 3 aus #34) – vorgelegt, Option B freigegeben, umgesetzt, [ERLEDIGT] mit CI-Ausnahme.
- **Erreichter Stand:** Stopp-Kriterium 9 (Phasen-Wucherung), Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?" durch getrennte Instanz, Reaktiv-Klassifikation in der STABILISIERUNG, Drift-Anker „Phasenumfang"; ADR-012. PR [#40](https://github.com/Paddel87/Dev-Templates/pull/40) als sechste dokumentierte Ausnahme ohne grüne CI gemergt. Pakete 1–3 aus #34 sind umgesetzt.
- **README-Synchronisation:** Methodik-Stand um #40 ergänzt; Überblick im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 6/6 grün (erstmals mit „Phasenumfang").
- **Ablaufdaten-Register:** unverändert; Reset-Datum des Actions-Kontingents weiterhin offen.
- **Modell-Bilanz:** Laufzeitabfrage vor dem Sessionende: `claude-opus-5-5` (Entscheidungs-Klasse). S-20: Empfehlung Entscheidung, eingehalten. Nachführen von Fahrplan, README, Logbuch: Routine, als kleine Teilarbeit nicht abgegeben. Keine Abgabe an Unteragenten. Sitzungszähler: 302,85 → 304,97 $ Listenpreis-Äquivalent.
- **Offen geblieben:** Paket 4 (wartet auf Optionswahl in #20), Punkt 2 (Betriebsmodus), S-17, S-18, S-19, S-7-Nebenbefund, Kommentar in Zeile 3 von `CLAUDE.md`.
- **Nächster Schritt:** Optionswahl in #20 durch den Eigentümer, dann Paket 4; alternativ die Grundsatzfrage Betriebsmodus.

### 2026-09-24 18:30 – [ADR-ANGELEGT]

- **ADR:** ADR-012 – Stopp bei Phasen-Wucherung, Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?", Reaktiv-Klassifikation in der STABILISIERUNG
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Freigabe des Eigentümers zu S-20, Option B. Reaktiv-Quote 1/12 (8 %).
- **Grep-Gegenprobe:** Zählangaben zu den Stopp-Kriterien (etwa „acht Kriterien") und zur Drift-Prüfung (etwa „fünf Anker") gibt es in `CLAUDE.md`, `README.md` und `templates/` nicht – die neuen Einträge (Kriterium 9, Zeile „Phasenumfang") ändern keine Zahl im Text. Frühere Logbuch-Einträge nennen „5/5" für den Drift-Check; ab jetzt hat die Prüfung sechs Anker. „Phasengrenze" in `templates/` kommt nicht vor; der Phasenkopf der Fahrplan-Vorlage ist die einzige Stelle mit Phasen-Metadaten und wurde ergänzt. Die §0-Eskalation deckt die Pflichtfrage über Auslöser 1 ab (Entscheidungsblock), ein eigener Auslöser ist nicht nötig.
- **Erste Anwendung:** Bündel „Umsetzung von Issue #34" im Fahrplan erfasst – 8 Schritte bei einem Plan von 5 Einheiten, Schwelle mehr als 10. Knapp unter der Schwelle; die Nebenschritte S-17 bis S-19 zählen mit.

### 2026-09-24 18:10 – [ENTWURF-VORGELEGT]

- **Schritt:** S-20 – Paket 3 aus #34 („Roter Faden": Punkte 1 und 4).
- **Optionen:** A – wie in #34 vorgeschlagen, die bauende KI bewertet die Umbau-Frage selbst. B – wie A, aber die Umbau-Frage bereitet eine getrennte Instanz vor (empfohlen). C – Umbau-Frage nur beim Wucherungs-Auslöser, nicht an jeder Phasengrenze.
- **Voreinstellungen zur Nebenfrage:** Faktor 2 mit Mindestzuwachs von 5 Schritten; automatische Reaktiv-Zählung nur für Architekturentscheidungen (Kategorien 1, 2, 4, 5 aus `CLAUDE.md` §4), nicht für Methodik-ADRs.
- **Modell:** laut Laufzeitabfrage 17:55 `claude-opus-5-5`, Entscheidungs-Klasse; Empfehlung für S-20: Entscheidung – eingehalten.

### 2026-09-24 18:05 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 17:55, PR #39 gemergt (`main` = `dd00873`).
- **Modell:** eingestellt und bedient `claude-opus-5-5` (Laufzeitabfrage 17:55), Entscheidungs-Klasse.
- **Geplant:** Paket 3 aus #34 als S-20 anlegen und vorlegen, keine Umsetzung vor Freigabe.

### 2026-09-24 17:55 – [SESSIONENDE]

- **Dauer:** ca. 16:45–17:55
- **Bearbeitet:** S-16 (Paket 2, Stufe 2 aus #34) – Entwurf, Option C freigegeben, umgesetzt mit Probelauf, [ERLEDIGT] mit CI-Ausnahme. Neu: S-19 (Erkundung Ersparnis und Hook).
- **Erreichter Stand:** Vier Modellklassen in `CLAUDE.md` §0, Abgabe an Unteragenten, Modell-Abfrage beim Sessionstart, Bilanz beim Sessionende; Vorlagen nachgezogen; aktuelle Modellzuordnung; ADR-011. PR [#39](https://github.com/Paddel87/Dev-Templates/pull/39) als fünfte dokumentierte Ausnahme ohne grüne CI gemergt. Paket 2 aus #34 ist damit vollständig.
- **README-Synchronisation:** Methodik-Stand um #39 ergänzt; Abschnitt „Reguläre Sessions" im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 5/5 grün.
- **Ablaufdaten-Register:** unverändert ein Eintrag (Actions-Kontingent), Schritt S-17 vorhanden; Reset-Datum weiterhin offen.
- **Modell-Bilanz (erste Anwendung):** Laufzeitabfrage vor dem Sessionende: eingestellt und bedient `claude-opus-5-5` (Entscheidungs-Klasse). S-16 selbst: Empfehlung Entscheidung, eingehalten. Das Nachführen von Fahrplan, README und Logbuch ist Routine und lief oberhalb der Empfehlung auf der Entscheidungs-Klasse – als kleine Teilarbeit im geladenen Kontext nach §0 nicht abgegeben. An Unteragenten abgegeben: drei Probeaufgaben (zwei Routine, eine Mechanik). Der Sitzungszähler der Laufzeitumgebung stieg über Entwurf und Umsetzung von S-16 von 299,02 auf 302,85 $ Listenpreis-Äquivalent; ob die Unteragenten darin enthalten sind, ist nicht dokumentiert – Datenpunkt für S-19.
- **Offen geblieben:** S-17 nach dem Reset; S-18 als Vorschlag; S-19; Nebenbefund S-7 (Feld „Betroffene Bestandteile" fehlt); Kommentar in Zeile 3 von `CLAUDE.md`.
- **Nächster Schritt:** Paket 3 aus #34 („Roter Faden": Punkte 1 und 4) als Fahrplan-Schritt anlegen und vorlegen.

### 2026-09-24 17:40 – [ADR-ANGELEGT]

- **ADR:** ADR-011 – Vier Modellklassen, empfohlene Klasse je Schritt, Abgabe an Unteragenten
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Freigabe des Eigentümers zu S-16, Option C. Reaktiv-Quote 1/11 (9 %).
- **Probelauf mit erzwungenem Fehler:** Kopie des Repos im Arbeitsverzeichnis der Session, zwei Fehler eingebaut (`docs/decisions.md`: Reaktiv-Quote „2 / 10", ADR-010 verweist auf „S-96"). Drei Unteragenten, nur lesend:
  - Routine-Klasse (Sonnet 5), Drift-Prüfung nach §16: 3/5 grün, beide Fehler mit Zeile und Soll-Wert gefunden; zusätzlicher echter Befund: S-7 hat kein Feld „Betroffene Bestandteile". Etwa 71 s, 168.355 Token laut Laufzeit.
  - Mechanik-Klasse (Haiku 4.5), Zählen und Abgleichen: beide Fehler gefunden, 10 ADRs / 1 REAKTIV / 18 Schritte mit korrektem Status. Etwa 40 s, 115.643 Token.
  - Routine-Klasse (Sonnet 5), Schreibprobe README-Eintrag zu #38: sachlich korrekt und belegt, aber ohne „Abkündigung wird Schritt mit Frist" und ohne „ausgereifte Linie". Etwa 16 s, 131.796 Token.
- **Folgerung:** Qualität für Prüf- und Zählaufgaben belegt, für Schreibaufgaben mit Nachprüfung. Die Token-Zahlen zeigen, dass jeder Unteragent seinen Kontext neu lädt – kleine Teilarbeiten lohnen die Abgabe vermutlich nicht. In §0 festgehalten; die Messung der tatsächlichen Ersparnis ist S-19.
- **Grep-Gegenprobe:** „Rückstuf" (Modellklassen-Sinn) in `CLAUDE.md`, `README.md`, Vorlagen und `docs/decisions.md` Regel-001 angepasst; übrige Treffer betreffen die Reifegrad-Rückstufung, nicht die Modellklassen. `README.md` Abschnitt 4 („Reguläre Sessions") beschreibt jetzt vier Klassen. Die Methodik-Stand-Zeile zu #21 bleibt als historischer Eintrag unverändert. Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md`: leer.
- **Nebenbefund, nicht geändert:** S-7 hat kein Feld „Betroffene Bestandteile" (vom Routine-Unteragenten gefunden). Beim nächsten Pflegeschritt an S-7 ergänzen.

### 2026-09-24 16:50 – [ENTWURF-VORGELEGT]

- **Schritt:** S-16 – Punkt 15 aus #34 (Modellwahl nach Aufgabe) samt Ergänzung „Modell erkennen".
- **Kern des Entwurfs:** vier Klassen in `CLAUDE.md` §0 ohne Modellnamen (Mechanik, Routine, Entscheidung, Ausnahme) mit objektiven Auslösern; Feld „Empfohlene Klasse" je Fahrplan-Schritt; Modell-Feststellung über die Laufzeitumgebung beim Sessionstart und Abgleich je Schritt; Sessionende-Bilanz (Schritte über der Empfehlung); Zuordnung und Bezugsmodell in `project-context.md`; Mechanik-Klasse erst nach Probelauf; Hook-Frage als eigener Erkundungsschritt.
- **Optionen für Arbeit oberhalb der Empfehlung:** A – echter Stopp wie bei der Eskalation. B – Warnung mit Bilanz am Sessionende, kein Stopp. C – Routinearbeit wird strukturell an einen Unteragenten mit fest eingestelltem günstigerem Modell abgegeben, Rest wie B (empfohlen).
- **Beleg aus dieser Session:** Die Laufzeitabfrage (`get_session`) liefert konfiguriertes und bedientes Modell (`claude-opus-5-5`) sowie das Fenster des 5-Stunden-Limits mit Zurücksetz-Zeitpunkt; ein Wochenlimit zeigt sie nicht. Die gesamte Session – auch Logbuch-, README- und Fahrplanpflege – lief auf der Entscheidungs-Klasse. Damit bestätigt die Session selbst den Befund aus Punkt 15.

### 2026-09-24 16:45 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 16:30, PR #38 gemergt (`main` = `c467ce5`).
- **Modell (Laufzeitabfrage):** konfiguriert und bedient `claude-opus-5-5` (Entscheidungs-Klasse).
- **Geplant:** Entwurf für S-16 vorlegen, keine Umsetzung vor Freigabe.

### 2026-09-24 16:30 – [SESSIONENDE]

- **Dauer:** ca. 15:15–16:30
- **Bearbeitet:** S-15 (Paket 2, Stufe 1 aus #34) – vorgelegt, Option B freigegeben, umgesetzt, [ERLEDIGT] mit CI-Ausnahme. Neu angelegt: S-16 (Punkt 15), S-17 (CI-Nachlauf), S-18 (Vorschlag Beispielversionen).
- **Erreichter Stand:** „Warnungen und Abkündigungen" und „Versionswahl" in `CLAUDE.md` §15, Präzisierung von „CI grün" in §9, Ablaufdaten-Register als §12 Punkt 8, Vorlagen nachgezogen, ADR-010. PR [#38](https://github.com/Paddel87/Dev-Templates/pull/38) auf Anweisung des Eigentümers als vierte dokumentierte Ausnahme ohne grüne CI gemergt.
- **README-Synchronisation:** Methodik-Stand um #38 ergänzt; Überblick („Was Qualität minimal heißt") im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 5/5 grün.
- **Ablaufdaten-Register (neuer Punkt 8, erste Anwendung):** ein Eintrag (Actions-Kontingent), Vorlauf überschritten, Schritt S-17 vorhanden – in Ordnung. Das genaue Reset-Datum fehlt noch (Angabe des Eigentümers).
- **Offen geblieben:** S-16 Entwurf; S-17 nach dem Reset; S-18 als Vorschlag; Kommentar in Zeile 3 von `CLAUDE.md` (Beobachtung 12:40).
- **Nächster Schritt:** S-16 – Entwurf zu Punkt 15 samt Frage „anhalten oder warnen" vorlegen.

### 2026-09-24 16:10 – [ADR-ANGELEGT]

- **ADR:** ADR-010 – Warnungen als Fehler, Ablaufdaten-Register, Versionswahl „ausgereifte Linie"
- **Tag:** `[STRATEGISCH]` `[METHODIK]` `[STACK]`
- **Auslöser:** Freigabe des Eigentümers zu S-15, Option B. Stufe 1 (Punkte 7, 10, 14) umgesetzt, Punkt 15 als S-16 angelegt. Reaktiv-Quote 1/10 (10 %).
- **Grep-Gegenprobe (Lehre aus S-10):** „sieben Punkten" in `CLAUDE.md` §12 auf „acht" angepasst. „Versions-Verifikation", „Verifiziert", „offiziellen Quelle" und „Trainingsstand" geprüft: `templates/docs/project-context.md` §3 legte die Prüfung beim Menschen ab – angepasst; `README.md` Zeile zu Modus 2 bleibt gültig. `pnpm lint` in der TS-CI-Vorlage durch einen direkten ESLint-Aufruf mit Obergrenze ersetzt, weil der Inhalt eines projekteigenen `lint`-Skripts nicht prüfbar ist. Neutralitäts-Grep über die neuen Zeilen in `CLAUDE.md`: keine Werkzeug- oder Modellnamen.
- **Landeplatz-Befund:** Der CI-Nachlauf für #35–#37 stand seit drei Sessionenden nur als „offen geblieben" im Logbuch – ohne Schritt-ID. Jetzt S-17, verknüpft mit dem neuen Ablaufdaten-Register dieses Repos.
- **Nebenbefund, nicht geändert:** `templates/pre-commit/typescript.yaml` pinnt Prettier auf `v4.0.0-alpha.8` (mit `TBD: aktuelle stabile Version pinnen`). Als Platzhalter gekennzeichnet und in Modus 2 Schritt 10 zu ersetzen; nach der neuen Versionsregel aber ein schlechtes Beispiel. Als Vorschlag S-18 im Fahrplan notiert (zusammen mit den Beispielversionen Node 20, pnpm 9, Python 3.12 in den CI-Vorlagen), nicht Teil von S-15.

### 2026-09-24 15:20 – [ENTWURF-VORGELEGT]

- **Schritt:** S-15 – Paket 2 aus #34 („Stille Fehler": Punkte 7, 10, 14, 15).
- **Optionen:** A – ganzes Paket in einem Schritt. B – zwei Stufen: zuerst 7, 10, 14 (mechanisch, hohe Konfidenz), danach 15 mit Probelauf der Mechanik-Klasse (empfohlen). C – zwei Stufen in umgekehrter Reihenfolge: 15 zuerst.
- **Vorab zur Stufe mit Punkt 15 offen:** Ob die KI bei Arbeit oberhalb der empfohlenen Klasse anhält oder nur warnt – wird mit dieser Stufe vorgelegt.
- **Nebenbefund, behoben:** Die Zeile „Aktiver Schritt" im Fahrplan enthielt seit dem Merge von #37 einen Rest der S-13-Meldung (Folge einer Teilersetzung beim Nachführen). Mit diesem Commit bereinigt.
- **Modellklasse:** `claude-opus-5-5`, Opus-Linie = Entscheidungs-Klasse; Eskalations-Auslöser 1 erfüllt.

### 2026-09-24 15:15 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 15:00, PR #37 gemergt (`main` = `5a471aa`).
- **Geplant:** Paket 2 aus #34 als S-15 anlegen und zur Freigabe vorlegen, keine Umsetzung vor Freigabe.

### 2026-09-24 15:00 – [SESSIONENDE]

- **Dauer:** ca. 13:55–15:00
- **Bearbeitet:** S-14 (Paket 1, Stufe 2 aus #34) – Entwurf vorgelegt, Option A freigegeben, umgesetzt, [ERLEDIGT] mit CI-Ausnahme.
- **Erreichter Stand:** Gate vor dem ersten öffentlichen Deployment in `CLAUDE.md` §12, DoD-Zeile in §9, Sicherheitsgrundriss in Modus 2, fünf Vorlagen nachgezogen, ADR-009. PR [#37](https://github.com/Paddel87/Dev-Templates/pull/37) auf Anweisung des Eigentümers als dritte dokumentierte Ausnahme ohne grüne CI gemergt. Paket 1 aus #34 ist damit vollständig.
- **README-Synchronisation:** Methodik-Stand um #37 ergänzt; Überblick und Modus-2-Beschreibung bereits im Umsetzungs-Commit angepasst.
- **Inter-Pflicht-Drift-Check:** 5/5 grün. Der Nebenbefund von 14:40 („Parallelisierbarkeit" nannte S-7 als letzten offenen Schritt) ist behoben.
- **Offen geblieben:** nachträglicher CI-Lauf auf `main` nach dem Kontingent-Reset (#35, #36, #37); Modellzuordnung in `docs/project-context.md` Abschnitt 6 (#34 Punkt 15); Kommentar in Zeile 3 von `CLAUDE.md` (Beobachtung 12:40). Erprobung des Gates an einem echten Projekt steht aus.
- **Nächster Schritt:** Paket 2 aus #34 („Stille Fehler": 7, 10, 14, 15) als Fahrplan-Schritt anlegen und zur Freigabe vorlegen.

### 2026-09-24 14:40 – [ADR-ANGELEGT]

- **ADR:** ADR-009 – Gate vor dem ersten öffentlichen Deployment, Sicherheitsgrundriss in Modus 2
- **Tag:** `[STRATEGISCH]` `[METHODIK]` `[SECURITY]` `[DEPLOYMENT]`
- **Auslöser:** Freigabe des Eigentümers zu S-14, Option A. Reaktiv-Quote 1/9 (11 %), unter dem Schwellenwert.
- **Grep-Gegenprobe (Lehre aus S-10):** „Sicherheits-Review" stand in `templates/docs/fahrplan.md` als STABILISIERUNG-Kriterium – angepasst auf „nachgeprüft". „Pflegehinweise" im Runbook verschiebt sich auf Abschnitt 8; keine Fundstelle verweist mit Nummer darauf. Die Modus-2-Beschreibung in `README.md` (Überblick und Triggerphrasen-Zeile) nannte den Sicherheitsgrundriss nicht – ergänzt. `CLAUDE.md` §3 Runbook-Zeile um den Notfall-Abschnitt ergänzt. Übrige Fundstellen von „Bedrohungsmodell" und „Notfall" stammen aus dieser Änderung.
- **Nebenbefund, nicht geändert:** `docs/fahrplan.md` Abschnitt „Parallelisierbarkeit" nennt S-7 noch als „letzten verbleibenden offenen Schritt" – veraltet seit S-12. Kein Teil von S-14; beim nächsten Sessionende mit dem Drift-Check zu bereinigen.

### 2026-09-24 14:00 – [ENTWURF-VORGELEGT]

- **Schritt:** S-14 – Gate vor dem ersten öffentlichen Deployment (Punkte 5, 8, 11 aus #34).
- **Optionen:** A – ein einheitliches, blockierendes Gate für jedes öffentlich erreichbare Deployment, Verzicht auf einzelne Punkte nur per ADR (empfohlen). B – gestuft: Kern für alle, unabhängige und externe Prüfung nur bei personenbezogenen Daten oder Anmeldung. C – Gate erst vor Go-Live statt vor dem ersten öffentlichen Deployment.
- **Inhalt des Entwurfs:** neuer Unterabschnitt in `CLAUDE.md` §12 mit acht Prüfpunkten (Bedrohungsmodell, Sicherheitsniveau, Grundhärtung Host, Secrets und KI-Zugriff, Backup mit erprobter Wiederherstellung, unabhängige Prüfung, Vertretung und Notfall-Handbuch, KI im Betrieb samt Kontingent); eine DoD-Zeile in §9 für die unabhängige Prüfung; Sicherheitsgrundriss als neuer Schritt in Modus 2; Rubriken in `templates/docs/architecture.md` §6; Security-Review in `templates/docs/fahrplan.md` vor das erste Deployment; Betriebsangaben in `templates/docs/project-context.md` §8; Notfall-Abschnitt in `templates/docs/onboarding-runbook.md`.
- **Landeplatz-Befund:** Der dritte Vorschlag aus #34 Punkt 9 („Produktionszugriff des Agenten beschränken und festhalten") wurde in S-13 nicht umgesetzt und hatte keinen Landeplatz. Er ist in diesen Entwurf aufgenommen (Prüfpunkt 4).
- **Modellklasse:** `claude-opus-5-5`, Opus-Linie = Entscheidungs-Klasse; Eskalations-Auslöser 1 erfüllt, kein Wechsel nötig.

### 2026-09-24 13:55 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-09-24 13:10, PR #36 gemergt (`main` = `1b9977f`).
- **Geplant:** Entwurf für S-14 zur Freigabe vorlegen, keine Umsetzung vor Freigabe.

### 2026-09-24 13:10 – [SESSIONENDE]

- **Bearbeitet:** S-13 (Paket 1, Stufe 1 aus #34) – [ERLEDIGT] mit CI-Ausnahme; S-14 als [OFFEN] angelegt.
- **Erreichter Stand:** Zwei neue harte Regeln in `CLAUDE.md` §6 (Secrets nicht in der eigenen Ausgabe; Schutzmechanismen durch erzwungenen Fehler belegen), Ausnahme in der Beförderungsregel von `templates/docs/architecture.md`, ADR-008. PR [#36](https://github.com/Paddel87/Dev-Templates/pull/36) auf Anweisung des Eigentümers als zweite dokumentierte Ausnahme ohne grüne CI gemergt (Actions-Kontingent erschöpft, wie bei #35).
- **README-Synchronisation:** Methodik-Stand um #36 ergänzt; übrige Blöcke ohne Drift.
- **Inter-Pflicht-Drift-Check:** 5/5 grün – ADR-008 → S-13/S-14 existieren; Reaktiv-Quote 1/8 stimmt; kein Reifegrad-Wechsel; keine Blocker; Modul-Liste unverändert.
- **Offen geblieben:** Nachträglicher CI-Lauf auf `main` nach dem Kontingent-Reset (betrifft #35 und #36). Die Modellzuordnung in `docs/project-context.md` Abschnitt 6 nennt noch ältere Modellversionen (in #34 Punkt 15 vermerkt). Der Kommentar in Zeile 3 von `CLAUDE.md` (Beobachtung 12:40).
- **Nächster Schritt:** S-14 – Entwurf des Gates vor dem ersten öffentlichen Deployment (Punkte 5, 8, 11) erneut zur Freigabe vorlegen.

### 2026-09-24 12:40 – [BEOBACHTUNG]

Beim Grep auf Werkzeug- und Modellnamen in `CLAUDE.md` (Constraint „Werkzeug- und Modellneutralität", `project-context.md` Abschnitt 6) gibt es einen Treffer: Zeile 3 ist ein HTML-Kommentar „Verbindliches Regelwerk für Claude Code in diesem Repository." Er stammt nicht aus S-13 und stand schon vorher dort. Formal verletzt er den Constraint; da er ein Kommentar und keine Regel ist, wird er hier nur vermerkt und nicht im Rahmen von S-13 geändert (keine heimliche Scope-Erweiterung).

### 2026-09-24 12:35 – [ADR-ANGELEGT]

- **ADR:** ADR-008 – Secrets nicht in der eigenen Ausgabe; Schutzmechanismen durch erzwungenen Fehler belegen
- **Tag:** `[STRATEGISCH]` `[METHODIK]` `[SECURITY]`
- **Auslöser:** Freigabe des Eigentümers zu S-13, Option B (zwei Stufen). Stufe 1 umgesetzt: zwei harte Regeln in `CLAUDE.md` §6, Ausnahme in der Beförderungsregel von `templates/docs/architecture.md`. Stufe 2 als S-14 angelegt. Reaktiv-Quote 1/8 (12,5 %), unter dem Schwellenwert von 30 %.
- **Grep-Gegenprobe (Lehre aus S-10):** Weitere Fundstellen zu Secrets (`CLAUDE.md` §4 Nr. 6) und zur Beförderung (§0 Auslöser 4, §6 „Architektur-Reifegrad respektieren") geprüft – keine widerspricht den neuen Regeln. Die Reifegrad-Skala des Tooling-Inventars (`[ROH]`/`[GEHÄRTET]`) bleibt unberührt; sie betrifft Skripte, nicht Schutzmechanismen als Architektur-Bestandteile.

### 2026-09-24 12:00 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` vom 2026-09-23 17:30; danach die CI-Ausnahme und der Merge von PR #35
- **Geplant für diese Session:** S-13 anlegen – Paket 1 („Sicherheit und Betrieb") aus der Triage von Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34) als Freigabe-Vorlage. Zwischen den beiden Sessions wurde #34 um die Punkte 7–15, zwei Ergänzungen und die Triage des Eigentümers erweitert (ohne Änderung an diesem Repo).
- **Vorabprüfung:** `git fetch` – `origin/main` = `bc45c95` (Merge #35). Der Session-Branch war vollständig gemergt und wurde von `origin/main` neu angesetzt. Fremd-Branches unverändert gegenüber dem Vortag (`claude/s7-drift-check-hpsg8b` weiter ungemergt).
- **Modus / Werkzeug:** Claude Code, `claude-opus-5-5` (Opus-Linie, Entscheidungs-Klasse). S-13 ist ein Eskalations-Auslöser nach Abschnitt 0 Punkt 1; er ist auf dieser Klasse erfüllt, kein Modellwechsel nötig.

### 2026-09-23 17:45 – [BEOBACHTUNG]

- **CI ohne Runner:** Die CI von Dev-Templates läuft auf `ubuntu-latest` und damit auf dem Actions-Kontingent des Kontos, das laut Eigentümer und ADR-213 im Pilotprojekt seit dem 2026-09-15 erschöpft ist. Das Pilotprojekt ist deshalb auf einen eigenen Runner ausgewichen, der nur für dieses Repository registriert ist; Dev-Templates kann ihn nicht mitbenutzen. Symptom hier: Job nach 3–6 s `failure`, `runner_id: 0`, keine Schritte, Logs HTTP 404 – dasselbe Bild wie die CI-Aussetzung im Pilotprojekt im Juli.
- **Entscheidung des Eigentümers:** Option A (auf den Reset warten, Pipeline unverändert) und PR #35 als einmalige Ausnahme ohne grüne CI mergen. Vermerkt in S-12. Kein ADR, weil die Pipeline nicht geändert wird.
- **Lerneffekt:** Das Kontingent ist eine Ressource, die sich zwei Repos desselben Kontos teilen, ohne dass eines vom Verbrauch des anderen weiß. Die Ausweichlösung im Pilotprojekt hat das Problem dort gelöst und hier unsichtbar gemacht.

### 2026-09-23 17:30 – [SESSIONENDE]

- **Session-Dauer:** eine Session; überwiegend Analyse (Vergleich Vorlage ↔ Pilotprojekt, Ursachen der Phase-7-Wucherung, Sicherheit, Datenschutz), danach eine Änderung an diesem Repo
- **Bearbeitet:** S-12 (README-Zielgruppe) auf `[ERLEDIGT]`. Außerhalb des Fahrplans: Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34) angelegt – sechs Methodik-Lücken aus dem Pilotprojekt (Auslöser zum Neuplanen, Betriebsmodus, Anforderungsschicht, Umbau-Frage an Phasengrenzen, Sicherheitsgrundriss und Deploy-Gate, Schutzbedarf als Obergrenze)
- **Erreicht:** `README.md` „Für wen es ist" und `docs/project-context.md` Abschnitt 2 beschreiben jetzt dieselbe, geschärfte Zielgruppe, den Erprobungsstand und die Grenzen. `pre-commit` grün, Drift-Check 5/5 grün, Reaktiv-Quote unverändert 1/7.
- **Offen geblieben:**
  - Issue #34 ist nicht triagiert; jeder der sechs Punkte braucht eine Eigentümer-Entscheidung und einen eigenen ADR.
  - Die Modellklassen-Zuordnung in `project-context.md` Abschnitt 6 nennt veraltete Modelle (Opus 4.7 / Sonnet 4.6); diese Session lief auf `claude-opus-5-5`. Aktualisierung ist Sache des Eigentümers, hier nur vermerkt.
  - Branch `claude/s7-drift-check-hpsg8b` (siehe `[BEOBACHTUNG]` unten).
- **Nächster Schritt:** Triage von Issue #34 durch den Eigentümer; Klärung, was mit dem S-7-Branch geschieht.
- **Stimmung / Beobachtung:** Der Leitsatz aus Issue #34 steht in der README bewusst als Anspruch mit Verweis, nicht als Regel. Eine README, die eine noch nicht beschlossene Regel als geltend darstellt, wäre genau die Sorte Spiegel-Drift, gegen die `CLAUDE.md` Abschnitt 16 schützen soll.

### 2026-09-23 17:25 – [BEOBACHTUNG]

Der Sichtbarkeits-Check beim Sessionstart (Vorschlag aus Issue #33, hier freiwillig ausgeführt) hat in **diesem** Repo einen Fall genau der Art gefunden, die er verhindern soll: `origin/claude/s7-drift-check-hpsg8b` trägt seit dem 2026-08-13 einen ungemergten Commit (`db3d72c`) mit einem fertigen Drift-Check-Skript (`scripts/check-drift.py`, Tests, `ruff` im Pre-Commit) und README-Einträgen für PR #30 und #31. Der Fahrplan auf `main` führt S-7 dagegen als `[OFFEN]`, „noch nicht angefordert". Außerdem vergibt der Branch die Nummer **ADR-006**, die auf `main` inzwischen für die Vision-Verlust-Lücke belegt ist – ein Merge würde kollidieren. Eine Session, die nur `main` liest, würde S-7 neu beginnen. Kein Eingriff in dieser Session; der Befund stützt Issue #33 mit einem zweiten, repo-eigenen Beleg.

### 2026-09-23 17:00 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` vom 2026-08-28 23:40 (S-10, S-11, PR #32)
- **Geplant für diese Session:** S-12 – README-Abschnitt „Für wen es ist" schärfen, auf Grundlage der Auswertung des Pilotprojekts und der Schilderung des Eigentümers zur Entstehung der Methodik. Vorausgegangen in derselben Session (ohne Änderung an diesem Repo): Anlage von Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34) mit sechs Methodik-Lücken aus Phase 7 des Pilotprojekts.
- **Vorabprüfung:** `git fetch --prune` – lokaler Stand deckungsgleich mit `origin/main` (`849c64a`). Fremd-Branches geprüft: `origin/claude/s7-drift-check-hpsg8b` trägt einen ungemergten Commit vom 2026-08-13 (`db3d72c`, S-7 Drift-Check-Skript), der auch `README.md` berührt – aber nur den Block „Methodik-Stand", nicht „Für wen es ist". Kein Überschneidungs-Konflikt mit S-12. Pflichtlektüre nach Abschnitt 2 vollständig durchlaufen.
- **Modus / Werkzeug:** Claude Code. Laufzeit-Abfrage meldet `claude-opus-5-5` (konfiguriert und zuletzt bedient). Das Modell ist in der Zuordnung in `project-context.md` Abschnitt 6 nicht genannt (dort Opus 4.7 / Sonnet 4.6); nach der Definition „stärkstes verfügbares Modell" gilt es als **Entscheidungs-Klasse**. S-12 ist Formulierungsarbeit ohne Regelwirkung, kein Eskalations-Auslöser.

### 2026-08-28 23:40 – [SESSIONENDE]

- **Session-Dauer:** eine Session, zwei Themen: Vergleichsanalyse Vorlage ↔ Pilotprojekt, danach Übernahme zweier Befunde
- **Bearbeitet:** S-10 (Vision-Verlust-Lücke, ADR-006) und S-11 (Teil-C-Lücke, ADR-007), beide auf `[ERLEDIGT]`; zugestellt als PR [#32](https://github.com/Paddel87/Dev-Templates/pull/32)
- **Erreicht:** `CLAUDE.md` kennt jetzt die Landeplatz-Pflicht (§6), den Marker `[VERSCHOBEN]` (§7) und zwei Vision-Checkpoints (§12); Teil C von `decisions.md` ist Mindest-Lektüre (§2, §14). Folgestellen mitgezogen: §3 Dokumenten-Index, die Vision-Ausnahme in §2, Schritt-Format und Kopfkommentar der Vorlagen. Verifikation: `pre-commit` grün (0 Findings, 21 Dateien), Marker-Sets Regelwerk ↔ Vorlage deckungsgleich (7/7), Inter-Pflicht-Drift-Check 5/5 grün, Reaktiv-Quote 1/7 (14 %) unter dem Schwellenwert.
- **Offen geblieben:** S-7 (Drift-Check-Skript) unverändert `[OFFEN]`, freigabepflichtig. Aus derselben Rückkopplungs-Quelle weiterhin **nicht** übernommen und bewusst nicht in dieser Session mitgenommen: A2 (WIP-Branch-Sichtbarkeit, gehört mit Issue [#14](https://github.com/Paddel87/Dev-Templates/issues/14) zusammen, weil beides derselbe Fehlermodus ist), das BDR-Register und der Befund aus Issue [#20](https://github.com/Paddel87/Dev-Templates/issues/20) zur fehlenden Requirements-/Business-Analyse-Schicht.
- **Nächster Schritt:** Entscheidung des Eigentümers, ob A2 + Issue #14 als **ein** Schritt angelegt werden. Danach S-7.
- **Stimmung / Beobachtung:** Zwei Dinge sind in dieser Session durch bloßes Hinsehen aufgefallen, nicht durch eine Prüfung: die Folgewirkung von A1.3 auf zwei andere Abschnitte und die zweite Teil-C-Fundstelle im Vorlagen-Kopfkommentar. Beide hätte kein bestehendes Gate gefangen – der Linter prüft Form, der Drift-Check prüft die fünf definierten Anker. Semantische Widersprüche zwischen zwei Stellen desselben Dokuments fängt bislang nichts. Als Anforderung an S-7 vorgemerkt, aber ohne Illusion: Das ist schwerer zu automatisieren als die fünf Anker.

### 2026-08-28 23:20 – [BEOBACHTUNG]

Beide Änderungen dieser Session waren zum Zeitpunkt ihrer Umsetzung **längst bekannt** – A1 seit dem 2026-06-25 im Pilotprojekt produktiv, die Teil-C-Lücke dort seit Monaten notdürftig umschifft. Liegengeblieben sind sie aus zwei verschiedenen Gründen, die beide nichts mit ihrer Schwere zu tun haben:

- **A1** kam als Sammel-Issue, das den schweren Befund mit acht bereits erledigten Patches bündelte. Das Issue sah dadurch weitgehend abgearbeitet aus. Ein Container ohne eigene Landeplätze garantiert seinen Inhalt nicht – **exakt das Versagensmuster, das A1 selbst beschreibt**, eine Ebene höher angewendet auf die Methodik-Pflege statt auf den Fahrplan.
- **Die Teil-C-Lücke** war nie ein Issue. Sie wurde im Piloten lokal geflickt und blieb damit unsichtbar für die Vorlage.

Verallgemeinerbar: Die Reihenfolge, in der Methodik-Befunde abgearbeitet werden, folgt ihrer **Verpackung**, nicht ihrer Schwere. Ein fertig ausformulierter Einzelpatch wird umgesetzt, ein Sammelbericht mit demselben Inhalt nicht. Kein Regelvorschlag daraus – erst prüfen, ob sich das über weitere Wellen bestätigt.

### 2026-08-28 23:10 – [ADR-ANGELEGT]

- **ADR:** ADR-007 – Teil C von `decisions.md` gehört zur Mindest-Lektüre
- **Tag:** `[OPERATIV]` `[METHODIK]`
- **Auslöser:** Auftrag des Eigentümers. Der Defekt ist am Wortlaut von Abschnitt 2 Punkt 5 direkt nachweisbar: Teil A wird genannt, Teil B ausgeschlossen, Teil C kommt nicht vor. Die Aufzählung wirkt abschließend und wird deshalb als solche gelesen.

### 2026-08-28 23:05 – [ADR-ANGELEGT]

- **ADR:** ADR-006 – Vision-Verlust-Lücke geschlossen (Landeplatz-Pflicht, `[VERSCHOBEN]`, Vision-Checkpoints)
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Auftrag des Eigentümers nach einem Vergleich zwischen Pilot-Regelwerk und Vorlage. Kein `[REAKTIV]`: Der Schritt S-10 wurde angelegt, bevor eine Zeile geändert wurde. Reaktiv-Quote damit 1/7 (14 %), unter dem Schwellenwert von 30 %.

### 2026-08-28 22:50 – [PROBLEM-GELÖST]

- **Kontext:** Umsetzung von A1.3 (Vision-Re-Derivation an Phasengrenzen), Schritt S-10
- **Symptom:** Der neue Checkpoint verlangt an Phasengrenzen eine vollständige `vision.md`-Lektüre. An zwei anderen Stellen stand aber weiterhin, `vision.md` werde ausschließlich zu Projektstart gelesen: Abschnitt 2 („Was nicht zur Pflichtlektüre gehört") und die `vision.md`-Zeile im Dokumenten-Index in Abschnitt 3.
- **Ursache:** Der Patch aus dem Pilotprojekt war gegen Abschnitt 12 formuliert und hat die Folgewirkung auf zwei andere Abschnitte nicht mitgeführt. Im Piloten fiel das nicht auf, weil dort dieselbe Inkonsistenz seit dem 2026-06-25 unbemerkt mitläuft.
- **Lösung:** Beide Stellen um die Ausnahme ergänzt und dabei ausdrücklich begründet, warum sie die **einzige** wiederkehrende ist – sonst weicht der „einmalig gelesen"-Status der Vision über die Zeit auf, und das Pflichtlektüre-Budget aus Abschnitt 2 wächst still mit.
- **Aufwand:** wenige Minuten
- **Lerneffekt:** Ein Patch, der in einem Projekt produktiv läuft, ist nicht automatisch vollständig. Er ist gegen den dortigen Anlass formuliert, nicht gegen das Dokument als Ganzes. Bei jeder Übernahme aus einem Pilotprojekt gehört ein Grep über die berührten Begriffe dazu – hier `vision.md` und `einmalig`.
- **Wiederkehrgefahr:** hoch bei den noch offenen Übernahmen (A2, BDR, Issue #20). Gegenmittel ist die Grep-Gegenprobe, keine Regeländerung.

### 2026-08-28 21:35 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` vom 2026-08-13 13:35; danach die Einträge zum verlorenen Merge von PR #29
- **Geplant für diese Session:** Vergleich des Regelwerks gegen das Pilotprojekt EB-Digital; daraus abgeleitet die Übernahme zweier dort belegter Befunde (A1, Teil-C-Lücke)
- **Vorabprüfung:** `git fetch` – lokaler Stand deckungsgleich mit `origin/main`; keine offenen Fremd-Branches mit Bezug zu S-10 oder S-11. Pflichtlektüre nach Abschnitt 2 vollständig durchlaufen.
- **Modus / Werkzeug:** Claude Code, Entscheidungs-Klasse (per Laufzeit-Abfrage bestätigt, nicht angenommen)

### 2026-08-13 14:25 – [PROBLEM-GELÖST]

- **Kontext:** Merge von PR #28 (S-8) und PR #29 (S-9)
- **Symptom:** Beide PRs zeigten `merged: true`, aber `main` enthielt die S-9-Änderungen nicht – die alte `Ohne Wechsel:`-Zeile stand noch in `CLAUDE.md`, ADR-005 und Regel-001 fehlten.
- **Ursache:** PR #29 war auf den Branch von PR #28 aufgesetzt worden, weil #28 zum Zeitpunkt der Erstellung noch offen war und dieselben Dateien berührte. Gemergt wurde dann in dieser Reihenfolge: #28 nach `main` um 12:10:58, #29 in den Branch von #28 um 12:11:14. Der Ziel-Branch von #29 war zu diesem Zeitpunkt bereits nach `main` gemergt – die Änderungen landeten damit auf einem toten Gleis. Beide PRs meldeten korrekt „gemergt", nur eben nicht beide nach `main`.
- **Lösung:** Neuer Branch von `origin/main`, Cherry-Pick des intakten S-9-Commits. Ging konfliktfrei durch, weil dessen Elternteil über den #28-Merge bereits in `main` lag. Alle fünf Bestandteile (STOPP-Block, Entfernung der alten Zeile, ADR-005, Regel-001, S-9 im Fahrplan) nach dem Pick einzeln verifiziert.
- **Aufwand:** ca. 10 Minuten
- **Lerneffekt:** Ein PR auf einen offenen PR-Branch aufzusetzen war die falsche Entscheidung. Sie vermeidet zwar Merge-Konflikte, erzeugt aber eine Reihenfolge-Abhängigkeit beim Mergen, die niemand im Blick behält – und deren Verletzung sich als „alles gemergt" tarnt. Richtig wäre gewesen: Merge des ersten PR abwarten, dann vom aktualisierten `main` neu ansetzen und den Konflikt in Kauf nehmen. Ein sichtbarer Konflikt ist harmloser als ein unsichtbar verlorener Merge.
- **Wiederkehrgefahr:** hoch bei dem in dieser Session üblichen Takt (PR öffnen → mergen → sofort weiterarbeiten). Gegenmittel ist einfach und braucht keine Regeländerung: keine gestapelten PRs, und nach jedem Merge prüfen, ob `main` den erwarteten Inhalt tatsächlich trägt.

### 2026-08-13 14:10 – [BEOBACHTUNG]

Beim Verifikationslauf für S-9 meldete der Ad-hoc-Drift-Check eine Reaktiv-Quote von 2/5 (40 %, über dem Schwellenwert) statt korrekt 1/5 (20 %). Ursache: Er zählte eine Erwähnung von `[REAKTIV]` im Fließtext von ADR-005 als Klassifikations-Tag mit. Der Fehler lag im Prüfcode, nicht im Dokument.

Das ist der interessantere Befund von beiden: Derselbe Ad-hoc-Prüfcode lief in dieser Session bereits mehrfach durch und galt als Beleg für „Drift-Check grün". Er hätte an anderer Stelle genauso gut still falsch-negativ sein können. Ein Prüfwerkzeug, dem man vertraut, ohne es selbst geprüft zu haben, ist dieselbe Fehlerklasse wie eine Konvention, die niemand nachsieht – nur eine Ebene höher. Als konkrete Anforderung in S-7 festgehalten: strukturelle Auswertung statt Volltextsuche, plus Tests des Prüfskripts mit konstruierten Positiv- und Negativfällen.

### 2026-08-13 14:05 – [ADR-ANGELEGT]

- **ADR:** ADR-005 – Modellwechsel-Eskalation wird echter Stopp
- **Tag:** `[REAKTIV]` `[METHODIK]`
- **Auslöser:** Der Eigentümer wies darauf hin, dass die Eskalations-Empfehlung aus S-1 im Betrieb wirkungslos blieb. Erster `[REAKTIV]`-ADR des Projekts. Erzeugte außerdem Regel-001, den ersten Eintrag in `decisions.md` Teil C.

### 2026-08-13 14:00 – [PROBLEM-GELÖST]

- **Kontext:** Modellklassen-Disziplin aus S-1, nach drei Anwendungen in S-4, S-5 und S-6
- **Symptom:** Der `MODELLWECHSEL EMPFOHLEN`-Block feuerte dreimal korrekt – und blieb dreimal folgenlos. Kein einziger Wechsel fand statt.
- **Ursache:** Die in S-1 selbst vorgeschriebene Zeile `Ohne Wechsel` war als Schutz gegen Nörgelei gedacht („Ein Hinweis ohne Handlungsalternative blockiert entweder die Arbeit oder wird nach dreimaligem Auftreten ignoriert"). Sie führte aber dazu, dass die KI in derselben Antwort weiterarbeitete. Der Mensch hätte die laufende Antwort unterbrechen, `/model` ausführen und die Aufgabe neu anstoßen müssen, um den Entscheidungspunkt zu nutzen – in dem Zeitfenster, bevor die Arbeit ohnehin fertig war.
- **Lösung:** Umbau zu einem blockierenden Stopp (ADR-005): kein Werkzeugaufruf, keine Ersatzhandlung, Warten auf Antwort, Bestätigung der aktiven Klasse beim Wiederanlauf. Voraussetzung vorab getestet: zwei aufeinanderfolgende Modellwechsel wurden korrekt erkannt.
- **Aufwand:** ca. 30 Minuten inkl. Test und Dokumentation
- **Lerneffekt:** Verallgemeinert als Regel-001. Die ursprüngliche Sorge (Hinweis wird zur Nörgelei) war berechtigt – die gewählte Gegenmaßnahme hat den Hinweis aber gleich ganz entwertet. Zwischen „blockiert die Arbeit" und „ist folgenlos" liegt keine Mitte; man muss sich entscheiden, welche der beiden Eigenschaften die Regel haben soll.
- **Wiederkehrgefahr:** hoch, wenn nicht bewusst gegengesteuert wird – jede künftige Regel mit Entscheidungspunkt kann in dieselbe Falle laufen. Deshalb Regel-001 statt nur eines ADR-Eintrags.

### 2026-08-13 13:35 – [SESSIONENDE]

- **Session-Dauer:** weitere Fortsetzung derselben Session, nach dem `[SESSIONENDE]`-Eintrag von 13:15.
- **Bearbeitet:** S-8 (Action-Versionen in Vorlagen-Workflows, PR [#28](https://github.com/Paddel87/Dev-Templates/pull/28)) auf `[ERLEDIGT]` gesetzt. Erster Schritt dieser Session ohne Modellwechsel-Hinweis – keiner der sechs Eskalations-Auslöser griff, korrekt als Routinearbeit erkannt.
- **Erreicht:** Fünf Actions in `templates/github-workflows/*.yml` live verifiziert und aktualisiert (`actions/checkout`, `actions/setup-python`, `actions/upload-artifact`, `actions/setup-node`, `pnpm/action-setup`). Kompatibilität der verwendeten `with:`-Parameter gegen aktuelle `action.yml`-Definitionen geprüft, keine Breaking Changes.
- **Offen geblieben:** S-7 (Drift-Check-Skript) – `[OFFEN]`, freigabepflichtig, letzter verbleibender Schritt.
- **Nächster Schritt:** S-7 nach Freigabe.
- **Stimmung / Beobachtung:** Ein Fund am Rande bewusst nicht mitgenommen: `pnpm/action-setup` hat für pnpm v11+ einen Nachfolger (`pnpm/setup`), das wäre aber ein Werkzeugwechsel statt eines Versions-Updates und damit außerhalb des S-8-Scopes – dieselbe Disziplin wie bei den Nebenbefunden in S-5.

### 2026-08-13 13:15 – [SESSIONENDE]

- **Session-Dauer:** weitere Fortsetzung derselben Session, nach dem `[SESSIONENDE]`-Eintrag von 12:45. Wie angekündigt bleibt der Eintragstyp vorerst unverändert (`[SESSIONENDE]` statt eines neuen „Zwischenstand"-Typs) – die Konvention selbst wurde nicht ohne Freigabe geändert, nur als Beobachtung notiert.
- **Bearbeitet:** S-6 (Lizenzfrage klären, PR [#27](https://github.com/Paddel87/Dev-Templates/pull/27)) auf `[ERLEDIGT]` gesetzt.
- **Erreicht:** `LICENSE` (CC0 1.0 Universal, ADR-004) im Repo-Root, live von `creativecommons.org` bezogen. `README.md` Lizenz-Abschnitt und `docs/project-context.md` Abschnitt 6/11 nachgezogen. Abhängigkeitslizenzen (`markdownlint-cli2`, `pre-commit`, beide MIT) live verifiziert, kompatibel mit CC0.
- **Offen geblieben:** S-7 (Drift-Check-Skript), S-8 (Action-Versionen in Vorlagen) – beide `[OFFEN]`, S-7 freigabepflichtig, S-8 nicht.
- **Nächster Schritt:** S-7 oder S-8 nach Freigabe – beide unabhängig voneinander.
- **Stimmung / Beobachtung:** Damit ist der komplette ursprüngliche Fahrplan (S-1 bis S-6) abgeschlossen. Was übrig bleibt, sind ausschließlich Punkte, die während der Bearbeitung selbst neu entdeckt wurden (S-7, S-8) – ein sauberes Ende einer Iteration, keine liegen gebliebenen Ausgangs-Schulden.

### 2026-08-13 13:05 – [ADR-ANGELEGT]

- **ADR:** ADR-004 – Lizenz: CC0 1.0 Universal
- **Tag:** `[OPERATIV]` `[METHODIK]`
- **Auslöser:** S-6 (Lizenzfrage klären), freigegeben durch den Eigentümer (Option B: CC0).

### 2026-08-13 13:00 – [BEOBACHTUNG]

Bei der S-6-Bearbeitung fiel auf, dass die eigene Fahrplan-Formulierung von S-6 („Bis zur Klärung ist die Nutzungserlaubnis für Dritte formal ungeregelt – das ist der Punkt mit der größten Außenwirkung unter den offenen Schritten") sachlich falsch war: Das Repo ist privat, es gab nie Dritte mit Zugriff. Die Übertreibung entstand vermutlich, weil „README verweist auf etwas, das nicht existiert" reflexhaft als dringlich eingestuft wurde, ohne die tatsächliche Sichtbarkeit des Repos zu prüfen. Korrigiert in S-6 und in `project-context.md` Abschnitt 11. Lehre: Dringlichkeits-Einschätzungen in Fahrplan-Notizen sollten denselben Verifikations-Anspruch haben wie technische Fakten, nicht nur plausibel klingen.

### 2026-08-13 12:45 – [SESSIONENDE]

- **Session-Dauer:** weitere Fortsetzung derselben durchgehenden Session, nach dem `[SESSIONENDE]`-Eintrag von 12:15. Auch dieser war kein echtes Ende, siehe dessen eigene Anmerkung dazu – Muster wiederholt sich, siehe `[BEOBACHTUNG]` unten.
- **Bearbeitet:** S-5 (CI-Pipeline einrichten, PR [#26](https://github.com/Paddel87/Dev-Templates/pull/26)) auf `[ERLEDIGT]` gesetzt. Dabei S-7 (Drift-Check-Skript) und S-8 (veraltete Action-Versionen in Vorlagen) neu als `[OFFEN]` angelegt, statt sie in S-5 unterzubringen.
- **Erreicht:** `.github/workflows/ci.yml` – erster CI-Workflow dieses Repos, führt den bestehenden pre-commit-Hook remote aus (ADR-003). Lokal simuliert vor dem Commit: frisches venv, Hook installierte sich isoliert, lief grün auf sauberem Bestand und rot bei absichtlichem Fehler (Exit-Codes 0/1 wie erwartet).
- **Offen geblieben:** S-6 (Lizenzfrage), S-7 (Drift-Check-Skript), S-8 (Action-Versionen in Vorlagen) – alle `[OFFEN]`, S-6 und S-7 freigabepflichtig.
- **Nächster Schritt:** S-6, S-7 oder S-8 nach Freigabe – alle drei gegenseitig unabhängig.
- **Stimmung / Beobachtung:** Der `ENTSCHEIDUNG ERFORDERLICH`-Block für S-5 legte eine Lücke im eigenen Fahrplan offen (Zu-tun vs. Akzeptanzkriterien wichen voneinander ab) und einen Nebenbefund in den Vorlagen (veraltete Action-Version) – beide sauber als eigene Schritte abgespalten statt im Vorbeigehen mitgefixt.

### 2026-08-13 12:40 – [BEOBACHTUNG]

Zweites `[SESSIONENDE]` in derselben Session, das keines war (das erste war der 12:15-Eintrag selbst). Der PR-Merge-Zyklus dieser Session (PR öffnen → merge → sofort weiterarbeiten) passt nicht zum Sessionende-Konzept aus `CLAUDE.md` Abschnitt 12, das von einem tatsächlichen Gesprächsende ausgeht. Kein Regelverstoß – die Einträge bleiben stehen, nichts wird rückwirkend korrigiert –, aber ein Hinweis, dass die Logbuch-Konvention für lange, PR-getaktete Sessions nachschärfbar wäre (z. B. ein eigener Eintragstyp „Zwischenstand" statt eines echten `[SESSIONENDE]` pro PR). Keine Fahrplan-Konsequenz jetzt, nur festgehalten für später.

### 2026-08-13 12:35 – [ADR-ANGELEGT]

- **ADR:** ADR-003 – CI: pre-commit/action, Drift-Checks von S-5 abgespalten
- **Tag:** `[OPERATIV]` `[STACK]` `[DEPLOYMENT]` `[METHODIK]`
- **Auslöser:** S-5 (CI-Pipeline einrichten), freigegeben durch den Eigentümer (Option A, Drift-Checks als eigener Schritt S-7 abgespalten).

### 2026-08-13 12:15 – [SESSIONENDE]

- **Session-Dauer:** Fortsetzung derselben durchgehenden Session nach dem `[SESSIONENDE]`-Eintrag von 11:15 – der PR-Merge war kein echtes Sessionende, sondern nur ein Zwischenstand. Frühere Eintrag bleibt unverändert stehen (Einträge werden nicht nachträglich korrigiert).
- **Bearbeitet:** S-4 (Markdown-Linter einrichten, PR [#25](https://github.com/Paddel87/Dev-Templates/pull/25)) auf `[ERLEDIGT]` gesetzt; die offene README-Struktur-Frage aus dem letzten Sessionende wurde zwischenzeitlich geklärt (siehe `[BEOBACHTUNG]` 11:30).
- **Erreicht:** `markdownlint-cli2` v0.23.2 über pre-commit eingerichtet (ADR-002), 0 Issues auf dem aktuellen Bestand. Der Testlauf deckte 24 echte strukturelle Lücken auf (behoben) neben 891 reinen Stil-Treffern (Regeln dokumentiert deaktiviert). Erster echter Testfall der Modellklassen-Eskalation aus S-1 – mit realer Reibung, siehe `[BEOBACHTUNG]` 11:35.
- **Offen geblieben:** S-5 (CI-Pipeline) und S-6 (Lizenzfrage), beide `[OFFEN]`, freigabepflichtig, noch nicht angefordert.
- **Nächster Schritt:** S-5 oder S-6 nach Freigabe.
- **Stimmung / Beobachtung:** Die Duplicate-Heading-Prüfung des neuen Linters fand einen Fehler im eigenen, gerade erst geschriebenen Logbuch-Eintrag (fehlende Uhrzeit). Bestätigt die Kernthese hinter S-4/S-5 aus der letzten Reflexion noch am selben Tag, schneller als erwartet.

### 2026-08-13 12:05 – [ADR-ANGELEGT]

- **ADR:** ADR-002 – Markdown-Linter: markdownlint-cli2 über pre-commit
- **Tag:** `[OPERATIV]` `[STACK]` `[METHODIK]`
- **Auslöser:** S-4 (Markdown-Linter einrichten), freigegeben durch den Eigentümer (Option A aus dem `ENTSCHEIDUNG ERFORDERLICH`-Block).

### 2026-08-13 11:50 – [PROBLEM-GELÖST]

- **Kontext:** S-4, erster Testlauf von `markdownlint-cli2` gegen den kompletten Repo-Bestand vor der endgültigen Konfiguration
- **Symptom:** 915 gemeldete Probleme im Standard-Regelsatz.
- **Ursache:** Zwei Gruppen. 891 Treffer waren reine Stilfragen (754× Zeilenlänge, 137× Tabellen-Pipe-Ausrichtung) – erwartbar, da dieses Repo bewusst nicht hart umbricht. Die restlichen 24 waren echte strukturelle Lücken: 14 Codeblöcke ohne Sprachangabe, 7 fehlende Leerzeilen um Listen/Codeblöcke, 3 kollidierende Überschriften (davon einer in `docs/logbuch.md` selbst – zwei `[BEOBACHTUNG]`-Einträge ohne Uhrzeit, obwohl die eigene Konvention „Format: YYYY-MM-DD HH:MM" das verlangt).
- **Lösung:** Die 24 echten Treffer behoben (Sprachtag `text` für Format-Blöcke, Leerzeilen ergänzt, zwei Vorlagen-Überschriften minimal disambiguiert, Uhrzeiten in `docs/logbuch.md` nachgetragen). Die zwei Stil-Regeln (MD013, MD060) mit Begründung deaktiviert, analog zur bestehenden `.prettierignore`-Logik.
- **Aufwand:** ca. 20 Minuten
- **Lerneffekt:** Der eigene Duplicate-Heading-Fund in `docs/logbuch.md` ist der beste Beleg für S-4 selbst – die Uhrzeit-Konvention stand von Anfang an im Dokument, wurde aber ohne automatisierte Prüfung nicht eingehalten. Derselbe Mechanismus wie beim ANCHOR-Drift aus der letzten Session.
- **Wiederkehrgefahr:** gering – der Hook verhindert das jetzt strukturell.

### 2026-08-13 11:35 – [BEOBACHTUNG]

Bei der Eskalations-Empfehlung zu S-4 (Modellwechsel auf Entscheidungs-Klasse für die Linter-Werkzeugwahl) gab der Eigentümer zurück, dass für den Wechsel keine Zeit gewesen wäre – die Session lief ohnehin schon auf Sonnet. Erste reale Reibung an der in PR #21 eingeführten Regel: Der Hinweis ist zwar korrekt ausgelöst worden (Kategorie 3, `ENTSCHEIDUNG ERFORDERLICH`-Block), aber die Handlungsalternative „erst wechseln, dann fortsetzen" hat einen Rüstzeit-Preis, den die Regel selbst nicht einpreist. Relevant für eine spätere Bewertung, ob die Eskalations-Schwelle zu niedrig hängt.

### 2026-08-13 11:30 – [BEOBACHTUNG]

Offene Grundsatzfrage aus dem `[SESSIONENDE]`-Eintrag unten geklärt: `README.md` behält die freie Form, keine Umstellung auf `readme-vorlage.md`-Struktur. Der Eigentümer begründet das mit der Doppelrolle der Datei – Statusbild und Werbetext für das Vorlagen-Set zugleich. Festgehalten in `project-context.md` Abschnitt 11.

### 2026-08-13 11:15 – [SESSIONENDE]

- **Session-Dauer:** eine durchgehende Session, mehrere Themenwechsel (Cursor/Grok-Evaluation → Modellklassen-Disziplin → ANCHOR-Konvention → Selbstanwendung)
- **Bearbeitet:** S-1 (Modellklassen-Disziplin, PR #21), S-2 (ANCHOR-Konvention, PR #22), S-3 (Selbstanwendung, PR #23) – alle drei auf `[ERLEDIGT]` gesetzt
- **Erreicht:** Regelwerk kennt jetzt zwei Modellklassen mit objektiven Eskalations-/Rückstufungs-Auslösern; alle Pflicht-Dokumente tragen Sprung-Anker; `docs/` ist erstmals der echte Arbeitsdokumenten-Satz dieses Repos statt einer Vorlagen-Ablage. Der Inter-Pflicht-Drift-Check aus Abschnitt 16 lief zum ersten Mal auf dieses Repo und ist grün.
- **Offen geblieben:** S-4 (Markdown-Linter), S-5 (CI-Pipeline), S-6 (Lizenzfrage) – alle drei `[OFFEN]`, freigabepflichtig, noch nicht angefordert. README.md folgt weiterhin nicht der Struktur aus `readme-vorlage.md` (kein Status-Block, keine „Nächste Schritte"-Sektion) – das ist keine Drift im Sinne von Abschnitt 16 (die README entstand vor der Selbstanwendung), aber eine offene Frage, ob die volle Vorlagen-Struktur auch für README.md nachgezogen werden soll. Nicht eigenmächtig entschieden, um keine stille Scope-Erweiterung zu erzeugen.
- **Nächster Schritt:** S-4, S-5 oder S-6 nach Freigabe; alternativ die offene README-Struktur-Frage klären.
- **Stimmung / Beobachtung:** Erste vollständige Sessionende-Disziplin dieses Repos auf sich selbst angewendet – lief ohne Überraschungen durch, was für S-1 bis S-3 als guter erster Beleg für Abschnitt 16 zählt (siehe `[BEOBACHTUNG]` unten zur ANCHOR-Konvention: unbemerkter Drift entsteht genau dort, wo niemand nachsieht).

### 2026-08-13 10:55 – [ADR-ANGELEGT]

- **ADR:** ADR-001 – Selbstanwendung der Methodik und Klasse-K-Einstufung
- **Tag:** `[STRATEGISCH]` `[METHODIK]`
- **Auslöser:** Die Frage, wo die Modellklassen-Zuordnung aus `CLAUDE.md` Abschnitt 0 für dieses Repo einzutragen ist. Sie ließ sich nicht beantworten, ohne vorher zu klären, ob `docs/` Vorlage oder Arbeitsdokument ist.

### 2026-08-13 10:45 – [BEOBACHTUNG]

Zwei der drei Schritte dieser Session waren keine neue Regelarbeit, sondern das Nachholen von Umsetzungen zu Regeln, die längst beschlossen waren: Die ANCHOR-Konvention stand seit Patch-Welle #7 in `CLAUDE.md`, war aber in keinem Dokument gesetzt.

Auffällig ist der Mechanismus: Der Drift blieb unbemerkt, weil nichts ihn prüft. Die Konvention war in `.prettierignore` sogar als Begründung für den Markdown-Ausschluss genannt – der Text setzte also voraus, dass die Anker existieren, ohne dass es je jemand nachgesehen hätte. Das ist das Argument für S-4 und S-5 (Linter, CI): Selbst formulierte Regeln zerfallen, wenn ihre Einhaltung von manueller Aufmerksamkeit abhängt.

### 2026-08-13 10:40 – [PROBLEM-GELÖST]

- **Kontext:** Umsetzung der ANCHOR-Konvention, Ableitung der Namensregel
- **Symptom:** `CLAUDE.md` gibt genau ein Beispiel vor (`<!-- ANCHOR:reifegrad-uebersicht -->`), aber keine Regel, nach der weitere Anker zu benennen wären.
- **Ursache:** Die Konvention wurde eingeführt, ohne sie zu spezifizieren – ein Beispiel ist keine Regel.
- **Lösung:** Namensregel aus dem Beispiel rekonstruiert: Titel ohne Nummerierung und Klammerzusätze, Umlaute transliteriert, kleingeschrieben, Bindestriche. Rückprobe: `## 9. Reifegrad-Übersicht (Stand vom YYYY-MM-DD)` ergibt genau `reifegrad-uebersicht`. Die Nummerierung entfällt bewusst, damit Anker eine Umnummerierung überleben.
- **Aufwand:** wenige Minuten
- **Lerneffekt:** Wenn eine Vorlage ein Beispiel statt einer Regel enthält, ist das eine offene Stelle. Beim nächsten neuen Konventionsbegriff die Regel gleich mitschreiben.

### 2026-08-13 09:00 – [SESSIONSTART]

- **Letzter Stand:** kein vorheriger Logbuch-Eintrag – dieses Dokument entsteht mit ADR-001
- **Geplant für diese Session:** Bewertung eines möglichen Wechsels der Entwicklungsumgebung; daraus abgeleitet die Verankerung der Modellklassen-Disziplin im Regelwerk
- **Vorabprüfung:** entfällt – keine UMSETZUNG-Schritte, ausschließlich Methodik-Arbeit
- **Modus / Werkzeug:** Claude Code, Entscheidungs-Klasse

---

<!-- ANCHOR:eintragstypen -->
## Eintragstypen (Übersicht)

| Typ | Wann | Pflicht bei Klasse K? |
|---|---|---|
| `[SESSIONSTART]` | Zu Beginn jeder Session | Ja |
| `[SESSIONENDE]` | Vor Sessionabschluss | Ja |
| `[PROBLEM-GELÖST]` | Nach Behebung eines Problems, das Reibung war | Optional, proaktiv |
| `[BEOBACHTUNG]` | Wenn etwas auffällt, das später nützlich sein könnte | Optional, proaktiv |
| `[REIFEGRAD-WECHSEL]` | Bei jeder Reifegrad-Änderung in `architecture.md` | Ja |
| `[ADR-ANGELEGT]` | Bei Anlage eines neuen ADR | Ja |

<!-- ANCHOR:hinweise-zur-pflege -->
## Hinweise zur Pflege

- **Neueste Einträge oben.**
- **Datum ist Pflicht**, Uhrzeit bei Klasse K optional.
- **Detailtiefe lieber zu hoch als zu niedrig.** Mini-Reibungen sind im Moment unscheinbar und später wertvoll.
- **Keine sensiblen Daten** – keine Tokens, keine Zugangsdaten, auch nicht in Zitaten aus Fehlermeldungen.

<!-- ANCHOR:archivierung -->
## Archivierung

Bei Klasse K greift der Zeilen-Trigger doppelt: Auslagerung nach `docs/archiv/logbuch-YYYY-MM.md` erst ab 1.600 Zeilen (`CLAUDE.md` Abschnitt 14, Klassen-Dämpfung). Derzeit weit darunter.
