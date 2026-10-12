# Prüfbericht: Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?" – Bündel #34

- **Datum:** 2026-09-24
- **Rolle:** getrennte Instanz nach `CLAUDE.md` Abschnitt 12, Unterabschnitt „Weiterbauen, umbauen oder neu aufsetzen – Pflichtfrage"
- **Gegenstand:** Regelwerk (`CLAUDE.md`) und Vorlagen (`templates/`) nach dem Schritt-Bündel „Umsetzung von Issue #34" (Fahrplan-Schritte S-13 bis S-22, ADR-008 bis ADR-014)
- **Geprüfter Stand:** Branch `claude/zwei-repositories-zugriff-23hd2k`
- **Vision-Quelle:** `docs/project-context.md` Abschnitt 2 plus Leitsatz aus Issue #34: „Technische Wachsamkeit muss bei der KI und den Regeln liegen, nicht beim Menschen."

## Zusammenfassung

1. **Empfehlung: gezielt umbauen.** Konfidenz: **mittel**.
2. Das Fundament trägt. Vision-Abgleich, Landeplatz-Pflicht, Freigabe-Form und Stopp-Kriterien sind am Pilotprojekt über Monate belegt. Ein Neuaufbau würde das ohne Not aufs Spiel setzen.
3. Das Bündel hat das Regelwerk aber an **einem einzigen Tag um gut ein Drittel verlängert** (von 58 auf 80 KB). Keine der sieben neuen Grundsatzregeln ist an einem zweiten Projekt erprobt.
4. `CLAUDE.md` selbst hat **keine Größengrenze**. Das Regelwerk und die Pflichtlektüre dieses Repos verbrauchen zusammen geschätzt rund 90.000 Token, bevor die Arbeit beginnt. Das ist fast die Hälfte der neuen Sessiongrenze von 200.000 Token.
5. Ich habe **acht Widersprüche** gefunden, bei denen Regel und gelebte Praxis auseinanderlaufen. Zwei davon wiederholen genau das Fehlermuster, das das Bündel beheben sollte (Abschnitt 3.2).
6. Für einen Nicht-Programmierer gibt es inzwischen zu viele Arten von Unterbrechungen. Nebenfragen werden auffällig oft „mit Voreinstellung" beantwortet – ein Frühzeichen dafür, dass Freigaben nur noch abgenickt werden.
7. Der Leitsatz aus #34 steht nirgends in `CLAUDE.md`. Mehrere neue Regeln legen die Wachsamkeit wieder beim Menschen ab.
8. **Vorschlag:** vier Umbau-Blöcke (Abschnitt 5.2). Bis sie erledigt sind, keine neue Regel. Punkt 2 aus #34 (Betriebsmodus) erst danach angehen.
9. Der Bericht beruht nur auf den Dokumenten. Logbuch und Commit-Nachrichten habe ich wie beauftragt nicht gelesen, Nutzungsdaten aus einem zweiten Projekt gibt es nicht.

---

## 1. Was ich geprüft habe und was nicht

**Vollständig gelesen:** `CLAUDE.md`, `AGENTS.md`, `docs/project-context.md`, `docs/fahrplan.md`, `docs/decisions.md` (Teile A, B, C), `docs/architecture.md`, `docs/blockers.md`, `templates/projektstart.md`, der Kopf von `templates/werkzeuge/claude-code/kontingent-warnung.py`, Issue #34 mit allen acht Kommentaren, Issue #20.

**Nur gezielt durchsucht:** die übrigen Dateien unter `templates/`, auf Modellnamen, Feldnamen und Querverweise.

**Bewusst nicht gelesen:** `docs/logbuch.md` und alle Commit-Nachrichten. Aus der Git-Historie habe ich nur Datum und Dateigröße von `CLAUDE.md` je Stand ausgewertet.

**Grenzen meiner Unabhängigkeit:** Ich laufe auf derselben Modell-Linie wie die Session, die gebaut hat (siehe Abschnitt 7). `CLAUDE.md` Abschnitt 12 wünscht „möglichst ein anderes Modell". Das ist hier nicht erfüllt. Gleichartige blinde Flecken sind deshalb möglich.

---

## 2. Was sich durch das Bündel verändert hat (Zahlen)

| Messgröße | Vor dem Bündel | Nach dem Bündel | Beleg |
|---|---|---|---|
| Größe `CLAUDE.md` | 58.363 Byte, 613 Zeilen (Stand 2026-08-28) | 79.934 Byte, 712 Zeilen | Git-Historie der Datei, 7 Stände am 2026-09-24 |
| Größter Abschnitt | – | Abschnitt 0 „Betriebsmodus" mit 102 Zeilen, steht ganz vorn | `CLAUDE.md` Zeilen 8–109 |
| Neue ADRs | 7 | 14 (davon 7 `[STRATEGISCH]` an einem Tag) | `docs/decisions.md` Teil A, Zeilen 19–25 |
| Reaktiv-Quote | 1/7 | 1/14 | `docs/decisions.md` Zeile 29 |
| PRs ohne grüne CI gemergt | 1 (#35) | 9 (#35 bis #43) | `docs/fahrplan.md` S-12 bis S-22 |
| Bündel-Schritte | Plan: 5 „Einheiten" | 10 Schritte, Schwelle „mehr als 10" | `docs/fahrplan.md` Zeile 14 |

Zum Vergleich: Im Mai 2026 wurde `CLAUDE.md` schon einmal von 61.777 auf 46.050 Byte gekürzt, indem der Projektstart ausgelagert wurde. Der heutige Stand liegt deutlich über der Größe vor dieser Kürzung.

---

## 3. Befunde

### 3.1 Länge und Lesbarkeit für den Agenten

**B1 – `CLAUDE.md` hat keine Größengrenze, die Mindest-Lektüre schon.** `CLAUDE.md` Abschnitt 2 (Zeile 153) setzt eine harte Grenze nur für `docs/project-context.md`: 15.000 Token. Das Regelwerk selbst ist mit 80 KB das größte Dokument, das in jeder Session vollständig geladen wird. Für seine Größe gibt es weder eine Grenze noch eine Prüfung.

**B2 – Die Lektüre frisst einen großen Teil der eigenen Sessiongrenze.** Meine Schätzung stützt sich auf das Verhältnis, das das Lesewerkzeug für `docs/fahrplan.md` gemeldet hat: 54 KB entsprachen rund 29.000 Token. Daraus folgt für dieses Repo:

| Posten | geschätzt |
|---|---|
| `CLAUDE.md` | ca. 43.000 Token |
| `docs/fahrplan.md`, Klasse K, ganz zu lesen (siehe B3) | ca. 29.000 Token |
| `docs/project-context.md` | ca. 9.000 Token |
| die übrigen Teile | ca. 10.000 Token |
| **Summe** | **ca. 90.000 Token** |

Die Sessiongrenze aus ADR-014 liegt bei 200.000 Token (`docs/project-context.md` Zeile 97). Fast die Hälfte ist also schon vor dem ersten Arbeitsschritt verbraucht. Das ist eine Schätzung, keine Messung.

Die Regel zur Sessiongröße verstärkt das Problem selbst: Jede neue Session, die sie erzwingt, liest die gesamte Pflichtlektüre noch einmal.

**B3 – Bei Klasse K wächst der Fahrplan ohne Ende.** Laut `CLAUDE.md` Zeile 141 wird „die aktuell laufende Phase" gelesen. Klasse K hat aber keine Phasen, also wird die ganze flache Liste gelesen. Archiviert wird ein Fahrplan nur, wenn eine Phase vollständig erledigt ist (`CLAUDE.md` Zeile 479). Bei Klasse K tritt dieser Auslöser nie ein – `docs/fahrplan.md` Zeile 365 bestätigt das. Erledigte Schritte wie S-1 bis S-6 bleiben deshalb für immer in der Pflichtlektüre.

**B4 – Begründungen und Pilot-Geschichten stehen im Regeltext.** `CLAUDE.md` enthält mehrere „Warum"-Absätze und Erzählungen aus dem Pilotprojekt, zum Beispiel:

- Zeile 91 („Warum kein Ersatz-Vorgehen mehr")
- Abschnitt 12, „Warum eine getrennte Instanz"
- Abschnitt 12, „Warum vor dem ersten öffentlichen Deployment"
- Abschnitt 0, „Arbeit oberhalb der empfohlenen Klasse", erster Absatz

Die Begründungen sind richtig und wertvoll. Nach der eigenen Logik des Repos gehören sie aber in die ADRs: Die Regel sagt, was gilt; der ADR sagt, warum.

**B5 – Wirtschaftlichkeit steht vor den Kernregeln.** Abschnitt 0 ist mit 102 Zeilen der längste Abschnitt und steht ganz vorn. Etwa die Hälfte davon betrifft Kosten:

- Modellklassen
- Abgabe an Unteragenten
- Cache-Lesepreise
- Sessiongröße
- Kontingent-Warnung

Die Regeln, die tatsächlich Schaden verhindern – Freigaben (Abschnitt 4), Stopps (Abschnitt 8), Fertig-Definition (Abschnitt 9) –, folgen erst danach. Ein Agent liest also als Erstes, wie er Geld spart, und erst später, wann er anhalten muss.

### 3.2 Widersprüche zwischen Abschnitten und zwischen Regel und Praxis

**W1 – Der Fertig-Status kennt keine Ausnahme, die Praxis schon neunmal.** `CLAUDE.md` Zeile 356 sagt: „Unvollständige DoD = Status bleibt `[IN ARBEIT]`. Keine Ausnahme." S-12 bis S-22 stehen trotzdem als `[ERLEDIGT] – mit CI-Ausnahme`. Das Regelwerk hat keinen Weg für eine vom Eigentümer freigegebene Ausnahme vom Fertig-Status. Es gibt also entweder eine ungeschriebene Regel, oder die Praxis bricht die geschriebene.

Das eigentliche Risiko ist gering: Die CI prüft nur den Markdown-Linter, und der lief laut Fahrplan lokal. Der Mechanismus ist trotzdem gefährlich. Eine harte Regel, die neunmal in Folge per Zuruf ausgesetzt wird, gewöhnt alle Beteiligten daran, dass harte Regeln verhandelbar sind.

**W2 – Neue Regelbereiche in einer Phase, die „keine neuen Regelbereiche" verspricht.** `docs/fahrplan.md` Zeile 12 sagt: „STABILISIERUNG – die Methodik wird gehärtet, keine neuen Regelbereiche". Das Bündel hat trotzdem ganz neue Bereiche eingeführt:

- das Sicherheits-Gate
- vier Modellklassen
- die Sessiongröße
- ein neues Pflichtdokument `requirements.md` samt „Modus 1.5"
- ein ausführbares Skript

`CLAUDE.md` Zeile 289 (Phasentyp-Disziplin) verbietet genau diese Vermischung: Wer in der STABILISIERUNG Neues bauen will, muss dafür einen UMSETZUNG-Schritt anlegen.

**W3 – Die Reaktiv-Quote kann in diesem Repo nicht anschlagen.** Dasselbe Muster wie im Piloten, das ADR-012 beheben sollte. ADR-012 hat die Regel eingeführt, dass jede Architekturentscheidung in der STABILISIERUNG als `[REAKTIV]` zählt. Im selben ADR nimmt sich das Repo davon aus: Methodik-ADRs seien keine Architekturentscheidungen (`docs/decisions.md` Zeile 361, `docs/project-context.md` Zeile 85).

Folge: Sieben Grundsatzänderungen an einem Tag, alle in einer STABILISIERUNG, alle als `[STRATEGISCH]` etikettiert. Die Quote steht bei 1/14. ADR-014 ist dabei ausdrücklich eine Korrektur von ADR-011 vom selben Tag („ändert ADR-011 in einem Punkt", Zeile 406), weil sich die ursprüngliche Abgaberegel als unwirksam erwiesen hatte. Das ist der Kern dessen, was „reaktiv" meint. Issue #34 Punkt 4 beschreibt das Muster wörtlich: Das Etikett setzt dieselbe Stelle, die ein schlechtes Ergebnis vermeiden will.

**W4 – Eine Option wurde verworfen, weil sie einen Stopp ausgelöst hätte.** `docs/decisions.md` Zeile 411 begründet, warum Option B bei ADR-014 verworfen wurde: Sie „hätte bei Bündel #34 (10 Schritte) Stopp-Kriterium 9 ausgelöst, ohne inhaltlichen Gewinn". Eine Schutzregel hat also die Wahl zwischen Optionen beeinflusst, und zwar in Richtung „Stopp vermeiden".

Der Schritt ist offen dokumentiert, das spricht für die bauende Session. Er zeigt aber: Die Zählregel lässt sich steuern, indem man Arbeit in weniger Schritte packt.

**W5 – Die Zählregel für die Wucherung ist nicht eindeutig.**

- **Wer zum Bündel gehört, ist frei festgelegt.** Kriterium 9 (`CLAUDE.md` Zeile 318) zählt bei Klasse K die Schritte „eines zusammenhängenden Schritt-Bündels". Was dazugehört, sagt keine Regel. `docs/fahrplan.md` Zeile 14 zählt einfach den Nummernbereich S-13 bis S-22. Damit gehören auch S-17 (CI-Nachlauf) und S-18 (Vorschlag ohne Anforderung) dazu, obwohl beide keine Umsetzung von #34 sind.
- **Der Plan wurde in „Einheiten" gezählt, der Ist-Stand in Schritten.** Zwei Pakete wurden bewusst in je zwei Stufen geteilt, um kleinere Prüfeinheiten zu bekommen (ADR-008, ADR-010). Die Regel bestraft damit genau die Vorsicht, die ADR-008 empfiehlt.
- **Der Plan entstand nachträglich.** Er wurde erst mit ADR-012 festgehalten, als das Bündel schon 8 Schritte hatte (ADR-012, Zeile 363).

**W6 – Eigene Angaben widersprechen sich (Selbstanwendung).**

- *Abhängigkeiten:* `docs/project-context.md` sagt „Externe Abhängigkeiten: keine" (Zeilen 33 und 52). Zeile 75 derselben Datei nennt `markdownlint-cli2` und `pre-commit`, und ADR-002 bestätigt beide als Abhängigkeiten.
- *Skripte:* `docs/architecture.md` Abschnitt 10 (Zeile 117) sagt „Keine Hilfsskripte im Repo". `docs/project-context.md` Abschnitt 7 (Zeilen 110–113) sagt „kein ausführbarer Code". Seit S-22 liegt aber das Python-Skript `templates/werkzeuge/claude-code/kontingent-warnung.py` im Repo – zusammen mit einer mit eingecheckten Kompilat-Datei (`__pycache__/kontingent-warnung.cpython-311.pyc`). Für dieses Skript läuft kein Linter, und es hat keinen Eintrag im Werkzeug-Verzeichnis.
- *Skript-Pflichten:* Das Skript hat zwei Betriebsarten („statuszeile", „hook"). Damit überschreitet es die Komplexitätsschwelle aus `CLAUDE.md` Abschnitt 15 („>1 Subkommando"). Sein Kopf erklärt die Idempotenz-Aussage aber nur mit „unter 100 Zeilen" für nicht anwendbar und prüft das Subkommando-Kriterium nicht.
- *Reifegrad:* `docs/architecture.md` Zeile 107 führt das Regelwerk weiter als `[BELASTBAR]`, Stand 2026-08-13. Seitdem ist gut ein Drittel des Textes neu hinzugekommen. Nach Abschnitt 0 derselben Datei gilt ein Bestandteil als validiert, wenn er „ohne Nachbesserung" angewendet wurde. ADR-011 wurde aber noch am selben Tag durch ADR-014 nachgebessert.

**W7 – Das Regelwerk selbst ist nicht frei von Modell- und Werkzeugnamen.** `docs/project-context.md` Zeile 60 verlangt, dass eine Suche nach Modell- und Produktnamen über `CLAUDE.md` leer bleibt. `CLAUDE.md` nennt aber:

- „Claude Code" in Zeile 3
- „Claude" als Handelnden in den Zeilen 135, 163, 215, 219, 254 und 307

Beides ist älter als das Bündel. Die Neutralitäts-Suche aus S-15 und S-16 hat nur die *neuen* Zeilen geprüft und die Stellen deshalb nicht gefunden.

In den Vorlagen, die ausgeliefert werden, steht außerdem `templates/projektstart.md` Zeile 13: „stärkste verfügbare Modell (derzeit Opus) … nicht in Claude Code". Das widerspricht in zwei Punkten `CLAUDE.md`:

- **Welche Klasse gilt?** Das stärkste verfügbare Modell ist nach `CLAUDE.md` Abschnitt 0 die *Ausnahme-Klasse*, die nur mit Freigabe genutzt werden darf. Der Auslöser 6 aus Abschnitt 0 (Zeile 33) verlangt für den Projektstart dagegen die *Entscheidungs-Klasse*.
- **Kann die Regel überhaupt greifen?** Modus 1 und 2 sollen außerhalb des Werkzeugs laufen, das `CLAUDE.md` lädt. Dann feuert Auslöser 6 dort gar nicht.

**W8 – Kleinere Verweisfehler.**

- *Nummerierung in `templates/projektstart.md`:*
  - Zeile 69 sagt „nach Schritt 3 (Architektur)", die Architektur ist aber Schritt 4.
  - Zeile 125 sagt „Architektur in Schritt 3 … Stack-Option aus Schritt 1", der Stack ist aber Schritt 2.
  - Abschnitt 2.1 legt die Stufe-2-Bestätigung „vor `docs/architecture.md`-Befüllung", Schritt 4 legt sie *während* dieser Befüllung.
- *README-Vorlage:* `CLAUDE.md` Zeile 650 verweist auf `docs/readme-vorlage.md`, richtig ist `templates/docs/readme-vorlage.md` (so auch Zeile 211).
- *Feldname:* Die Drift-Prüfung in `CLAUDE.md` Abschnitt 16 vergleicht „Betroffene Module". Die Fahrplan-Vorlage nutzt dieses Feld auch, dieses Repo schreibt aber „Betroffene Bestandteile" (`docs/fahrplan.md` Zeile 41).

### 3.3 Regeln, die sich gegenseitig aushebeln

**A1 – Die Ausnahme in Regel-001 hat ihre Grundlage verloren.** Regel-001 (`docs/decisions.md` Zeile 438) erlaubt den nicht blockierenden Hinweis bei zu teurer Arbeit nur mit einer Begründung: Die *Wirkung* sichere ein Mechanismus, nämlich die Abgabe an Unteragenten. ADR-014 hat danach festgestellt, dass die Abgabe von Lesearbeit an die Routine-Klasse kaum etwas spart (gleicher Cache-Lesepreis). Seitdem wird sie nur noch bei „ausgabelastiger" Arbeit empfohlen. Für den größten Teil der Routinearbeit bleibt als Mittel damit nur die Warnung. Genau diese Warnung hat ADR-011 als alleiniges Mittel verworfen, weil sie im Piloten wirkungslos war (Option B, `docs/decisions.md` Zeile 316).

**A2 – Die Abgaberegel verlangt die Selbsteinschätzung, die Abschnitt 0 verbietet.** Abschnitt 0 legt fest, dass Klassen „nicht über Selbsteinschätzung" zugeordnet werden, sondern über eine Liste. Die Abgabe hängt jetzt aber davon ab, ob Arbeit „überwiegend Ausgabe erzeugt" und ob sie „tatsächlich spart" (Zeile 53). Beides ist eine Schätzung ohne prüfbaren Auslöser.

**A3 – Zwei Stopps für eine Entscheidung.** Jede Freigabe nach Abschnitt 4 ist zugleich Eskalations-Auslöser 1 (Zeile 26 ff.). Arbeitet der Mensch im Alltag – wie vorgesehen – auf der Routine-Klasse, erlebt er für eine einzige Frage zwei Stopps nacheinander:

1. „Modell wechseln"
2. „Entscheidung erforderlich"

Umgekehrt gehört die Pflichtfrage aus Abschnitt 12 zu keiner der acht Kategorien, wird aber „als `ENTSCHEIDUNG ERFORDERLICH`" vorgelegt. Ob sie den Modellwechsel auslöst, ist nicht geregelt.

**A4 – Die Logbuch-Pflicht kostet Kontext.** Abschnitt 6 verlangt „lieber zu detailreich als lückenhaft" und auch „mehrminütige Reibungen". Abschnitt 0 behandelt Kontextgröße als größten Kostenhebel. Wie viel das Logbuch zur Kontextgröße beiträgt, habe ich nicht gemessen (ich habe es nicht gelesen). Die beiden Ziele stehen aber in Spannung, und keine Regel wägt sie gegeneinander ab.

### 3.4 Regeln ohne prüfbaren Auslöser

- Abgabe „nur, was tatsächlich spart" und „überwiegend Ausgabe" (Abschnitt 0) – siehe A2.
- Zugehörigkeit zu einem „zusammenhängenden Schritt-Bündel" (Abschnitt 8, Kriterium 9) – siehe W5.
- „Nennenswerte Beobachtungen" im Logbuch (Abschnitt 6).
- „Nutzersichtbar" als Auslöser für die README-Pflege (Abschnitt 16, Trigger 1).
- „Warnungsquellen ohne Schalter" lesen (Abschnitt 15): Das ist an einen Anlass gebunden, bleibt aber eine Lese-Pflicht. ADR-010 benennt das selbst als schwächsten Teil.
- Komplexitätsschwelle für Skripte „Richtwert" (Abschnitt 15) – siehe W6.

### 3.5 Belastung eines nicht programmierenden Nutzers

Wer das Regelwerk befolgt, kann heute an diesen Stellen angehalten oder um eine Entscheidung gebeten werden:

| Art der Unterbrechung | Fundstelle |
|---|---|
| Stopp für einen Modellwechsel (sechs Auslöser) | `CLAUDE.md` Abschnitt 0 |
| Vorschlag der Ausnahme-Klasse | Abschnitt 0 |
| Bitte um eine neue Session (Sessiongröße) | Abschnitt 0 |
| Freigabe (acht Kategorien) | Abschnitt 4 |
| Grenzfälle „wie Freigaben" | Abschnitt 5 |
| Stopp bei ausgegebenem Secret | Abschnitt 6 |
| Optional vorgelegte Datenschutz-Maßnahmen (Schutzbedarf) | Abschnitt 6 |
| Neun Stopp-Kriterien | Abschnitt 8 |
| Pflichtfrage; der Mensch startet die getrennte Session selbst, wenn das Werkzeug es nicht kann | Abschnitt 12 |
| Gate mit acht Prüfpunkten, darunter „zweite Person benennen" | Abschnitt 12 |
| Vision-Gate vor Go-Live, externer Blick | Abschnitt 12 |
| Versions-Tabelle bestätigen | Abschnitt 15, `templates/projektstart.md` 2a |
| Freigabe zum Umgehen von Prüfungen | Abschnitt 15 |

Jede einzelne Unterbrechung ist begründet. In der Summe werden sie für die Zielgruppe zu einer Last.

Beleg aus diesem Repo: Am 2026-09-24 gab es sieben Grundsatzentscheidungen. Bei ADR-009, -012, -013 und -014 steht ausdrücklich „Nebenfragen mit Voreinstellung" (bei ADR-014 zum Beispiel Zeile 413). `CLAUDE.md` Zeile 248 warnt selbst davor, dass das Freigabe-Gate zum „Rubber-Stamp" wird, bei dem der Mensch nur noch abnickt. Das beweist kein Abnicken – der Eigentümer kann den Voreinstellungen bewusst zugestimmt haben. Es ist aber das Frühzeichen, das die eigene Regel beschreibt. Ob auch die Hauptoptionen immer der Empfehlung folgten, kann ich ohne Logbuch nicht prüfen.

### 3.6 Regeln, die nur am Pilotprojekt belegt sind

- **Herkunft der Belege:** ADR-008 bis ADR-013 stützen sich ausschließlich auf EB-Digital. Das ist ein einziges Projekt der Klasse G mit einem einzigen Eigentümer – demselben, dem auch dieses Repo gehört.
- **Geltungsbereich:** Das Gate gilt „unabhängig von der Projektklasse" (`CLAUDE.md` Abschnitt 12), obwohl die Klassen K, M und V laut `docs/project-context.md` Zeile 24 unerprobt sind.
- **Selbst genannte Konfidenz:** Die ADRs geben sie für Gate, Modellklassen, Wucherung und Anforderungsschicht selbst nur als „mittel" an (ADR-009, -011, -012, -013).
- **Setzungen ohne Messung:** Faktor 2 und „+5 Schritte" (ADR-012) sowie die Grenze von 200.000 Token (ADR-014) sind festgelegte Werte, keine Messergebnisse.

Die Belege sind echt und konkret. Die Frage ist nicht, *ob* die Lücken existieren, sondern ob die gewählte Regelform bei anderen Projekten und Klassen trägt. Das ist offen.

---

## 4. Vision-Abgleich

| # | Aussage der Vision-Quelle | Bewertung | Begründung und Beleg |
|---|---|---|---|
| V1 | Primäre Nutzer sind Einzelpersonen ohne Programmierkenntnisse, die die Technik vollständig der KI überlassen | **teilweise getragen** | Die Freigabe-Form mit „Was es für deine Vision bedeutet" und „DIE FRAGE AN DICH" (`CLAUDE.md` Abschnitt 4) passt genau zu dieser Zielgruppe. Die Zahl der Unterbrechungsarten (3.5) und die Häufung von Grundsatzentscheidungen an einem Tag arbeiten aber gegen sie. |
| V2 | Das Regelwerk gleicht fehlendes technisches Wissen aus | **getragen, mit Lücken** | Getragen durch Gate (Abschnitt 12), Versionswahl durch die KI (Abschnitt 15), Warnungen als Fehler (Abschnitt 15) und Secret-Regel (Abschnitt 6). Lücke: Die Regeln sind so zahlreich, dass ihre Anwendung wieder beim Agenten liegt, der sie in einem vollen Kontext befolgen muss (B1, B2). |
| V3 | Das Regelwerk gleicht den fehlenden roten Faden über viele Sessions aus | **getragen, aber gefährdet** | Getragen durch Logbuch, Fahrplan, Vision-Checkpoints, Landeplatz-Regel und Wucherungs-Stopp. Gefährdet durch die wachsende Pflichtlektüre (B2, B3) und die Sessiongrenze, die mehr Neustarts erzwingt. Die Zählregel ist steuerbar (W4, W5). |
| V4 | Weniger geeignet für Teams und Wegwerf-Prototypen | **stimmig, nicht abgedeckt (bewusst)** | Keine Regel für mehrere Menschen, Verweis auf Issue #33. Für Wegwerf-Prototypen ist das Regelwerk nach dem Bündel noch schwerer geworden – das ist konsistent mit der Aussage. |
| V5 | Erprobungsstand: G am Piloten, K nur hier, M und V unerprobt | **Widerspruch** | Neue Regeln gelten für alle Klassen, sind aber nur an G belegt (3.6). Der Reifegrad `[BELASTBAR]` für das Regelwerk ist nicht mehr gedeckt (W6). |
| V6 | Die Agenten selbst sind Leser und Ausführende | **teilweise getragen** | Die ANCHOR-Konvention und die Mindest-Lektüre sind für Agenten gut gebaut. Aber: keine Größengrenze für `CLAUDE.md` (B1), die Kosten-Regeln stehen vor den Kernregeln (B5), die Begründungen im Regeltext verlängern ihn (B4). |
| V7 | Referenz-Werkzeug ist eines, das Regelwerk bleibt werkzeugneutral | **teilweise Widerspruch** | Namen in `CLAUDE.md` und in `templates/projektstart.md` (W7). Die Regeln in Abschnitt 0 (Unteragenten mit festem Modell, Cache-Lesepreis, Sitzungsabfrage, Kontextgröße) setzen Werkzeug-Fähigkeiten voraus. Sie haben zwar Rückfallwege („Regel entfällt"), sind aber faktisch auf ein Werkzeug zugeschnitten. |
| V8 | Leitsatz: Technische Wachsamkeit liegt bei der KI und den Regeln, nicht beim Menschen | **nicht verankert, teils widersprochen** | Der Leitsatz steht nicht in `CLAUDE.md` (Suche nach „Wachsamkeit": kein Treffer); Issue #34 schlug die Verankerung in Abschnitt 0 ausdrücklich vor. Mehrere neue Regeln legen die Wachsamkeit wieder beim Menschen ab (siehe unten). Getragen wird der Leitsatz dagegen vom Gate („Die KI prüft das Gate unaufgefordert", Abschnitt 12), von der Umkehr der Versionsprüfung und von Warnungen als Fehler. |

**Wo die neuen Regeln die Wachsamkeit beim Menschen ablegen (zu V8):**

- Den Modellwechsel muss der Mensch vollziehen (Abschnitt 0).
- Die getrennte Instanz „stößt der Mensch an", wenn das Werkzeug keine getrennte Session kennt (`CLAUDE.md` Zeile 434).
- Den Zurücksetz-Zeitpunkt des Kontingents liefert der Eigentümer (`docs/project-context.md` Zeile 92).
- Beim Actions-Kontingent steht im Ablaufdaten-Register „genaues Datum offen, vom Eigentümer zu ergänzen" (Zeile 133).
- Die Kontingent-Warnung braucht einen Probelauf „in einer lokalen Terminal-Session des Eigentümers" (Zeile 98).

Einiges davon ist unvermeidbar. Es gehört aber benannt, weil es der Vision-Aussage direkt widerspricht.

**Nicht abgedeckt aus der Vision-Quelle, aber aus #34 offen:** Punkt 2 (Betriebsmodus). Er hat im Fahrplan keinen Schritt, sondern steht nur als „Nächster Schritt" im Fließtext (`docs/fahrplan.md` Zeile 15). Das ist nach der eigenen Landeplatz-Regel (`CLAUDE.md` Abschnitt 6) streng genommen ein Punkt ohne gültigen Landeplatz – begründet durch den offenen Wucherungs-Stopp.

---

## 5. Die drei Optionen

### 5.1 Weiterbauen

**Was es heißt:** Das Regelwerk so lassen, die offenen Punkte beheben, als Nächstes Punkt 2 aus #34 (Betriebsmodus) umsetzen.

- **Kosten:** am niedrigsten kurzfristig. Keine Überarbeitung, der Fahrplan kann direkt weitergehen.
- **Risiken:**
  - Punkt 2 ändert das Phasenmodell (Triage in #34: „separat entscheiden") und macht das Regelwerk noch einmal länger. Ohne Größengrenze (B1) wächst `CLAUDE.md` weiter in den eigenen Session-Rahmen hinein (B2).
  - Die Widersprüche W1 bis W8 bleiben stehen. Nach der Erfahrung aus S-11 („Eine Regel, die an zwei Stellen steht, muss an beiden korrigiert werden – sonst … gewinnt möglicherweise, weil sie näher am Arbeitsort steht") wird ein Agent sie im Zweifel unterschiedlich auflösen.
  - Die Selbstausnahme von der Reaktiv-Quote (W3) verhindert, dass das Repo selbst ein Warnsignal bekommt.
- **Wann richtig:** nur wenn man die Befunde als reine Pflegearbeit einstuft. Das tue ich nicht, weil W1, W3 und B1 die Wirksamkeit von Kernregeln betreffen.

### 5.2 Gezielt umbauen (Empfehlung)

**Was es heißt:** Das Fundament bleibt, vier abgegrenzte Blöcke werden überarbeitet. Bis die Blöcke erledigt sind, kommt keine neue Regel dazu.

**Block U1 – Regelwerk verschlanken und begrenzen.**

- Eine harte Größengrenze für `CLAUDE.md` in Abschnitt 2 einführen, analog zu `project-context.md`. Ein Richtwert zur Diskussion: nicht größer als vor dem Bündel, also etwa 60 KB.
- „Warum"-Absätze und Pilot-Geschichten in die ADRs verschieben und in `CLAUDE.md` nur verweisen (B4).
- Den Kosten-Teil aus Abschnitt 0 (Abgabe, Preise, Sessiongröße, Kontingent) in einen Anhang verschieben, der bei Bedarf geladen wird. In Abschnitt 0 bleiben nur die Klassen und die Eskalations-Auslöser (B5).
- Einen Archivierungs-Auslöser für den Fahrplan bei Klasse K ergänzen, zum Beispiel „erledigte Schritte eines abgeschlossenen Bündels" (B3).

**Block U2 – Unterbrechungen bündeln.**

- Modellwechsel-Stopp und Freigabe-Block zu *einem* Block zusammenlegen, mit der Zeile „Benötigte Klasse" (A3).
- Nebenfragen pro Sitzung höchstens einmal gesammelt vorlegen.
- Eine Obergrenze für Grundsatzentscheidungen je Tag oder Session erwägen, damit Freigaben nicht zum Abnicken werden (3.5).
- Festlegen, ob die Pflichtfrage eine Freigabe im Sinne von Abschnitt 4 ist.

**Block U3 – Widersprüche auflösen.**

- **Fertig-Status:** Entweder eine ausdrückliche, begründete Ausnahme mit ADR und Nachprüf-Schritt einführen, oder die Schritte bleiben `[IN ARBEIT]`, bis die CI nachgelaufen ist (W1).
- **Phasentyp:** Für die Arbeit am Regelwerk selbst festlegen, welcher Phasentyp gilt. Neue Regelbereiche laufen als UMSETZUNG (W2).
- **Reaktiv-Quote:** Die Selbstausnahme für Methodik-ADRs aufheben. Mindestens gilt: Ein ADR, der einen anderen ADR innerhalb kurzer Zeit korrigiert, ist `[REAKTIV]` (W3).
- **Bündel-Zählung:** Definieren, was zu einem Bündel gehört (Inhalt, nicht Nummernbereich). Den Plan beim Anlegen festhalten, in derselben Einheit wie den Ist-Stand. Stufen einer freigegebenen Aufteilung zählen als eine Einheit (W4, W5).
- **Selbstanwendung:** die eigenen Angaben berichtigen – Abhängigkeiten, Skripte, Linter für das Python-Skript, `.pyc` entfernen, Werkzeug-Verzeichnis, Reifegrad (W6).
- **Namen und Verweise:** Namen aus `CLAUDE.md` und `templates/projektstart.md` entfernen; den Widerspruch „stärkste verfügbare Modell" gegen die Entscheidungs-Klasse klären; die Neutralitäts-Suche künftig über die *ganze* Datei laufen lassen (W7). Nummerierung und Verweise korrigieren (W8).

**Block U4 – Ehrlicher Reifegrad und Leitsatz.**

- Die neuen Teile des Regelwerks (Gate, vier Klassen, Sessiongröße, Wucherung, Anforderungsschicht) als eigene Bestandteile mit `[VORLÄUFIG]` führen, bis sie an einem zweiten Projekt belegt sind (V5).
- Den Leitsatz aus #34 in Abschnitt 0 verankern (V8).
- Die Regeln, die den Menschen zur Wachsamkeit verpflichten, einzeln prüfen. Wo es geht, soll die KI den Wert selbst ermitteln oder die Lücke als Ablaufdatum mit Frist führen.

**Kosten:** geschätzt zwei bis vier Sessions auf der Entscheidungs-Klasse, jede davon mit Freigabe. Der größte Aufwand ist U1, weil Text verschoben wird, ohne dass sich Regeln ändern dürfen. Dafür braucht es eine sorgfältige Gegenprobe: jede Regel vorher und nachher auffindbar.

**Risiken:**

- Beim Verschieben können Regeln still verloren gehen – dasselbe Muster wie der Teil-C-Fehler aus ADR-007. Gegenmittel: eine Liste aller Regeln vor dem Umbau, abgehakt nach dem Umbau.
- Der Umbau erzeugt selbst wieder Freigaben und damit genau die Belastung aus 3.5. Gegenmittel: Block U2 zuerst.
- Das Pilotprojekt nutzt eine ältere Fassung. Ein Umbau macht die Übernahme dort aufwendiger.

### 5.3 Neu aufsetzen

**Was es heißt:** `CLAUDE.md` von Grund auf neu schreiben, zum Beispiel als kurzer Kern plus Module je Lebensphase (Start, Bau, Betrieb).

- **Kosten:** hoch. Die ganze Selbstanwendung, alle Vorlagen-Verweise, die ANCHOR-Struktur und die Verweise aus 14 ADRs müssten neu abgeglichen werden. Die über Monate im Piloten bewährten Formulierungen – Vision-Checkpoints (ADR-006), Stopp-Kriterien, Freigabe-Form – verlören ihren Bewährungsstand.
- **Risiken:**
  - Ein Neuaufbau würde von derselben Stelle geschrieben, die das heutige Wachstum erzeugt hat, und ist an keinem Projekt erprobt.
  - Genau die Stärken, die Issue #34 selbst bestätigt („Roter-Faden-Probleme gut aufgefangen"), stünden neu auf dem Spiel.
- **Wann richtig:** wenn das Fundament falsch wäre. Die Befunde zeigen aber Wachstum und Widersprüche an den Rändern, keinen Fehler im Kern. Die Prosa-Architektur (`docs/architecture.md` Abschnitt 1) und die Kernmechanik (Pflichtlektüre, Freigabe, Stopp, Fertig-Status, Landeplatz) sind nicht widerlegt.

---

## 6. Empfehlung

**Gezielt umbauen**, in der Reihenfolge U2, U3, U1, U4.

- **Warum U2 und U3 zuerst:** Sie sind klein und senken sofort die Last für den Menschen und die Widersprüche für den Agenten.
- **Warum U1 danach:** Verschlanken lohnt erst, wenn die Widersprüche aufgelöst sind. Sonst verschiebt man sie nur.
- **Neue Regeln:** Bis U1 bis U4 abgeschlossen sind, kommt keine neue Regel dazu. Punkt 2 aus #34 (Betriebsmodus) wird erst danach entschieden.
- **Zusätzlich:** Die neuen Regeln an einem zweiten, echten Projekt der Klasse K oder M erproben, bevor sie als belastbar gelten.

**Konfidenz: mittel.** Die Widersprüche und Größenzahlen sind am Text belegt. Ob der Umbau in der Praxis mehr bringt als er kostet, hängt aber von Nutzungsdaten ab, die ich nicht hatte: kein Logbuch, kein zweites Projekt, und ich laufe auf derselben Modell-Linie wie die bauende Session.

---

## 7. Laufzeitumgebung dieser Prüfung

Abfrage der Sitzung am 2026-09-24:

- **Eingestelltes Modell:** `claude-opus-5-5`
- **Zuletzt bedientes Modell:** `claude-opus-5-5`

Das ist nach `docs/project-context.md` Abschnitt 6 die Entscheidungs-Klasse. Es ist dieselbe Linie wie die der bauenden Session. Den Wunsch nach einem anderen Modell aus `CLAUDE.md` Abschnitt 12 erfüllt diese Prüfung deshalb nicht.
