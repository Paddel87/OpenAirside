# AGENTS.md

Diese Datei ist der **werkzeugneutrale Einstiegspunkt** für Coding-Agents, die
nicht Claude Code sind (z. B. OpenAI Codex und andere `AGENTS.md`-fähige Tools).

## Das verbindliche Regelwerk steht in `CLAUDE.md`

Die vollständige Arbeitsmethodik dieses Repos ist **kanonisch und ausschließlich**
in [`CLAUDE.md`](CLAUDE.md) definiert. Diese Datei hier dupliziert die Regeln
**nicht** – sie verweist nur darauf. So gibt es genau eine Quelle der Wahrheit,
und es entsteht keine Drift zwischen zwei Regelwerken.

**Bevor du irgendetwas änderst:**

1. Lies [`CLAUDE.md`](CLAUDE.md) **vollständig** und befolge es **exakt** –
   unabhängig davon, welches Werkzeug oder Modell du bist.
2. Führe die **Mindest-Lektüre zu Sessionbeginn** durch (`CLAUDE.md` Abschnitt 2):
   `docs/project-context.md`, dann `docs/logbuch.md`, `docs/fahrplan.md`,
   `docs/architecture.md`, `docs/decisions.md`, `docs/blockers.md` – jeweils in
   dem dort beschriebenen, selektiven Umfang.
3. Lege **vor jeder anderen Aktion** einen `[SESSIONSTART]`-Eintrag im Logbuch an.

## Was du besonders beachten musst

Die Methodik ist anspruchsvoll und lebt davon, dass der Agent sie diszipliniert
befolgt – nicht davon, dass ein Harness sie erzwingt. Diese Punkte aus `CLAUDE.md`
sind die häufigsten Stolpersteine bei werkzeug- oder modellfremder Ausführung:

- **Pflichtlektüre nicht überspringen** (Abschnitt 2) – auch bei kleinen Änderungen.
- **Freigabepflichtige Entscheidungen nicht selbst treffen** (Abschnitt 4) – im
  `ENTSCHEIDUNG ERFORDERLICH`-Format vorlegen und warten.
- **Stopp-Kriterien einhalten** (Abschnitt 8) – inkl. „Architektur-Rateschluss"
  bei niedriger Konfidenz und teurer Umkehrbarkeit.
- **Keine Erfolgsmeldung ohne Verifikation** (Abschnitt 6) – Aussagen nur auf
  Basis von tatsächlichem Output.
- **Konfidenz ehrlich offenlegen** (`templates/architektur-heuristiken.md` Teil 3) –
  besonders wichtig bei Vision-Driven Development, wo kein menschlicher Experte
  einen selbstbewusst vorgetragenen Fehlschluss abfängt.

## Pflege dieser Datei

`AGENTS.md` bleibt ein **reiner Verweis**. Sie wird niemals zu einer zweiten,
abweichenden Kopie der Regeln ausgebaut. Inhaltliche Änderungen an der Methodik
gehören in `CLAUDE.md` (bzw. als ADR in `docs/decisions.md`), nie hierher.
