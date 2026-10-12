# Project Context – Dev-Templates

<!-- ANCHOR:kerndaten -->
## 1. Kerndaten

- **Projektname:** Dev-Templates
- **Kurzbeschreibung:** Regelwerk (`CLAUDE.md`) und Vorlagen (`templates/`) für Software-Projekte, die einen KI-Coding-Agent als Hauptentwickler einsetzen. Das Repo ist selbst kein Softwareprojekt, sondern das Produkt, das in andere Projekte kopiert wird.
- **Dokumentationssprache:** Deutsch
- **Lizenz:** CC0 1.0 Universal (Begründung: `docs/archiv/selbstanwendung/decisions.md`, ADR-004)

<!-- ANCHOR:arbeitsweise-in-diesem-repo -->
## 2. Arbeitsweise in diesem Repo

**Entscheidung des Eigentümers vom 2026-10-12:** Die Methodik aus `CLAUDE.md` wird auf dieses Repo nur noch in schlanker Form angewendet. Sie ist für langlebige Softwareprojekte gebaut; hier verwaltete sie vor allem sich selbst. Rückmeldungen zum Regelwerk kommen aus dem Einsatz in abgeleiteten Projekten, nicht aus der Selbstanwendung.

**Es gilt in diesem Repo:**

- Diese Datei ist die einzige Pflichtlektüre. Logbuch, Fahrplan, Architektur-, Entscheidungs- und Blocker-Dokument gibt es hier nicht mehr; die Abschnitte von `CLAUDE.md`, die sie verlangen, gelten für dieses Repo nicht. Ihr letzter Stand liegt in `docs/archiv/selbstanwendung/`.
- Änderungen laufen über Pull Requests. Die PR-Beschreibung nennt Anlass, Änderung und Prüfung. Sie ersetzt ADR und Logbuch.
- Änderungen am Regelwerk oder an den Vorlagen bespricht die KI vorher kurz mit dem Eigentümer (was, warum, Folge für abgeleitete Projekte). Kein `ENTSCHEIDUNG ERFORDERLICH`-Block, keine Modellklassen-Stopps, keine Reaktiv-Quote.
- Pflicht vor jedem Commit: `pre-commit run --all-files` grün. CI muss vor dem Merge grün sein.
- Nutzerrelevante Änderungen am Produkt bekommen einen Eintrag unter „Methodik-Stand" in `README.md`.
- Offene Themen stehen in `docs/offene-punkte.md`, als einfache Liste.

**Unverändert gilt:** Das Regelwerk bleibt werkzeug- und modellneutral. Ein Grep über die ganze Datei `CLAUDE.md` und über `templates/projektstart.md` auf Modell- und Produktnamen muss leer bleiben (ausgenommen der Dateiname `CLAUDE.md`). Werkzeuggebundene Hilfen liegen unter `templates/werkzeuge/<werkzeug>/`. `AGENTS.md` verweist nur auf `CLAUDE.md`. Dateien unter `templates/docs/` behalten ihre Platzhalter.

<!-- ANCHOR:werkzeuge -->
## 3. Werkzeuge

- **Markdown:** `markdownlint-cli2` v0.23.2 über pre-commit, Konfiguration in `.markdownlint-cli2.jsonc` (MD013 und MD060 aus). Kein Formatter für Markdown (`.prettierignore`).
- **Python** (nur `templates/werkzeuge/claude-code/kontingent-warnung.py`): `ruff` v0.16.9 (check und format, Regeln in `ruff.toml`), Tests mit `unittest` und `-W error` als lokaler pre-commit-Hook.
- **CI:** `.github/workflows/ci.yml` führt dieselben Hooks über `pre-commit/action@v3.0.1` aus.
- **Bekannte CI-Warnung:** „Node.js 20 is deprecated … actions/cache@v4" aus `pre-commit/action` (seit 2026-10-12 im Protokoll). Sie ist harmlos, solange die Aktion auf Node 24 läuft; Abhilfe steht in `docs/offene-punkte.md`.
