# Projektstart-Verfahren

<!-- Vollständige Anleitung für die ALLERERSTE Session eines neuen Projekts.
     Wird NICHT pro Session geladen — nur einmal pro Projekt zu Beginn.
     Im regulären Betrieb gilt das Regelwerk in CLAUDE.md ab Abschnitt 2.
     Diese Datei bleibt im Repo als Referenz und für spätere Reklassifikation.

     Bezug: ausgelagert aus CLAUDE.md §1A und §1B als Teil der
     Selbstanwendung des §2-Größen-Budgets (Issue #6 Anmerkung a). -->

## 1. Vision-zu-Vorlagen-Überführung

Bei der **allerersten Session** eines neuen Projekts gilt ein eigenes Verfahren mit zwei klar getrennten Modi. Beide laufen auf der **Entscheidungs-Klasse** (`CLAUDE.md` Abschnitt 0, Auslöser 6) und bevorzugt **im normalen Chat**, **nicht im Coding-Werkzeug**. Weil `CLAUDE.md` im Chat nicht geladen ist, greift die Eskalationsregel dort nicht: Der Mensch stellt die Entscheidungs-Klasse beim Start selbst ein – das stärkste regulär eingesetzte Modell, nicht ein Modell der Ausnahme-Klasse. Die Wahl der Umgebung ist Empfehlung, kein hartes Verbot – aber begründet: Modus 1 ist reiner Dialog ohne Datei- oder Code-Arbeit (siehe 1.1), und eine Code-Umgebung verführt zu vorzeitiger Strukturierung, die Modus 1 ausdrücklich verbietet (Stack, Architektur, Module, Roadmap). Wer die Modi doch im Coding-Werkzeug führt, muss diese Verbote selbst diszipliniert einhalten. Der reguläre Betrieb (`CLAUDE.md` ab Abschnitt 2) beginnt erst nach Abschluss von Modus 2.

### 1.1 Modus 1: Konzeptphase (Vision-Erstellung)

**Zweck:** Aus einer rohen Idee eine vollständige `docs/vision.md` erarbeiten. Reiner Dialog. Keine Implementierungsstruktur, kein Stack-Festlegen, kein Architektur-Entwurf.

**Verhalten der KI in Modus 1:**

- **Ausdifferenzieren statt strukturieren.** Die KI nimmt die Idee auf, stellt Rückfragen, deckt Lücken und Mehrdeutigkeiten auf, schlägt Alternativen vor, hinterfragt Annahmen.
- **Keine vorzeitige Festlegung.** Auch wenn die Versuchung groß ist: keine Vorschläge zu Stack, Frameworks, Modulgliederung, Tools, Deployment. Diese Themen werden bewusst auf Modus 2 verschoben.
- **Aktiv lückenscannen entlang der `docs/vision.md`-Struktur:**
  1. Kernidee – ist sie in 1–3 Sätzen ausdrückbar?
  2. Problem und Anlass – wer hat das Problem konkret, wie wird es heute gelöst, warum reicht das nicht?
  3. Zielbild – kann ein konkretes Nutzungsszenario beschrieben werden?
  4. Erfolgskriterien – woran ist Erfolg messbar?
  5. **Bewusste Abgrenzung** – was soll das System ausdrücklich NICHT tun?
  6. Harte Randbedingungen – was steht von Anfang an fest?
  7. Weiche Präferenzen – was wäre angenehm, ist aber verhandelbar?
  8. Inspirationen – was gibt es bereits, woran orientiert man sich (oder bewusst nicht)?
  9. Risiken – wo ist die Vision selbst unsicher?
- **Eine Frage nach der anderen.** Keine Fragebatterien. Eine Lücke nach der anderen schließen, in der oben angegebenen Reihenfolge (harte Bedingungen vor weichen Präferenzen).
- **Output am Ende:** ausschließlich `docs/vision.md`. Keine anderen Dokumente. Auch dann nicht, wenn die Vision die Antwort eigentlich schon enthält.
- **Ausgeschlossenes festhalten.** Anwendungsfälle, die im Dialog besprochen und bewusst weggelassen wurden, stehen mit Begründung in `docs/vision.md` Abschnitt 5. Der Dialog selbst läuft oft außerhalb des Repos und geht verloren – ohne diesen Eintrag ist später nicht mehr erkennbar, ob etwas vergessen oder absichtlich ausgelassen wurde.
- **Pause-Zustand nach abgeschlossener Vision.** Wenn alle Abschnitte der `docs/vision.md` plausibel befüllt sind: KI kennzeichnet das, schlägt aber den Übergang **nicht** proaktiv vor. Sie wartet auf eine explizite Triggerphrase (siehe unten).

**Was die KI in Modus 1 nicht tut:**

- Keine Architektur-Vorschläge, auch nicht skizzenhaft.
- Keine Stack-Empfehlungen, auch nicht „Python wäre naheliegend".
- Keine Modul-Aufteilung, auch nicht informell.
- Keine Roadmap-Vorschläge oder Phasenstrukturen.
- Keine Vorlagen-Befüllung – auch nicht als Gefälligkeit oder „weil es schneller geht".

### 1.2 Übergang zu Modus 2: strikte Triggerphrase

Modus 2 wird **ausschließlich** durch eine der folgenden expliziten Weisungen des Menschen ausgelöst:

- „Vorlagen vorbereiten"
- „Initialisierung starten"
- „Modus 2 starten"
- „Vorlagen-Set initialisieren"

**Sinngemäße Formulierungen reichen nicht.** Wenn der Mensch andeutet, dass die Vision fertig sei, ohne eine dieser Phrasen zu verwenden, bleibt die KI in Modus 1. Sie darf höchstens **einmal** beiläufig erwähnen, dass die Vision aus ihrer Sicht abgeschlossen wirkt, und dass Modus 2 mit einer der Triggerphrasen gestartet werden kann. Keine wiederholten Erinnerungen, kein Drängen.

Diese Strenge ist Absicht: Sie schützt vor unbemerktem Übergang, der in der Praxis dazu führt, dass Architekturentscheidungen getroffen werden, bevor die Vision wirklich gereift ist.

### 1.3 Modus 2: Vorlagen-Initialisierung

**Voraussetzung:** Vision liegt vollständig in `docs/vision.md` vor und Mensch hat explizite Triggerphrase gegeben.

**Zweck:** Aus der fertigen Vision die übrigen Pflicht-Dokumente erstellen.

**Vorbereitung (vor Schritt 1):** Die Pflicht-Dokument-Vorlagen liegen unter [`templates/docs/`](docs/). Sie werden **nach `docs/` kopiert**, bevor sie befüllt werden – die Vorlagen selbst bleiben unverändert im Repo, damit sie für spätere Reklassifikation und als Referenz verfügbar bleiben. Welche Dateien kopiert werden, richtet sich nach der Klassifikations-Hypothese aus Schritt 1: `onboarding-runbook.md` und `requirements.md` erst ab Klasse M, `readme-vorlage.md` wandert nicht nach `docs/`, sondern wird in Schritt 9 nach `README.md` im Repo-Root überführt.

**Befüllungsreihenfolge (verbindlich):**

1. **Projektgrößen-Klassifikation** nach Abschnitt 2 durchführen. Stufe-1-Hypothese auf Basis der Vision formulieren, Klassifikations-ADR (ADR-001) als Entwurf vorbereiten. Die endgültige Klasse wird in Schritt 4 (Architektur) bestätigt oder korrigiert.
   **1a. Anforderungsklärung („Modus 1.5", Klasse G und V; verkürzt für Klasse M; entfällt für Klasse K).** Bevor Stack oder Architektur festgelegt werden, wird geklärt, was genau gebraucht wird. Ergebnis ist `docs/requirements.md` (Vorlage: `templates/docs/requirements.md`).
   - **Klasse G und V – eigener Dialog wie in Modus 1**, eine Frage nach der anderen: Beteiligte (Rollen, Interessen, Betroffenheit), Anwendungsfälle vollständig einschließlich der bewusst ausgeschlossenen, Kernprozesse heute und mit dem System, funktionale Anforderungen mit ID, Priorität (Muss/Soll/Kann) und prüfbarer Akzeptanz. Kein Stack, keine Architektur, keine Module – dieselben Verbote wie in Modus 1. Abschluss: Der Mensch bestätigt die Liste der Anwendungsfälle und der Muss-Anforderungen ausdrücklich; erst dann weiter mit Schritt 2. Bringt die Klärung einen Widerspruch zur Vision, gilt die Regel „Keine Erweiterung der Vision" unten.
   - **Klasse M – ohne eigenen Dialog:** Die KI leitet Anwendungsfälle und Anforderungen aus der Vision ab und legt die Liste zur Bestätigung vor.
   - **Klasse K:** kein eigenes Dokument; Vermerk „nicht anwendbar, Begründung: …" in `docs/project-context.md` Abschnitt 6.
   - Bestätigt Schritt 4 (Architektur) eine andere Klasse als die Hypothese, wird 1a in der Form der bestätigten Klasse nachgeholt oder zurückgebaut.

2. **`docs/project-context.md`** auf Basis der Vision und der Klassifikations-Hypothese vorbefüllen. Wo die Vision keine Festlegung hat (z. B. Stack offen): zwei bis drei Optionen mit Konsequenzen formulieren und vorlegen, **nicht selbst entscheiden**. Diese Vorlage-Entscheidungen sind freigabepflichtig (`CLAUDE.md` Abschnitt 4). Strukturform der Datei nach Klasse (siehe Initialisierungshinweis in der Datei). Dazu die Vision-Frage **„Was darf das Projekt monatlich kosten?"** – Antwort als Kostenrahmen in Abschnitt 8.

   **2a. Versions-Verifikation (Pflicht-Stopp vor ADR-002).** Regeln: `CLAUDE.md` Abschnitt 15, „Versionswahl". Die KI schlägt für jede Komponente (Sprachen, Frameworks, Datenbanken, Laufzeitumgebungen, Major-Bibliotheken) die **ausgereifte Linie** vor und **belegt sie selbst** gegen offizielle Quellen – Trainingswissen gilt nur als Ausgangsvermutung. Vorher legt sie mit dem Menschen die Mindestreife und die geplante Projektdauer fest (`docs/project-context.md` Abschnitt 3). Dann legt sie **eine** Tabelle zur Bestätigung vor:

   ```text
   VERSIONS-VERIFIKATION ERFORDERLICH
   Geprüft am [YYYY-MM-DD]. Mindestreife: [Wert]. Projektdauer: [Wert].
   | Komponente | Vorschlag | Neueste Linie | Lebensende | Tragende Abh. unterstützen? | Quellen |
   | [A]        | [Linie]   | [Linie]       | [Datum]    | [ja, Beleg]                 | [Links] |
   Begründung, wo nicht die neueste Linie: [je Zeile ein Satz]
   Antwortformat pro Zeile: „bestätigt" ODER „ersetzt durch <Version>".
   ```

   Nach Bestätigung wird jede Version in `docs/project-context.md` Abschnitt 3 mit `Verifiziert: YYYY-MM-DD` und Quelle eingetragen, ihr Lebensende ins Ablaufdaten-Register (Abschnitt 8) übernommen und in ADR-002 gesperrt. Kann die KI die Quellen nicht erreichen, bleibt die Zeile als „ungeprüft" markiert und Modus 2 steht (Informationslücke, `CLAUDE.md` Abschnitt 8) – keine stillen Annahmen, keine Rate-Versionen.

3. **Härtungs-Schritt:** Konzept aus Entwicklersicht prüfen. Inkonsistenz-Suche zwischen Vision-Aussagen, Constraints und vorgeschlagenen Optionen. Gefundene Probleme entweder in Modus 2 auflösen oder als ersten Eintrag in `docs/blockers.md` mit Status „Aktiv" anlegen.
4. **`docs/architecture.md`** mit dem Architektur-Grobschnitt befüllen, soweit aus Vision und Stack-Entscheidung ableitbar. Schnittstellenverträge so weit, wie sie aus dem Konzept ableitbar sind. Lücken explizit als „TBD nach Schritt X.Y" markieren mit Fahrplan-Referenz. **Stufe-2-Bestätigung der Klassifikation:** Architektur-Indikatoren mit Hypothese aus Schritt 1 abgleichen. Bei Abweichung: ADR-001 anpassen, betroffene Strukturentscheidungen korrigieren.
   **4a. Sicherheitsgrundriss (Pflicht für jedes Projekt, das öffentlich erreichbar werden soll).** Vorbereitung auf das Gate vor dem ersten öffentlichen Deployment (`CLAUDE.md` Abschnitt 12). In `docs/architecture.md` Abschnitt 6:
   - **Bedrohungsmodell** für das Gesamtsystem als Entwurf `[VORLÄUFIG]` – wer greift was an, was wird bewusst nicht abgedeckt.
   - **Sicherheitsniveau** als Option vorlegen (z. B. eine Stufe des OWASP ASVS, mit Begründung aus der Vision); nach Freigabe ADR in Schritt 5.
   - **Rubriken Host, Netz, Secrets im Betrieb, Backups und Wiederherstellung** anlegen, jede mit Reifegrad (in Modus 2 meist `[OFFEN]` oder `[VORLÄUFIG]`).
   - **Schutzbedarf der Daten** je Datenkategorie festlegen (z. B. normal / hoch / sehr hoch nach dem Standard-Datenschutzmodell) mit der Frage an den Menschen: „Wie schlimm wäre es für deine Nutzer, wenn diese Daten nach außen gelangen?" Er ist die Obergrenze für alle späteren Datenschutz-Maßnahmen (`CLAUDE.md` Abschnitt 6, „Schutzbedarf ist Obergrenze"). Dieser Punkt gilt auch ohne öffentliches Deployment, sobald personenbezogene Daten verarbeitet werden.

   Dazu zwei Fragen an den Menschen in Alltagssprache; die Antworten landen in `docs/project-context.md` Abschnitt 8:
   - „Wer kann eingreifen, wenn du nicht erreichbar bist – und darf das Projekt laufen, wenn es niemanden gibt?"
   - „Über welches Konto arbeitet die KI, und wann setzt sich ihr Nutzungskontingent zurück?"

   Sieht die Vision kein öffentliches Deployment vor (z. B. ein rein lokales Werkzeug), wird das mit einem Satz in Abschnitt 6 vermerkt und der Schritt entfällt.

5. **`docs/decisions.md`** befüllen:
   - **ADR-001:** Projektgrößen-Klassifikation und Anpassung des Vorlagen-Sets.
   - **ADR-002:** Stack-Entscheidung mit Optionen aus Schritt 2 und Begründung.
   - **ADR-003:** Architektur-Pattern-Entscheidung.
   - **ADR zum Sicherheitsniveau** aus Schritt 4a (entfällt, wenn kein öffentliches Deployment vorgesehen ist).
   - **ADR zum Schutzbedarf** aus Schritt 4a (entfällt, wenn keine personenbezogenen Daten verarbeitet werden).
   - Weitere ADRs für jede in der Konzeptphase getroffene Grundsatzentscheidung.
6. **`docs/fahrplan.md`** befüllen: Phasen aus dem Konzept ableiten, erste Phase mit konkreten Schritten füllen (jeder Schritt im vollen Format), spätere Phasen können gröber sein und werden im Verlauf verfeinert. Strukturform nach Klasse. Ab Klasse M nennt jeder Schritt, der eine Anforderung umsetzt, deren ID; jede Muss-Anforderung landet in einem Schritt. Ist ein öffentliches Deployment vorgesehen, bekommt es **einen eigenen Gate-Schritt davor** (`CLAUDE.md` Abschnitt 12, „Gate vor dem ersten öffentlichen Deployment") – auch wenn die Phase dafür erst grob geplant ist.
7. **`docs/blockers.md`** auf Startzustand setzen (entweder leer mit „Keine aktiven Blocker" oder mit den im Härtungs-Schritt identifizierten Blockern).
8. **`docs/logbuch.md`** auf Startzustand setzen: Beispiel-Einträge entfernen, Pflege-Hinweise behalten. Erster realer Eintrag entsteht beim Start der ersten regulären Session nach Modus-2-Abschluss.
9. **`README.md`** befüllen aus den jetzt vorliegenden Dokumenten. Vorlage liegt unter [`templates/docs/readme-vorlage.md`](docs/readme-vorlage.md) – Inhalt nach `README.md` im Repo-Root kopieren und befüllen. Badge-Auswahl und Strukturwahl nach Klasse (siehe Vorlage). Initialisierungshinweis am Dateiende entfernen.
10. **CI-Workflow- und Pre-Commit-Skelett** aus `templates/` in das Projekt kopieren und anpassen. Quelle pro Klasse und Sprache:
    - **Klasse K:** `templates/github-workflows/ci-minimal.yml` → `.github/workflows/ci.yml`.
    - **Klasse M/G:** `templates/github-workflows/ci-<sprache>.yml` → `.github/workflows/ci.yml` (bei mehreren Sprachen mehrere Workflow-Dateien oder Job-Komposition); bei G zusätzlich `security.yml` und `release.yml` ableiten.
    - **Klasse V:** `ci-<sprache>.yml` pro Service nach `.github/workflows/ci-<service>.yml` mit Path-Filtern, plus eigene `integration.yml`.
    - **Pre-Commit:** `templates/pre-commit/<sprache>.yaml` → `.pre-commit-config.yaml` im Repo-Root.
    Alle `# TBD:`-Platzhalter (Sprachversion, Pfade, Coverage-Schwellen, Tool-Versionen) durch konkrete Werte aus `docs/project-context.md` Abschnitt 7 ersetzen. GitHub Actions ist **Diagnoseschicht zusätzlich zu lokalen Hooks**, kein Ersatz.
11. **`docs/vision.md` Überführungs-Status** am Dateiende abhaken.
12. **Initialisierungs-Commit** mit allen Dokumenten und der Workflow-/Hook-Konfiguration. Commit-Message: `init: Projekt initialisiert aus vision.md, Klasse [K/M/G/V], ADR-001 bis ADR-NNN`.

**Iterationsregel innerhalb von Modus 2:** Wenn ein späterer Schritt eine frühere Annahme kippt (z. B. die Architektur in Schritt 4 zeigt, dass eine Stack-Option aus Schritt 2 nicht trägt): zurückspringen, betroffene Abschnitte überarbeiten. Keine stillen Korrekturen.

**Was Modus 2 nicht tut:**

- Keine Erweiterung der Vision. `docs/vision.md` wird nicht mehr inhaltlich verändert. Wenn neue Erkenntnisse die Vision sprengen würden: Modus 2 abbrechen, zurück zu Modus 1, Mensch entscheiden lassen.
- Keine Anwendungslogik. Modus 2 produziert Dokumente sowie das CI-/Hook-Skelett aus Schritt 10 – aber keinen Anwendungscode, keine Tests, keine Konfiguration über das Qualitätsgate-Setup hinaus.

### 1.4 Übergang zu regulärem Betrieb

Nach dem Initialisierungs-Commit ist die Initialisierung abgeschlossen. Ab diesem Punkt:

- `docs/vision.md` wird nicht mehr verändert. Substantielle Vision-Pivots erzeugen einen neuen ADR mit Verweis auf den ursprünglichen Vision-Abschnitt.
- Reguläres Regelwerk (`CLAUDE.md` Abschnitt 2 ff.) gilt.
- Weitere Sessions finden typischerweise im Coding-Werkzeug statt, das `CLAUDE.md` lädt.

## 2. Projektgrößen-Klassifikation

Vor der Befüllung der Vorlagen in Modus 2 wird das Projekt in eine von vier Größenklassen eingestuft. Die Klasse bestimmt die **Default-Struktur** des Vorlagen-Sets. Abweichungen sind möglich, aber begründungspflichtig per ADR.

### 2.1 Zweistufige Ableitung

Die Klassifikation erfolgt in zwei Stufen, weil Vision allein häufig zu vorsichtig (oder zu ehrgeizig) schätzt und Architektur die Realität korrigiert:

**Stufe 1: Vision-basierte Schätzung** (zu Beginn von Modus 2, vor `docs/project-context.md`-Befüllung)

Indikatoren aus `docs/vision.md`:

- Anzahl der im Zielbild und in den Beispielszenarien ableitbaren Module
- Dichte und Schärfe der Constraints (z. B. DSGVO + Self-Hosting + Compliance = hohe Dichte)
- Anzahl externer Stakeholder oder Nutzergruppen
- Erwartete Last und Skalierungsanforderung
- Anzahl externer Abhängigkeiten und Integrationen

Diese Stufe erzeugt eine vorläufige Klassen-Hypothese.

**Stufe 2: Architektur-basierte Bestätigung** (nach `docs/project-context.md`, während der Befüllung von `docs/architecture.md` in Modus 2 Schritt 4)

Indikatoren aus dem Architektur-Grobschnitt:

- Anzahl Services oder Deployment-Einheiten (1 / 2–3 / 4–7 / >7)
- Anzahl modulübergreifender Schnittstellen
- Synchron-Monolith vs. asynchron/verteilt
- Stateless vs. Stateful-Komponenten
- Persistenzschichten (eine Datenbank / mehrere / heterogen)

Wenn Stufe 2 die Hypothese bestätigt: weiter wie geplant. Wenn Stufe 2 abweicht: zurück zu Stufe 1, neue Klasse als ADR-001 dokumentieren mit Begründung.

### 2.2 Die vier Klassen

#### Klasse K – Klein

**Indikatoren:** 1 Modul, 0–1 externe Abhängigkeiten, einzelne Sprache, kein Persistenzlayer oder triviale lokale Datei, kein verteilter Aspekt, einzelner Nutzer oder kleine homogene Nutzergruppe.

**Typische Vertreter:** CLI-Tool, einzelnes Skript, Library mit klar abgegrenztem Zweck, Automatisierung.

**Default-Struktur:**

- `docs/vision.md` – wie immer, aber kann kürzer ausfallen
- `docs/project-context.md` – nicht-relevante Abschnitte entfernen (z. B. Skalierung, Observability)
- `docs/architecture.md` – Modul-Karte entfällt, Schnittstellenverträge nur wenn nicht-trivial, kein Datenfluss-Abschnitt
- `docs/fahrplan.md` – flache Schrittliste ohne Phasenstruktur, Phasentyp pro Schritt
- `docs/decisions.md` – als Einzeldatei, Teil C (Regeln) bleibt leer bis Bedarf entsteht
- `docs/blockers.md` – wie immer
- `docs/requirements.md` – entfällt; Vermerk „nicht anwendbar, Begründung: …" in `docs/project-context.md`. Anforderungen stehen als Prosa in den Constraints und in den Akzeptanzkriterien der Schritte. Beteiligte: Vision Abschnitt 2 genügt. Geschäftsentscheidungen (`decisions.md` Teil D): nicht anwendbar.
- **CI-Workflow** (`.github/workflows/ci.yml`) – minimal: ein Job mit Lint und Test auf Push und PR. Pre-Commit-Hooks tragen lokal die Hauptlast; Actions ist hier reine Diagnoseschicht.

#### Klasse M – Mittel

**Indikatoren:** 2–5 Module, 2–5 externe Abhängigkeiten, ein bis zwei Sprachen, eine Persistenzschicht, monolithisches Deployment oder eng gekoppeltes Frontend+Backend, eine Nutzergruppe mit Rollen.

**Typische Vertreter:** typische Web-Anwendung (Backend+Frontend mit Datenbank), interne Tools mit UI, einzelner Microservice mit eigener Persistenz, Daten-Pipeline mit ein bis zwei Stufen.

**Default-Struktur:**

- Alle Pflicht-Dokumente in voller Form, dazu `docs/requirements.md` (aus der Vision abgeleitet, Schritt 1a verkürzt): Anwendungsfälle, funktionale Anforderungen mit ID und Muss/Soll/Kann; Beteiligte und Kernprozesse optional. Vertiefung auf Anforderung, nicht Mindest-Lektüre.
- `docs/decisions.md` Teil D (Geschäftsentscheidungen) optional – nur bei echten Geschäftsentscheidungen
- `docs/architecture.md` – ein Dokument, alle Abschnitte
- `docs/decisions.md` – als Einzeldatei
- `docs/fahrplan.md` – Phasenstruktur mit 3–5 Phasen
- Ein gemeinsamer `docs/`-Ordner
- **CI-Workflow** (`.github/workflows/ci.yml`) – voller Satz an Gates pro Sprache: Lint, Format-Check, Type-Check, Security-Scan, Dependency-Audit, Tests mit Coverage. Matrix-Build, falls mehrere Sprachversionen unterstützt werden. Branch-Protection auf Hauptbranch verlangt grüne Pipeline.

#### Klasse G – Groß

**Indikatoren:** 6+ Module, 5+ externe Abhängigkeiten, oft mehrere Sprachen, mehrere Persistenzschichten oder gemischte Speichertechnologien, ein bis zwei Deployment-Einheiten aber nicht stark verteilt, mehrere Nutzergruppen mit unterschiedlichen Rollen, NFR-Komplexität (Performance, Skalierung, Compliance).

**Typische Vertreter:** komplexe Web-Anwendung mit Auth-System, Multi-Tenant-Plattform, Daten-Pipeline mit mehreren Stufen und Speichern, ML-System mit Trainings- und Inference-Pfad ohne starke Verteilung.

**Default-Struktur:**

- Alle Pflicht-Dokumente, plus optional Modul-spezifische Architektur-Dokumente
- **Modus 1.5** (Schritt 1a) mit eigenem Dialog; `docs/requirements.md` vollständig: Beteiligte, Anwendungsfälle, Kernprozesse Ist/Soll, Anforderungen mit Test je Muss-Anforderung. Abschnitt „Übersicht" gehört zur Mindest-Lektüre.
- `docs/decisions.md` Teil D (Geschäftsentscheidungen) aktiv
- `docs/architecture.md` – als Index mit Verweisen auf `architecture-<modul>.md`-Unterdokumente, sobald die zentrale Datei unübersichtlich wird (>500 Zeilen oder >5 Module mit eigenen Schnittstellen)
- `docs/decisions.md` – als Index mit Teil B (Übersicht) und Teil C (Regeln) zentral; einzelne ADRs in `decisions/ADR-NNN.md`-Dateien sobald die ADR-Anzahl zweistellig wird
- `docs/fahrplan.md` – Phasenstruktur mit 5–7 Phasen, ggf. Teil-Fahrpläne pro Modul unter `docs/fahrplan-<modul>.md`
- Reaktiv-ADR-Schwellenwert in `docs/project-context.md` strenger fassen (z. B. 20 % statt 30 %), weil bei dieser Größe reaktive Architekturentscheidungen schneller eskalieren
- **CI-Workflow** – voller Gate-Satz wie Klasse M, zusätzlich aufgeteilt in mehrere Workflow-Dateien, sobald die Pipeline unübersichtlich wird (z. B. `ci.yml`, `security.yml`, `release.yml`). Pflicht-Job für modulübergreifende Integrationstests. Caching von Abhängigkeiten und Build-Artefakten ist Default.

#### Klasse V – Verteilt-Groß

**Indikatoren:** mehrere unabhängig deploybare Services (3+), asynchrone Kommunikation zwischen Services, mehrere heterogene Persistenzschichten, GPU- oder andere Spezial-Workloads, hohe Compliance-Anforderungen (DSGVO, regulatorisch), externe Drittsysteme mit eigenem Lifecycle, Multi-Repo möglich.

**Typische Vertreter:** ML-Plattform mit getrenntem Training/Inference/Serving, Multi-Service-Architektur mit Event-Bus, Compliance-pflichtige verteilte Systeme.

**Default-Struktur:**

- `docs/vision.md` – wie immer, aber Stakeholder- und Compliance-Abschnitte besonders sorgfältig
- **Modus 1.5** wie Klasse G; `docs/requirements.md` als Index mit `requirements-<service>.md` und `requirements-integration.md`; Beteiligte mit Zuordnung zu Compliance-Pflichten
- `docs/decisions.md` Teil D (Geschäftsentscheidungen) Pflicht
- `docs/project-context.md` – als Index mit `project-context-<service>.md` für service-spezifische Stack-Details, oder als ein Dokument mit klar getrennten Service-Abschnitten
- `docs/architecture.md` – **immer** als Index, Pflicht-Splitting in `architecture-<service>.md`-Dateien, plus separates `architecture-integration.md` für service-übergreifende Verträge und Event-Definitionen
- `docs/decisions.md` – als Index mit Teil B und Teil C zentral, einzelne ADRs in `decisions/ADR-NNN.md`-Dateien von Anfang an
- `docs/fahrplan.md` – als Master-Index mit Teil-Fahrplänen pro Service unter `docs/fahrplan-<service>.md`
- Reaktiv-ADR-Schwellenwert besonders streng (z. B. 15 %), zusätzlich Service-spezifische Reaktiv-Quoten getrennt überwachen
- **Pflicht-ADR-Themen für Klasse V:** Service-Grenzen-Definition, Versionierungsstrategie zwischen Services, Failure-Mode-Handling, Datenkonsistenz-Strategie (eventual / strict), Observability-Standard
- **CI-Workflow** – pro Service eigener Workflow unter `.github/workflows/ci-<service>.yml` mit Path-Filtern, plus separater `integration.yml` für service-übergreifende End-to-End-Tests und Vertrags-Prüfungen (z. B. Pact, Schema-Kompatibilität). `release.yml` und `security.yml` zentral. Reusable Workflows zur Vermeidung von Duplikation sind Pflicht.

### 2.3 Klassifikations-ADR

Die Klassifikation wird in **ADR-001 als `[STRATEGISCH] [METHODIK]`** dokumentiert mit:

- gewählte Klasse mit Begründung aus Vision- und Architektur-Indikatoren
- daraus folgende Strukturentscheidungen (welche Dateien einzeln, welche als Index, welche Teil-Dokumente)
- abweichende Entscheidungen vom Default mit Begründung

### 2.4 Reklassifikation

Wenn ein Projekt im Verlauf wächst (typischerweise K→M oder M→G):

- **Trigger:** mindestens zwei Indikatoren der höheren Klasse erfüllt, oder die Dokumentstruktur wird spürbar unübersichtlich.
- **Vorgehen:** ADR mit `[STRATEGISCH] [METHODIK]` und Tag `Reklassifikation`, Plan zur Migration der Dokumentstruktur, Migration als eigene STABILISIERUNG-Phase im Fahrplan.
- **Keine Reklassifikation rückwärts** ohne starken Grund – wenn ein Projekt vereinfacht wurde, ist das eher Anlass für Refactoring als für Vorlagen-Schrumpfung.
