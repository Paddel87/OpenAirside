# Dev-Templates

Methodik-Framework und Vorlagen-Set für Software-Projekte mit einem KI-Coding-Agent als Hauptentwickler. Referenz-Werkzeug ist Claude Code; das Regelwerk ist werkzeugneutral und auf andere Agents portierbar (siehe [„Verwendung mit anderen Coding-Agents"](#verwendung-mit-anderen-coding-agents)).

## Was es ist

Ein leeres Gerüst aus **Regelwerk** (`CLAUDE.md`) und **Pflicht-Dokument-Vorlagen** (`docs/`), das in ein neues Projekt geklont oder geforkt wird. Der Coding-Agent liest das Regelwerk zu Sessionbeginn und arbeitet diszipliniert danach: Architektur-Entscheidungen werden vorgelegt, nicht selbst getroffen; Implementierungen folgen einem Fahrplan; jeder Schritt durchläuft eine Definition of Done mit echten Gates (Lint, Type-Check, Tests, CI grün).

Es enthält **keinen Anwendungscode**. Es enthält stattdessen:

- **Wie KI und Mensch arbeiten** (semi-autonomer Modus, Freigabe-Kategorien, Stopp-Kriterien)
- **Welche Dokumente jedes Projekt führt** (Vision, Stack, Anforderungen ab Klasse M, Architektur, Fahrplan, Entscheidungen, Blocker, Logbuch, Onboarding-Runbook)
- **Wie der Projektstart abläuft** (Modus 1: Vision; ab Klasse G „Modus 1.5": Anwendungsfälle und Anforderungen klären, bevor der Stack feststeht; Modus 2: Vorlagen-Befüllung mit Versions-Verifikation, Sicherheitsgrundriss, Schutzbedarf und Kostenrahmen)
- **Was Qualität minimal heißt** (Code-Standards pro Sprache, Warnungen als Fehler, Versionswahl nach „ausgereifter Linie", Ablaufdaten-Register, Pflichten für Hilfsskripte, harte Archivierungs- und Drift-Trigger)
- **Wann ein Projekt innehält** (Stopp bei Phasen-Wucherung; an jeder Phasengrenze die Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?", bewertet von einer getrennten Instanz)
- **Was vor dem ersten öffentlichen Deployment feststehen muss** (Gate mit acht Prüfpunkten: Bedrohungsmodell, Sicherheitsniveau, Host-Härtung, Secrets, erprobte Wiederherstellung, unabhängige Prüfung, Vertretung und Notfall-Handbuch, KI im Betrieb)

## Für wen es ist

Für Menschen, die eine Software-Idee haben und wissen, wie sie funktionieren soll, aber **keine Programmiersprache beherrschen** und die technische Umsetzung vollständig einem KI-Coding-Agent überlassen. Die Methodik nennt das **Vision-Driven Development**: Der Mensch bringt die Vision und das Fachwissen seines Anwendungsbereichs, die KI bringt die Technik.

Das Regelwerk gleicht zwei Schwächen aus, an denen solche Projekte typischerweise scheitern:

- **Fehlendes technisches Wissen.** Architektur-, Sicherheits- und Abhängigkeitsentscheidungen trifft die KI nicht selbst. Sie legt sie als Frage vor, die sich aus der Vision beantworten lässt, und gibt dazu an, wie sicher sie ist und wie teuer die Entscheidung rückgängig zu machen wäre. Ob etwas fertig ist, entscheiden maschinelle Prüfungen (Lint, Tests, CI), nicht „läuft bei mir".
- **Fehlender roter Faden.** Ohne feste Struktur geraten solche Projekte vom Kurs ab: Nebenaufgaben verdrängen das Ziel, der Überblick geht verloren, und die nächste Session weiß nicht mehr, wo die letzte stand. Fahrplan-Pflicht, Logbuch, ein fester Landeplatz für jede Verschiebung und der Abgleich mit der Vision an Phasengrenzen halten den Faden fest, auch wenn der Mensch ihn selbst nicht halten kann.

Der Anspruch dahinter: Wer nicht programmieren kann, kann auch nicht nach Problemen fragen, die er nicht kennt. Die technische Wachsamkeit muss deshalb bei der KI und den Regeln liegen, nicht beim Menschen. Wo das Regelwerk diesen Anspruch noch nicht einlöst, ist in Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34) festgehalten.

**Was du mitbringen musst:**

- eine klare Vorstellung davon, was die Software tun soll und für wen,
- die Bereitschaft, Entscheidungsfragen zu beantworten: Die KI hält bei freigabepflichtigen Themen an und wartet, bis du entschieden hast,
- Geduld mit Disziplin: Pflichtlektüre, Logbuch und Definition of Done kosten auch bei kleinen Änderungen Zeit – das ist der Preis für den roten Faden.

**Erprobungsstand:** Die Methodik ist an einem Pilotprojekt der Klasse G erprobt (Web-Plattform mit mehreren Frontends, ein Mensch ohne Programmierkenntnisse plus Coding-Agent, seit Mai 2026). Klasse K wird nur an diesem Repo selbst angewendet, das keinen Anwendungscode enthält. Die Klassen M und V sind beschrieben, aber noch nicht in der Praxis erprobt. Klassen-Differenzierung in [`templates/projektstart.md`](templates/projektstart.md) Abschnitt 2.

**Weniger geeignet für:**

- **Teams mit mehreren Menschen**, die parallel arbeiten: Rollen, Review-Zuständigkeiten und die Abstimmung zwischen parallelen Sessions regelt das Regelwerk nicht (siehe Issue [#33](https://github.com/Paddel87/Dev-Templates/issues/33)).
- **Wegwerf-Prototypen**: Der Aufwand für Dokumentation und Gates lohnt sich erst bei Projekten, die länger leben sollen.
- **Erfahrene Entwickler** können die Methodik nutzen, sind aber nicht die Zielgruppe: Viele Regeln erklären, was sie ohnehin wissen.

## Verzeichnisbaum

```text
CLAUDE.md                       Verbindliches Regelwerk; pro Session geladen
AGENTS.md                       Werkzeugneutraler Einstiegspunkt (verweist auf CLAUDE.md)
templates/                      Das ausgelieferte Produkt
├── projektstart.md             Vollständige Anleitung Modus 1 + Modus 2
├── architektur-heuristiken.md  Entwurfs- und Konfidenz-Heuristiken (on-demand bei Architekturentscheidungen)
├── wirtschaftlichkeit.md       Anhang zum Regelwerk: Abgabe an Unteragenten, Probelauf, Kontingent-Warnung (on-demand)
├── gate-deployment.md          Anhang zum Regelwerk: Prüfpunkte vor dem ersten öffentlichen Deployment (on-demand)
├── docs/                       Pflicht-Dokument-Vorlagen (→ nach docs/ kopieren und dort befüllen)
│   ├── vision.md               ursprüngliche Idee (eingefroren nach Modus 2)
│   ├── project-context.md      Stack, Constraints, Plattform-Matrix, Modellklassen
│   ├── architecture.md         Module, Schnittstellen, Tooling-Inventar
│   ├── requirements.md         Anwendungsfälle und Anforderungen (Pflicht ab Klasse M)
│   ├── fahrplan.md             Phasen und Schritte
│   ├── decisions.md            ADRs (Architektur-Entscheidungen mit Begründung)
│   ├── blockers.md             ungelöste Probleme
│   ├── logbuch.md              chronologische Sessions und Reibungen
│   ├── onboarding-runbook.md   End-to-End-Setup (Pflicht ab Klasse M)
│   └── readme-vorlage.md       Vorlage für die README des Ziel-Projekts
├── werkzeuge/                  Werkzeugspezifische Hilfen (optional), z. B. Kontingent-Warnung für Claude Code
├── github-workflows/           CI-Skelette pro Klasse und Sprache
└── pre-commit/                 Pre-Commit-Konfigurationen pro Sprache
docs/                           Arbeitsdokumente von Dev-Templates SELBST (schlank, seit 2026-10-12):
                                project-context.md, offene-punkte.md, Prüfberichte, Archiv.
                                Beim Start eines eigenen Projekts wird dieser Inhalt ersetzt.
```

## So benutzt du es

1. **Klonen oder Forken** dieses Repos als Ausgangspunkt für dein Projekt. Der Inhalt von `docs/` gehört zu Dev-Templates selbst und wird in Modus 2 durch deine eigenen, aus `templates/docs/` kopierten Dokumente ersetzt.
2. **Erste Session im normalen Chat** (200K Kontext, stärkstes verfügbares Modell – derzeit Opus). Modus 1: Vision erarbeiten in `docs/vision.md`. Die KI darf in Modus 1 **keine** Technologie-Vorschläge machen – die Vision reift zuerst. Vollständige Anleitung: [`templates/projektstart.md`](templates/projektstart.md) Abschnitt 1.1.
3. **Triggerphrase** (eine von: „Vorlagen vorbereiten", „Initialisierung starten", „Modus 2 starten", „Vorlagen-Set initialisieren") → Modus 2: Stack-Optionen, Architektur-Grobschnitt, **Sicherheitsgrundriss** (Bedrohungsmodell, Sicherheitsniveau, Vertretung, KI-Kontingent), alle Pflicht-Dokumente befüllen, CI-Workflows aus `templates/` kopieren, **Versions-Verifikation** (Pflicht-Stopp vor ADR-002), Initialisierungs-Commit.
4. **Reguläre Sessions im Coding-Agent** (Claude Code oder ein anderer, siehe [„Verwendung mit anderen Coding-Agents"](#verwendung-mit-anderen-coding-agents)). Pflichtlektüre nach `CLAUDE.md` Abschnitt 2 (Mindest-Lektüre + Vertiefung auf Anforderung), dann arbeiten. Vier Modellklassen (Mechanik, Routine, Entscheidung, Ausnahme) mit empfohlener Klasse je Fahrplan-Schritt. Bei definierten Auslösern (Freigabe-Entscheidungen, `[STRATEGISCH]`-ADRs, Reifegrad-Beförderungen) **hält die KI an** und wartet auf einen Wechsel zur **Entscheidungs-Klasse**; Routinearbeit gibt sie nach bestandenem Probelauf selbst an einen günstigeren Unteragenten ab – siehe `CLAUDE.md` Abschnitt 0, „Modellklassen-Disziplin".

## Verwendung mit anderen Coding-Agents

Die Methodik steckt fast vollständig in **Prosa-Disziplin und Markdown-Konventionen**, nicht in Claude-Code-spezifischer Maschinerie (keine tragenden Hooks, Skills oder Slash-Commands). Dadurch ist das Regelwerk **werkzeug- und modellneutral** – jeder hinreichend fähige Coding-Agent kann ihm folgen.

Gebunden ist nur der **Einstiegspunkt**, also welche Datei das Werkzeug automatisch lädt:

| Werkzeug | Auto-geladene Regeldatei |
|---|---|
| Claude Code | `CLAUDE.md` |
| OpenAI Codex (und andere `AGENTS.md`-fähige Tools) | `AGENTS.md` |
| Gemini CLI | `GEMINI.md` |
| Cursor | `.cursor/rules/` bzw. `.cursorrules` |

[`AGENTS.md`](AGENTS.md) ist dafür bereits angelegt: ein schlanker **Verweis** auf `CLAUDE.md`, keine Kopie. So bleibt `CLAUDE.md` die einzige Quelle der Wahrheit, und es entsteht keine Drift. Für weitere Werkzeuge legst du analog eine dünne Einstiegsdatei an, die ebenfalls nur auf `CLAUDE.md` verweist.

**Ehrlicher Vorbehalt:** Portabel heißt *mechanisch lauffähig*, nicht *gleich gut befolgt*. Pflichtlektüre, Stopp-Kriterien und die Konfidenz-Offenlegung setzen ein Modell voraus, das lange Kontexte verarbeitet, Instruktionen diszipliniert befolgt und ehrlich über eigene Unsicherheit ist. Wie treu ein fremdes Modell die Methodik einhält, ist Modell-Verhalten – das prüfst du am besten an konkreten Lackmus-Tests (Hält es die Mindest-Lektüre durch? Stoppt es bei freigabepflichtigen Entscheidungen? Gibt es Konfidenz ehrlich an?).

## Methodik-Stand

Die Methodik wird laufend gegen ein reales Pilotprojekt (Klasse G) rückgekoppelt. Bisherige Patch-Wellen:

- **[#7](https://github.com/Paddel87/Dev-Templates/pull/7) – Disziplin-Schärfung:** Größen-Budget für Pflicht-Lektüre, harte Archivierungs-Trigger, Onboarding-Validierung als DoD-Punkt, Inter-Pflicht-Drift-Prüfung.
- **[#8](https://github.com/Paddel87/Dev-Templates/pull/8) – Vorlagen-Erweiterungen:** Plattform-Matrix in `project-context.md`, Tooling-Inventar in `architecture.md`, Pflichten für Hilfsskripte in `scripts/`.
- **[#9](https://github.com/Paddel87/Dev-Templates/pull/9) – Onboarding-Verankerung:** Neuer `CLAUDE.md` §17, neue Vorlage `docs/onboarding-runbook.md`.
- **[#10](https://github.com/Paddel87/Dev-Templates/pull/10) – Selbstanwendung:** Projektstart-Verfahren nach `templates/projektstart.md` ausgelagert, weil `CLAUDE.md` selbst sonst sein eigenes Größen-Budget gerissen hätte.
- **[#21](https://github.com/Paddel87/Dev-Templates/pull/21) – Modellklassen-Disziplin:** Zwei Modellklassen (Routine, Entscheidung) in `CLAUDE.md` §0, Eskalation an objektive Auslöser gebunden statt an Selbsteinschätzung, Rückstufungs-Hinweis für Routinearbeit, Zuordnung in `docs/project-context.md`.
- **[#22](https://github.com/Paddel87/Dev-Templates/pull/22) – ANCHOR-Konvention umgesetzt:** 42 Sprung-Anker in den sechs Pflicht-Dokumenten, plus die bis dahin fehlende Namensregel in `CLAUDE.md` §2. Schließt den Drift zwischen dokumentierter und umgesetzter Konvention.
- **[#23](https://github.com/Paddel87/Dev-Templates/pull/23) – Selbstanwendung:** Vorlagen nach `templates/docs/` verschoben, `docs/` enthält jetzt die ausgefüllten Arbeitsdokumente von Dev-Templates selbst (ADR-001, Klasse K). Damit gilt `CLAUDE.md` wortgleich für dieses Repo und für Ziel-Projekte.
- **[#25](https://github.com/Paddel87/Dev-Templates/pull/25) – Erster Markdown-Linter:** `markdownlint-cli2` über `pre-commit` eingerichtet (ADR-002) – erste externe Abhängigkeit des Repos. Wer an `CLAUDE.md`, `templates/` oder `docs/` mitarbeitet, braucht ab sofort `pre-commit` installiert.
- **[#26](https://github.com/Paddel87/Dev-Templates/pull/26) – Erste CI-Pipeline:** `.github/workflows/ci.yml` erzwingt den pre-commit-Hook jetzt auch remote (ADR-003). Drift-Check-Automatisierung bewusst als eigener Schritt abgespalten, nicht mitgeliefert.
- **[#27](https://github.com/Paddel87/Dev-Templates/pull/27) – Lizenz geklärt:** CC0 1.0 Universal (ADR-004), live von creativecommons.org bezogen. Schließt den ursprünglichen Fahrplan S-1 bis S-6 vollständig ab.
- **[#28](https://github.com/Paddel87/Dev-Templates/pull/28) – Aktions-Versionen aktualisiert:** Fünf veraltete GitHub-Actions-Versionen in `templates/github-workflows/*.yml` live verifiziert und nachgezogen. Nicht freigabepflichtig, ohne Modellklassen-Eskalation bearbeitet.
- **[#32](https://github.com/Paddel87/Dev-Templates/pull/32) – Vision-Verlust-Lücke und Teil-C-Lücke geschlossen:** Zwei im Pilotprojekt belegte Befunde übernommen. `CLAUDE.md` §6 verbietet jetzt die Verschiebung ohne Landeplatz, §7 kennt den Marker `[VERSCHOBEN]` mit Pflicht zur Ziel-Schritt-ID, §12 erzwingt eine Vision-Re-Derivation an jeder Phasengrenze plus ein Go-Live-Gate (ADR-006). §2 nimmt `decisions.md` Teil C ausdrücklich in die Mindest-Lektüre auf – er war im Wortlaut nie erwähnt und fiel dadurch strukturell durch (ADR-007).
- **[#36](https://github.com/Paddel87/Dev-Templates/pull/36) – Secrets und Schutzmechanismen:** Paket 1, Stufe 1 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34). `CLAUDE.md` §6 verbietet Secret-Werte auch in der eigenen Ausgabe der KI; ein trotzdem ausgegebener Wert gilt als kompromittiert und wird rotiert. Schutzmechanismen (Alarme, Backups, Rate-Limits, Gates) werden erst `[BELASTBAR]`, wenn ein absichtlich herbeigeführter Fehlerfall vollständig durchlief (ADR-008). Das Gate vor dem ersten öffentlichen Deployment folgt als S-14.
- **[#37](https://github.com/Paddel87/Dev-Templates/pull/37) – Gate vor dem ersten öffentlichen Deployment:** Paket 1, Stufe 2 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34). `CLAUDE.md` §12 verlangt vor dem ersten öffentlichen Deployment acht blockierende Prüfpunkte, von Bedrohungsmodell und Host-Härtung über eine erprobte Wiederherstellung und eine unabhängige Prüfung bis zu Vertretung, Notfall-Handbuch und KI-Kontingent. Modus 2 legt dafür einen Sicherheitsgrundriss an (ADR-009).
- **[#38](https://github.com/Paddel87/Dev-Templates/pull/38) – Stille Fehler, Stufe 1:** Paket 2 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34). Warnungen gelten als Fehler, wo das Werkzeug es erlaubt, sonst als Bestand mit Obergrenze, die nur sinken darf; jede Abkündigung wird ein Fahrplan-Schritt mit Frist. Ein Ablaufdaten-Register wird beim Sessionende geprüft. Versionen belegt die KI selbst mit Quellen, gewählt wird die „ausgereifte Linie" (ADR-010).
- **[#39](https://github.com/Paddel87/Dev-Templates/pull/39) – Modellwahl nach Aufgabe:** Paket 2, Stufe 2 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34). `CLAUDE.md` §0 kennt vier Modellklassen (Mechanik, Routine, Entscheidung, Ausnahme) mit objektiven Auslösern und einer empfohlenen Klasse je Fahrplan-Schritt. Routinearbeit gibt die KI nach bestandenem Probelauf selbst an einen Unteragenten mit günstigerem Modell ab; das Modell wird beim Sessionstart abgefragt, am Sessionende bilanziert (ADR-011). Ob die Abgabe Kontingent spart, ist noch nicht gemessen (S-19).
- **[#40](https://github.com/Paddel87/Dev-Templates/pull/40) – Roter Faden:** Paket 3 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34). Wächst eine Phase über das Doppelte ihres Plans hinaus, hält die KI an und plant neu (`CLAUDE.md` §8, Kriterium 9). An jeder Phasengrenze kommt die Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?", bewertet von einer getrennten Instanz statt von der KI, die gebaut hat. Architekturentscheidungen in einer STABILISIERUNG zählen automatisch als reaktiv (ADR-012).
- **[#41](https://github.com/Paddel87/Dev-Templates/pull/41) – Anforderungsschicht:** Paket 4 aus Issue [#34](https://github.com/Paddel87/Dev-Templates/issues/34), zugleich Abschluss von [#20](https://github.com/Paddel87/Dev-Templates/issues/20). Neue Vorlage `requirements.md` ab Klasse M mit Anwendungsfällen (auch bewusst ausgeschlossenen) und nummerierten Anforderungen; bei Klasse G und V klärt „Modus 1.5" sie, bevor der Stack feststeht. Schutzbedarf der Daten ist Obergrenze für den Datenschutz-Aufwand, Modus 2 fragt den Kostenrahmen ab, Geschäftsentscheidungen bekommen einen eigenen Teil in `decisions.md` (ADR-013).
- **[#42](https://github.com/Paddel87/Dev-Templates/pull/42) – Erkundung Modellkosten:** Messung zu ADR-011. Die Abgabe von Lesearbeit an eine günstigere Modellklasse spart kaum, weil das Lesen aus dem Cache bei den beteiligten Modellen gleich viel kostet; größter Hebel ist die Kontextgröße der laufenden Session. Die Statuszeile des Werkzeugs meldet 5-Stunden- und 7-Tage-Limit, Hooks nur Modellwechsel. Folgerungen als Vorschlag S-22, noch nicht entschieden.
- **[#43](https://github.com/Paddel87/Dev-Templates/pull/43) – Sessiongröße und sparsame Abgabe:** Folgerungen aus der Erkundung zu #42. Arbeit wird nur noch an eine Modellklasse abgegeben, wo das tatsächlich spart. Über einer Grenze der Sessiongröße beginnt die KI keinen neuen Schritt mehr und bittet um eine neue Session. Für Claude Code liegt eine optionale Kontingent-Warnung unter `templates/werkzeuge/` bei, aktiv erst nach einem Probelauf (ADR-014).
- **[#44](https://github.com/Paddel87/Dev-Templates/pull/44) – Prüfung durch eine getrennte Instanz:** Erste Anwendung der Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?" auf dieses Repo selbst. Eine unabhängige Session empfahl, das Regelwerk gezielt umzubauen: es war an einem Tag um ein Drittel gewachsen und enthielt acht Widersprüche zwischen Regel und Praxis. Bericht und Stellungnahme liegen unter `docs/pruefberichte/`, der Umbau folgt in vier Blöcken (ADR-015).
- **[#45](https://github.com/Paddel87/Dev-Templates/pull/45) – Sessiongröße als Hinweis:** Erster Umbau-Block (U2). Die Grenze der Sessiongröße löst keinen Stopp mehr aus, sondern nur einen einzeiligen Hinweis beim Sessionende; Vorgabe 400.000 Token bis zur Messung. Auslöser war die erste Rückmeldung aus einem zweiten Projekt. Die übrigen Unterbrechungsformen bleiben auf Entscheidung des Eigentümers unverändert (ADR-016).
- **[#47](https://github.com/Paddel87/Dev-Templates/pull/47) – Regelwerk verschlankt:** Dritter Umbau-Block (U1, ADR-018). Begründungen stehen im Regelwerk nur noch als ein Satz, die Details zu Kosten und zum Gate in zwei Zusatzdateien, die die KI nur bei Bedarf lädt. Bei Klasse K wird der Fahrplan beim Abschluss eines Bündels archiviert. `CLAUDE.md` ist 14 % kleiner, der Kontext je Aufruf aber nur 0,8 % (gemessen); eine Größengrenze gibt es deshalb nicht.
- **[#46](https://github.com/Paddel87/Dev-Templates/pull/46) – Widersprüche aufgelöst:** Zweiter Umbau-Block (U3, ADR-017). Merges ohne CI-Lauf bekommen einen eigenen Status `[CI-AUSSTEHEND]` mit Nachprüfung und Frist. Korrektur-ADRs zählen als reaktiv. Die Wucherung wird nach Inhalt gezählt. Das Regelwerk ist frei von Modell- und Werkzeugnamen. Das Python-Hilfsskript wird mit `ruff` und Tests geprüft.
- **Selbstanwendung verschlankt (2026-10-12):** Dieses Repo wendet die eigene Methodik nicht mehr vollständig auf sich selbst an. Fahrplan, Logbuch und ADRs liegen eingefroren unter `docs/archiv/selbstanwendung/`. Es gelten Linter, CI, Pull Requests mit Begründung und dieser Methodik-Stand. Rückmeldungen zum Regelwerk kommen aus abgeleiteten Projekten. Das Regelwerk selbst ist unverändert. Zuvor lief der ausstehende CI-Nachlauf für PR #35 bis #48 grün.

Adressierte Fall-Studien: Issues [#5](https://github.com/Paddel87/Dev-Templates/issues/5) (Onboarding-Tauglichkeit und Tooling-Reifegrad) und [#6](https://github.com/Paddel87/Dev-Templates/issues/6) (Dokument-Hygiene).

## Was du anpassen darfst

`CLAUDE.md` bleibt projektübergreifend **unverändert**. Projektspezifika (Stack, Versionen, Constraints, Coverage-Mindestwerte) gehören in `docs/project-context.md`. Wenn dein Projekt die Methodik selbst ändern muss, ist das ein ADR und gehört in `docs/decisions.md` – nicht in `CLAUDE.md`.

## Lizenz

[CC0 1.0 Universal](LICENSE) – Public-Domain-Widmung, keine Attributionspflicht. Gewählt, weil der Zweck dieses Repos das Kopieren der Vorlagen in fremde Projekte ist (siehe „So benutzt du es"); eine Lizenz mit Namensnennungspflicht würde jedem abgeleiteten Projekt eine Pflicht mitgeben, die zum eigentlichen Zweck nicht passt. Begründung mit Alternativen in `docs/archiv/selbstanwendung/decisions.md` ADR-004.
