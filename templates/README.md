# Templates

Wiederverwendbare Skelette: Pflicht-Dokument-Vorlagen, CI-Workflows und Pre-Commit-Hooks. Sie werden in **Modus 2** (siehe [`projektstart.md`](projektstart.md) Abschnitt 1.3) in das neue Projekt kopiert und angepasst – die Dokument-Vorlagen in der Vorbereitung vor Schritt 1, die CI- und Hook-Skelette in Schritt 10.

**Abgrenzung zu `docs/`:** Dieses Verzeichnis enthält die **Vorlagen**. Das Verzeichnis `docs/` im Repo-Wurzelverzeichnis enthält die **ausgefüllten Arbeitsdokumente von Dev-Templates selbst** (Selbstanwendung der Methodik, siehe `docs/decisions.md` ADR-001). Wer dieses Repo als Ausgangspunkt für ein eigenes Projekt nutzt, ersetzt den Inhalt von `docs/` durch die eigenen, aus `templates/docs/` kopierten Dokumente.

Die Templates sind bewusst pragmatisch gehalten: vollständige, lauffähige Skelette mit klar markierten Platzhaltern, nicht Best-Practice-Showcases. Jede Anpassung an den Projekt-Stack erfolgt während Modus 2 und wird durch ADR-002 (Stack-Entscheidung) gerechtfertigt.

## Verzeichnisstruktur

```text
templates/
├── projektstart.md            Anleitung Modus 1 + Modus 2
├── architektur-heuristiken.md Entwurfs- und Konfidenz-Heuristiken
├── wirtschaftlichkeit.md      Anhang zu CLAUDE.md §0: Abgabe, Probelauf, Kontingent-Warnung (on-demand)
├── gate-deployment.md         Anhang zu CLAUDE.md §12: Prüfpunkte des Gates (on-demand)
├── docs/                      Pflicht-Dokument-Vorlagen (→ nach docs/ kopieren)
│   ├── vision.md              Eingangs-Idee (Modus 1)
│   ├── project-context.md     Stack, Constraints, Plattform-Matrix, Modellklassen
│   ├── architecture.md        Module, Schnittstellen, Tooling-Inventar
│   ├── requirements.md        Anwendungsfälle und Anforderungen (Pflicht ab Klasse M)
│   ├── fahrplan.md            Phasen und Schritte
│   ├── decisions.md           ADRs
│   ├── blockers.md            ungelöste Probleme
│   ├── logbuch.md             chronologische Sessions
│   ├── onboarding-runbook.md  End-to-End-Setup (Pflicht ab Klasse M)
│   └── readme-vorlage.md      Vorlage für die README des Ziel-Projekts
├── werkzeuge/                 werkzeugspezifische Hilfen (optional), z. B. claude-code/ mit der Kontingent-Warnung
├── github-workflows/
│   ├── ci-minimal.yml         Klasse K: ein Job, Lint + Test
│   ├── ci-python.yml          Klasse M/G: voller Gate-Satz für Python
│   └── ci-typescript.yml      Klasse M/G: voller Gate-Satz für TypeScript/Node
└── pre-commit/
    ├── python.yaml            pre-commit-Framework, Python-Stack
    └── typescript.yaml        pre-commit-Framework, TypeScript/Node-Stack
```

Klasse V (verteilt-groß) wird **nicht** als Template ausgeliefert: Die Workflow-Aufteilung pro Service ist zu projektspezifisch. Stattdessen werden die `ci-*.yml`-Templates pro Service als Ausgangspunkt kopiert und mit Path-Filtern versehen, plus eine `integration.yml` aus dem M/G-Template abgeleitet.

## Verwendung in Modus 2

1. Klasse aus ADR-001 ablesen.
2. Pro Sprache aus `project-context.md` Abschnitt 7 das passende Template wählen.
3. Template nach `.github/workflows/` (Workflow) bzw. ins Repo-Root (`.pre-commit-config.yaml`) kopieren.
4. Alle `# TBD:`-Platzhalter durch konkrete Werte ersetzen (Sprachversion, Coverage-Schwellen, Pfade).
5. Werkzeug-Konfigurationen (z. B. `pyproject.toml`, `package.json`-Scripts) konsistent mit dem Template anlegen.
6. Lokal einmal komplett durchlaufen lassen, bevor der Initialisierungs-Commit erfolgt.

## Anpassungs-Disziplin

- **Werkzeugwechsel** (z. B. `ruff` → `flake8 + black`) ist eine ADR-pflichtige Stack-Entscheidung.
- **Gate weglassen** ist freigabepflichtig: jede Pflichtkategorie aus `CLAUDE.md` Abschnitt 15 muss aktiv sein, sofern für die Sprache anwendbar.
- **Versionen pinnen**: Action-Versionen (`actions/checkout@v4`) und Tool-Versionen werden gepinnt, nicht auf `latest` gesetzt.
- **Keine Geheimnisse im Workflow**: Tokens und Secrets ausschließlich über `secrets.*`-Referenzen, nie hardcoded.
