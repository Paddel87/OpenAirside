# CLAUDE.md

<!-- Verbindliches Regelwerk für den KI-Coding-Agenten in diesem Repository.
     Wird zu Sessionbeginn automatisch geladen und gilt für alle Sessions.
     Betriebsmodus: semi-autonom. KI arbeitet eigenständig innerhalb definierter Grenzen;
     strategische Entscheidungen werden dem Menschen zur Freigabe vorgelegt. -->

## 0. Betriebsmodus

- **Modus:** Semi-autonom.
- **KI entscheidet eigenständig:** Implementierungsdetails, lokale Refactorings innerhalb eines Moduls, Bugfixes ohne Architekturwirkung, Testerstellung, Dokumentationspflege, Commit-Erstellung, Branch-Verwaltung.
- **KI legt zur Freigabe vor (siehe Abschnitt 4):** Architekturänderungen, neue Module, neue externe Abhängigkeiten, Datenmodelländerungen, API-Vertragsänderungen, Sicherheits- und Datenschutz-relevante Entscheidungen, Änderungen an Build-/Deploy-Pipeline.
- **KI stoppt zwingend (siehe Abschnitt 8):** fehlende Information, widersprüchliche Anforderungen, wiederholtes Scheitern, destruktive Eingriffe.

### Modellklassen-Disziplin

Die Methodik unterscheidet vier **Modellklassen**, nicht konkrete Modelle. Welches Modell welche Klasse besetzt, legt `docs/project-context.md` Abschnitt 6 fest; dieses Dokument bleibt modell- und werkzeugneutral.

- **Mechanik-Klasse:** Arbeit ohne Urteil. Erst aktiv, wenn ein Probelauf sie im Projekt belegt hat (siehe „Probelauf" unten); bis dahin übernimmt die Routine-Klasse ihre Arbeit.
- **Routine-Klasse:** trägt den Normalbetrieb. Durchspezifizierte Arbeit ohne Architektur- oder Freigabewirkung.
- **Entscheidungs-Klasse:** das stärkste im Projekt regulär eingesetzte Modell. Wird gezielt dort eingesetzt, wo ein Fehlschluss teuer ist.
- **Ausnahme-Klasse:** ein noch stärkeres oder teureres Modell, falls verfügbar. Nur auf begründeten Vorschlag der KI mit Freigabe des Menschen.

Die Zuordnung erfolgt **nicht** über Selbsteinschätzung („fühle ich mich überfordert?"), sondern über die unten gelisteten, objektiv prüfbaren Auslöser. Grund: Ein Modell, das an einer Aufgabe scheitert, bemerkt dieses Scheitern meist auch nicht – die beiden Fehler korrelieren (ADR-011).

**Pflicht-Eskalation auf die Entscheidungs-Klasse** bei jedem dieser Auslöser:

1. Ein `ENTSCHEIDUNG ERFORDERLICH`-Block nach Abschnitt 4 steht an (alle acht Kategorien).
2. Stopp-Kriterium 8 greift: Konfidenz *niedrig* **und** Umkehrbarkeit *teuer* (Abschnitt 8).
3. Ein ADR mit Klassifikations-Tag `[STRATEGISCH]` entsteht.
4. Ein Architektur-Bestandteil wird von `[VORLÄUFIG]` auf `[BELASTBAR]` befördert.
5. Die Reaktiv-Quote ist überschritten und ein Reflexions-Schritt steht an.
6. Modus 1 oder Modus 2 des Projektstarts läuft (siehe `templates/projektstart.md` Abschnitt 1).

**Vorschlag der Ausnahme-Klasse** nur bei einem dieser Auslöser: (1) Derselbe Ansatz ist auf der Entscheidungs-Klasse dreimal gescheitert (Abschnitt 10) – der Vorschlag kommt dann zusätzlich zum Blocker-Protokoll, nicht statt seiner; (2) der Mensch verlangt sie ausdrücklich. Form: der Stopp-Block unten mit `Benötigte Klasse: Ausnahme`. Lehnt der Mensch ab, läuft die Arbeit auf der Entscheidungs-Klasse weiter.

**Routine-Klasse** genügt für:

- Logbuch-Einträge und Sessionrahmen (Abschnitt 12)
- README-Synchronisation und Inter-Pflicht-Drift-Prüfung (Abschnitt 16)
- Archivierung (Abschnitt 14)
- Fahrplan-Status-Updates ohne inhaltliche Neuplanung
- Umsetzung klar spezifizierter Fahrplan-Schritte, die keinen Eskalations-Auslöser berühren; Test-Erstellung und Testwartung

**Mechanik-Klasse** genügt für: Formatierung und Linter-Korrekturen, Umbenennen nach vorgegebenem Muster, Nachziehen von Tabellen und Verweisen aus einer benannten Quelle, Suchen und Zählen.

**Empfohlene Klasse je Schritt.** Jeder Fahrplan-Schritt trägt das Feld „Empfohlene Klasse" mit einem Satz Begründung aus den Listen oben. Enthält ein Schritt Arbeit mehrerer Klassen, gilt die höchste für den Schritt; Teilarbeiten niedrigerer Klassen dürfen abgegeben werden (nächster Abschnitt).

### Arbeit oberhalb der empfohlenen Klasse

Läuft Arbeit auf einer höheren Klasse als nötig, ist das eine Frage der Wirtschaftlichkeit, kein Stopp (`docs/decisions.md` Regel-001, Ausnahme). Die Wirkung sichert ein Mechanismus ohne Zutun des Menschen:

1. **Abgabe an einen Unteragenten:** Arbeit der Routine- und Mechanik-Klasse gibt die KI selbständig an einen Unteragenten mit fest eingestelltem Modell dieser Klasse ab, prüft sein Ergebnis vor dem Commit und verantwortet es – nur nach bestandenem Probelauf und nur, wo die Abgabe tatsächlich spart.
2. **Warnung:** Ist keine Abgabe möglich, sagt die KI einmal zu Beginn des Schritts, dass er oberhalb seiner Empfehlung läuft. Kein Stopp.
3. **Bilanz:** Der `[SESSIONENDE]`-Eintrag nennt die aktive Klasse, die Zahl der Schritte oberhalb ihrer Empfehlung und die abgegebenen Teilarbeiten.

Kriterien der Abgabe, Probelauf und Kontingent-Warnung stehen in [`templates/wirtschaftlichkeit.md`](templates/wirtschaftlichkeit.md) (geladen nach Abschnitt 2, „Vertiefung auf Anforderung").

### Sessiongröße

Jeder Werkzeugaufruf liest den ganzen bisherigen Kontext erneut; mit wachsender Session wird deshalb jeder Aufruf teurer. Beim Sessionende stellt die KI die Kontextgröße über die Laufzeitumgebung fest und trägt sie in den `[SESSIONENDE]`-Eintrag ein. Liegt sie über der Grenze aus `docs/project-context.md` Abschnitt 6, folgt ein einzeiliger Hinweis im Eintrag und in der Abschlussmeldung mit der Empfehlung, die nächste Arbeit in einer neuen Session zu beginnen – kein Stopp. Ist die Größe nicht feststellbar, entfällt der Hinweis; das wird vermerkt.

### Eskalation ist ein echter Stopp, keine Empfehlung

Bei einem Eskalations-Auslöser **hält die KI die Arbeit an**. Sie gibt den Block unten aus und wartet – kein Werkzeugaufruf, keine Dateiänderung, keine Ersatzhandlung, bis der Mensch geantwortet hat. Der Stopp folgt damit demselben Muster wie die Stopp-Kriterien aus Abschnitt 8.

**Form des Stopps:**

```text
STOPP – MODELLWECHSEL ERFORDERLICH
Aktuelle Modellklasse: [Mechanik | Routine | Entscheidung | unbestimmbar]
Benötigte Klasse: [Entscheidung | Ausnahme]
Auslöser: [welcher Punkt aus der Eskalations-Liste oben]
Warum jetzt: [1–2 Sätze]
Was ich brauche: Wechsel auf die benötigte Klasse. Ich bestätige
    den Wechsel, sobald ich ihn sehe, und arbeite dann weiter.
Falls du nicht wechseln willst: sag es ausdrücklich – dann arbeite ich
    auf der aktuellen Klasse weiter und vermerke die Abweichung im
    Logbuch.
```

**Beim Wiederanlauf:** Die KI benennt zuerst ausdrücklich, welche Klasse jetzt aktiv ist, und erst danach arbeitet sie weiter. Ohne diese Bestätigung ist der Stopp nicht aufgehoben.

**Kein Ersatz-Vorgehen:** Eine frühere Fassung bot im selben Block eine Alternative an und arbeitete faktisch weiter; der Entscheidungspunkt war damit wirkungslos (ADR-005, Regel-001).

### Woher die KI ihre Modellklasse kennt

Nicht durch Selbstprüfung. Die Kenntnis stammt ausschließlich aus **Mitteilungen der Laufzeitumgebung**: dem Systemkontext zu Sessionbeginn, den Benachrichtigungen, die ein Werkzeug beim Modellwechsel einspeist, und – wo vorhanden – einer Abfrage, die das eingestellte und das tatsächlich bediente Modell liefert.

**Pflicht beim Sessionstart:** Bietet die Laufzeitumgebung eine solche Abfrage, nutzt die KI sie und trägt eingestelltes und bedientes Modell samt Klasse in den `[SESSIONSTART]`-Eintrag ein; sonst den Systemkontext, mit Vermerk der Quelle. Vor einem Eskalations-Stopp und vor der Bilanz am Sessionende wird die Abfrage wiederholt, sofern verfügbar.

Zwei Grenzen: Wird das Modell **ohne solche Mitteilung** getauscht, bemerkt die KI es nicht. Wenn Systemkontext und spätere Mitteilung sich **widersprechen**, gilt die zuletzt eingegangene Mitteilung; bleibt der Widerspruch unauflösbar, ist die Klasse `unbestimmbar`.

**Wenn die eigene Modellklasse unbestimmbar ist:** Der Stopp wird trotzdem ausgelöst, mit `Aktuelle Modellklasse: unbestimmbar`. Ein überflüssiger Stopp ist billiger als eine übersehene Eskalation. Warnung und Bilanz zur Arbeit oberhalb der Empfehlung entfallen; die Abgabe an Unteragenten bleibt möglich, weil deren Modell fest eingestellt ist.

**Was diese Regel nicht leistet:** Sie greift ausschließlich bei den gelisteten Auslösern; Fälle, in denen Tiefe ohne Auslöser nötig gewesen wäre, fängt sie nicht – bewusst, weil eine Selbsteinschätzungs-Pflicht unzuverlässiger wäre (ADR-011). Den Verbrauch eines Kontingents misst sie nicht; eine Warnung dazu regelt `templates/wirtschaftlichkeit.md`.

## 1. Projektkontext

Der projektspezifische Kontext liegt in `docs/project-context.md`. Diese Datei wird zu Sessionbeginn **zuerst** gelesen (Abschnitt 2).

Die vorliegende `CLAUDE.md` bleibt projektübergreifend unverändert. Projektspezifika gehören ausschließlich in `docs/project-context.md` oder die in Abschnitt 3 gelisteten Projekt-Dokumente.

## 1A. Projektstart (separate Anleitung)

Das zweistufige Projektstart-Verfahren (Modus 1: Vision-Erstellung; Modus 2: Vorlagen-Initialisierung) ist in [`templates/projektstart.md`](templates/projektstart.md) Abschnitt 1 vollständig beschrieben. Es läuft **einmalig pro Projekt** und wird **nicht pro Session geladen**.

Der reguläre Betrieb (Abschnitt 2 ff. dieses Dokuments) beginnt nach dem Initialisierungs-Commit, der Modus 2 abschließt.

## 1B. Projektgrößen-Klassifikation (Glossar)

Die vollständige Klassifikations-Methodik steht in [`templates/projektstart.md`](templates/projektstart.md) Abschnitt 2. Im regulären Betrieb genügt das folgende Glossar:

- **Klasse K (Klein):** 1 Modul, 0–1 externe Abhängigkeiten, einzelne Sprache, kein Persistenzlayer. Typisch: CLI-Tool, Library mit klar abgegrenztem Zweck.
- **Klasse M (Mittel):** 2–5 Module, 2–5 externe Abhängigkeiten, ein bis zwei Sprachen, eine Persistenzschicht, monolithisches Deployment. Typisch: Web-Anwendung, internes Tool mit UI.
- **Klasse G (Groß):** 6+ Module, 5+ externe Abhängigkeiten, mehrere Persistenzschichten, mehrere Nutzergruppen, NFR-Komplexität. Typisch: komplexe Multi-Tenant-Plattform, ML-System ohne starke Verteilung.
- **Klasse V (Verteilt-Groß):** 3+ unabhängig deploybare Services, asynchrone Kommunikation, heterogene Persistenzen, Compliance-Anforderungen. Typisch: ML-Plattform mit getrenntem Training/Inference/Serving, Multi-Service-Architektur mit Event-Bus.

Die Klasse wird einmalig in ADR-001 fixiert. Reklassifikation (typischerweise K→M oder M→G) erfordert ADR mit Migration der Dokumentstruktur als STABILISIERUNG-Phase – Details siehe [`templates/projektstart.md`](templates/projektstart.md) Abschnitt 2.4.

## 2. Pflichtlektüre zu Sessionbeginn

**Vor jeder Änderung** liest die KI die Pflichtlektüre. Sie ist zweistufig: eine **strikte Mindest-Lektüre**, die immer geladen wird, und eine **Vertiefung auf Anforderung**, die nur bei konkretem Bedarf erfolgt.

### Mindest-Lektüre (Pflicht, in dieser Reihenfolge)

1. **`docs/project-context.md`** – das gesamte Dokument. Es enthält Stack, Constraints und projektspezifische Regeln, die jede Änderung betreffen können. Vollständig.
2. **`docs/logbuch.md`** – nur den **letzten `[SESSIONENDE]`-Eintrag plus alle Einträge danach**. Diese Auswahl liefert den Wiedereinstiegspunkt. Frühere Sessions werden nicht gelesen.
3. **`docs/fahrplan.md`** – nur den Abschnitt **„Aktueller Stand"** plus die **aktuell laufende Phase mit ihren Schritten** (bei Klasse K ohne Phasen: alle Schritte im aktiven Dokument, das nach Abschnitt 14 nur offene Schritte und das laufende Bündel hält). Andere Phasen, Replanning-Historie und Archiv werden nicht gelesen.
4. **`docs/architecture.md`** – nur die Abschnitte **„Überblick" (1)**, **„Modul-Karte" (2)** und **„Reifegrad-Übersicht" (9)**. Detailspezifikationen einzelner Module und Schnittstellen werden nicht gelesen.
5. **`docs/decisions.md`** – **Teil A (ADR-Übersicht und Reaktiv-Quote)** und **Teil C (Entscheidungsregeln)**, sofern Teil C nicht leer ist. Einzelne ADR-Einträge in **Teil B** werden nicht gelesen. Teil C führt dauerhaft geltende Betriebsregeln aus ADRs, die im Alltag laufend angewendet werden. Wächst Teil C über das Lesbare hinaus, ist das ein Auslagerungs-Fall nach Abschnitt 14, kein Grund, ihn zu überspringen.
6. **`docs/blockers.md`** – nur den Abschnitt **„Aktive Blocker"**. Gelöste Blocker und Erkennungs-Heuristiken werden nicht gelesen (letztere sind in CLAUDE.md Abschnitt 8 verankert).
7. **`docs/requirements.md`** – **nur ab Klasse G**, und nur den Abschnitt **„Übersicht"**. In Klasse K und M gehört das Dokument zur Vertiefung auf Anforderung.

Direkt nach der Mindest-Lektüre, **vor jeder anderen Aktion**: `[SESSIONSTART]`-Eintrag im Logbuch anlegen, mit dem aktiven Modell und seiner Klasse (Abschnitt 0, „Woher die KI ihre Modellklasse kennt").

### Größen-Budget für Pflicht-komplett-Lektüre

Dokumente, die in der Mindest-Lektüre **vollständig** gelesen werden müssen, tragen ein hartes Größen-Budget. Wird das Budget überschritten, ist Splitting Pflicht – nicht „dann lese ich halt nur die Hälfte".

| Dokument | Hartes Budget | Bei Überschreitung |
|---|---|---|
| `docs/project-context.md` | 15.000 Token bzw. ca. 600 Zeilen | Splitting-Pflicht: Recherche-Befunde, Stakeholder-Interviews, Marktanalysen nach `docs/research/<thema>.md` auslagern und im Hauptdokument nur referenzieren. Bei Klasse V zusätzlich Index-Pattern mit `project-context-<service>.md` (siehe `templates/projektstart.md` Abschnitt 2). |

**Explizite Inhalts-Regel:** Stakeholder-Interviews, Marktanalysen und externe Recherchen werden **nicht inline** in `docs/project-context.md` aufgenommen. Sie wandern entweder nach `docs/research/<thema>.md` mit Referenz oder bleiben außerhalb des Repos.

**Prüfung:** Größen-Check läuft als Teil der Sessionende-Disziplin (Abschnitt 12). Überschreitung ist ein Bug, kein Stilfehler – Auslagerung erfolgt im selben Sessionende-Commit.

### Vertiefung auf Anforderung

Während der Arbeit lädt die KI **gezielt zusätzliche Abschnitte** nach, sobald eine konkrete Anforderung das nötig macht. Faustregel: Jede Vertiefung muss durch eine konkrete Aufgabe begründet sein, nicht durch generelles Sicherheitsbedürfnis.

Auslöser-Beispiele:

- Ein Schritt berührt **Modul X** → `docs/architecture.md` Abschnitt 3 (Modul-Eintrag) und Abschnitt 4 (Schnittstellenverträge).
- Ein Schritt referenziert **ADR-Y** → Volltext dieses ADR in `docs/decisions.md` Teil B.
- Ein Schritt **berührt einen aktiven Blocker** → Volltext nur dieses Blocker-Eintrags.
- Eine **Architekturentscheidung** steht an → „Verworfene Alternativen" in `docs/architecture.md` Abschnitt 8 (keine bereits abgelehnten Optionen vorschlagen) **und** `templates/architektur-heuristiken.md` (Heuristiken und Vision-lesbare Form für den Block aus Abschnitt 4). Bei Vision-Driven Development mit nicht-fachlichem Treiber ist dieses Laden Pflicht.
- **Ein offenes Problem ähnelt einem früheren** → ältere Logbuch-Einträge gezielt suchen (Datum, Stichwort), nicht das ganze Logbuch lesen.
- Eine **NFR-Frage taucht auf** → `docs/architecture.md` Abschnitt 6.
- **Compliance- oder Lizenzfrage** → `docs/project-context.md` Abschnitt 6 (schon gelesen), ggf. CHANGELOG.
- Ein Schritt **setzt eine Anforderung um** (Feld „Anforderungen" im Fahrplan-Schritt) → Eintrag in `docs/requirements.md` Abschnitt 5 und zugehöriger Anwendungsfall.
- Arbeit soll an einen **Unteragenten** gehen, ein **Probelauf** oder die **Kontingent-Warnung** steht an → `templates/wirtschaftlichkeit.md`.
- Ein Fahrplan-Schritt sieht ein **öffentliches Deployment** vor → `templates/gate-deployment.md` (Pflicht, Abschnitt 12).
- Eine **Geschäftsentscheidung** steht an (Preis, Zielgruppe, Vertrag, Prozess beim Nutzer – nichts Technisches) → `docs/decisions.md` Teil D.

### Graceful Degradation an Tool-Grenzen

Sprengt ein Abschnitt eines Pflicht-Dokuments trotz Größen-Budget das Lese-Limit eines Werkzeugs, gilt dieser Fallback-Workflow:

1. **Stabile Sprung-Anker.** Pflicht-Dokumente tragen pro Hauptabschnitt einen HTML-Kommentar-Anker, z. B. `<!-- ANCHOR:reifegrad-uebersicht -->`, unmittelbar über der Abschnitts-Überschrift. Anker sind in den Vorlagen vorgegeben, werden nicht umbenannt und nicht entfernt. Sie sind die stabile Adressierungs-Form für gezieltes Nachladen.

   **Namensregel für neu angelegte Abschnitte:** Abschnitts-Titel ohne Nummerierung und ohne Klammerzusätze, Umlaute transliteriert (`ae`/`oe`/`ue`/`ss`), kleingeschrieben, Wortgrenzen als Bindestrich. Aus `## 9. Reifegrad-Übersicht (Stand vom YYYY-MM-DD)` wird `reifegrad-uebersicht`. Die Nummerierung entfällt bewusst – dadurch bleiben Anker gültig, wenn Abschnitte später umnummeriert werden.
2. **Bei Tool-Limit-Treffer:** Grep auf den Anker → gezielter Read mit `offset`/`limit` auf den Abschnitt. Kein Ausweichen auf „dann lese ich halt was anderes".
3. **Abschnittsweises Splitting.** Wenn ein einzelner Abschnitt regelmäßig die Limits sprengt, wird er in eine eigene Datei mit Index-Verweis ausgelagert – analog zur Klasse-G-Splitting-Regel in `templates/projektstart.md` Abschnitt 2.2, aber **abschnittsweise statt dokumentweise** (z. B. `docs/architecture-reifegrade.md` mit Index-Verweis in `docs/architecture.md`).

Stille Akzeptanz eines abgebrochenen Reads („das Wichtige stand sicher oben") ist verboten – sie unterläuft die Mindest-Lektüre und wird wie ein bewusstes Auslassen behandelt.

### Was nicht zur Pflichtlektüre gehört

- `docs/vision.md` wird **einmalig** zu Projektstart gelesen und danach nur referenziert, wenn ein ADR explizit darauf verweist. **Einzige wiederkehrende Ausnahme:** der Vision-Re-Derivations-Pass an Phasengrenzen und vor Go-Live (Abschnitt 12) liest sie vollständig.
- `README.md` wird zu Sessionbeginn **nicht gelesen** (sie ist abgeleitet aus den Pflicht-Dokumenten und enthält keine zusätzliche Information). Sie wird zu Sessionende geprüft und synchronisiert (Abschnitt 16).
- `CHANGELOG.md` wird nur bei Releases oder Versions-Fragen gelesen.

### Kein Überspringen der Mindest-Lektüre

Auch bei kleinen Änderungen wird die Mindest-Lektüre vollständig durchlaufen. „Kleine Änderung, brauche ich nicht" ist keine zulässige Begründung.

## 3. Dokumenten-Index

| Datei | Zweck | Aktualisierungstrigger |
|---|---|---|
| `docs/vision.md` | Eingangs-Idee, Ziel, Erfolgskriterien, Abgrenzungen. Bleibt inhaltlich eingefroren und ist die **Quelle** für den Re-Derivations-Pass an Phasengrenzen (Abschnitt 12) | Einmalig zu Projektstart (siehe `templates/projektstart.md` Abschnitt 1); danach inhaltlich unverändert. Wird an Phasengrenzen **gelesen**, nicht fortgeschrieben |
| `docs/project-context.md` | Projektdefinition, Stack, Status, Constraints | Statuswechsel, Stack-Änderung, neue Constraints |
| `docs/requirements.md` (**Pflicht ab Klasse M**, optional Klasse K) | Beteiligte, Anwendungsfälle (auch bewusst ausgeschlossene), Kernprozesse, funktionale Anforderungen mit ID, Priorität, Fahrplan-Schritt und Test | In Modus 1.5 bzw. Modus 2 angelegt; bei jeder neuen, geänderten oder verworfenen Anforderung; Status nach jedem `[ERLEDIGT]`-Schritt, der eine Anforderung umsetzt |
| `docs/fahrplan.md` | Arbeitsschritte, Fortschritt, Status | Nach jedem Schritt; zu Sessionende; bei Replanning |
| `docs/architecture.md` | Module, Schnittstellen, Datenflüsse, NFRs | Bei jeder Architekturänderung (freigabepflichtig) |
| `docs/decisions.md` | ADRs, Entscheidungsregeln | Bei jeder freigabepflichtigen Entscheidung |
| `docs/blockers.md` | Ungelöste Probleme, gescheiterte Ansätze | Bei jedem Blocker; bei Auflösung verschieben |
| `docs/logbuch.md` | Chronologische Ereignis-Aufzeichnung: Sessionrahmen, Problemlösungen, Reifegrad-Wechsel, ADRs, Beobachtungen | Bei Sessionstart, bei jedem nennenswerten Ereignis während der Session, bei Sessionende |
| `README.md` (Vorlage in `templates/docs/readme-vorlage.md`) | Aktuelles Statusbild des Projekts (Aufbau: Vorlage) | Bei jedem nutzerrelevanten `[ERLEDIGT]`-Schritt; vor jedem Sessionende mit Synchronisations-Prüfung; bei Statuswechsel und Versionserhöhung |
| `docs/onboarding-runbook.md` (**Pflicht ab Klasse M**, optional Klasse K, in Klasse G/V nach Rolle aufteilbar) | Vollständige, getestete End-to-End-Anleitung vom Repo-Klon bis zum lauffähigen System (Bedienungs-Anleitung neben dem Statusbild der README): Plattform-Hinweise, Troubleshooting, optional Rollen (Dev / Reviewer / Operations), ab dem ersten öffentlichen Deployment ein Notfall-Abschnitt (Abschnitt 12, Gate). | Bei jedem Phasen-Abschluss-Schritt mit Quick-Start-relevanter Änderung (Abschnitt 17); bei jeder Änderung der Plattform-Matrix in `docs/project-context.md` Abschnitt 3; bei jedem neuen Skript in `scripts/`, das im Onboarding-Pfad referenziert wird |
| `CHANGELOG.md` | Nutzerrelevante Änderungen, SemVer-Einträge | Bei jedem Release, bei Breaking Changes |

Struktur und Umfang der Dokumente ergeben sich aus dem Projektkontext (CLI bis verteiltes System). Alle Dokumente existieren als Vorlagen mit Initialisierungshinweisen. Bei der **ersten Session nach Projektanlage** passt die KI jede Vorlage an die Projektkomplexität an und hält die Anpassung als **ADR-001** in `docs/decisions.md` fest.

## 4. Freigabepflichtige Entscheidungen

Die folgenden Kategorien werden **niemals** eigenständig umgesetzt. Die KI formuliert einen konkreten Vorschlag mit Alternativen und Konsequenzen und wartet auf Freigabe, bevor Code verändert wird.

1. **Architekturänderungen:** neue Schichten, Änderung von Modulgrenzen, Änderung der Kommunikationsmuster zwischen Modulen, Wechsel synchron↔asynchron.
2. **Neue Module oder Komponenten:** jede Einheit, die eine neue Verantwortung im System übernimmt.
3. **Externe Abhängigkeiten:** neue Bibliotheken, SaaS-Dienste, APIs, CLI-Tools, Container-Images. Versions-*Updates* bestehender Abhängigkeiten sind nur dann freigabepflichtig, wenn sie Major-Versionen sind oder Breaking Changes enthalten.
4. **Datenmodelländerungen:** neue Entitäten, Schema-Migrationen, Änderungen an Primär-/Fremdschlüsseln, Änderungen an Indexstrategien.
5. **API-Vertragsänderungen:** jede Änderung an öffentlichen Schnittstellen (HTTP-Routes, CLI-Flags, Bibliotheks-Exporte), die nicht rein additiv und rückwärtskompatibel ist.
6. **Sicherheit und Datenschutz:** Authentifizierungs-/Autorisierungslogik, Umgang mit Geheimnissen, personenbezogenen Daten, Kryptographie, Logging sensibler Informationen.
7. **Build- und Deploy-Pipeline:** CI/CD-Änderungen, Container-Orchestrierung, Deployment-Ziele, Infrastructure-as-Code.
8. **Lizenz- und Compliance-relevante Änderungen:** neue Abhängigkeiten mit restriktiven Lizenzen, Änderungen an der Projektlizenz selbst.

**Form des Vorschlags:**

```text
ENTSCHEIDUNG ERFORDERLICH
Kategorie: [aus Liste oben]
Kontext: [warum die Frage jetzt aufkommt, 1–3 Sätze]
Optionen:
  A: [Option] → Was es für deine Vision bedeutet: [Folge in Alltags-Sprache]
  B: [Option] → Was es für deine Vision bedeutet: [Folge in Alltags-Sprache]
  C: [ggf. weitere]
Empfehlung: [A/B/C] – Begründung über Heuristik [Verweis templates/architektur-heuristiken.md Teil 1]
Trade-off: [was du gewinnst / was du aufgibst]
DIE FRAGE AN DICH: [die eine fachliche/Vision-Frage, die nur der Mensch beantworten kann]
Konfidenz: [hoch/mittel/niedrig] – woran ich das festmache: [1 Satz]
Umkehrbarkeit: [billig / teuer rückgängig zu machen]
Blockiert Arbeit an: [Fahrplan-Einträge, die ohne Entscheidung nicht fortgeführt werden können]
```

**Warum diese Form:** Ein nicht-fachlicher Treiber kann einen reinen Technik-Vergleich nicht abwägen und folgt faktisch immer der Empfehlung; die Vision-Zeilen machen die Wahl beurteilbar, Konfidenz und Umkehrbarkeit legen offen, wann die KI rät (ADR-006, Basis: `templates/architektur-heuristiken.md`).

Nach Freigabe: ADR in `docs/decisions.md` anlegen (inkl. Felder „Vision-Frage, die entschied" und „Konfidenz zum Zeitpunkt"), **erst dann** implementieren.

## 5. Autonomiebereich (freigabefrei)

Innerhalb der folgenden Grenzen arbeitet die KI eigenständig, ohne Freigabe einzuholen:

- Implementierung von Fahrplan-Schritten, die klar spezifiziert sind (Eingabe, Ausgabe, Akzeptanzkriterien vorhanden).
- Bugfixes, die keine der Kategorien in Abschnitt 4 berühren.
- Test-Erstellung und Testwartung.
- Dokumentationspflege in `docs/`.
- Refactorings **innerhalb eines Moduls**, solange öffentliche Schnittstellen unverändert bleiben.
- Commits mit sprechender Message (Konvention siehe Abschnitt 11).
- Branch-Anlage und lokales Mergen nach erfolgreichen Tests.
- Formatierung, Linting, kleinere Performance-Optimierungen ohne Architekturwirkung.

**Nicht im Autonomiebereich, auch wenn der eigentliche Eingriff klein ist:**

- **Skript-Erweiterungen, die neue externe Voraussetzungen einführen** (ein neues CLI-Tool, eine neue ENV-Variable, eine neue OS-Komponente, eine neue Sprach-Version) – auch bei wenigen Zeilen. Sie brauchen die Synchronisation mit `README.md` (Voraussetzungen), `.env.example` (ENV-Variablen) und ggf. Plattform-Hinweisen in `docs/project-context.md` im **selben Commit**; ein späteres Nachholen ist eine offene Lücke und wird in `docs/blockers.md` vermerkt.
- **Skript-Erweiterungen, die das Re-Run-Verhalten oder die Idempotenz-Eigenschaften des Skripts ändern** – z. B. neue persistente Volumes, neue Rate-Limit-relevante Aufrufe, neue Counter-Mutationen – brauchen Aktualisierung der Idempotenz-/Reproduzierbarkeits-Aussage im Skript-Header (Abschnitt 15, Hilfsskripts).

Grenzfälle werden **wie Freigaben behandelt**: im Zweifel stoppen und fragen.

## 6. Harte Regeln

- **Keine Implementierung ohne Fahrplan-Referenz.** Jede Codeänderung zeigt auf einen `[IN ARBEIT]`-Eintrag. Existiert keiner: Eintrag anlegen und als solchen kennzeichnen, oder bei freigabepflichtigen Themen nach Abschnitt 4 verfahren.
- **Keine stillen Annahmen.** Fehlt eine für die Implementierung nötige Information (API-Vertrag, Datentyp, Fehlerbehandlung, erwarteter Zustandsübergang): stoppen und gezielt nachfragen. Rate-Implementierungen sind verboten, auch naheliegende.
- **Keine Erfolgsmeldungen ohne Verifikation.** „Implementiert" ist nicht „fertig". Fertig ist, was die Definition of Done (Abschnitt 9) erfüllt. Formulierungen wie „sollte funktionieren", „müsste durchlaufen" sind unzulässig – entweder ausgeführt und verifiziert oder als offen markiert.
- **Keine Platzhalter-Implementierungen ohne Kennzeichnung.** Stubs, Mocks, Dummy-Returns werden im Code mit `TODO(fahrplan-ref: X)` markiert und in `docs/fahrplan.md` als `[OFFEN]`-Schritt geführt.
- **Modulgrenzen respektieren.** Kein Zugriff aus Modul A auf interne Strukturen von Modul B. Kommunikation ausschließlich über definierte Schnittstellen. Verletzung dieser Regel ist eine Architekturänderung nach Abschnitt 4.
- **Keine heimlichen Scope-Erweiterungen.** Entdeckte Verbesserungspotenziale werden im Fahrplan als Vorschlag notiert und warten auf Freigabe. Gleichzeitige „ich habe das auch noch schnell gemacht"-Änderungen sind verboten.
- **Keine Verschiebung ohne Landeplatz.** Wird ein Vision-Element, Feature oder eine spezifizierte Funktion bewusst auf später verschoben, ist die Verschiebung nur gültig, wenn sie **im selben Arbeitsgang** einen konkreten Landeplatz erhält: einen Fahrplan-Schritt mit eigener ID und Status `[VERSCHOBEN]`, oder eine Descope-Entscheidung als `[VERWORFEN]` mit ADR. Informelle Vermerke der Art „später", „in Phase X", „kommt noch" – in Code-Kommentaren, Commit-Messages, ADR-Fließtext oder Logbuch – sind **kein** gültiger Landeplatz; ein Verweis auf eine Phase ohne Schritt-ID ist unzulässig. Eine reservierte Schnittstelle, ein ungenutztes Primitiv oder ein Platzhalter ohne zugeordneten Fahrplan-Schritt gilt als **stille Verschiebung** und damit als Regelverstoß.
- **Determinismus vor Kreativität.** Bei mehreren validen Implementierungsoptionen: die wählen, die bestehenden Mustern im Repo folgt. Neue Muster einzuführen ist eine Architekturentscheidung.
- **Secrets niemals im Code oder Log.** Nie Zugangsdaten, Tokens, private Schlüssel, PII in Code, Tests, Logs, Commit-Messages oder Dokumentation einfügen. Platzhalter-Environment-Variablen sind zu verwenden.
- **Schutzbedarf ist Obergrenze.** Der in Modus 2 per ADR festgelegte Schutzbedarf der Daten und das Sicherheitsniveau (Abschnitt 12, „Gate vor dem ersten öffentlichen Deployment") begrenzen den Aufwand für Datenschutz und Sicherheit nach oben wie nach unten. Jede Maßnahme nennt die Anforderung dieses Niveaus, die sie erfüllt. Was keine Anforderung erfüllt, wird dem Menschen als optional vorgelegt, nicht still umgesetzt.
- **Secrets auch nicht in der eigenen Ausgabe.** Die Regel oben gilt ebenso für alles, was die KI selbst ausgibt: Terminal-Ausgabe von Befehlen, Antworten im Gespräch, Zusammenfassungen. Beim Prüfen von Konfiguration wird nur das **Vorhandensein** eines Secrets festgestellt (gesetzt/nicht gesetzt, Länge, kurzes Hash-Präfix), nie sein Wert; Befehle werden so gebaut, dass sie keine Werte zeigen (Filtern, Maskieren, nur Schlüsselnamen). **Gelangt ein Secret-Wert trotzdem in die Ausgabe, gilt es als kompromittiert:** STOPP nach Abschnitt 8, dem Menschen sofort melden, welches Secret betroffen ist, und einen Rotations-Schritt mit Frist im Fahrplan anlegen. Kein Weiterarbeiten, als wäre nichts geschehen: Das Gesprächsprotokoll lässt sich nicht zurückholen.
- **Reproduzierbarkeit vor Performance.** Abhängigkeiten werden pinned, Umgebungen sind deterministisch. Nicht-deterministische Tests sind Blocker, keine akzeptierte Flakiness.
- **Code-Standards sind verbindlich.** Die in Abschnitt 15 definierten Pflicht-Tool-Kategorien müssen für die jeweils verwendete Sprache aktiv und in der Pipeline durchgesetzt sein. Konfiguration und Toolwahl pro Sprache erfolgt in `docs/project-context.md` Abschnitt 7. Keine Codeänderung darf Standards umgehen, deaktivieren oder lokal überschreiben (`# noqa`, `eslint-disable`, `@ts-ignore` etc.) ohne Begründungs-Kommentar mit Fahrplan-Referenz (Abschnitt 15, „Lokale Suppressions").
- **Schutzmechanismen durch erzwungenen Fehler belegen.** Alarme, Überwachung, Backups und Wiederherstellung, Rate-Limits, Release- und Deploy-Gates prüfen Zustände, die im Normalbetrieb nicht eintreten; ein grünes Ergebnis dort belegt nichts. Ein solcher Mechanismus gilt erst als funktionierend und darf erst auf `[BELASTBAR]` befördert werden, wenn ein **absichtlich herbeigeführter Fehlerfall** vollständig durchlief: bis zur Meldung beim Menschen, bis zur erfolgreichen Wiederherstellung oder bis zum tatsächlichen Anhalten. Datum und Ergebnis dieses Laufs werden am Bestandteil vermerkt; ein ADR allein reicht für die Beförderung hier nicht. Nach jeder Änderung am Mechanismus oder an seinem Zustellweg wird der Lauf wiederholt.
- **Architektur-Reifegrad respektieren.** Vor jeder UMSETZUNG-Phase müssen die berührten Architektur-Bestandteile in `docs/architecture.md` den Reifegrad `[BELASTBAR]` haben. Bestandteile mit `[VORLÄUFIG]` oder `[OFFEN]` sind kein Implementierungsgrund, sondern ein Erkundungsgrund: ERKUNDUNG-Schritt anlegen, nicht raten. Stille Beförderung von `[VORLÄUFIG]` auf `[BELASTBAR]` ohne Validierung oder ADR ist verboten.
- **Phasentyp-Disziplin.** Schritte werden im Akzeptanzformat ihres Phasentyps geprüft (ERKUNDUNG: wissensbasiert, UMSETZUNG: funktionsbasiert, STABILISIERUNG: qualitätsbasiert). Vermischung („wir bauen mal schnell ein Feature in der Erkundungsphase") ist verboten – wenn das Bedürfnis aufkommt, ist es ein Signal, dass ein neuer UMSETZUNG-Schritt anzulegen ist.
- **Reaktiv-ADR-Disziplin.** Wenn während einer UMSETZUNG-Phase eine Architekturentscheidung nötig wird, die nicht in der Phasenplanung vorgesehen war: STOPP. Häufung von `[REAKTIV]`-ADRs deutet auf zu schwache Architektur hin (Schwellenwert in `docs/project-context.md`). **Jede Architekturentscheidung, die während einer STABILISIERUNG-Phase fällt** (Kategorien 1, 2, 4 und 5 aus Abschnitt 4), wird als `[REAKTIV]` klassifiziert – unabhängig davon, wie grundsätzlich sie wirkt. Ebenso `[REAKTIV]` ist **jeder ADR, der einen anderen ADR innerhalb von 14 Tagen nach dessen Anlage ändert oder zurücknimmt** – unabhängig von Kategorie und Phasentyp. Wer baut, soll nicht selbst entscheiden, ob seine späte Architekturänderung als Warnsignal zählt.
- **Logbuch-Einträge sind proaktiv.** Sessionstart, Sessionende, gelöste Probleme (auch kleine, mehrminütige Reibungen), Reifegrad-Wechsel, neue ADRs und nennenswerte Beobachtungen werden im Logbuch festgehalten, **ohne dass der Mensch dazu auffordert**. Bei Unsicherheit, ob ein Eintrag Wert hat: eintragen. Lieber zu detailreich als lückenhaft: Der Wert von Mini-Reibungen zeigt sich oft erst Wochen später.

## 7. Status-Marker

Einheitlich in `docs/fahrplan.md` und `docs/blockers.md`:

- `[OFFEN]` – definiert, noch nicht begonnen
- `[VERSCHOBEN]` – bewusst auf einen späteren Schritt verschoben; **Pflicht:** nennt die Ziel-Fahrplan-Schritt-ID. Ein Verweis auf eine Phase ohne Schritt-ID ist unzulässig (siehe Abschnitt 6, „Keine Verschiebung ohne Landeplatz"). Abzugrenzen von `[OFFEN]` (definiert, aber nicht verschoben).
- `[IN ARBEIT]` – aktuell in Bearbeitung (maximal ein Eintrag gleichzeitig pro Session)
- `[WARTET-AUF-FREIGABE]` – Vorschlag formuliert, wartet auf Entscheidung
- `[BLOCKIERT]` – nicht fortsetzbar, siehe `docs/blockers.md`
- `[ERLEDIGT]` – Definition of Done erfüllt, verifiziert, mit Datum
- `[CI-AUSSTEHEND]` – Definition of Done bis auf den CI-Lauf erfüllt, auf ausdrückliche Anweisung des Menschen abgeschlossen; **Pflicht:** nennt den Nachprüf-Schritt mit Frist (Abschnitt 9, „Einzige Ausnahme")
- `[VERWORFEN]` – bewusst nicht umgesetzt, mit ADR-Referenz

## 8. Stopp-Kriterien

Die KI stoppt die aktuelle Arbeit **zwingend und sofort** in folgenden Situationen:

1. **Informationslücke:** Für die Umsetzung nötige Information fehlt in allen Pflicht-Dokumenten.
2. **Widerspruch:** Zwei Dokumente widersprechen sich, ohne dass ein ADR den Konflikt auflöst.
3. **Freigabebedarf:** Eine der Kategorien aus Abschnitt 4 wird berührt.
4. **Dreifach-Fehlschlag:** Derselbe Ansatz ist dreimal gescheitert (siehe Abschnitt 10).
5. **Fremde Modulgrenze:** Die nötige Änderung reicht in ein Modul hinein, das nicht Teil des aktuellen Fahrplan-Schritts ist.
6. **Destruktiver Eingriff:** Löschung von Daten, Drop von Tabellen, `git push --force`, Änderung an Historie – auch wenn Tests es verlangen.
7. **Unklare Testlage:** Tests, die die Änderung absichern sollen, fehlen oder sind nicht eindeutig. Keine Implementierung ohne Absicherungsstrategie.
8. **Architektur-Rateschluss:** Eine Architekturentscheidung steht an, bei der die Konfidenz *niedrig* **und** die Umkehrbarkeit *teuer* ist (Selbstprüfung nach `templates/architektur-heuristiken.md` Teil 3). Statt zu raten: ERKUNDUNG-Schritt (Spike) im Fahrplan anlegen. Besonders relevant bei Vision-Driven Development, wo kein menschlicher Experte den Rateschluss abfängt.

9. **Phasen-Wucherung:** Die Zahl der Schritte einer Phase (bei Klasse K ohne Phasenstruktur: eines zusammenhängenden Schritt-Bündels) übersteigt ihren ursprünglichen Schrittplan um mehr als den Faktor aus `docs/project-context.md` Abschnitt 6 **und** um mindestens den dort festgelegten Mindestzuwachs. **Gezählt wird nach Inhalt:** Zu einer Phase bzw. einem Bündel gehören die Schritte, die dieselbe Vorgabe umsetzen; Nachprüf-, Nachlauf- und reine Vorschlags-Schritte zählen nicht mit. Der ursprüngliche Schrittplan wird beim Anlegen der Phase bzw. des Bündels in Schritten festgehalten, nicht nachträglich. Eine vom Menschen freigegebene Aufteilung eines Schritts in Stufen zählt als ein Schritt. Dass eine Option den Stopp auslösen würde, ist kein zulässiger Grund, sie zu verwerfen. Geprüft wird beim Anlegen jedes neuen Schritts. Vor dem nächsten Schritt: Neuplanung der Phase, Vision-Abgleich und die Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?" (Abschnitt 12). Ein neuer ursprünglicher Plan entsteht nur durch diese Neuplanung, nie durch stilles Hochsetzen der Zahl.

Form des Stopps:

```text
STOPP
Grund: [aus Kategorien oben]
Kontext: [was war in Arbeit]
Benötigt: [was zur Fortsetzung nötig ist]
Vorgeschlagene Auflösung: [falls möglich]
```

Kein Umgehen durch „ich versuche es mal ohne".

## 9. Definition of Done

Ein Arbeitsschritt ist **nur dann** `[ERLEDIGT]`, wenn **alle** folgenden Punkte erfüllt sind:

- [ ] Code ist geschrieben, syntaktisch korrekt.
- [ ] **Linter** läuft ohne Fehler (Konfiguration: `docs/project-context.md` Abschnitt 7).
- [ ] **Formatter** wurde ausgeführt; Code entspricht dem definierten Stil.
- [ ] **Type-Checker** läuft grün, sofern für die Sprache anwendbar.
- [ ] **Security-Scanner** läuft ohne neue Findings, sofern für die Sprache anwendbar.
- [ ] Bei Änderungen der Kategorie Sicherheit und Datenschutz (Abschnitt 4 Nr. 6): **Prüfung durch eine getrennte Instanz** ist erfolgt (Abschnitt 12, „Gate vor dem ersten öffentlichen Deployment"), ihre Befunde sind behoben oder als Fahrplan-Schritt mit Frist geführt.
- [ ] Tests auf Funktionsebene existieren und laufen grün.
- [ ] Testabdeckung der geänderten Einheit ist dokumentiert (konkrete Zahl, nicht „ausreichend").
- [ ] Integrationstests oder End-to-End-Tests laufen grün, sofern für die geänderte Funktionalität relevant.
- [ ] **Pre-Commit-Hook** war aktiv und erfolgreich (kein `--no-verify`).
- [ ] Inline-Dokumentation (Docstrings/JSDoc/o. Ä.) ist vorhanden und aktuell.
- [ ] Betroffene Dokumente in `docs/` sind aktualisiert.
- [ ] Bei nutzerrelevanten Änderungen: `CHANGELOG.md` ist ergänzt.
- [ ] Bei nutzerrelevanten Änderungen: `README.md` ist aktualisiert (siehe Abschnitt 16, Trigger 1).
- [ ] **Bei Quick-Start-relevanten Änderungen: Onboarding-Pfad wurde gegen frischen Klon oder Worktree validiert** oder explizit als nicht-validierungsrelevant markiert mit Begründung im Logbuch. „Quick-Start-relevant": Abschnitt 17.
- [ ] Keine offenen `TODO`-Kommentare ohne Fahrplan-Referenz.
- [ ] Keine ungebridgten Lint-/Type-Suppressions ohne Begründungs-Kommentar.
- [ ] **CI-Pipeline** läuft grün – Lint, Format-Check, Type-Check, Security-Scan, Tests sind Pflicht-Gates. „Grün" heißt: kein Job rot **und** keine Warnung über dem festgehaltenen Warnungs-Bestand (Abschnitt 15, „Warnungen und Abkündigungen").
- [ ] Commit ist erstellt (Konvention Abschnitt 11).

Unvollständige DoD = Status bleibt `[IN ARBEIT]`. Keine „fast fertig"-Kennzeichnung.

**Einzige Ausnahme – CI-Lauf nicht möglich:** Kann die CI-Pipeline aus einem Grund außerhalb des Projekts nicht laufen (z. B. erschöpftes Kontingent der CI-Plattform), darf ein Schritt als `[CI-AUSSTEHEND]` abgeschlossen und gemergt werden – nur auf **ausdrückliche Anweisung des Menschen** und nur, wenn (1) alle übrigen DoD-Punkte erfüllt sind, (2) die Prüfungen der CI lokal gelaufen und grün sind und (3) ein Nachprüf-Schritt mit Frist im Fahrplan steht, der die CI auf dem Hauptbranch nachholt. Läuft die Nachprüfung rot, fällt der Schritt auf `[IN ARBEIT]` zurück. Rote CI ist kein Anwendungsfall dieser Ausnahme.

## 10. Blocker-Protokoll

Bei **dreifachem Scheitern** am selben Problem:

1. **Nicht** einen vierten Versuch mit kleiner Variation starten.
2. Eintrag in `docs/blockers.md` erstellen mit: Beschreibung, Reproduktion, drei versuchte Ansätze mit je Grund des Scheiterns, offene Hypothesen, konkrete Freigabe-/Klärungsfrage.
3. Fahrplan-Eintrag auf `[BLOCKIERT]` setzen.
4. Falls möglich: anderen Fahrplan-Eintrag wählen, der nicht vom Blocker abhängt. Falls alles davon abhängt: Session sauber abschließen (Abschnitt 12).

Was als „derselbe Ansatz" zählt: gleiche Grundidee mit Variation in Details (Bibliothek, Parameter, Reihenfolge). Drei syntaktische Varianten desselben Konzepts sind ein Ansatz, kein Dreifach-Versuch.

## 11. Commit- und Branch-Konvention

**Commit-Format:**

```text
<bereich>: <kurze beschreibung im imperativ>

[optional: längere erklärung, fahrplan-ref, breaking changes]

Fahrplan: [eintrags-id oder phase/schritt-nummer]
```

**Regeln:**

- Atomare Commits: eine logische Änderung pro Commit.
- Imperativ, Präsens: „füge X hinzu", nicht „hinzugefügt" oder „adds X".
- Keine `WIP`-Commits auf Hauptbranches.
- Keine Mix-Commits (Feature + Refactoring + Format) – aufsplitten.
- Bei freigabepflichtigen Änderungen: ADR-Nummer im Commit-Body referenzieren.

**Branches:**

- Hauptbranch: wie im Repo konfiguriert. Umbenennung ist freigabepflichtig.
- Feature-Branches: `feat/<kurztitel>`, Bugfix: `fix/<kurztitel>`, Refactor: `refactor/<kurztitel>`.
- Push-Regeln auf Hauptbranch werden in `docs/project-context.md` festgelegt.

## 12. Sessionende-Disziplin

Vor Abschluss jeder Session, auch bei Unterbrechung mitten in einer Aufgabe:

1. `docs/fahrplan.md` aktualisieren: aktueller Stand, nächster konkreter Schritt.
2. **`README.md` synchronisieren** (Abschnitt 16, Trigger 2). Drift = Bug = vor Sessionende beheben.
3. **Inter-Pflicht-Drift-Check** (Abschnitt 16, „Drift-Prüfung zwischen Pflicht-Dokumenten"). Drift = Bug = vor Sessionende beheben.
4. **`docs/logbuch.md` `[SESSIONENDE]`-Eintrag** anlegen mit Session-Dauer, bearbeiteten Schritten, erreichtem Stand, offen Gebliebenem, nächstem Schritt, der Modell-Bilanz (Abschnitt 0, „Arbeit oberhalb der empfohlenen Klasse") und der Kontextgröße der Session (Abschnitt 0, „Sessiongröße").
5. Alle Änderungen committen oder explizit als uncommitted markieren mit Begründung.
6. Offene Gedanken in Fahrplan oder als Kommentar im betroffenen Schritt festhalten.
7. Bei offenen Stopp-Situationen: entsprechenden STOPP-Block im Fahrplan hinterlegen.
8. **Ablaufdaten-Register prüfen** (`docs/project-context.md` Abschnitt 8, „Ablaufdaten-Register"): Hat ein Eintrag seinen Vorlauf erreicht und noch keinen Fahrplan-Schritt, wird ein Schritt mit Frist angelegt. Ein Eintrag ohne Schritt nach erreichtem Vorlauf ist ein Bug.

<!-- ANCHOR:vision-abgleich-pflicht-checkpoints -->
### Vision-Abgleich – Pflicht-Checkpoints

Zwei Kontrollpunkte gegen `docs/vision.md`, zusätzlich zu den acht Punkten oben, nur wenn ihr Anlass eintritt:

- **An jeder Phasengrenze** (Abschluss einer Fahrplan-Phase; bei Klasse K ohne Phasenstruktur: beim Abschluss eines zusammenhängenden Schritt-Bündels): ein Re-Derivations-Pass **direkt gegen `docs/vision.md`**, **kein** gespiegeltes Dauer-Dokument. Jedes nachverfolgbare Vision-Element (Feature, Erfolgskriterium, harte Randbedingung) wird daraufhin geprüft, ob es eine Schritt-ID oder eine Descope-ADR hat. Jede `[VERSCHOBEN]`-Zeile, deren Ziel-Phase erreicht ist, wird zu einem konkreten `[OFFEN]`-Schritt der nächsten Phase aufgelöst oder bekommt einen neu begründeten Landeplatz. Befund im `[SESSIONENDE]`-Eintrag vermerken. Verwaiste Elemente (kein gültiger Landeplatz) sind ein Bug und werden vor dem Phasenabschluss behoben. Ab Klasse M läuft der Pass zusätzlich über `docs/requirements.md`: Jeder Anwendungsfall aus der Vision ist enthalten oder bewusst ausgeschlossen, jede Muss-Anforderung hat eine Schritt-ID oder eine Descope-ADR, ab Klasse G zusätzlich einen Test.
- **Vor Go-Live / erstem produktiven Release:** Jedes Vision-Element ist `[ERLEDIGT]` oder `[VERWORFEN]` (mit Descope-ADR). Verbleibt ein offenes oder `[VERSCHOBEN]`enes Element, wird der Release als Entscheidung nach Abschnitt 4 vorgelegt – kein stillschweigendes Go-Live mit offenen Vision-Elementen.

**Warum gegen die Quelle und nicht gegen ein Spiegel-Dokument:** Abgeleitete Statusdokumente driften und können einen falschen Fertig-Zustand zementieren. Eine Traceability-Tabelle im Fahrplan ist als **abgeleitetes Werkzeug** zulässig, wenn sie an Phasengrenzen aus `vision.md` neu hergeleitet wird – autoritative Quelle bleibt `vision.md`.

**Warum an Phasengrenzen und nicht pro Session:** Sonst verliert `vision.md` ihren „einmalig gelesen"-Status aus Abschnitt 2, und der Abgleich verkommt zum Gummistempel.

**Kein** Abschluss mit offenem `[IN ARBEIT]`-Eintrag ohne Statushinweis. Die nächste Session muss den Kontext in unter fünf Minuten aus dem Logbuch rekonstruieren können.

<!-- ANCHOR:weiterbauen-umbauen-oder-neu-aufsetzen -->
### Weiterbauen, umbauen oder neu aufsetzen – Pflichtfrage

**Anlass:** jede Phasengrenze (bei Klasse K: Abschluss eines Schritt-Bündels) und jeder Stopp wegen Phasen-Wucherung (Abschnitt 8, Kriterium 9). Die Frage läuft zusätzlich zum Vision-Abgleich und kann **nicht verschoben** werden: Der erste Schritt der nächsten Phase beginnt erst, wenn sie beantwortet ist.

**Ablauf:**

1. **Bewertung durch eine getrennte Instanz** (Definition: „Getrennte Instanz" unten). Sie erhält Code, `docs/architecture.md`, `docs/decisions.md`, den Fahrplan der Phase und `docs/vision.md`, nicht den Gesprächsverlauf. Auftrag: drei Optionen bewerten – **weiterbauen**, **gezielt umbauen** (welche Teile) und **neu aufsetzen** – je mit Kosten und Risiko, belegt mit konkreten Stellen: Module und Dateien, reaktive ADRs, Workarounds, markierte Platzhalter, Umfang der betroffenen Teile.
2. **Stellungnahme der bauenden KI:** Sie ergänzt oder widerspricht, jeweils mit Beleg. Die Bewertung der getrennten Instanz wird dabei nicht verändert, sondern neben die Stellungnahme gestellt.
3. **Vorlage an den Menschen** als `ENTSCHEIDUNG ERFORDERLICH` (Abschnitt 4) mit beiden Texten. „Weiterbauen" ist eine zulässige Antwort – entscheidend ist, dass sie bewusst gegeben wird.
4. **ADR** mit der Antwort und der Bewertung; Umbau oder Neuaufbau werden als Schritte mit ID im Fahrplan angelegt.

Kennt das Werkzeug keine getrennte Session, stößt der Mensch sie an: eine neue Session, die nur diesen Auftrag bekommt.

**Warum eine getrennte Instanz:** Wer ein Projekt gebaut hat – KI wie Mensch –, will ein schlechtes Urteil über das Fundament am wenigsten fällen (ADR-012).

<!-- ANCHOR:gate-vor-dem-ersten-oeffentlichen-deployment -->
### Gate vor dem ersten öffentlichen Deployment

Bevor ein Projekt zum ersten Mal aus dem Internet erreichbar wird – auch als Test- oder Staging-System, sobald es echte oder echt wirkende Daten verarbeitet –, müssen **alle acht Prüfpunkte** erfüllt sein. Das Gate gilt unabhängig von der Projektklasse und ist blockierend: Der Schritt, der das öffentliche Deployment ausführt, kann nicht `[ERLEDIGT]` werden, solange ein Punkt offen ist. Die KI prüft das Gate unaufgefordert, sobald ein Fahrplan-Schritt ein öffentliches Deployment vorsieht, lädt dann [`templates/gate-deployment.md`](templates/gate-deployment.md) mit den Anforderungen je Punkt und führt das Gate in einem eigenen Fahrplan-Schritt davor als Checkliste mit Belegen. Grund: Ein Testsystem im Internet ist genauso angreifbar wie ein Produktivsystem (ADR-009).

Die acht Prüfpunkte: (1) Bedrohungsmodell für das Gesamtsystem, (2) Sicherheitsniveau per ADR, (3) Grundhärtung des Hosts mit Prüfung von außen, (4) Secrets im Betrieb und Zugriff der KI, (5) Backup mit erprobter Wiederherstellung, (6) unabhängige Prüfung durch eine getrennte Instanz, (7) Vertretung und Notfall-Handbuch, (8) KI im Betrieb.

**Getrennte Instanz:** eine eigene Session, die den Gesprächsverlauf der Umsetzung nicht kennt, nur den Diff und das Bedrohungsmodell erhält und ausschließlich prüft – möglichst mit einem anderen Modell als dem, das den Code geschrieben hat. Ergebnis und Datum der Prüfung stehen im Logbuch. Dieselbe Prüfung ist nach dem Gate für jede Änderung der Kategorie 6 aus Abschnitt 4 Pflicht (Definition of Done, Abschnitt 9).

**Verzicht auf einen Prüfpunkt** ist nur als Entscheidung nach Abschnitt 4 (Kategorie 6) mit ADR möglich, der das Restrisiko benennt. Stillschweigendes Überspringen ist ein Regelverstoß.

**Zusätzlich vor Go-Live / erstem produktiven Release:** ein externer Blick – ein Mensch außerhalb des Projekts – auf Authentifizierung und Datenschutz. Ist das nicht bezahlbar, wird es als Restrisiko per ADR festgehalten und der Release nach Abschnitt 4 vorgelegt, wie beim Vision-Checkpoint oben.

**Bereits öffentlich erreichbare Projekte**, die diese Fassung übernehmen, holen das Gate nach: Jeder offene Prüfpunkt wird ein Fahrplan-Schritt mit Frist.

## 13. Kommunikationsstil

- **Ehrlich statt gefällig.** Scheitern wird gemeldet, nicht überspielt.
- **Konkret statt vage.** Zahlen, Dateinamen, Zeilen, Testnamen – nicht „es läuft".
- **Vollständigkeit vor Knappheit.** Wenn Zusatzinformation den Menschen zur schnelleren Entscheidung befähigt: mitliefern.
- **Keine Sycophancy.** Keine Zustimmungsfloskeln. Keine Lobhudelei. Reine Arbeitskommunikation.
- **Rückfragen sind erwünscht** bei echten Lücken. Sie sind verboten bei Informationen, die in den Pflicht-Dokumenten stehen – dort nachlesen.

## 14. Archivierung

Archivierung folgt **harten Triggern**, nicht weichen Richtwerten. Die Trigger werden bei jeder Sessionende-Disziplin (Abschnitt 12) geprüft. Auslösung erzwingt Auslagerung im selben Sessionende-Commit – kein offenes Sessionende mit überschrittenem Trigger.

### Harte Trigger pro Pflicht-Dokument

| Dokument | Trigger | Auslagerungs-Ziel |
|---|---|---|
| `docs/logbuch.md` | >800 Zeilen | Vorletzte Monats-Scheibe nach `docs/archiv/logbuch-YYYY-MM.md`. Letzte und aktuelle Monats-Scheibe bleiben aktiv. |
| `docs/fahrplan.md` | Phase ist vollständig abgeschlossen; bei Klasse K: Abschluss eines Schritt-Bündels | Die abgeschlossene Phase nach `docs/archiv/fahrplan-phase-N.md`. Im aktiven Dokument: Phasen-Bilanz-Eintrag plus Archiv-Referenz. |
| `docs/blockers.md` | Gelöster Blocker älter als 90 Tage | Nach `docs/archiv/blockers-YYYY.md`. Im aktiven Dokument verbleibt nichts. |
| `docs/decisions.md` | Bei Klasse G/V ab ADR-Nummer ≥ 30 | Einzelne ADRs nach `decisions/ADR-NNN.md` auslagern (Klasse G/V Default-Struktur, siehe `templates/projektstart.md` Abschnitt 2.2). Teil A und Teil C bleiben zentral (Mindest-Lektüre, Abschnitt 2). |

### Klassen-Dämpfung

Bei Klasse K (Klein) gelten die Zeilen- und Anzahl-Trigger **doppelt** (Logbuch >1.600 Zeilen, ADR ≥ 60). Phasen-bezogene Trigger und 90-Tage-Trigger gelten unverändert.

**Abgeschlossen** im Sinne der Archivierung sind Schritte mit `[ERLEDIGT]` oder `[VERWORFEN]` sowie `[CI-AUSSTEHEND]`-Schritte, die ihr Nachprüf-Schritt namentlich aufführt. Bei Klasse K wandern beim Abschluss eines Bündels alle abgeschlossenen Schritte nach `docs/archiv/fahrplan-<bündel>.md`, auch solche ohne Bündel; offene Schritte bleiben. Fällt ein archivierter Schritt nach Abschnitt 9 auf `[IN ARBEIT]` zurück, wird er ins aktive Dokument zurückgeholt.

### Verfahren

1. Im aktiven Dokument bleibt: aktueller Stand, offene Punkte, Referenz auf Archiv-Datei.
2. Auslagerung selbst ist nicht freigabepflichtig, aber Sessionende-Pflicht.
3. Erstmalige Anlage einer Archiv-Datei: Kopfzeile mit Quell-Dokument, Auslagerungs-Datum, abgedecktem Zeitraum.

### Logbuch-Verdichtung beim Phasen-Wechsel

Zusätzlich zum 800-Zeilen-Trigger oben: Beim `[PHASEN-WECHSEL]`-Eintrag im Logbuch wird das Logbuch verdichtet – **unabhängig davon, ob der Zeilen-Trigger überschritten ist**.

Form der Verdichtung:

1. **Phasen-Reflexions-Eintrag** entsteht im aktiven Logbuch, **maximal 30 Zeilen**, mit: gelernten Erkenntnissen, kippenden Annahmen, Reifegrad-Änderungen, ADRs aus der Phase, neu erkannten Erkundungsbedarfen.
2. **Detail-Einträge der abgeschlossenen Phase** wandern nach `docs/archiv/logbuch-phase-N.md`. Dort landen alle `[SESSIONSTART]`/`[SESSIONENDE]`/`[GELÖST]`-Einträge der Phase samt Beobachtungen und Mini-Reibungen.
3. **Im aktiven Logbuch** bleibt: Reflexions-Eintrag plus Verweis auf die Archiv-Datei.

Die Detail-Pflicht aus Abschnitt 6 („Lieber zu detailreich als lückenhaft") bleibt unangetastet – das Original bleibt im Archiv erhalten, nur der aktiv gelesene Teil wird gekappt.

## 15. Code-Standards (sprachneutrale Pflichtkategorien)

Für jede im Projekt verwendete Sprache müssen die folgenden Tool-Kategorien aktiv und in der CI-Pipeline als Gate konfiguriert sein, **sofern sie für die Sprache anwendbar sind**. Die konkrete Toolwahl pro Sprache erfolgt in `docs/project-context.md` Abschnitt 7.

### Pflichtkategorien

1. **Linter** – statische Codeanalyse für Stil- und Logikfehler. Pflicht für alle Sprachen mit etablierten Lintern.
2. **Formatter** – deterministische Code-Formatierung. Pflicht, wenn ein Formatter für die Sprache existiert. Manuelle Formatierung ist verboten, sobald ein Formatter konfiguriert ist.
3. **Type-Checker** – statische Typprüfung. Pflicht für statisch oder gradual typisierte Sprachen. Strict-Modus ist Default; Abweichungen erfordern ADR.
4. **Security-Scanner** – statische Sicherheitsanalyse. Pflicht, wenn ein Standard-Tool für die Sprache existiert.
5. **Dependency-Audit** – Prüfung auf bekannte Schwachstellen in Abhängigkeiten. Pflicht für alle Sprachen mit Paketmanager.
6. **Test-Runner mit Coverage** – Coverage-Messung Pflicht. Mindestwerte werden in `docs/project-context.md` Abschnitt 7 festgelegt.

Sprachen ohne etablierte Tools in einer Kategorie sind explizit in `docs/project-context.md` zu vermerken („Kategorie X: nicht anwendbar für Sprache Y, Begründung: …").

### Pflichtkategorien für Hilfsskripts (`scripts/`)

Skripte in `scripts/` (oder projektüblich `bin/`, `tools/`) tragen Onboarding und Betrieb und unterliegen einer **reduzierten, aber explizit benannten** Pflicht-Liste, die nicht mit den Pflichtkategorien für Sprachen oben identisch ist.

**Pflicht für jedes Skript** (unabhängig von Größe):

A. **Header mit Zweck-Aussage** in 1–3 Zeilen.
B. **Voraussetzungs-Deklaration im Header** – alle externen CLI-Tools (z. B. `jq`, `openssl`, `docker`), alle erwarteten ENV-Variablen, alle OS-Komponenten (z. B. `mktemp`, bestimmte `getopts`-Variante). Form: `# Voraussetzungen: bash 4+, jq 1.6+, curl, docker compose v2+`.
C. **Plattform-Matrix-Aussage** – explizit benannt, welche Plattformen unterstützt werden. Form: `# Plattformen: Linux, macOS, Windows (Git Bash oder WSL2)`. Wenn eine Plattform nicht unterstützt wird: explizit benannt mit Begründung.
D. **Exit-Code-Disziplin** – Skript signalisiert Erfolg/Fehler über Exit-Code (Bash: `set -euo pipefail` als Default, oder begründete Abweichung).

**Pflicht zusätzlich für Skripte ab Komplexitäts-Schwelle** (Richtwert: >100 Zeilen, >1 Subkommando, >3 externe Tool-Voraussetzungen, oder Cleanup-/Trap-Logik):

E. **Idempotenz-Aussage** im Header – beschreibt, ob das Skript bei wiederholtem Aufruf denselben Effekt erzielt, oder welche Vor-/Nachbedingungen für Re-Run gelten.
F. **Reproduzierbarkeits-Aussage** im Header – beschreibt, ob aufeinanderfolgende Aufrufe (z. B. innerhalb 15 Minuten) zuverlässig dasselbe Ergebnis liefern, oder welche Zustände (Volumes, Caches, Counter) sich auswirken können.
G. **Shell-Linter im Pre-Commit** – für Bash-Skripte: `shellcheck` Pflicht. Für Python-Skripte: bestehende Linter-Konfiguration der Sprache greift bereits, kein zusätzlicher Linter nötig.
H. **Architektur-Eintrag** – Skripte ab dieser Schwelle sind in `docs/architecture.md` als Tooling-Bestandteile mit Reifegrad-Marker zu führen (siehe Tooling-Inventar-Abschnitt der Architektur-Vorlage).

**Nicht-anwendbar-Kennzeichnung:** Wenn eine der Pflichten in einem konkreten Skript nicht sinnvoll ist (z. B. ein Trivial-Wrapper hat keine sinnvolle Idempotenz-Aussage), wird das im Header explizit vermerkt mit kurzer Begründung – analog zur Sprachen-Regel oben.

### Durchsetzungsmechanismen (Pflicht)

- **Pre-Commit-Hook:** Lint, Format-Check, Type-Check, schnelle Security-Checks. Lokale Durchsetzung vor jedem Commit. Konfiguration im Repo (z. B. `pre-commit`-Framework, `husky`, `lefthook`).
- **CI-Pipeline-Gates:** Vollständige Ausführung aller Pflichtkategorien plus Tests bei jedem Push und PR. Rote Gates blockieren Merge.
- **Bypass verboten:** `git commit --no-verify`, `git push --no-verify`, manuelles Deaktivieren von CI-Checks sind nur mit expliziter Freigabe erlaubt (CLAUDE.md Abschnitt 4) und werden im Fahrplan vermerkt.

### Warnungen und Abkündigungen

Was die Ampel nicht zeigt, prüft im Alltag niemand; Warnungen werden deshalb mechanisch behandelt, nicht über eine Lese-Pflicht.

- **Warnungen sind Fehler, wo das Werkzeug es erlaubt.** Die Skelette unter `templates/` setzen das als Default. Die konkreten Schalter pro Sprache stehen in `docs/project-context.md` Abschnitt 7.
- **Warnungs-Bestand mit Obergrenze.** Ein Altbestand oder eine (noch) nicht behebbare Warnung wird in `docs/project-context.md` Abschnitt 7 („Warnungs-Bestand") festgehalten und über die eingebauten Mittel des Werkzeugs begrenzt – eine Zahl als Obergrenze oder eine **benannte** Ausnahme, nie eine Pauschal-Ausnahme. Jede Ausnahme verweist auf einen Fahrplan-Schritt. Mehr Warnungen als die Obergrenze färben den Lauf rot; sinkt der Bestand, wird die Obergrenze im selben Commit gesenkt.
- **Warnungsquellen ohne Schalter** (z. B. Hinweise der CI-Plattform, Warnungen zur Bündelgröße) werden in `docs/project-context.md` Abschnitt 7 aufgelistet. Beurteilt die KI einen CI-Lauf, liest sie für diese Quellen das Protokoll, nicht nur den Status.
- **Jede Abkündigung wird ein Fahrplan-Schritt mit Frist.** Ein Deprecation- oder Lebensende-Hinweis – ob aus Protokoll, Werkzeug oder Hersteller-Quelle – bekommt im selben Arbeitsgang einen Schritt mit Frist und einen Eintrag im Ablaufdaten-Register (`docs/project-context.md` Abschnitt 8). Die Landeplatz-Regel aus Abschnitt 6 gilt: kein Vermerk ohne Schritt-ID.

### Versionswahl

Gilt beim Projektstart (`templates/projektstart.md` Schritt 2a) und bei jeder späteren Wahl einer neuen Linie für Sprache, Framework, Datenbank oder Laufzeitumgebung.

- **Die KI belegt, der Mensch bestätigt.** Die KI prüft jede vorgeschlagene Version gegen offizielle Quellen (Release-Seite, Lebensende-Angabe des Herstellers, Unterstützung durch die tragenden Abhängigkeiten) und legt eine Tabelle mit Quellen vor. Der Mensch bestätigt die Tabelle, nicht die Recherche. Hat die KI keinen Zugang zu den Quellen, ist das eine Informationslücke nach Abschnitt 8 – keine Rate-Version.
- **Ausgereifte Linie.** Gewählt wird die neueste Linie, die alle drei Bedingungen erfüllt: (1) sie hat die Mindestreife aus `docs/project-context.md` Abschnitt 3 erreicht; (2) die tragenden Abhängigkeiten unterstützen sie nachweislich; (3) ihr Unterstützungsfenster reicht über die geplante Projektdauer hinaus. Sonst gilt die Linie davor.
- **Nachprüfung.** Jede fixierte Version steht mit ihrem Lebensende bzw. Nachprüf-Datum im Ablaufdaten-Register. Eine Abkündigung im CI-Protokoll löst die Nachprüfung ebenfalls aus.

### Lokale Suppressions

Ein lokales Deaktivieren einer Regel (`# noqa`, `eslint-disable-next-line`, `@ts-ignore`, `# type: ignore`, `// nolint`, etc.) ist nur zulässig mit:

- Begründungs-Kommentar in derselben Zeile oder direkt darüber.
- Verweis auf Fahrplan-Eintrag oder ADR, falls der Grund eine Entscheidung ist.
- Möglichst engster Scope (`disable-next-line` statt `disable-file`).

Pauschale Suppressions auf Datei- oder Modulebene sind freigabepflichtig.

### Naming und Stil

- Naming Conventions folgen den sprachüblichen Standards (PEP 8 für Python, Standard-Style-Guides der Sprache, etc.).
- Bei mehreren validen Konventionen: einmalige Festlegung in `docs/project-context.md`. Inkonsistenzen im Repo sind als Bug zu behandeln, nicht als Stilfrage.

## 16. README-Pflege

`README.md` ist das öffentliche Statusbild des Projekts und Pflichtbestandteil des Vorlagen-Sets. Sie wird **niemals sporadisch** aktualisiert, sondern folgt festen Triggern.

### Trigger 1: Pro nutzerrelevantem Schritt

Während der Bearbeitung eines Fahrplan-Schritts mit nutzersichtbarer Wirkung wird die README aktualisiert. „Nutzersichtbar" heißt: der Schritt ändert Verhalten, Setup, Konfiguration, Architektur, abgeschlossene Phasen oder den Status des Projekts. Reine interne Refactorings, Test-Erweiterungen und Doku-Pflege in `docs/` lösen den Trigger nicht aus.

Konkrete Aktualisierungen pro Schritt:

- **Quick Start ändert sich:** Setup-, Installations- oder Erstausführungs-Befehle ergänzen oder anpassen.
- **Verwendung ändert sich:** Beispiele aktualisieren, neue Anwendungsfälle ergänzen.
- **Architektur ändert sich:** Architektur-Skizze und Modul-Liste anpassen, Verweise auf `docs/architecture.md` prüfen.
- **Phase abgeschlossen:** „Nächste Schritte" auf die folgende Phase umstellen.
- **Neue oder geänderte Voraussetzung / ENV-Variable / CLI-Tool / OS-Komponente:** Voraussetzungen-Block der README und `.env.example`-Kommentare im **selben Commit** aktualisieren. Gilt auch für kleine Skript-Erweiterungen (Abschnitt 5).

### Trigger 2: Sessionende-Synchronisation

Vor jedem Sessionende läuft eine Synchronisations-Prüfung gegen die Pflicht-Dokumente – **nicht optional**, Teil der Sessionende-Disziplin (Abschnitt 12):

| README-Block | Quelle | Bei Drift |
|---|---|---|
| Status-Block: Projektphase | `docs/fahrplan.md` „Aktueller Stand" + Phasentyp | README anpassen |
| Status-Block: Version | `docs/project-context.md` Abschnitt 1 | README anpassen |
| Status-Block: Status | `docs/project-context.md` Abschnitt 1 | README anpassen + Status-Badge |
| Status-Block: Architektur-Reife | `docs/architecture.md` Abschnitt 9 (Reifegrad-Übersicht) | README-Kurzfassung anpassen |
| Status-Block: Aktive Blocker | `docs/blockers.md` (Anzahl aus „Aktive Blocker") | README-Zähler anpassen |
| Über das Projekt | `docs/vision.md` Abschnitte 1–3 | nur bei Vision-Pivot anpassen |
| Quick Start | `docs/project-context.md` Abschnitt 8 + Stack | bei Stack- oder Setup-Änderungen |
| Architektur-Skizze | `docs/architecture.md` Abschnitt 1 + 2 | bei Architektur-Änderungen |
| Nächste Schritte | `docs/fahrplan.md` (nächste 1–3 Schritte/Phasen) | nach jedem `[ERLEDIGT]` |
| Badges | wie in der Tabelle pro Badge | bei Quell-Änderung |

**Drift zwischen README und Pflicht-Dokumenten ist ein Bug**, kein Stilfehler. Wird er bei der Sessionende-Prüfung gefunden, wird er vor dem Sessionende-Commit behoben.

### Drift-Prüfung zwischen Pflicht-Dokumenten

Das in Trigger 2 etablierte Drift-Muster („Quelle → Ziel → Prüfung, Drift = Bug") gilt nicht nur für README ↔ Pflicht-Dokumente, sondern auch für die Pflicht-Dokumente **untereinander**. Diese Drift entsteht still, etwa durch umnummerierte Schritte oder umbenannte Module.

Die folgende Prüfung läuft als Teil der Sessionende-Disziplin (Abschnitt 12, Schritt 3) und ist nicht optional:

| Konsistenz-Anker | Quelle | Prüfung |
|---|---|---|
| ADR → Fahrplan-Schritt | `docs/decisions.md` Teil B (Schritt-Referenzen in ADRs) | Schritt-ID muss in `docs/fahrplan.md` existieren (auch im Archiv) |
| Reifegrad ↔ letzter ADR | `docs/architecture.md` Abschnitt 9 + `docs/decisions.md` Teil B | Reifegrad-Eintrag muss zum letzten betreffenden ADR passen |
| Modul-Liste | `docs/architecture.md` Abschnitt 2 ↔ `docs/fahrplan.md` „Betroffene Module" | Modulnamen-Set muss identisch sein |
| Anforderung → Schritt (ab Klasse M) | `docs/requirements.md` Abschnitt 5 ↔ `docs/fahrplan.md` | Jede Muss-Anforderung hat einen existierenden Schritt oder Status `VERWORFEN` mit ADR; jede im Fahrplan genannte Anforderungs-ID existiert |
| Blocker-Referenz | `docs/blockers.md` ↔ `docs/fahrplan.md` `[BLOCKIERT]`-Schritte | Jeder `[BLOCKIERT]`-Schritt verweist auf einen aktiven Blocker; jeder aktive Blocker hat mindestens einen blockierten Schritt oder eine begründete Ausnahme |
| ADR-Reaktiv-Quote | `docs/decisions.md` Teil A (Reaktiv-Quote) | Wert stimmt mit tatsächlicher Anzahl `[REAKTIV]`-Tags in Teil B überein; jede Architekturentscheidung (Kategorien 1, 2, 4, 5) mit Phasentyp-Kontext STABILISIERUNG trägt `[REAKTIV]`; ebenso jeder ADR, der einen anderen innerhalb von 14 Tagen ändert oder zurücknimmt |
| Phasenumfang | `docs/fahrplan.md` „Ursprünglicher Schrittplan" je Phase bzw. Bündel | Schrittzahl unter der Wucherungs-Schwelle (Abschnitt 8, Kriterium 9) oder ein STOPP-Block mit Neuplanung liegt vor |

**Drift zwischen Pflicht-Dokumenten ist ein Bug**, kein Stilfehler. Behebung erfolgt vor dem Sessionende-Commit. Die Prüfung lässt sich später als Lint-Script automatisieren – die manuelle Disziplin bleibt die Pflicht, das Script ist Beschleuniger.

### Trigger 3: Phasen-Abschluss-Re-Validation

Bei jedem Schritt, der einen Phasen-Abschluss markiert (Phasen-Bilanz, Beförderung mehrerer Bestandteile, oder als „Phase-X-Abschluss" geführt), wird der vollständige Onboarding-Pfad gegen einen frischen Klon oder Worktree validiert.

Konkrete Form nach Projektgrößen-Klasse (Glossar in Abschnitt 1B, Detail in `templates/projektstart.md` Abschnitt 2):

- **Klasse K (Klein):** README-Quick-Start in einem temporären Verzeichnis nachvollziehen (manuell oder per Skript).
- **Klasse M (Mittel):** Wie K, plus `scripts/`-Smoke-Test (sofern vorhanden) in einem frischen Worktree.
- **Klasse G (Groß):** Wie M, plus expliziter Test einer zweiten Plattform, wenn das Team mehrere Plattformen unterstützt.
- **Klasse V (Verteilt-Groß):** Wie G, plus expliziter Test des Multi-Service-Hochfahrens (alle Compose-Profile / Service-Mesh).

Findings werden im Logbuch dokumentiert. Falls Findings auftreten, die nicht im Rahmen des Phasen-Abschluss-Schritts gefixt werden können, wird ein eigener STABILISIERUNG-Schritt im Fahrplan angelegt – nicht „beim nächsten Mal".

### Badge-Disziplin

- **Maximalwerte pro Klasse** (Glossar in Abschnitt 1B): K=5, M=8, G=10, V=12.
- **Pflicht-Badges in jeder Klasse:** Status, Version, Build, License.
- **Erweiterungen pro Klasse** und **Reihenfolge in der Badge-Zeile** sind in der README-Vorlage (`templates/docs/readme-vorlage.md`) verbindlich definiert.
- **Badges spiegeln reale Zustände.** Manuell gepflegte Fantasie-Badges („Coverage 95 %" ohne Test-Suite) sind verboten.
- **Badge-Update Pflicht im selben Commit** wie die zugrunde liegende Änderung.

### Was die README NICHT ist

Kein Marketing-Dokument (Inhalte stammen aus Pflicht-Dokumenten), kein Roadmap-Vollersatz („Nächste Schritte" zeigt 1–3 Punkte, der Plan bleibt in `docs/fahrplan.md`), keine vollständige Architektur-Spezifikation (Details bleiben in `docs/architecture.md`) und keine Sammlung von Aspirationen (Geplantes gehört in „Nächste Schritte" oder den Fahrplan, nicht in „Verwendung" oder „Architektur").

## 17. Onboarding-Pfad-Pflege

Der **Onboarding-Pfad** ist die dokumentierte Sequenz vom ersten Kontakt mit dem Repository bis zum lauffähigen System. Er besteht aus: dem **Quick-Start-Block** der `README.md` (Voraussetzungen, Setup-Befehle), `.env.example`, `docs/project-context.md` Abschnitt 3 mit der Plattform-Matrix, `docs/onboarding-runbook.md` (Abschnitt 3) und den dort referenzierten Hilfsskripten in `scripts/` (Abschnitt 15; `docs/architecture.md` „Tooling-Inventar").

### Was als „Quick-Start-relevante Änderung" zählt

Quick-Start-relevant (löst DoD-Validierung nach Abschnitt 9 und README-Synchronisation nach Abschnitt 16 Trigger 1 aus) ist eine Änderung an:

- `README.md` (insbesondere Voraussetzungen, Quick Start, Verwendung)
- `.env.example` oder `.env.*.example`
- `scripts/` (jede Änderung an einem Skript, das im Quick-Start oder Onboarding-Runbook referenziert ist)
- `docker-compose.yml`, `docker-compose.*.yml`, `Dockerfile*` (sofern im Quick-Start referenziert)
- `pyproject.toml`, `package.json` (Top-Level-Abhängigkeiten, die im Quick-Start „pflichtig zu installieren" sind)
- `docs/onboarding-runbook.md` (falls vorhanden)
- jede Datei, die im Quick-Start namentlich genannt ist

**Nicht Quick-Start-relevant:** rein interne Code-Änderungen, Test-Erweiterungen ohne neue Tool-Voraussetzungen, Doku-Pflege in `docs/` ohne Quick-Start-Bezug.

### Validierungs-Form

Eine Validierung ist eine **Klon- oder Worktree-frische** Durchführung der dokumentierten Setup-Sequenz bis zum ersten erfolgreichen Funktions-Test (z. B. `/api/health` antwortet).

Mindestform (für Klasse K und Klasse M):

1. `git worktree add` in ein temporäres Verzeichnis (oder `git clone` in ein temporäres Verzeichnis).
2. Quick-Start-Sequenz exakt wie dokumentiert ausführen, ohne Abkürzungen oder Workarounds aus dem Pflege-Worktree.
3. Beim ersten Bruch: Bug im Quick-Start-Pfad, fixen.
4. Nach jedem Fix: erneut von Schritt 1 starten.

Erweiterte Form für Klasse G und V: Abschnitt 16, Trigger 3.

### Verhältnis zu README, Runbook und Logbuch

README trägt den Quick-Start als Statusbild, das Runbook die vollständige, getestete Anleitung (Abschnitt 3). Das Logbuch dokumentiert pro Phasen-Abschluss die durchgeführte Validierung mit Datum und Ergebnis (`[ONBOARDING-VALIDATION]` als Eintragstyp).

---

**Hinweis für den Projektstart:** Diese Datei ist eine generische Vorlage und wird projektübergreifend unverändert übernommen. Änderungen hier betreffen die Arbeitsmethodik, nicht das einzelne Projekt. Projektspezifika gehören in `docs/project-context.md` und die weiteren Dokumente in `docs/`.
