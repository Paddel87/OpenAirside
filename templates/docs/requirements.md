# Requirements

<!-- Anforderungen des Projekts: was die Software können muss, für wen, mit welcher Priorität.
     Pflicht ab Klasse M, optional in Klasse K (dort als „nicht anwendbar, Begründung: …" in
     docs/project-context.md vermerken; Anforderungen stehen dann als Prosa in den Constraints
     und in den Akzeptanzkriterien der Fahrplan-Schritte).
     Klasse G/V: befüllt in Modus 1.5 (templates/projektstart.md, Schritt 1a).
     Klasse M: in Modus 2 aus der Vision abgeleitet (ebenfalls Schritt 1a, verkürzte Form).
     Ab Klasse G gehört Abschnitt 1 („Übersicht") zur Mindest-Lektüre (CLAUDE.md Abschnitt 2);
     alle anderen Abschnitte werden bei Bedarf gelesen. -->

<!-- ANCHOR:uebersicht -->
## 1. Übersicht (Stand vom YYYY-MM-DD)

| Kennzahl | Wert |
|---|---|
| Anforderungen gesamt | [N] |
| davon Muss / Soll / Kann | [n / n / n] |
| Muss-Anforderungen ohne Fahrplan-Schritt | [0 – jeder andere Wert ist ein Bug] |
| Muss-Anforderungen ohne Test (Pflicht ab Klasse G) | [0] |
| Verworfen (mit ADR) | [n] |

**Offene Punkte:** [1–3 Zeilen, z. B. „FR-012 wartet auf Klärung mit Beteiligtengruppe X"]

<!-- ANCHOR:beteiligte -->
## 2. Beteiligte

[Pflicht ab Klasse G, optional in Klasse M. Rollen und Gruppen, **keine Namen oder Kontaktdaten**
(CLAUDE.md Abschnitt 6, „Secrets niemals im Code oder Log" gilt auch für personenbezogene Daten).]

| Rolle / Gruppe | Interesse am System | Einfluss auf Entscheidungen | Betroffenheit | Wie wird sie gehört? |
|---|---|---|---|---|
| [z. B. Disponent:in] | [schnell Aufträge erfassen] | [mittel] | [täglich, Hauptnutzer] | [Live-Test, Rückmeldungen] |

<!-- ANCHOR:anwendungsfaelle -->
## 3. Anwendungsfälle

### Enthalten

| ID | Wer | Was | Quelle |
|---|---|---|---|
| UC-001 | [Rolle] | [Ziel in einem Satz] | [`docs/vision.md` Abschnitt X] |

### Bewusst ausgeschlossen

[Was besprochen und bewusst weggelassen wurde – mit Begründung. Ohne diesen Abschnitt ist später
nicht mehr nachvollziehbar, ob etwas vergessen oder absichtlich ausgelassen wurde.]

| Anwendungsfall | Begründung | Entschieden am | Wieder aufgreifen, wenn … |
|---|---|---|---|
| [z. B. Abrechnung mit Kostenträgern] | [anderes System zuständig] | [YYYY-MM-DD] | [Verband verlangt es] |

<!-- ANCHOR:kernprozesse -->
## 4. Kernprozesse

[Pflicht ab Klasse G; in Klasse M nur, wenn ein Ablauf nicht trivial ist. Je Prozess: heutiger
Ablauf (Ist) und Ablauf mit dem System (Soll), knapp als nummerierte Liste.]

### Prozess: [Name]

- **Ist:** [1. … 2. …]
- **Soll:** [1. … 2. …]
- **Betroffene Anwendungsfälle:** [UC-IDs]

<!-- ANCHOR:funktionale-anforderungen -->
## 5. Funktionale Anforderungen

| ID | Anforderung | Anwendungsfall | Priorität | Prüfbare Akzeptanz | Fahrplan-Schritt | Test (ab Klasse G) | Status |
|---|---|---|---|---|---|---|---|
| FR-001 | [was das System können muss] | [UC-001] | [Muss / Soll / Kann] | [woran erkennbar erfüllt] | [Schritt-ID] | [Testname oder Pfad] | [OFFEN / ERLEDIGT / VERWORFEN (ADR-NNN)] |

**Regeln:**

- IDs werden nie wiederverwendet, auch nicht nach Streichung.
- Eine Muss-Anforderung ohne Fahrplan-Schritt ist ein Bug (Drift-Prüfung, CLAUDE.md Abschnitt 16).
- Eine Muss-Anforderung zu streichen oder herabzustufen ist ein Descope: Status `VERWORFEN` mit ADR (CLAUDE.md Abschnitt 6, „Keine Verschiebung ohne Landeplatz").
- Neue Anforderungen während des Projekts bekommen Datum und Quelle (z. B. „Live-Test 2026-06-20").

<!-- ANCHOR:nicht-funktionale-anforderungen -->
## 6. Nicht-funktionale Anforderungen

[Einzige Quelle bleibt `docs/architecture.md` Abschnitt 6. Hier nur, welche Anwendungsfälle
von welcher dortigen Anforderung besonders betroffen sind – keine Wiederholung der Werte.]

---

**Initialisierungshinweis (Modus 2):**

- **Klasse K:** Datei nicht anlegen; Vermerk „nicht anwendbar, Begründung: …" in `docs/project-context.md` Abschnitt 6.
- **Klasse M:** Abschnitte 1, 3, 5 und 6 Pflicht; 2 und 4 optional.
- **Klasse G:** alle Abschnitte; Spalte „Test" Pflicht für Muss-Anforderungen; bei mehr als 5 Modulen optional aufteilen in `requirements-<modul>.md` mit dieser Datei als Index.
- **Klasse V:** diese Datei als Index, dazu `requirements-<service>.md` und `requirements-integration.md` für service-übergreifende Anforderungen.
- Beispielzeilen durch echte Einträge ersetzen, dann diesen Hinweis entfernen.
