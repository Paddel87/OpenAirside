# Project Context

<!-- Projektspezifischer Kontext. Wird zu Sessionbeginn als erste Datei gelesen.
     Dient als Entscheidungsgrundlage für alle autonomen Schritte der KI.
     Jede Angabe muss so konkret sein, dass daraus maschinell eindeutig Regeln ableitbar sind. -->

<!-- ANCHOR:kerndaten -->
## 1. Kerndaten

- **Projektname:** [ausfüllen]
- **Kurzbeschreibung:** [1–2 Sätze: was das System tut und für wen]
- **Status:** [Konzeption | Aufbau | aktive Entwicklung | Wartung | deprecated]
- **Version (SemVer):** [z. B. v0.1.0]
- **Dokumentationssprache:** [z. B. Deutsch]
- **Codesprache (Kommentare, Variablennamen):** [z. B. Englisch]
- **Projekttyp:** [CLI | Web-Backend | Web-Frontend | Full-Stack | Daten-Pipeline | ML-System | Library | Mixed]

<!-- ANCHOR:zielgruppe-und-nutzungskontext -->
## 2. Zielgruppe und Nutzungskontext

- **Primäre Nutzer:** [wer verwendet das System, technisches Level]
- **Sekundäre Nutzer / Betreiber:** [wer installiert, konfiguriert, wartet]
- **Erwartete Last:** [z. B. „10 concurrent users", „1M Requests/Tag", „Batch-Jobs wöchentlich"]
- **Nutzungsumgebung:** [z. B. „Browser Desktop + Mobile", „CLI auf Linux/macOS", „Kubernetes-Cluster"]

<!-- ANCHOR:technischer-stack -->
## 3. Technischer Stack

### Fixiert

Pflicht: jede Version trägt einen Vermerk `Verifiziert: YYYY-MM-DD` mit Quelle (Datum, an dem die KI die Version gegen offizielle Quellen belegt und der Mensch die Tabelle bestätigt hat). Auswahl nach der Regel „ausgereifte Linie" (`CLAUDE.md` Abschnitt 15, „Versionswahl"); ausgelöst in Modus 2 Schritt 2a (`templates/projektstart.md` Abschnitt 1.3). Lebensende bzw. Nachprüf-Datum jeder Version steht im Ablaufdaten-Register (Abschnitt 8). Major-Updates erfordern eine erneute Verifikation und einen ADR.

- **Mindestreife neuer Linien:** [z. B. „mindestens 3 Monate allgemein verfügbar oder mindestens zwei Fehlerkorrektur-Versionen"]
- **Geplante Projektdauer:** [z. B. „mindestens 2 Jahre Betrieb" – bestimmt, wie lange das Unterstützungsfenster reichen muss]

- **Sprachen und Versionen:** [z. B. Python 3.12 — Verifiziert: 2026-05-02; TypeScript 5.3 — Verifiziert: 2026-05-02]
- **Frameworks:** [z. B. FastAPI 0.115 — Verifiziert: 2026-05-02; Next.js 15 — Verifiziert: 2026-05-02]
- **Datenbank:** [z. B. PostgreSQL 16 — Verifiziert: 2026-05-02]
- **Laufzeitumgebung:** [z. B. Docker Compose lokal, K8s in Prod — Verifiziert: 2026-05-02]
- **Package Manager:** [z. B. uv, pnpm — Verifiziert: 2026-05-02]

### Empfohlen (freigabefrei nutzbar)

[Bibliotheken, die bei Bedarf ohne separate Freigabe eingesetzt werden dürfen.
Beispiele: Standard-Test-Runner, Logging-Bibliothek, ORM, Linter.]

- [Bibliothek 1 – Einsatzzweck]
- [Bibliothek 2 – Einsatzzweck]

### Explizit nicht erlaubt

[Was bewusst ausgeschlossen ist, mit Begründung.
Verhindert, dass die KI naheliegende, aber unerwünschte Lösungen wählt.]

- [z. B. „Keine externen Cloud-Services (Self-Hosting-Prinzip)"]
- [z. B. „Keine GPL-lizenzierten Abhängigkeiten"]

### Unterstützte Entwickler-Plattformen

Diese Tabelle ist **explizit, nicht implizit**. Jede Plattform, die nicht hier steht, ist nicht unterstützt – auch wenn sie technisch funktionieren mag. Plattformen, die mit Einschränkungen unterstützt werden, tragen den Einschränkungs-Hinweis.

| Aspekt                        | Linux (Ubuntu 22.04+ / Debian 12+ / Fedora 40+) | macOS 14+ (Apple Silicon und Intel) | Windows 11 mit Git Bash | Windows 11 mit WSL2 |
|---|---|---|---|---|
| **Backend-Entwicklung**       | [✓ / ✗ / ✓ mit Einschränkung] | [...] | [...] | [...] |
| **Frontend-Entwicklung**      | [...] | [...] | [...] | [...] |
| **Hilfsskripte (`scripts/`)** | [...] | [...] | [...] | [...] |
| **Container-Workloads** (Docker Compose lokal) | [...] | [...] | [...] | [...] |
| **CI-Pipeline**               | [✓ (GitHub-Hosted-Runner)] | [— (kein macOS-Runner in CI)] | [...] | [...] |

**Pflicht-Voraussetzungen pro Plattform:** Siehe README „Voraussetzungen"-Block. Plattform-spezifische Zusatz-Voraussetzungen sind dort namentlich vermerkt (z. B. „Windows: Git Bash oder WSL2 für `scripts/`-Hilfsskripte; `jq` separat installieren").

**Pflege-Regel:** Diese Tabelle wird bei jedem Touch an `scripts/`, `docker-compose.yml`, `pyproject.toml`/`package.json` (Top-Level-Dependencies) oder bei jeder neuen plattform-bezogenen Eskalation (z. B. neuer Plattform-Blocker in `docs/blockers.md`) re-validiert. Verstöße sind im selben Commit zu korrigieren.

**Nicht-Unterstützung:** Wenn eine Plattform bewusst nicht unterstützt wird (z. B. Windows ohne Git Bash und ohne WSL2), wird die Spalte trotzdem gelistet, mit einem ✗ und kurzer Begründung. Stille Nicht-Unterstützung („wir testen halt nur Linux") ist unzulässig – sie wird explizit oder die Plattform wird zur Test-Matrix hinzugefügt.

**Initialisierungs-Hinweis:** Spalten in der Vorlage sind generisch; bei Modus-2-Befüllung werden nicht-relevante Spalten entfernt und projekt-spezifische Plattformen ergänzt (z. B. eingebettete Linux-Distribution, Cloud-Runner mit GPU).

<!-- ANCHOR:architektur-grobstruktur -->
## 4. Architektur-Grobstruktur

[2–5 Sätze. Details gehören in `architecture.md`.
Hier nur das, was für die Gesamtorientierung nötig ist.]

**Module (Kurzübersicht):**

- [Modul A] – [Kurzbeschreibung der Verantwortung]
- [Modul B] – [...]

**Kommunikationsmuster:** [z. B. „REST synchron intern, Events über Queue zwischen Backend und Worker"]

<!-- ANCHOR:externe-abhaengigkeiten -->
## 5. Externe Abhängigkeiten

### Services

| Service | Zweck | Authentifizierung | Ausfallverhalten |
|---|---|---|---|
| [Name] | [wofür] | [wie] | [Fallback bei Nichterreichbarkeit] |

### APIs

[Externe APIs mit Version, Rate Limits, Failure Modes.]

<!-- ANCHOR:constraints -->
## 6. Constraints (operationalisierbar)

**Regel: Jeder Constraint muss in eine prüfbare Regel übersetzt sein. Schwammige Angaben wie „sicher" oder „schnell" gehören hier nicht hin.**

### Datenschutz

- [z. B. „Keine personenbezogenen Daten in Logs" → Regel: Logger-Wrapper mit Redaction-Liste verwenden]
- [z. B. „DSGVO-Art. 17: Löschfunktion für Nutzerdaten" → Regel: API-Endpunkt `DELETE /users/{id}` kaskadiert auf verknüpfte Tabellen]

### Sicherheit

- [z. B. „Alle Endpoints erfordern Authentifizierung außer `/health` und `/login`"]
- [z. B. „Passwörter werden mit argon2id gehasht, minimum 12 Zeichen, kein Maximum"]

### Performance

- [z. B. „p95-Antwortzeit < 200 ms bei bis zu 100 concurrent users"]
- [z. B. „Datenbankabfragen dürfen keine `SELECT *` verwenden"]

### Plattform und Kompatibilität

- [z. B. „Lauffähig auf x86_64 und arm64"]
- [z. B. „Minimum Node 20 LTS"]

### Compliance und Lizenz

- **Projektlizenz:** [z. B. MIT, AGPLv3, proprietär]
- **Erlaubte Abhängigkeitslizenzen:** [z. B. MIT, BSD, Apache-2.0, MPL-2.0]
- **Ausgeschlossene Lizenzen:** [z. B. GPL außer explizit freigegeben]

### Methodik-Schwellenwerte

- **Reaktiv-ADR-Schwellenwert:** [z. B. „maximal 30 % `[REAKTIV]`-Anteil über die letzten 10 ADRs"] – bei Überschreitung wird in `decisions.md` Teil B (Reaktiv-Quote) ein Hinweis ausgelöst, und Claude legt einen Reflexions-Schritt im Fahrplan an, bevor weitere Umsetzungsschritte beginnen.
- **Wucherungs-Schwelle** (`CLAUDE.md` Abschnitt 8, Kriterium 9): [Default: Faktor 2 gegenüber dem ursprünglichen Schrittplan **und** mindestens 5 zusätzliche Schritte]
- **Vorläufig-zu-Belastbar-Verhältnis:** [optional, z. B. „spätestens nach jeder UMSETZUNG-Phase soll mindestens ein `[VORLÄUFIG]`-Bestandteil der berührten Module auf `[BELASTBAR]` befördert sein, sonst Reflexion"]
- **Modellklassen-Zuordnung** (Regelwerk: `CLAUDE.md` Abschnitt 0, „Modellklassen-Disziplin"). Die Auslöser stehen dort; hier wird nur benannt, welches Modell welche Klasse besetzt und ob der Probelauf bestanden ist. Bei neuen Modellen nachziehen (Eintrag im Ablaufdaten-Register, Abschnitt 8):
  - **Mechanik-Klasse:** [Modell] – Probelauf: [bestanden am YYYY-MM-DD / offen / gescheitert am YYYY-MM-DD]
  - **Routine-Klasse:** [Modell] – Probelauf: [bestanden am YYYY-MM-DD / offen / gescheitert am YYYY-MM-DD]
  - **Entscheidungs-Klasse:** [Modell] – stärkstes regulär eingesetztes Modell
  - **Ausnahme-Klasse:** [Modell oder „keine"] – nur auf Vorschlag mit Freigabe
  - **Bezugsmodell und knappe Ressource:** [z. B. „Abo, knapp ist das Wochenkontingent, Zurücksetzung So 10:00" oder „API, knapp ist das Monatsbudget von X"]
  - **Abgabe an Unteragenten:** [welche Werkzeug-Einstellung ein fest eingestelltes Modell für Unteragenten bewirkt, oder „Werkzeug kennt keine Unteragenten – Abgabe entfällt"]
  - **Meldet die Laufzeitumgebung das Modell / das Kontingent?** [z. B. „Modell ja, per Sitzungsabfrage; Kurzzeitlimit ja; Wochenlimit nein – Stand YYYY-MM-DD"]
  - **Preise je Klasse** (Eingabe / Ausgabe / Lesen aus dem Cache, je 1 Mio. Token, mit Stand und Quelle): [Grundlage für die Abgabe-Regel in `templates/wirtschaftlichkeit.md` – Abgabe nur an eine Klasse mit niedrigerem Cache-Lesepreis oder bei ausgabelastiger Arbeit]
  - **Grenze der Sessiongröße:** [Default 400.000 Token Kontext] – darüber gibt die KI beim Sessionende einen Hinweis, keinen Stopp (`CLAUDE.md` Abschnitt 0, „Sessiongröße")
  - **Kontingent-Warnung:** [z. B. „aktiv – Statuszeile und Hook nach `templates/werkzeuge/claude-code/`, Schwellen 80/95 %, Probelauf bestanden am YYYY-MM-DD" oder „inaktiv, Begründung: …"]
  - **Falls nur eine Klasse verfügbar ist:** „nur ein Modell verfügbar, Begründung: …". Die Eskalations-Auslöser bleiben dann gültig und erzeugen statt eines Modellwechsels einen ERKUNDUNG-Schritt im Fahrplan – die Regel verliert ihren Zweck also nicht, sie wechselt nur das Mittel.

<!-- ANCHOR:code-standards-und-qualitaetsziele -->
## 7. Code-Standards und Qualitätsziele

Pflichtkategorien sind in `CLAUDE.md` Abschnitt 15 definiert. Hier wird pro im Projekt verwendeter Sprache die konkrete Toolwahl festgelegt. Nicht anwendbare Kategorien sind mit Begründung zu vermerken, nicht wegzulassen.

### Tool-Festlegung pro Sprache

#### [Sprache, z. B. Python]

- **Linter:** [z. B. `ruff` mit Konfiguration `pyproject.toml`]
- **Formatter:** [z. B. `ruff format` oder `black`, Zeilenlänge: 100]
- **Type-Checker:** [z. B. `mypy --strict`]
- **Security-Scanner:** [z. B. `bandit`]
- **Dependency-Audit:** [z. B. `pip-audit`, `safety`]
- **Test-Runner:** [z. B. `pytest` mit `pytest-cov`]
- **Naming-Konvention:** [z. B. PEP 8, snake_case für Funktionen/Variablen, PascalCase für Klassen]

#### [Sprache, z. B. TypeScript]

- **Linter:** [z. B. `eslint` mit `@typescript-eslint`]
- **Formatter:** [z. B. `prettier`]
- **Type-Checker:** [z. B. `tsc --strict --noUncheckedIndexedAccess`]
- **Security-Scanner:** [z. B. `eslint-plugin-security`]
- **Dependency-Audit:** [z. B. `npm audit` oder `pnpm audit` mit Schwellenwert `high`]
- **Test-Runner:** [z. B. `vitest` mit `--coverage`]
- **Naming-Konvention:** [z. B. camelCase für Variablen/Funktionen, PascalCase für Typen/Klassen]

[Weitere Sprachen analog. Sprachen ohne etabliertes Tool in einer Kategorie:
„nicht anwendbar, Begründung: …"]

### Warnungs-Bestand

[Regeln: `CLAUDE.md` Abschnitt 15, „Warnungen und Abkündigungen". Default ist „Warnungen sind Fehler". Hier steht nur, was davon abweicht – mit Obergrenze, die nur sinken darf.]

| Quelle | Mittel des Werkzeugs | Obergrenze / benannte Ausnahme | Stand vom | Fahrplan-Schritt |
|---|---|---|---|---|
| [z. B. pytest] | [`filterwarnings` in `pyproject.toml`] | [z. B. `ignore:…:DeprecationWarning:bibliothek_x`] | [YYYY-MM-DD] | [Schritt-ID] |
| [z. B. ESLint] | [`--max-warnings`] | [z. B. 12] | [YYYY-MM-DD] | [Schritt-ID] |

**Warnungsquellen ohne Schalter** (werden bei jeder Beurteilung eines CI-Laufs im Protokoll gelesen): [z. B. „Hinweise der CI-Plattform zu Action-Versionen", „Bündelgrößen-Warnungen des Build-Werkzeugs"]

### Durchsetzungsmechanismen

Zwei Schichten, die identische Checks ausführen: lokale Pre-Commit-Hooks als erste Verteidigung, GitHub Actions als unabhängige Diagnoseschicht auf Push/PR. Beide Schichten sind Pflicht – die CI ersetzt die Hooks nicht und umgekehrt. Skelette für beide Schichten liegen unter `templates/` (siehe `templates/README.md`) und werden in Modus 2 Schritt 10 kopiert und angepasst.

- **Pre-Commit-Hook-Framework:** [z. B. `pre-commit`, `husky`, `lefthook`]
- **Konfigurationsdatei:** [z. B. `.pre-commit-config.yaml`, `.husky/`]
- **CI-Plattform:** GitHub Actions (Default; Abweichung erfordert ADR).
- **Workflow-Dateien:** Scope und Aufteilung nach Projektgrößen-Klasse (Glossar in `CLAUDE.md` Abschnitt 1B, Detail in `templates/projektstart.md` Abschnitt 2.2):
  - **Klasse K:** `.github/workflows/ci.yml` mit einem Job (Lint + Test).
  - **Klasse M/G:** `.github/workflows/ci.yml` mit allen Pflicht-Gates; bei G zusätzlich Aufteilung in `security.yml` / `release.yml`, sobald die Pipeline unübersichtlich wird.
  - **Klasse V:** je Service ein `.github/workflows/ci-<service>.yml` mit Path-Filtern, plus `integration.yml` für service-übergreifende Tests; `release.yml` und `security.yml` zentral.
- **Trigger:** mindestens `push` auf alle Branches und `pull_request` auf Hauptbranch.
- **Verpflichtende CI-Gates (Merge-Block bei Rot):**
  - Lint
  - Format-Check (kein Auto-Fix in CI)
  - Type-Check
  - Security-Scan
  - Dependency-Audit (Schwellenwert: [z. B. high oder critical])
  - Tests inklusive Coverage-Mindestwert
- **Branch-Protection auf Hauptbranch:** alle Pflicht-Gates müssen grün sein; Force-Push gesperrt; siehe Abschnitt 10.

### Coverage-Mindestwerte

- **Globaler Mindestwert:** [z. B. 80 % Lines, 70 % Branches]
- **Kritische Pfade (höhere Anforderung):** [Liste der Module/Pfade mit ihrem jeweiligen Mindestwert]
- **Ausnahmen:** [Module, für die Coverage nicht messbar ist, mit Begründung]

### Commit-Lint

- **Tool:** [z. B. `commitlint` mit Conventional-Commits-Konfiguration; falls nicht verwendet: „nicht aktiv, Begründung: …"]
- **Erlaubte Typen:** [z. B. feat, fix, refactor, docs, test, chore, perf, build, ci]
- **Branch-Präfixe:** wie `CLAUDE.md` Abschnitt 11; zusätzlich: [z. B. „keine" oder `init/` für den Initialisierungs-Commit]
- **Vom Werkzeug vergebene Branch-Namen:** [z. B. „keine" oder „Werkzeug X vergibt `<muster>/…`; Typ und Schritt-ID stehen dann im PR-Titel"]

### Editor-Integration (empfohlen, nicht erzwungen)

- **EditorConfig:** `.editorconfig` im Repo-Root (Zeilenenden, Einrückung, Encoding)
- **Editor-Snippets oder Linter-Plugins:** [optional auflisten]

<!-- ANCHOR:betrieb-und-deployment -->
## 8. Betrieb und Deployment

- **Deployment-Ziel:** [z. B. „eigener VPS via Ansible", „Kubernetes via Helm"]
- **CI/CD:** GitHub Actions (Default, siehe Abschnitt 7 für Workflow-Dateien). Deployment-Workflow: [z. B. `.github/workflows/release.yml` – Trigger und Ziel beschreiben, oder „kein Deploy-Workflow, manuelles Deployment"]
- **Umgebungen:** [z. B. lokal → staging → production]
- **Monitoring:** [falls vorhanden: was wird erfasst, wo]
- **Logging-Level Default:** [z. B. `INFO` in Prod, `DEBUG` nur lokal]
- **Vertretung:** [Person oder Rolle, die im Notfall eingreifen kann, oder „Verzicht, siehe ADR-NNN"]
- **Notfall-Handbuch:** [Pfad, z. B. `docs/onboarding-runbook.md` Abschnitt „Notfall"; zuletzt erprobt am YYYY-MM-DD]
- **KI im Betrieb:** [Konto bzw. Bezugsmodell (Abo oder API-Schlüssel); Kontingent und Zurücksetz-Zeitpunkt, z. B. „Wochenlimit, Zurücksetzung So 10:00", bzw. Budget und Nutzungsgrenzen; Rückfallweg ohne KI]
- **Zugriff der KI auf die Produktion:** [was darf sie lesen, ändern, ausführen – „kein Zugriff" ist zulässig]
- **Unbeaufsichtigtes Handeln der KI:** [„nein" oder: Befehlsliste unter [Pfad], Probelauf am YYYY-MM-DD]

[Die fünf Zeilen ab „Vertretung" prüft das Gate vor dem ersten öffentlichen Deployment (CLAUDE.md Abschnitt 12). Vorbefüllt in Modus 2, Schritt 4a.]

### Ablaufdaten-Register

[Alles, was zu einem Datum verfällt oder regelmäßig zurückgesetzt wird. Beim Sessionende prüft die KI, ob ein Vorlauf erreicht ist (`CLAUDE.md` Abschnitt 12, Punkt 8). Typische Einträge: Lebensende der fixierten Versionen aus Abschnitt 3, Zertifikate, Domains, Zugangs-Token (z. B. Registrierungs-Token eines CI-Runners, DNS-Schnittstelle), Kontingente mit Zurücksetz-Zeitpunkt, Abkündigungen aus dem CI-Protokoll.]

| Was | Ablauf / Lebensende | Vorlauf | Quelle | Fahrplan-Schritt |
|---|---|---|---|---|
| [z. B. Python 3.13] | [YYYY-MM-DD] | [z. B. 6 Monate] | [Hersteller-Angabe, Link] | [Schritt-ID, sobald Vorlauf erreicht] |
| [z. B. TLS-Zertifikat] | [YYYY-MM-DD] | [z. B. 14 Tage] | [automatisch erneuert? ja/nein] | [–] |

### Kosten

- **Kostenrahmen:** [Antwort auf „Was darf das Projekt monatlich kosten?" aus Modus 2 – Betrieb, Werkzeuge, KI]
- **Kostenregister** (optional ab Klasse M): [Pfad oder „nicht geführt"]

| Posten | Art (laufend / einmalig / KI-Verbrauch) | Betrag je Monat | Stand vom | Entscheidung nötig ab |
|---|---|---|---|---|
| [z. B. Server] | [laufend] | [Betrag] | [YYYY-MM-DD] | [z. B. Summe über Kostenrahmen] |

<!-- ANCHOR:entscheidungsbefugnisse -->
## 9. Entscheidungsbefugnisse

- **Freigabe-Entscheidungen trifft:** [Name/Rolle – normalerweise der Repo-Eigentümer]
- **Kommunikationskanal für Freigaben:** [z. B. „direkt im Chat / im Pull Request / im Fahrplan als Kommentar"]
- **Reaktionszeit-Erwartung:** [z. B. „asynchron, keine harte Antwortzeit"]

<!-- ANCHOR:repository-regeln -->
## 10. Repository-Regeln

- **Hauptbranch:** [z. B. `main`]
- **Push-Regel:** [z. B. „direkter Push erlaubt", „nur über PR", „PR + grüne CI + ein Approval"]
- **Schutzregeln:** [z. B. „keine Force-Pushes auf main", „gelöschte Branches nur nach Merge"]

<!-- ANCHOR:offene-grundsatzfragen -->
## 11. Offene Grundsatzfragen

[Wenn zu Projektstart Punkte noch ungeklärt sind, hier notieren.
Claude arbeitet nicht an Bereichen, die von offenen Grundsatzfragen abhängen,
ohne vorher eine Klärung anzustoßen.]

- [z. B. „Hosting-Modell (Self-Hosting vs. Managed) – offen bis Phase 2"]
- [z. B. „Auth-Provider (Keycloak vs. Better-Auth) – pending Spike"]

<!-- ANCHOR:glossar -->
## 12. Glossar (projektspezifische Begriffe)

[Begriffe, die im Projekt eine definierte Bedeutung haben und sonst mehrdeutig wären.
Verhindert, dass die KI Begriffe nach allgemeiner Lesart interpretiert.]

- **[Begriff]:** [Definition im Projektkontext]

---

**Pflegehinweis:** Änderungen an Status, Stack oder Constraints sind freigabepflichtig (siehe `CLAUDE.md` Abschnitt 4) und erzeugen einen ADR-Eintrag. Statuswechsel (z. B. `alpha` → `beta`) ziehen außerdem README-Badge- und CHANGELOG-Updates nach sich.

**Initialisierungshinweis (erste Session nach Projektanlage):**

- Alle Platzhalter in eckigen Klammern durch konkrete Werte ersetzen.
- Abschnitte, die für den Projekttyp nicht relevant sind (z. B. „Performance" bei einem einmaligen Skript), entfernen statt leer zu lassen.
- Abschnitt 11 (Offene Grundsatzfragen) darf nur Punkte enthalten, die echte Blocker sind – sonst entfernen.
- **Strukturwahl** richtet sich nach der Projektgrößen-Klassifikation (Glossar in `CLAUDE.md` Abschnitt 1B, Detail in `templates/projektstart.md` Abschnitt 2.2). Default pro Klasse:
  - **Klasse K (Klein):** Reduzierte Form – nicht relevante Abschnitte (Skalierung, Observability, Stakeholder) entfernen.
  - **Klasse M (Mittel) und G (Groß):** Ein Dokument, alle Abschnitte ausfüllen, Tiefe an Komplexität anpassen.
  - **Klasse V (Verteilt-Groß):** Ein Hauptdokument mit klar getrennten Service-Abschnitten, oder Index-Pattern mit `project-context-<service>.md` für Service-spezifische Stack-Details.
- Reaktiv-ADR-Schwellenwert in „Methodik-Schwellenwerte" klassen-abhängig setzen: K/M ≤ 30 %, G ≤ 20 %, V ≤ 15 %.
- Die Anpassung selbst als ADR-001 in `decisions.md` festhalten.
