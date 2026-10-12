# Offene Punkte

Einfache Liste, keine Schritt-IDs, keine Status-Marker. Erledigtes wird gestrichen und landet im Methodik-Stand der `README.md`. Die früheren Schritt-Nummern (S-…) stehen zur Orientierung dabei; ihre Vorgeschichte liegt in `docs/archiv/selbstanwendung/fahrplan.md`.

## Produkt (Regelwerk und Vorlagen)

- **Leitsatz verankern und Wachsamkeit prüfen** (früher S-26): Den Leitsatz aus Issue #34 („Technische Wachsamkeit muss bei der KI und den Regeln liegen, nicht beim Menschen") in `CLAUDE.md` Abschnitt 0 aufnehmen. Jede Regel prüfen, die Wachsamkeit beim Menschen ablegt. Grundlage: `docs/pruefberichte/2026-09-24-pflichtfrage-buendel-34.md`, Block U4.
- **Betriebsmodus nach dem ersten produktiven Deployment** (früher S-27, Issue #34 Punkt 2): als vierter Phasentyp oder als parallele Spur.
- **Beispielversionen in den Vorlagen** (früher S-18): `templates/pre-commit/typescript.yaml` pinnt Prettier auf eine Vorabversion (`v4.0.0-alpha.8`); `templates/github-workflows/ci-typescript.yml` nennt Node 20 – seit 2026-09-23 von den GitHub-Runnern entfernt. Gegen die Regel „ausgereifte Linie" prüfen oder durch Platzhalter ersetzen.
- **Teams** (Issue #33): Rollen und Abstimmung mehrerer Menschen sind nicht geregelt.
- **Kontingent-Warnung erproben:** `templates/werkzeuge/claude-code/kontingent-warnung.py` ist nur lokal getestet; der Zustellweg über Statuszeile und Hook braucht einen Probelauf in einer lokalen Terminal-Session.

## Dieses Repo

- **CI ohne `pre-commit/action`** (früher S-28): Die Aktion bringt `actions/cache@v4` mit Node 20 mit und erzeugt eine Warnung. Ersatz durch eigene Schritte (pre-commit installieren, aktuelle Cache-Aktion, `pre-commit run --all-files`). Spätestens prüfen am 2026-12-31.
- **PR #31 (Drift-Prüfskript):** Das Skript war für die Selbstanwendung gedacht, die es nicht mehr gibt. Der PR ist seit 2026-08-13 nicht mehr mergebar. Vorschlag: schließen.
