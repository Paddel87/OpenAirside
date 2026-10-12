# Architecture – Dev-Templates

<!-- Arbeitsdokument von Dev-Templates selbst (Selbstanwendung, ADR-001).
     Die Vorlage für Ziel-Projekte liegt unter templates/docs/architecture.md.
     Reduzierte Form nach Klasse K: keine Modul-Karte, keine Schnittstellenverträge,
     kein Datenmodell – das Repo hat keine Laufzeit-Kopplung. -->

<!-- ANCHOR:reifegrad-system -->
## 0. Reifegrad-System

Es gilt das Reifegrad-System aus der Vorlage unverändert: `[BELASTBAR]`, `[VORLÄUFIG]`, `[OFFEN]`. Beförderung nur durch Validierung oder ADR, Rückstufung nur durch ADR.

Besonderheit dieses Repos: „Validierung durch funktionierende Implementierung" bedeutet hier **Bewährung im Einsatz** – ein Methodik-Bestandteil gilt als validiert, wenn er in mindestens einem realen Projekt oder in mehreren Sessions dieses Repos angewendet wurde, ohne dass er nachgebessert werden musste.

<!-- ANCHOR:ueberblick -->
## 1. Überblick

Dev-Templates liefert eine Arbeitsmethodik für KI-gestützte Softwareentwicklung. Das Produkt besteht aus Prosa-Regeln und Markdown-Vorlagen; es gibt keinen ausführbaren Code, keine Laufzeit und keine Schnittstellen im technischen Sinn.

Die prägende Kernentscheidung: Die Methodik steckt bewusst in **Prosa-Disziplin**, nicht in werkzeugspezifischer Maschinerie (keine tragenden Hooks, Skills oder Slash-Commands). Das macht sie werkzeug- und modellneutral portierbar, verlagert die Durchsetzung aber vollständig auf das befolgende Modell.

**Architektur-Pattern:** Dokument-Sammlung mit einer Quelle der Wahrheit `[BELASTBAR]`

<!-- ANCHOR:modul-karte -->
## 2. Modul-Karte

Drei Bestandteile ohne Laufzeit-Kopplung. Die Pfeile bezeichnen Ableitungs- und Verweisbeziehungen, keine Aufrufe.

```text
CLAUDE.md  ──verweist auf──>  docs/            (Selbstanwendung)
    ^                             ^
    │                             │ kopiert aus
AGENTS.md                    templates/docs/   (Vorlagen)
(verweist nur)                    ^
                                  │ beschrieben in
                             templates/projektstart.md
```

<!-- ANCHOR:module -->
## 3. Bestandteile

### Regelwerk `[BELASTBAR]`

- **Reifegrad:** `[BELASTBAR]`, seit 2026-08-13, Begründung: über mehrere Patch-Wellen (#7–#22) gereift und in einem realen Pilotprojekt der Klasse G angewendet
- **Dateien:** `CLAUDE.md`, `AGENTS.md`
- **Verantwortung:** definiert die verbindliche Arbeitsmethodik – Pflichtlektüre, Freigabe-Gates, Stopp-Kriterien, Definition of Done, Dokumentpflege.
- **Nicht-Verantwortung:** enthält keine projektspezifischen Werte, keine Werkzeug- und Modellnamen, keine Projektstart-Anleitung (die liegt in `templates/projektstart.md`), keine anlassbezogenen Details zu Wirtschaftlichkeit und Gate (Anhänge unter `templates/`, ADR-018).
- **Abhängigkeiten:** keine. `AGENTS.md` verweist ausschließlich auf `CLAUDE.md` und dupliziert nichts.

### Vorlagen `[BELASTBAR]`

- **Reifegrad:** `[BELASTBAR]`, seit 2026-08-13, Begründung: vollständig, in sich konsistent, durch PR #22 um die fehlenden Sprung-Anker ergänzt
- **Verzeichnis:** `templates/`
- **Verantwortung:** liefert die kopierfertigen Artefakte – Dokument-Vorlagen (`templates/docs/`), CI-Skelette, Pre-Commit-Konfigurationen, Projektstart-Anleitung, Architektur-Heuristiken, zwei bei Bedarf geladene Anhänge zum Regelwerk (`wirtschaftlichkeit.md`, `gate-deployment.md`, ADR-018).
- **Nicht-Verantwortung:** enthält keine ausgefüllten Werte. Jede Datei unter `templates/docs/` behält ihre Platzhalter.
- **Offene Frage:** Die CI- und Pre-Commit-Skelette decken Python und TypeScript ab. Weitere Sprachen sind nicht abgedeckt und derzeit auch nicht geplant.

### Selbstanwendung `[VORLÄUFIG]`

- **Reifegrad:** `[VORLÄUFIG]`, seit 2026-08-13, Begründung: mit ADR-001 gerade erst eingeführt, noch nicht über mehrere Sessions bewährt
- **Verzeichnis:** `docs/`
- **Verantwortung:** hält die ausgefüllten Arbeitsdokumente dieses Repos – Projektkontext, Architektur, Fahrplan, Entscheidungen, Blocker, Logbuch.
- **Nicht-Verantwortung:** ist kein Vorlagen-Lieferant. Wer das Repo als Ausgangspunkt nutzt, ersetzt diesen Inhalt durch eigene, aus `templates/docs/` kopierte Dokumente.
- **Offene Frage:** Ob die Selbstanwendung im Alltag durchgehalten wird – insbesondere die Logbuch-Pflicht bei kurzen Vorlagen-Korrekturen – muss sich über mehrere Sessions zeigen. Beförderung auf `[BELASTBAR]` frühestens danach.

<!-- ANCHOR:schnittstellenvertraege -->
## 4. Schnittstellenverträge

Nicht anwendbar: keine technischen Schnittstellen. Die einzige vertragsartige Zusicherung ist struktureller Natur und in `docs/project-context.md` Abschnitt 6 als prüfbare Regel hinterlegt – Werkzeug- und Modellneutralität von `CLAUDE.md`, keine Regel-Duplikate in Einstiegsdateien, unbefüllte Vorlagen.

<!-- ANCHOR:datenfluss -->
## 5. Datenfluss

Ein einziger, manueller Fluss: **Vorlage → Kopie → Befüllung.** Beschrieben in `templates/projektstart.md` Abschnitt 1.3, Vorbereitung vor Schritt 1.

<!-- ANCHOR:nicht-funktionale-anforderungen -->
## 6. Nicht-funktionale Anforderungen

### Lesbarkeit unter Kontext-Budget `[BELASTBAR]`

Die Mindest-Lektüre muss in ein übliches Kontextfenster passen, ohne dass Abschnitte ausgelassen werden. Durchgesetzt über das Größen-Budget in `CLAUDE.md` Abschnitt 2 (15.000 Token bzw. ~600 Zeilen für `project-context.md`) und die Archivierungs-Trigger in Abschnitt 14.

### Adressierbarkeit einzelner Abschnitte `[BELASTBAR]`

Ein Read, der gegen ein Tool-Limit läuft, muss gezielt fortsetzbar sein. Umgesetzt über die ANCHOR-Kommentare in allen Pflicht-Dokumenten (PR #22).

### Portierbarkeit auf fremde Agents `[VORLÄUFIG]`

Mechanisch gegeben durch `AGENTS.md`. Wie treu ein fremdes Modell die Methodik befolgt, ist Modell-Verhalten und nicht durch das Repo erzwingbar – deshalb `[VORLÄUFIG]`. Validierung erfordert Lackmus-Tests pro Modell.

<!-- ANCHOR:datenmodell -->
## 7. Datenmodell

Nicht anwendbar – keine Persistenz.

<!-- ANCHOR:verworfene-alternativen -->
## 8. Verworfene Alternativen

- **Separates Selbstanwendungs-Dokument im Repo-Wurzelverzeichnis:** hätte zwei konkurrierende Orte für Projektkontext geschaffen und einen Sonderweg etabliert, den die Methodik selbst nicht kennt – siehe ADR-001 Option A.
- **Vorlagen in `docs/` belassen und dort konkrete Werte eintragen:** hätte die Vorlagen beschädigt und projektspezifische Werte an alle abgeleiteten Projekte ausgeliefert – siehe ADR-001 Option C.

<!-- ANCHOR:reifegrad-uebersicht -->
## 9. Reifegrad-Übersicht (Stand vom 2026-09-27)

| Bestandteil | Reifegrad | Seit | Validiert durch / wartet auf |
|---|---|---|---|
| Regelwerk (`CLAUDE.md`, `AGENTS.md`) | BELASTBAR | 2026-08-13 | Patch-Wellen #7–#22, Pilotprojekt Klasse G; Abschnitte 6/7/12 zusätzlich durch A1-Einsatz im Piloten seit 2026-06-25 ohne Nachbesserung (ADR-006) |
| Vorlagen (`templates/`) | BELASTBAR | 2026-08-13 | vollständig und konsistent nach PR #22 |
| Selbstanwendung (`docs/`) | VORLÄUFIG | 2026-08-13 | wartet auf Bewährung über mehrere Sessions |
| NFR Lesbarkeit unter Kontext-Budget | BELASTBAR | 2026-08-13 | Größen-Budget und Archivierungs-Trigger aktiv; seit ADR-018 auch Fahrplan-Archiv bei Klasse K |
| NFR Adressierbarkeit einzelner Abschnitte | BELASTBAR | 2026-08-13 | ANCHOR-Workflow in PR #22 durchgespielt |
| NFR Portierbarkeit auf fremde Agents | VORLÄUFIG | 2026-08-13 | wartet auf Lackmus-Tests pro Modell |
| Hilfsskript `kontingent-warnung.py` (`templates/werkzeuge/claude-code/`) | VORLÄUFIG | 2026-09-24 | erzwungener Fehlerfall lokal 8 von 8 (S-22); Linter und 8 Tests seit 2026-09-27 (ADR-017); wartet auf Probelauf des Zustellwegs in einer lokalen Terminal-Session |

Der Reifegrad `BELASTBAR` des Regelwerks ist seit Bündel #34 nicht mehr gedeckt (Prüfbericht 2026-09-24, Befund W6). Die Neubewertung ist Teil von S-26 (Umbau U4).

<!-- ANCHOR:tooling-inventar -->
## 10. Tooling-Inventar

`scripts/` existiert nicht. Ein Hilfsskript wird als Produkt ausgeliefert, nicht im Repo selbst betrieben:

| Skript | Zweck | Schwelle (`CLAUDE.md` Abschnitt 15) | Reifegrad |
|---|---|---|---|
| `templates/werkzeuge/claude-code/kontingent-warnung.py` | Warnung im Gespräch, wenn das 7-Tage-Kontingent eine Schwelle erreicht (ADR-014, S-22) | überschritten: zwei Betriebsarten („statuszeile", „hook") = mehr als ein Subkommando. Pflichten A–F im Kopf erfüllt; G über `ruff` und Tests im Pre-Commit-Hook (ADR-017); H mit diesem Eintrag | VORLÄUFIG |

Einzige Werkzeug-Konfiguration ist `.prettierignore`, die sämtliche Markdown-Dateien von der Formatierung ausnimmt. Sie richtet sich an abgeleitete Projekte, deren Pre-Commit-Hook Prettier auch über Markdown laufen ließe, und ist kein Bestandteil eines automatisierten Pfads dieses Repos.
