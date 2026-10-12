# Project Context – Dev-Templates

<!-- Arbeitsdokument von Dev-Templates selbst (Selbstanwendung der Methodik, ADR-001).
     Die unausgefüllten Vorlagen für Ziel-Projekte liegen unter templates/docs/.
     Reduzierte Form nach Klasse K (templates/projektstart.md Abschnitt 2). -->

<!-- ANCHOR:kerndaten -->
## 1. Kerndaten

- **Projektname:** Dev-Templates
- **Kurzbeschreibung:** Methodik-Framework und Vorlagen-Set für Software-Projekte, die einen KI-Coding-Agent als Hauptentwickler einsetzen. Das Produkt sind Regelwerk und Dokument-Vorlagen. Einziger ausführbarer Bestandteil ist ein optionales Hilfsskript für ein Werkzeug unter `templates/werkzeuge/`.
- **Status:** aktive Entwicklung
- **Version (SemVer):** keine. Der Methodik-Stand wird über Patch-Wellen (PR-Nummern) in `README.md` geführt, nicht über SemVer-Tags. Begründung: Es gibt kein installierbares Artefakt, dessen Kompatibilität versioniert werden müsste.
- **Dokumentationssprache:** Deutsch
- **Codesprache:** nicht anwendbar – das Repo enthält keinen Anwendungscode
- **Projekttyp:** Methodik- und Dokumentations-Repository
- **Projektgrößen-Klasse:** K (Klein) – fixiert in ADR-001

<!-- ANCHOR:zielgruppe-und-nutzungskontext -->
## 2. Zielgruppe und Nutzungskontext

- **Primäre Nutzer:** Einzelpersonen ohne Programmierkenntnisse, die eine Software-Idee und Fachwissen über ihren Anwendungsbereich mitbringen und die technische Umsetzung vollständig einem KI-Coding-Agent überlassen (Vision-Driven Development). Das Regelwerk gleicht zwei Schwächen aus: fehlendes technisches Wissen und fehlenden roten Faden über viele Sessions.
- **Weniger geeignet:** Teams mit mehreren parallel arbeitenden Menschen (Rollen und Abstimmung nicht geregelt, Issue #33), Wegwerf-Prototypen. Erfahrene Entwickler können die Methodik nutzen, sind aber nicht die Zielgruppe.
- **Erprobungsstand:** Klasse G an einem Pilotprojekt (seit Mai 2026), Klasse K nur an diesem Repo selbst (ohne Anwendungscode), Klassen M und V unerprobt.
- **Sekundäre Nutzer:** die Coding-Agents selbst – sie sind Leser und Ausführende des Regelwerks, nicht nur Gegenstand.
- **Nutzungsumgebung:** Referenz-Werkzeug ist Claude Code. Das Regelwerk ist werkzeugneutral gehalten; Einstiegspunkte für andere Agents sind in `README.md` beschrieben.
- **Vision-Quelle für den Abgleich an Bündelgrenzen** (festgelegt vom Eigentümer am 2026-09-24): Dev-Templates hat keine `docs/vision.md`. Quelle ist dieser Abschnitt 2 zusammen mit dem Leitsatz aus Issue #34: „Technische Wachsamkeit muss bei der KI und den Regeln liegen, nicht beim Menschen."

<!-- ANCHOR:technischer-stack -->
## 3. Technischer Stack

- **Sprachen:** Markdown (GitHub Flavored). Dazu Python für ein einzelnes ausgeliefertes Hilfsskript (`templates/werkzeuge/claude-code/kontingent-warnung.py`, nur Standardbibliothek). Keine Laufzeitumgebung für das Repo selbst, keine Paketverwaltung.
- **Externe Abhängigkeiten:** keine zur Laufzeit. Entwicklungswerkzeuge: `markdownlint-cli2` und `pre-commit` (ADR-002).
- **Tooling im Repo:** `.prettierignore` – schließt sämtliche Markdown-Dateien von Prettier aus. Die Datei richtet sich an abgeleitete Projekte, deren Pre-Commit-Hook Prettier auch über Markdown laufen ließe. Dazu `ruff.toml` (Linter- und Formatter-Regeln für das Python-Hilfsskript) und `.gitignore` (Python-Kompilate, S-24).

### Unterstützte Plattformen

Nicht anwendbar in der üblichen Form: Im Repo selbst wird nichts ausgeführt außer dem Pre-Commit-Hook. Das ausgelieferte Hilfsskript nennt seine Plattformen im eigenen Kopf. Verwendbar ist es überall dort, wo ein `AGENTS.md`- oder `CLAUDE.md`-fähiger Coding-Agent läuft.

<!-- ANCHOR:architektur-grobstruktur -->
## 4. Architektur-Grobstruktur

Drei Bestandteile ohne Laufzeit-Kopplung. Details in `docs/architecture.md`.

**Namenskonvention:** In diesem Repo heißen die Module „Bestandteile". Das Fahrplan-Feld „Betroffene Bestandteile" entspricht dem Feld „Betroffene Module" der Vorlage; der Drift-Anker „Modul-Liste" (`CLAUDE.md` Abschnitt 16) vergleicht diese Namen.

- **Regelwerk** (`CLAUDE.md`, `AGENTS.md`) – die verbindliche Methodik, projektübergreifend unverändert
- **Vorlagen** (`templates/`) – das ausgelieferte Produkt: Dokument-Vorlagen, CI-Skelette, Pre-Commit-Konfigurationen
- **Selbstanwendung** (`docs/`) – die ausgefüllten Arbeitsdokumente dieses Repos

<!-- ANCHOR:externe-abhaengigkeiten -->
## 5. Externe Abhängigkeiten

Keine Services, APIs oder Laufzeit-Bibliotheken. Entwicklungswerkzeuge: `markdownlint-cli2` und `pre-commit`, beide über `.pre-commit-config.yaml` mit fester Version (ADR-002). Das Hilfsskript unter `templates/werkzeuge/` nutzt nur die Python-Standardbibliothek.

<!-- ANCHOR:constraints -->
## 6. Constraints (operationalisierbar)

### Werkzeug- und Modellneutralität

- **Regel:** `CLAUDE.md` nennt keine konkreten Werkzeug- oder Modellnamen. Konkretisierungen gehören in `docs/project-context.md` des jeweiligen Projekts.
- **Prüfung:** Grep über die **ganze** Datei `CLAUDE.md` und über `templates/projektstart.md` auf Modell- und Produktnamen muss leer bleiben – nicht nur über geänderte Zeilen (ADR-017). Ausgenommen ist der Dateiname `CLAUDE.md` selbst. Werkzeugspezifische Anleitungen unter `templates/werkzeuge/<werkzeug>/` sind bewusst werkzeuggebunden und nicht Teil der Prüfung.

### Einzige Quelle der Wahrheit

- **Regel:** `AGENTS.md` und weitere Einstiegsdateien verweisen ausschließlich auf `CLAUDE.md` und duplizieren keine Regeln.
- **Prüfung:** Einstiegsdateien enthalten keine eigenständigen Verhaltensregeln.

### Vorlagen bleiben unbefüllt

- **Regel:** Dateien unter `templates/docs/` behalten ihre Platzhalter. Konkrete Werte gehören nach `docs/`.
- **Prüfung:** Jede Datei unter `templates/docs/` enthält mindestens ein Platzhalter-Feld in eckigen Klammern.

### Compliance und Lizenz

- **Projektlizenz:** CC0 1.0 Universal (Public-Domain-Widmung), ADR-004. `LICENSE`-Datei live von `creativecommons.org` bezogen, nicht aus dem Trainingsstand rekonstruiert.
- **Erlaubte Abhängigkeitslizenzen:** mit CC0 verträgliche freie Lizenzen. Bestand: nur die Entwicklungswerkzeuge `markdownlint-cli2` (MIT) und `pre-commit` (MIT).

### Anforderungen, Schutzbedarf, Kosten

- **`docs/requirements.md`:** nicht anwendbar, Begründung: Klasse K (ADR-001). Die Anforderungen an die Methodik stehen als Issues (z. B. #20, #34) und als Akzeptanzkriterien der Fahrplan-Schritte.
- **Schutzbedarf:** nicht anwendbar, Begründung: Das Repo verarbeitet keine personenbezogenen Daten.
- **Kostenrahmen:** keine laufenden Kosten außer dem KI-Verbrauch über das Abo des Eigentümers (Abschnitt „Methodik-Schwellenwerte", Modellklassen-Zuordnung); kein Kostenregister (Klasse K).

### Methodik-Schwellenwerte

- **Reaktiv-ADR-Schwellenwert:** maximal 30 % `[REAKTIV]`-Anteil über die letzten 10 ADRs (Klasse-K-Empfehlung). Methodik-ADRs dieses Repos fallen nicht unter die Kategorie-Regel für die STABILISIERUNG (Kategorien 1, 2, 4, 5 aus `CLAUDE.md` Abschnitt 4), wohl aber unter die Korrektur-Regel: Ein ADR, der einen anderen innerhalb von 14 Tagen ändert oder zurücknimmt, ist `[REAKTIV]` (ADR-017; die frühere Selbstausnahme aus ADR-012 ist aufgehoben).
- **Phasentyp bei Arbeit am Regelwerk:** Ein neuer Regelbereich (neue Pflicht, neues Pflichtdokument, neues Verfahren) ist ein UMSETZUNG-Schritt. Die STABILISIERUNG härtet nur bestehende Regeln: Korrekturen, Präzisierungen, Widersprüche auflösen, Kürzen (ADR-017).
- **Wucherungs-Schwelle:** Faktor 2 gegenüber dem ursprünglichen Schrittplan und mindestens 5 zusätzliche Schritte (Default, ADR-012).
- **Modellklassen-Zuordnung** (Regelwerk: `CLAUDE.md` Abschnitt 0, „Modellklassen-Disziplin"):
  - **Mechanik-Klasse:** Claude Haiku 4.5 – Probelauf **bestanden am 2026-09-24 für Suchen und Zählen** (Reaktiv-Quote nachzählen, Schritt-IDs gegen den Fahrplan abgleichen, Status-Liste): beide absichtlich eingebauten Fehler gefunden, alle Zählwerte korrekt. Für andere Mechanik-Arbeit noch nicht erprobt.
  - **Routine-Klasse:** Claude Sonnet 5 – trägt den Normalbetrieb: Vorlagen-Pflege, Logbuch, README-Synchronisation, Drift-Prüfung, Archivierung, Formulierungsarbeit an bestehenden Abschnitten. Probelauf **bestanden am 2026-09-24** für die Drift-Prüfung (beide eingebauten Fehler mit Beleg gefunden, übrige drei Anker korrekt grün, dazu ein echter Nebenbefund) und für einen README-Eintrag (inhaltlich korrekt, jede Aussage belegt; zwei Kernpunkte ausgelassen, daher Prüfung vor dem Commit durch die Entscheidungs-Klasse nötig). Logbuch-Einträge noch nicht erprobt.
  - **Entscheidungs-Klasse:** Claude Opus, aktuelle Linie (Stand 2026-09-24: Opus 5.5) – Zuständig für Änderungen am Regelwerk selbst, weil jede solche Änderung projektübergreifend wirkt und damit unter die Eskalations-Auslöser aus `CLAUDE.md` Abschnitt 0 fällt.
  - **Ausnahme-Klasse:** Claude Fable 5.1 – nur auf Vorschlag mit Freigabe.
  - **Bezugsmodell und knappe Ressource:** Abo (Max 5x); knapp ist das Wochenkontingent, Zurücksetzung sonntags 10:00 (Angabe des Eigentümers), dazu ein Kurzzeitlimit je 5 Stunden.
  - **Abgabe an Unteragenten:** Claude Code – Modell je Unteragenten-Aufruf oder als `model:` in der Agent-Definition. Erprobt am 2026-09-24.
  - **Meldet die Laufzeitumgebung das Modell / das Kontingent?** (Stand 2026-09-24, S-19) Sitzungsabfrage in Cloud-Sessions: eingestelltes und bedientes Modell, Kurzzeitlimit mit Zurücksetz-Zeitpunkt, kein Wochenlimit; ihr Kostenzähler wird verzögert und gebündelt aktualisiert und taugt nicht für Messungen je Schritt. Statuszeile laut Doku: `model.id`, `cost.total_cost_usd` (Listenpreis, clientseitig), Token im Kontext und – für Pro/Max – `rate_limits.five_hour` und `rate_limits.seven_day` mit Verbrauch in Prozent und Zurücksetz-Zeitpunkt. Hooks laut Doku: Modell nur optional bei `SessionStart`, Modellwechsel über `PreModelSwitch`/`PostModelSwitch` (`from_model`, `to_model`); keine Kosten- oder Limitfelder; Text ins Gespräch über stdout bei `SessionStart`, `UserPromptSubmit`, `UserPromptExpansion`, `PostModelSwitch`. Nichts davon selbst erprobt.
  - **Ersparnis der Abgabe (S-19, Modellrechnung, nicht direkt gemessen):** Bei Opus 5.5 kostet das Cache-Lesen so viel wie bei Sonnet 5 (je 0,20 $ je 1 Mio. Token, Listenpreis 2026-06-24); Sonnet ist nur bei neuer Eingabe und Ausgabe halb so teuer, Haiku 4.5 auch beim Cache-Lesen. Lesearbeit mit vielen Werkzeugaufrufen spart durch Abgabe an die Routine-Klasse deshalb kaum etwas; die Drift-Prüfung als Unteragent (13 Aufrufe, rund 136.000 Token Endkontext) war in der Rechnung eher teurer als 3–4 Aufrufe im geladenen Kontext. Größter Hebel ist die Kontextgröße der Hauptsitzung: Jeder Aufruf liest den ganzen Kontext aus dem Cache (bei rund 510.000 Token etwa 0,10 $ je Aufruf). Für das Abo ist die Gewichtung der Modelle im Kontingent nicht veröffentlicht; die Rechnung nutzt Listenpreise als Ersatz.
  - **Preise je Klasse** (Listenpreis, Stand 2026-06-24, Referenz des Werkzeugs; Eingabe / Ausgabe / Cache-Lesen je 1 Mio. Token): Mechanik (Haiku 4.5) 1 $ / 5 $ / 0,10 $; Routine (Sonnet 5) 2 $ / 10 $ / 0,20 $; Entscheidung (Opus 5.5) 4 $ / 20 $ / 0,20 $; Ausnahme (Fable 5.1) 10 $ / 50 $ / 0,25 $. Folge für die Abgabe (ADR-014; Regeltext seit ADR-018 in `templates/wirtschaftlichkeit.md`): an die Mechanik-Klasse auch Lesearbeit; an die Routine-Klasse nur ausgabelastige Arbeit.
  - **Grenze der Sessiongröße:** 400.000 Token Kontext, nur Hinweis beim Sessionende (ADR-016, vorher 200.000 mit Stopp nach ADR-014). Bestätigt am 2026-09-27 (ADR-018): Die Mindest-Lektüre einer frischen Session kostet rund 130.000 Token, für die Arbeit bleibt etwa das Doppelte. Quelle: Sitzungsabfrage (`context_usage.used_tokens`).
  - **Kontingent-Warnung:** inaktiv. Skript `templates/werkzeuge/claude-code/kontingent-warnung.py` am 2026-09-24 mit erzwungenem Fehlerfall geprüft (Schwelle 0 % → Warnung; 8 von 8 Fällen wie erwartet). Zustellweg über Statuszeile und Hook unerprobt: Diese Repo-Arbeit läuft in Cloud-Sessions ohne Statuszeile. Aktivierung erst nach einem Probelauf in einer lokalen Terminal-Session des Eigentümers.
  - **Besonderheit dieses Repos:** Das Produkt *ist* die Methodik. Änderungen an `CLAUDE.md` und `templates/projektstart.md` sind deshalb praktisch immer `[STRATEGISCH]` und lösen die Eskalation aus. Änderungen an `docs/` und an Formulierungen ohne Regelwirkung sind Routine.

<!-- ANCHOR:code-standards-und-qualitaetsziele -->
## 7. Code-Standards und Qualitätsziele

Die Pflichtkategorien aus `CLAUDE.md` Abschnitt 15 sind überwiegend nicht anwendbar, weil das Repo fast nur Markdown enthält. Ausnahme ist das Python-Hilfsskript `templates/werkzeuge/claude-code/kontingent-warnung.py` (71 Zeilen); für dieses Skript sind Linter, Formatter, Security-Regeln und Tests seit S-24 eingerichtet (ADR-017). Die Einordnung wird hier explizit vorgenommen, nicht weggelassen:

| Kategorie | Status |
|---|---|
| Linter | **eingerichtet** – `markdownlint-cli2` v0.23.2 über pre-commit, Konfiguration in `.markdownlint-cli2.jsonc`. MD013 (Zeilenlänge) und MD060 (Tabellen-Ausrichtung) bewusst deaktiviert, siehe ADR-002. Deckt strukturelle Markdown-Fehler ab (fehlende Sprachangabe an Codeblöcken, Leerzeilen-Konventionen, doppelte Überschriften) – **nicht** defekte relative Links und **nicht** fehlende ANCHOR-Kommentare, dafür existiert kein Standard-Regelsatz. |
| Formatter | bewusst deaktiviert für Markdown – Begründung steht in `.prettierignore`: Tabellen, ANCHOR-Kommentare und Status-Marker sind handgepflegte Strukturen, Auto-Formatierung erzeugt dort Diff-Lärm. |
| Linter (Python-Skript) | **eingerichtet** – `ruff` v0.16.9 (`ruff-check`) über pre-commit, Konfiguration in `ruff.toml`: Regeln E, F, W, B, BLE, S, UP und RUF100 (meldet ungenutzte Suppressions). Erzwungener Fehlerfall am 2026-09-27: ungenutzter Import und ungenutztes `noqa` → Hook rot (F401, RUF100). |
| Formatter (Python-Skript) | **eingerichtet** – `ruff-format`, auf Python beschränkt (`types_or: [python]`), damit Markdown weiter unformatiert bleibt. |
| Type-Checker | Markdown: nicht anwendbar. Python-Skript: **bewusst nicht eingerichtet** (ADR-017): 71 Zeilen ohne Typannotationen, nur Standardbibliothek; ein Type-Checker wäre eine weitere Abhängigkeit mit wenig Prüfwirkung. Neu zu entscheiden, sobald weiterer Python-Code hinzukommt. |
| Security-Scanner | Markdown: nicht anwendbar. Python-Skript: **eingerichtet** über die bandit-Regeln von `ruff` (Regelgruppe S), ohne eigene Abhängigkeit. |
| Dependency-Audit | nicht anwendbar – kein Paketmanager; das Skript nutzt nur die Standardbibliothek, die Entwicklungswerkzeuge sind über `.pre-commit-config.yaml` fest versioniert |
| Test-Runner mit Coverage | Markdown: nicht anwendbar; die Qualitätssicherung erfolgt über die Drift-Prüfungen aus `CLAUDE.md` Abschnitt 16. Python-Skript: **eingerichtet** – `unittest` (Standardbibliothek) als lokaler pre-commit-Hook mit `-W error`, 8 Tests in `templates/werkzeuge/claude-code/test_kontingent_warnung.py`, darunter der erzwungene Fehlerfall aus S-22. Abdeckung am 2026-09-27 mit dem Standardmodul `trace` gemessen: 37 von 42 ausführbaren Zeilen im Prozess (88 %). Die 5 fehlenden Zeilen sind der Startblock `__main__`; ihn prüft ein Test im Unterprozess, den `trace` nicht mitzählt. Mindestwert: keiner festgelegt (Klasse K, ein Skript). Erzwungener Fehlerfall am 2026-09-27: Schwellenvergleich absichtlich verfälscht → 2 Tests rot. |

### Warnungs-Bestand

Kein Bestand bei den Werkzeugen: `markdownlint-cli2` und `ruff` kennen nur Fehler, keine Warnungen; die Tests laufen mit `-W error`. **Benannte Ausnahme (CI-Plattform, seit 2026-10-12):** genau eine Warnung „Node.js 20 is deprecated … actions/cache@v4" je Lauf, aus `pre-commit/action@v3.0.1`. Obergrenze: diese eine. Landeplatz: S-28, Frist 2026-12-31. **Warnungsquellen ohne Schalter:** Hinweise der CI-Plattform zu Action-Versionen in `.github/workflows/ci.yml` (Abkündigungen von Laufzeitumgebungen der Actions) – bei jeder Beurteilung eines CI-Laufs im Protokoll zu lesen (`CLAUDE.md` Abschnitt 15).

### Durchsetzungsmechanismen

- **Pre-Commit-Hook:** eingerichtet – `.pre-commit-config.yaml`, vier Hooks: `markdownlint-cli2`, `ruff-check`, `ruff-format` und `test-kontingent-warnung` (lokal, Standardbibliothek). Die Skelette unter `templates/pre-commit/` bleiben davon unberührt Produkt, nicht Eigenkonfiguration.
- **CI-Pipeline:** eingerichtet – `.github/workflows/ci.yml`, ein Job (`pre-commit/action`, führt dieselben Hooks wie lokal aus). Deckt Linter, Formatter und die Tests des Hilfsskripts ab; die Inter-Pflicht-Drift-Checks aus Abschnitt 16 sind nicht automatisiert, siehe Abschnitt 11.

<!-- ANCHOR:betrieb-und-deployment -->
## 8. Betrieb und Deployment

Kein Deployment. Das Repo wird geklont oder geforkt und ist damit einsatzbereit.

### Ablaufdaten-Register

| Was | Ablauf / Lebensende | Vorlauf | Quelle | Fahrplan-Schritt |
|---|---|---|---|---|
| Actions-Kontingent des Kontos (CI läuft auf `ubuntu-latest`) | monatlicher Reset, vermutlich am 29. des Monats (nächster: 2026-09-29; Angabe des Eigentümers vom 2026-09-27, unbestätigt). Frist für den Nachlauf: spätestens 2026-10-31 (ADR-017) | – (bereits erschöpft seit 2026-09-15) | Angaben des Eigentümers, Logbuch 2026-09-23 und 2026-09-27; Repo privat (GitHub-API, 2026-09-27) | S-17 (erledigt 2026-10-12: Kontingent wieder verfügbar, Nachlauf grün) |
| `actions/cache@v4` (über `pre-commit/action@v3.0.1`) auf Node 20 | Node 20 seit 2026-09-23 von den Runnern entfernt, die Aktion läuft zwangsweise auf Node 24; kein hartes Lebensende veröffentlicht. Nachprüfung: 2026-12-31 | – (Warnung bereits im CI-Protokoll) | GitHub-Changelog 2025-09-19; CI-Lauf 121, Versuch 2 (2026-10-12); Release-Seite `pre-commit/action` (2026-10-12) | S-28 |

<!-- ANCHOR:entscheidungsbefugnisse -->
## 9. Entscheidungsbefugnisse

- **Freigabe-Entscheidungen trifft:** der Repo-Eigentümer (Paddel87).
- **Kommunikationskanal:** direkt im Chat mit dem Coding-Agent, Ergebnis als ADR in `docs/decisions.md`.

<!-- ANCHOR:repository-regeln -->
## 10. Repository-Regeln

- **Hauptbranch:** `main`
- **Push-Regel:** Änderungen laufen über Pull Requests, auch bei Alleinarbeit – die PR-Beschreibung ist der Ort, an dem Begründung und Verifikation dokumentiert werden.
- **Branch-Namen:** Agent-Sessions arbeiten auf `claude/<thema>`-Branches.
- **Schutzregeln:** keine Force-Pushes auf `main`.

<!-- ANCHOR:offene-grundsatzfragen -->
## 11. Offene Grundsatzfragen

- ~~**Lizenz**~~ – gelöst 2026-08-13, ADR-004: CC0 1.0 Universal. Korrektur zur ursprünglichen Formulierung dieses Punkts: Das Repo ist derzeit **privat** (per GitHub-API bestätigt) – es gab nie akute Nutzungsunsicherheit für Dritte, da keine Dritten Zugriff hatten. Die Lücke war eine Inkonsistenz zwischen README-Versprechen und Realität, kein akutes Rechtsrisiko.
- ~~**Markdown-Linter**~~ – gelöst 2026-08-13, ADR-002: `markdownlint-cli2` über pre-commit eingerichtet. Weiterhin ungedeckt: defekte relative Links und fehlende ANCHOR-Kommentare in neuen Abschnitten – dafür existiert kein Standard-Regelsatz, das bleibt manuelle Prüfung.
- ~~**CI-Pipeline (Linter-Gate)**~~ – gelöst 2026-08-13, ADR-003: `.github/workflows/ci.yml` erzwingt den pre-commit-Hook jetzt auch remote, nicht nur lokal.
- **CI-Pipeline (Drift-Checks):** Die Inter-Pflicht-Drift-Prüfungen aus `CLAUDE.md` Abschnitt 16 sind weiterhin nicht automatisiert und laufen ausschließlich als Sessionende-Disziplin. Bewusst von der CI-Einrichtung abgespalten, siehe `docs/fahrplan.md` S-7.
- ~~**README-Struktur**~~ – entschieden 2026-08-13: `README.md` behält die freie Form, keine Umstellung auf `readme-vorlage.md`. Begründung: `README.md` ist hier zugleich Werbetext für das Vorlagen-Set selbst, nicht nur Statusbild eines Projekts – die Status-Block-Form aus der Vorlage ist für diese Doppelrolle nicht passend. Kein ADR nötig, da keine der acht Kategorien aus `CLAUDE.md` Abschnitt 4 berührt ist.

<!-- ANCHOR:glossar -->
## 12. Glossar

- **Pflicht-Dokument:** eines der sechs Dokumente der Mindest-Lektüre aus `CLAUDE.md` Abschnitt 2.
- **Vorlage:** unbefüllte Datei unter `templates/docs/`, bestimmt für das Ziel-Projekt.
- **Selbstanwendung:** die Praxis, die Methodik auf dieses Repo selbst anzuwenden – festgelegt in ADR-001.
- **Patch-Welle:** eine zusammenhängende Methodik-Änderung, geführt als PR und in `README.md` unter „Methodik-Stand" verzeichnet.
