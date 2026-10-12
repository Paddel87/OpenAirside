# Decisions

<!-- Begründete Entscheidungen und daraus abgeleitete Regeln.
     Vier Teile, in dieser Reihenfolge:
       Teil A: ADR-Übersicht (kompakte Tabelle, Reaktiv-Quote) – Pflichtlektüre bei Sessionstart
       Teil B: Architecture Decision Records (ADRs) – chronologisch, mit Tags – Detailteil
       Teil C: Entscheidungsregeln – wiederkehrende Muster, die aus ADRs hervorgehen
       Teil D: Geschäftsentscheidungen (BDR) – optional ab Klasse M, aktiv ab G, Pflicht bei V
     Einträge werden nicht gelöscht oder verändert. Überholte ADRs werden durch
     neue ADRs ersetzt, die den alten Eintrag referenzieren.

     HINWEIS: Teil A steht bewusst zuerst. Bei Sessionstart liest Claude Teil A
     und Teil C (Entscheidungsregeln – dauerhaft geltende Betriebsregeln).
     Teil B (einzelne ADRs) wird nur bei konkretem Bedarf nachgelesen. -->

<!-- ANCHOR:teil-a-adr-uebersicht -->
## Teil A: ADR-Übersicht

[Tabelle für schnellen Zugriff. Wird zu Sessionende aktualisiert, wenn ADRs hinzugekommen oder Status geändert wurden.
Sortiert nach Nummer, durch Tag-Spalte filterbar. Diese Übersicht ist die Mindest-Lektüre bei Sessionstart.]

| ADR | Datum | Status | Klassifikation | Themen | Kategorie | Kurztitel |
|---|---|---|---|---|---|---|
| 001 | YYYY-MM-DD | Aktiv | STRATEGISCH | METHODIK | Methodik | Vorlagen-Anpassung |
| 002 | YYYY-MM-DD | Aktiv | STRATEGISCH | STACK | Externe Abh. | Stack-Wahl |
| ... | | | | | | |

### Reaktiv-Quote

[Anzahl `[REAKTIV]`-ADRs / Gesamtzahl ADRs der letzten 30 Tage. Wenn der Wert über
einem projektspezifischen Schwellenwert liegt: Hinweis auf strukturelles Architekturproblem,
ggf. ERKUNDUNG-Phase einschieben.]

- **Aktueller Wert:** [X / Y]
- **Schwellenwert (in `project-context.md` festgelegt):** [Z]
- **Bei Überschreitung:** STOPP, Reflexion in `fahrplan.md` ergänzen, prüfen ob Architektur-Refactoring nötig ist.

---

<!-- ANCHOR:teil-b-architecture-decision-records -->
## Teil B: Architecture Decision Records

<!-- Detailteil. Einzelne ADRs werden nur bei konkretem Bedarf gelesen –
     z. B. wenn ein Schritt einen referenzierten ADR berührt. -->

### Format

Jeder ADR folgt diesem Schema. Keine Abweichung.

```text
### ADR-NNN: [Kurztitel]

- **Datum:** YYYY-MM-DD
- **Status:** Aktiv | Überholt durch ADR-M | Verworfen
- **Tags:** [aus Tag-Liste unten]
- **Phasentyp-Kontext:** [ERKUNDUNG | UMSETZUNG | STABILISIERUNG | INITIALISIERUNG]
- **Reifegrad-Wirkung:** [welche Architektur-Bestandteile gehen durch diesen ADR auf welchen Reifegrad – falls zutreffend]
- **Kategorie:** [aus CLAUDE.md Abschnitt 4 oder "Methodik"]
- **Kontext:**
  [Problem, Rahmenbedingungen, was stand an, 2–5 Sätze]
- **Optionen:**
  - **A:** [Beschreibung] – Konsequenzen: [...]
  - **B:** [Beschreibung] – Konsequenzen: [...]
  - **C:** [falls relevant]
- **Entscheidung:** [Welche Option, warum]
- **Vision-Frage, die entschied:** [bei Architektur-Entscheidungen Pflicht, sonst „n/a": die fachliche/Vision-Frage aus dem `ENTSCHEIDUNG ERFORDERLICH`-Block (CLAUDE.md Abschnitt 4), die der Mensch beantwortet hat, plus seine Antwort. Macht nachvollziehbar, *worauf* die Wahl beruhte, nicht nur *dass* sie getroffen wurde.]
- **Konfidenz zum Zeitpunkt:** [bei Architektur-Entscheidungen Pflicht, sonst „n/a": hoch/mittel/niedrig + Umkehrbarkeit billig/teuer, aus der Selbstprüfung in `templates/architektur-heuristiken.md` Teil 3. Bei niedriger Konfidenz auf belastbarer Architektur: Verweis auf den ERKUNDUNG-Schritt, der die Annahme später validiert.]
- **Konsequenzen:**
  - [Welche Regeln folgen daraus]
  - [Welche Einschränkungen entstehen]
  - [Welche weiteren Entscheidungen werden dadurch nötig]
- **Abgeleitete Regel:** [Falls aus diesem ADR eine Regel für wiederkehrende Fälle entsteht, hier benennen und in Teil C aufnehmen]
```

### Tags

Jeder ADR trägt mindestens **einen Klassifikations-Tag** und beliebig viele Themen-Tags.

#### Klassifikations-Tags (genau einer pflichtig)

- `[STRATEGISCH]` – in der Konzeptphase oder Initialisierung getroffene Grundsatzentscheidung. Stack-Wahl, Architektur-Pattern, Modul-Schnitt.
- `[OPERATIV]` – während der Umsetzung getroffene Entscheidung im Rahmen geplanter Architektur. Bibliothekswahl innerhalb des Stacks, konkrete Schnittstellen-Spezifikation, Datenmodell-Detail.
- `[REAKTIV]` – Entscheidung, die nötig wurde, weil bei der Umsetzung etwas Unerwartetes auftrat. Workaround, Pivot, nachträgliche Architekturänderung. **Reaktive ADRs sind ein Indikator** – ihre Häufung in einem Modul deutet darauf hin, dass die Architektur dort nicht trägt. **Pflicht:** Jede Architekturentscheidung (Kategorien 1, 2, 4, 5 aus CLAUDE.md Abschnitt 4), die während einer STABILISIERUNG-Phase fällt, wird `[REAKTIV]` klassifiziert (CLAUDE.md Abschnitt 6, „Reaktiv-ADR-Disziplin").
- `[ERKENNTNIS]` – Entscheidung als Resultat einer Erkundungsphase oder eines Spikes. Validiert oder widerlegt eine vorherige Annahme.

#### Themen-Tags (optional, mehrere möglich)

- `[STACK]`, `[MODUL]`, `[SCHNITTSTELLE]`, `[DATENMODELL]`, `[SECURITY]`, `[PERFORMANCE]`, `[DEPLOYMENT]`, `[OBSERVABILITY]`, `[METHODIK]`

### Nummerierung

Durchgehend, keine Lücken. Auch verworfene oder überholte Einträge behalten ihre Nummer.

### Einträge

#### ADR-001: [Beispiel – bei Projektstart durch echte Initialisierungsentscheidung ersetzen]

- **Datum:** [YYYY-MM-DD]
- **Status:** Aktiv
- **Tags:** `[STRATEGISCH]` `[METHODIK]`
- **Phasentyp-Kontext:** INITIALISIERUNG
- **Reifegrad-Wirkung:** keine
- **Kategorie:** Methodik
- **Kontext:** Erste Session im Projekt. Vorlagen-Dokumente wurden an die Projektkomplexität angepasst.
- **Optionen:**
  - A: Volle Vorlagenstruktur übernehmen
  - B: Reduzierte Struktur für [Projekttyp]
  - C: Erweiterte Struktur für [Projekttyp]
- **Entscheidung:** [welche Option]
- **Konsequenzen:** [welche Abschnitte wurden behalten/entfernt/erweitert]
- **Abgeleitete Regel:** keine (Einzelfall-Entscheidung)

---

<!-- ANCHOR:teil-c-entscheidungsregeln -->
## Teil C: Entscheidungsregeln

<!-- Regeln für wiederkehrende Fälle, damit die KI in ähnlichen Situationen
     konsistent und ohne Rückfrage handeln kann.
     Jede Regel verweist auf den ADR, aus dem sie entstanden ist.
     ACHTUNG: Teil C gehört zur Mindest-Lektüre nach CLAUDE.md Abschnitt 2 –
     anders als die ADR-Volltexte in Teil B. Wer hier eine Regel einträgt,
     macht sie damit ab der nächsten Session verbindlich wirksam. -->

### Format (Regel)

```text
### Regel-NNN: [Kurztitel]

- **Herkunft:** ADR-[Nr.]
- **Gilt für:** [wann ist diese Regel anzuwenden]
- **Regel:** [was ist zu tun]
- **Ausnahmen:** [wann gilt die Regel nicht; leer lassen, wenn keine]
- **Gegenbeispiel:** [was wäre falsch]
```

### Regeln

#### Regel-001: [Beispiel]

- **Herkunft:** ADR-[Nr.]
- **Gilt für:** [...]
- **Regel:** [...]
- **Ausnahmen:** [...]
- **Gegenbeispiel:** [...]

<!-- ANCHOR:teil-d-geschaeftsentscheidungen -->
## Teil D: Geschäftsentscheidungen (BDR)

[Entscheidungen, die nicht die Technik betreffen, sondern das Vorhaben: Zielgruppe, Preis, Vertrag, Abläufe beim Nutzer, Zusammenarbeit mit Partnern. ADRs bilden das nicht ab. Optional ab Klasse M, aktiv ab Klasse G, Pflicht bei Klasse V. Nicht Teil der Mindest-Lektüre; gelesen, wenn eine Geschäftsentscheidung ansteht (`CLAUDE.md` Abschnitt 2).]

### BDR-NNN: [Kurztitel]

- **Datum:** [YYYY-MM-DD]
- **Entschieden von:** [Rolle, keine Namen]
- **Frage:** [was war zu entscheiden]
- **Optionen:** [A / B / C mit Folgen]
- **Entscheidung:** [...]
- **Folgen für Anforderungen und Fahrplan:** [FR-IDs, Schritt-IDs, ggf. ADR-Verweis]

---

**Initialisierungshinweis (erste Session nach Projektanlage):**

- Beispiel-ADR und Beispiel-Regel durch echte Einträge ersetzen. Typische erste ADRs:
  - **ADR-001:** Projektgrößen-Klassifikation und Dokumentationsumfang-Anpassung – `[STRATEGISCH] [METHODIK]`
  - **ADR-002:** Stack-Wahl – `[STRATEGISCH] [STACK]`
  - **ADR-003:** Verzeichnisstruktur – `[STRATEGISCH] [METHODIK]`
- Initialisierungs-ADRs sind alle `[STRATEGISCH]` mit Phasentyp-Kontext `INITIALISIERUNG`.
- Reaktiv-Schwellenwert in `project-context.md` festlegen, klassen-abhängig (Glossar in `CLAUDE.md` Abschnitt 1B, Detail in `templates/projektstart.md` Abschnitt 2.2). Empfehlung pro Klasse: K/M ≤ 30 %, G ≤ 20 %, V ≤ 15 % `[REAKTIV]`-Anteil über die letzten 10 ADRs.
- **Strukturwahl** (Einzeldatei vs. Verzeichnis mit `decisions/ADR-NNN.md`) richtet sich nach der Projektgrößen-Klassifikation (Glossar in `CLAUDE.md` Abschnitt 1B, Detail in `templates/projektstart.md` Abschnitt 2.2). Default pro Klasse:
  - **Klasse K/M:** Einzeldatei `decisions.md` mit allen Teilen (Teil D bei K entfernt, bei M optional).
  - **Klasse G:** Einzeldatei zunächst, Auslagerung in `decisions/ADR-NNN.md`-Dateien sobald die ADR-Anzahl zweistellig wird; `decisions.md` bleibt als Index mit Teil A und Teil C.
  - **Klasse V:** Pflicht-Verzeichnis `decisions/` mit einer Datei pro ADR von Anfang an; `decisions.md` als Index.
- **Klasse K:** Teil C (Entscheidungsregeln) kann leer bleiben, bis sich Muster herauskristallisieren. Teil A (Übersicht) trotzdem führen, schon ab ADR-001.
- Format in allen geführten Teilen ist **nicht optional**.
