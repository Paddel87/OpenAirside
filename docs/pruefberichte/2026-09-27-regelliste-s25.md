# Regel-Liste zu S-25 (Umbau U1)

- **Zweck:** Gegenmittel gegen stillen Regelverlust beim Verschlanken von `CLAUDE.md` (Prüfbericht 2026-09-24, Abschnitt 5.2 „Risiken"; Lehre aus ADR-007). Jede Regel wird vor dem Umbau erfasst und nach dem Umbau mit ihrer neuen Fundstelle abgehakt.
- **Stand vorher:** `CLAUDE.md` auf `main` nach PR #46 (Commit e0c0bb4), 714 Zeilen, 81.519 Byte. Zeilennummern beziehen sich auf diesen Stand.
- **Suchbegriffe:** je Regel ein wörtlicher Ausschnitt in `2026-09-27-regelliste-s25.txt` (Format `ID|Abschnitt|Regel|Suchbegriff|Art`). Vorher kommt jeder Suchbegriff genau einmal in `CLAUDE.md` vor (maschinell geprüft).
- **Umfang:** 238 Einträge, davon 214 Regeln (R), 13 Regeln zur Wirtschaftlichkeit (K, Kandidaten für den Anhang) und 11 Begründungen (B, Kandidaten für die Kürzung auf einen Satz). Listen, die als Ganzes eine Regel bilden (z. B. die DoD-Checkliste), stehen als ein Eintrag mit Anzahl der Punkte.
- **Nachher (abgehakt 2026-09-27, ADR-018):** Datei, Abschnitt und Zeile nach dem Umbau (`CLAUDE.md` 69.934 Byte, vor dem Streichen der Budget-Zeile). „Wortlaut gestrafft" heißt: Regelinhalt gleich, gestrichen wurden nur Beispiele, Begründungen oder Wiederholungen. Der Wortvergleich jeder geänderten Zeile ist im Logbuch belegt. Ergebnis: 238 von 238 Einträgen abgehakt; keine Regel entfallen. Einzige Streichung einer ganzen Einheit ist R-232, eine reine Wiederholung; die wiederholten Regeln bestehen weiter.

| ID | Abschnitt | Zeile vorher | Art | Regel (Kurzform) | Nachher |
|---|---|---|---|---|---|
| R-237 | Kopf | 4 | R | Kopfkommentar: gilt für alle Sessions, semi-autonom | CLAUDE.md §Kopf Z. 4 |
| R-001 | 0 | 10 | R | Betriebsmodus semi-autonom | CLAUDE.md §0 Z. 10 |
| R-002 | 0 | 11 | R | Liste: KI entscheidet eigenständig | CLAUDE.md §0 Z. 11 |
| R-003 | 0 | 12 | R | Liste: KI legt zur Freigabe vor | CLAUDE.md §0 Z. 12 |
| R-004 | 0 | 13 | R | Liste: KI stoppt zwingend | CLAUDE.md §0 Z. 13 |
| R-005 | 0 | 17 | R | Vier Modellklassen, Besetzung in project-context §6 | CLAUDE.md §0 Z. 17 – Wortlaut gestrafft |
| R-006 | 0 | 19 | R | Mechanik-Klasse erst nach Probelauf aktiv, bis dahin Routine | CLAUDE.md §0 Z. 19 |
| R-007 | 0 | 20 | R | Definition Routine-Klasse | CLAUDE.md §0 Z. 20 |
| R-008 | 0 | 21 | R | Definition Entscheidungs-Klasse | CLAUDE.md §0 Z. 21 |
| R-009 | 0 | 22 | R | Ausnahme-Klasse nur auf Vorschlag mit Freigabe | CLAUDE.md §0 Z. 22 |
| R-010 | 0 | 24 | R | Zuordnung über Auslöser, nicht Selbsteinschätzung | CLAUDE.md §0 Z. 24 – Wortlaut gestrafft |
| R-011 | 0 | 24 | B | Begründung: Scheitern und Nicht-Bemerken korrelieren | CLAUDE.md §0 Z. 24 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-012 | 0 | 28 | R | Eskalations-Auslöser 1: ENTSCHEIDUNG-Block | CLAUDE.md §0 Z. 28 |
| R-013 | 0 | 29 | R | Eskalations-Auslöser 2: Stopp-Kriterium 8 | CLAUDE.md §0 Z. 29 |
| R-014 | 0 | 30 | R | Eskalations-Auslöser 3: STRATEGISCH-ADR | CLAUDE.md §0 Z. 30 |
| R-015 | 0 | 31 | R | Eskalations-Auslöser 4: Beförderung auf BELASTBAR | CLAUDE.md §0 Z. 31 |
| R-016 | 0 | 32 | R | Eskalations-Auslöser 5: Reaktiv-Quote überschritten | CLAUDE.md §0 Z. 32 |
| R-017 | 0 | 33 | R | Eskalations-Auslöser 6: Modus 1/2 Projektstart | CLAUDE.md §0 Z. 33 |
| R-018 | 0 | 35 | R | Ausnahme-Klasse: zwei Auslöser, Form Stopp-Block, Ablehnung → weiter auf Entscheidung | CLAUDE.md §0 Z. 35 |
| R-019 | 0 | 37 | R | Liste Routine-Klasse genügt für | CLAUDE.md §0 Z. 37 |
| R-020 | 0 | 45 | R | Liste Mechanik-Klasse genügt für | CLAUDE.md §0 Z. 45 |
| R-021 | 0 | 47 | R | Feld Empfohlene Klasse je Schritt; höchste Klasse gilt, Teilarbeit abgebbar | CLAUDE.md §0 Z. 47 |
| R-022 | 0 | 51 | K | Arbeit oberhalb der Empfehlung: Wirtschaftlichkeit, kein Stopp | CLAUDE.md §0 (Kurzfassung) und templates/wirtschaftlichkeit.md §1 (vollständig) |
| R-023 | 0 | 51 | B | Begründung: bloßer Hinweis im Pilot wirkungslos | templates/wirtschaftlichkeit.md §1 (vollständig); in CLAUDE.md §0 ersetzt durch „Die Wirkung sichert ein Mechanismus ohne Zutun des Menschen" |
| R-024 | 0 | 53 | K | Abgabe an Unteragenten (Routine/Mechanik), Prüfung vor Commit | CLAUDE.md §0 Punkt 1 (Kurzfassung) und templates/wirtschaftlichkeit.md §1 Punkt 1 |
| R-025 | 0 | 53 | K | Abgabe nur nach bestandenem Probelauf | CLAUDE.md §0 Punkt 1 („nur nach bestandenem Probelauf") und templates/wirtschaftlichkeit.md §1 Punkt 1 |
| R-026 | 0 | 53 | K | Kleine Teilarbeit im geladenen Kontext bleibt | templates/wirtschaftlichkeit.md §1 Z. 16 – verschoben (ADR-018) |
| R-027 | 0 | 53 | K | Abgabe nur, wo sie spart (Cache-Lesepreis, Ausgabe) | CLAUDE.md §0 Punkt 1 („nur, wo die Abgabe tatsächlich spart") und templates/wirtschaftlichkeit.md §1 Punkt 1 (Kriterien) |
| R-028 | 0 | 54 | K | Warnung zu Beginn des Schritts, einmal, kein Stopp | CLAUDE.md §0 Punkt 2 (Kurzfassung) und templates/wirtschaftlichkeit.md §1 Punkt 2 |
| R-029 | 0 | 55 | K | Bilanz am Sessionende (Klasse, Schritte oberhalb, Abgaben) | CLAUDE.md §0 Punkt 3 (gleicher Inhalt) und templates/wirtschaftlichkeit.md §1 Punkt 3 |
| R-030 | 0 | 57 | K | Probelauf-Verfahren, Wiederholung nach Modellwechsel | templates/wirtschaftlichkeit.md §1 Z. 21 – verschoben (ADR-018) |
| R-031 | 0 | 61 | B | Begründung Sessiongröße (Cache-Lesen) | CLAUDE.md §0 „Sessiongröße", erster Satz – gekürzt, Wortlaut vorher in ADR-018 |
| R-032 | 0 | 63 | K | Kontextgröße beim Sessionende feststellen und eintragen | CLAUDE.md §0 „Sessiongröße" – zusammengefasst (Anhang enthält diesen Teil nicht mehr) |
| R-033 | 0 | 64 | K | Grenze überschritten: einzeiliger Hinweis, kein Stopp | CLAUDE.md §0 „Sessiongröße" – zusammengefasst |
| R-034 | 0 | 65 | K | Größe nicht feststellbar: vermerken | CLAUDE.md §0 „Sessiongröße" – zusammengefasst |
| R-035 | 0 | 67 | K | Kontingent-Warnung erst nach Probelauf aktiv | templates/wirtschaftlichkeit.md §2 (als Abschnitt „Kontingent-Warnung"); Verweis in CLAUDE.md §0 |
| R-036 | 0 | 71 | R | Eskalation = echter Stopp, kein Werkzeugaufruf bis Antwort | CLAUDE.md §0 Z. 65 |
| R-037 | 0 | 76 | R | Form des Modellwechsel-Stopps | CLAUDE.md §0 Z. 70 |
| R-038 | 0 | 88 | R | Wiederanlauf: aktive Klasse zuerst benennen | CLAUDE.md §0 Z. 82 |
| R-039 | 0 | 90 | B | Begründung: kein Ersatz-Vorgehen (ADR-005) | CLAUDE.md §0 „Kein Ersatz-Vorgehen" – gekürzt auf einen Satz, Wortlaut vorher in ADR-018 |
| R-040 | 0 | 92 | K | Arbeit oberhalb der Empfehlung ist kein Stopp | CLAUDE.md §0 „Arbeit oberhalb der empfohlenen Klasse", erster Satz („kein Stopp") – zusammengelegt |
| R-041 | 0 | 96 | R | Klasse nur aus Mitteilungen der Laufzeitumgebung | CLAUDE.md §0 Z. 88 – Wortlaut gestrafft |
| R-042 | 0 | 98 | R | Sessionstart: Modellabfrage, Eintrag; Wiederholung vor Stopp und Bilanz | CLAUDE.md §0 Z. 90 |
| R-043 | 0 | 102 | R | Grenze: Tausch ohne Mitteilung unbemerkt | CLAUDE.md §0 Z. 92 – Wortlaut gestrafft |
| R-044 | 0 | 103 | R | Widerspruch: letzte Mitteilung gilt, sonst unbestimmbar | CLAUDE.md §0 Z. 92 – Wortlaut gestrafft |
| R-045 | 0 | 105 | R | Unbestimmbar: Stopp trotzdem; Warnung/Bilanz entfallen, Abgabe bleibt | CLAUDE.md §0 Z. 94 – Wortlaut gestrafft |
| R-046 | 0 | 107 | B | Restrisiko: nur gelistete Auslöser; Kontingent nicht gemessen | CLAUDE.md §0 Z. 96 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-047 | 1 | 111 | R | Projektkontext in docs/project-context.md, zuerst lesen | CLAUDE.md §1 Z. 100 |
| R-048 | 1 | 113 | R | CLAUDE.md projektübergreifend unverändert | CLAUDE.md §1 Z. 102 |
| R-049 | 1A | 117 | R | Projektstart in templates/projektstart.md, nicht pro Session laden | CLAUDE.md §1A Z. 106 – Wortlaut gestrafft |
| R-050 | 1A | 119 | R | Regulärer Betrieb nach Initialisierungs-Commit | CLAUDE.md §1A Z. 108 |
| R-051 | 1B | 125 | R | Glossar Klassen K/M/G/V | CLAUDE.md §1B Z. 114 |
| R-052 | 1B | 130 | R | Klasse in ADR-001 fixiert; Reklassifikation per ADR als STABILISIERUNG | CLAUDE.md §1B Z. 119 |
| R-053 | 2 | 134 | R | Pflichtlektüre vor jeder Änderung, zweistufig | CLAUDE.md §2 Z. 123 – Wortlaut gestrafft |
| R-054 | 2 | 138 | R | Lektüre 1: project-context vollständig | CLAUDE.md §2 Z. 127 |
| R-055 | 2 | 139 | R | Lektüre 2: Logbuch ab letztem SESSIONENDE | CLAUDE.md §2 Z. 128 |
| R-056 | 2 | 140 | R | Lektüre 3: Fahrplan Aktueller Stand + laufende Phase | CLAUDE.md §2 Z. 129 – Wortlaut gestrafft |
| R-057 | 2 | 141 | R | Lektüre 4: Architektur 1, 2, 9 | CLAUDE.md §2 Z. 130 |
| R-058 | 2 | 142 | R | Lektüre 5: decisions Teil A und C | CLAUDE.md §2 Z. 131 – Wortlaut gestrafft |
| R-059 | 2 | 142 | R | Teil C zu groß → Auslagerung, nicht überspringen | CLAUDE.md §2 Z. 131 – Wortlaut gestrafft |
| R-060 | 2 | 143 | R | Lektüre 6: blockers Aktive Blocker | CLAUDE.md §2 Z. 132 |
| R-061 | 2 | 144 | R | Lektüre 7: requirements Übersicht ab G | CLAUDE.md §2 Z. 133 |
| R-062 | 2 | 146 | R | SESSIONSTART-Eintrag direkt nach Lektüre, vor jeder Aktion | CLAUDE.md §2 Z. 135 |
| R-063 | 2 | 154 | R | Größen-Budget project-context 15.000 Token / 600 Zeilen, Splitting | CLAUDE.md §2 Z. 143 – unverändert (die zusätzliche Zeile für `CLAUDE.md` wurde nach der Revision von ADR-018 Teilfrage 1 wieder gestrichen) |
| R-064 | 2 | 156 | R | Recherche, Interviews nicht inline | CLAUDE.md §2 Z. 146 |
| R-065 | 2 | 158 | R | Größen-Check beim Sessionende, Überschreitung = Bug | CLAUDE.md §2 Z. 148 |
| R-066 | 2 | 162 | R | Vertiefung nur mit konkreter Aufgabe begründet | CLAUDE.md §2 Z. 152 |
| R-067 | 2 | 166 | R | Vertiefung: Modul X | CLAUDE.md §2 Z. 156 – Wortlaut gestrafft |
| R-068 | 2 | 167 | R | Vertiefung: ADR-Y | CLAUDE.md §2 Z. 157 – Wortlaut gestrafft |
| R-069 | 2 | 168 | R | Vertiefung: aktiver Blocker | CLAUDE.md §2 Z. 158 – Wortlaut gestrafft |
| R-070 | 2 | 169 | R | Vertiefung: Architekturentscheidung → Verworfene Alternativen + Heuristiken (Pflicht bei VDD) | CLAUDE.md §2 Z. 159 – Wortlaut gestrafft |
| R-071 | 2 | 170 | R | Vertiefung: ähnliches früheres Problem | CLAUDE.md §2 Z. 160 – Wortlaut gestrafft |
| R-072 | 2 | 171 | R | Vertiefung: NFR-Frage | CLAUDE.md §2 Z. 161 – Wortlaut gestrafft |
| R-073 | 2 | 172 | R | Vertiefung: Compliance/Lizenz | CLAUDE.md §2 Z. 162 – Wortlaut gestrafft |
| R-074 | 2 | 173 | R | Vertiefung: Anforderung umsetzen | CLAUDE.md §2 Z. 163 – Wortlaut gestrafft |
| R-075 | 2 | 174 | R | Vertiefung: Geschäftsentscheidung → Teil D | CLAUDE.md §2 Z. 166 – Wortlaut gestrafft |
| R-076 | 2 | 180 | R | Sprung-Anker je Hauptabschnitt, nicht umbenennen | CLAUDE.md §2 Z. 172 |
| R-077 | 2 | 182 | R | Namensregel für Anker | CLAUDE.md §2 Z. 174 |
| R-078 | 2 | 183 | R | Tool-Limit: Grep auf Anker, gezielter Read | CLAUDE.md §2 Z. 175 |
| R-079 | 2 | 184 | R | Abschnittsweises Splitting | CLAUDE.md §2 Z. 176 |
| R-080 | 2 | 186 | R | Abgebrochenen Read nicht still akzeptieren | CLAUDE.md §2 Z. 178 |
| R-081 | 2 | 190 | R | Vision nicht Pflichtlektüre, außer Re-Derivations-Pass | CLAUDE.md §2 Z. 182 – Wortlaut gestrafft |
| R-082 | 2 | 191 | R | README nicht zu Sessionbeginn lesen | CLAUDE.md §2 Z. 183 |
| R-083 | 2 | 192 | R | CHANGELOG nur bei Releases | CLAUDE.md §2 Z. 184 |
| R-084 | 2 | 196 | R | Kein Überspringen der Mindest-Lektüre | CLAUDE.md §2 Z. 188 – Wortlaut gestrafft |
| R-085 | 3 | 200 | R | Dokumenten-Index mit Aktualisierungstriggern (10 Dokumente) | CLAUDE.md §3 Z. 192 – Wortlaut gestrafft (Zeilen `README.md` und `docs/onboarding-runbook.md`) |
| R-086 | 3 | 214 | R | Erste Session: Vorlagen anpassen, ADR-001 | CLAUDE.md §3 Z. 206 |
| R-087 | 4 | 218 | R | Freigabe-Kategorien nie eigenständig; Vorschlag mit Alternativen | CLAUDE.md §4 Z. 210 |
| R-088 | 4 | 220 | R | Kat. 1 Architekturänderungen | CLAUDE.md §4 Z. 212 |
| R-089 | 4 | 221 | R | Kat. 2 Neue Module | CLAUDE.md §4 Z. 213 |
| R-090 | 4 | 222 | R | Kat. 3 Externe Abhängigkeiten, Major-Updates | CLAUDE.md §4 Z. 214 |
| R-091 | 4 | 223 | R | Kat. 4 Datenmodell | CLAUDE.md §4 Z. 215 |
| R-092 | 4 | 224 | R | Kat. 5 API-Vertrag | CLAUDE.md §4 Z. 216 |
| R-093 | 4 | 225 | R | Kat. 6 Sicherheit und Datenschutz | CLAUDE.md §4 Z. 217 |
| R-094 | 4 | 226 | R | Kat. 7 Build/Deploy | CLAUDE.md §4 Z. 218 |
| R-095 | 4 | 227 | R | Kat. 8 Lizenz/Compliance | CLAUDE.md §4 Z. 219 |
| R-096 | 4 | 232 | R | Form ENTSCHEIDUNG ERFORDERLICH | CLAUDE.md §4 Z. 224 |
| R-097 | 4 | 247 | B | Begründung der Form (Rubber-Stamp) | CLAUDE.md §4 Z. 239 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-098 | 4 | 249 | R | Nach Freigabe: ADR mit Vision-Frage und Konfidenz, dann implementieren | CLAUDE.md §4 Z. 241 |
| R-099 | 5 | 253 | R | Autonomiebereich (8 Punkte) | CLAUDE.md §5 Z. 245 |
| R-100 | 5 | 266 | R | Nicht autonom: Skript-Erweiterung mit neuer Voraussetzung, Sync im selben Commit | CLAUDE.md §5 Z. 258 – Wortlaut gestrafft |
| R-101 | 5 | 267 | R | Nicht autonom: Skript-Erweiterung mit Idempotenz-Wirkung | CLAUDE.md §5 Z. 259 – Wortlaut gestrafft |
| R-102 | 5 | 269 | R | Grenzfälle wie Freigaben | CLAUDE.md §5 Z. 261 |
| R-103 | 6 | 273 | R | Keine Implementierung ohne Fahrplan-Referenz | CLAUDE.md §6 Z. 265 |
| R-104 | 6 | 274 | R | Keine stillen Annahmen | CLAUDE.md §6 Z. 266 – Wortlaut gestrafft |
| R-105 | 6 | 275 | R | Keine Erfolgsmeldungen ohne Verifikation | CLAUDE.md §6 Z. 267 |
| R-106 | 6 | 276 | R | Platzhalter mit TODO(fahrplan-ref) | CLAUDE.md §6 Z. 268 |
| R-107 | 6 | 277 | R | Modulgrenzen respektieren | CLAUDE.md §6 Z. 269 |
| R-108 | 6 | 278 | R | Keine heimlichen Scope-Erweiterungen | CLAUDE.md §6 Z. 270 |
| R-109 | 6 | 279 | R | Keine Verschiebung ohne Landeplatz (+ Diagnose-Heuristik) | CLAUDE.md §6 – Regel unverändert; der Satz „Diagnose-Heuristik: …" gestrichen (wiederholte die Regel), Wortlaut in ADR-018 |
| R-110 | 6 | 280 | R | Determinismus vor Kreativität | CLAUDE.md §6 Z. 272 |
| R-111 | 6 | 281 | R | Secrets niemals im Code oder Log | CLAUDE.md §6 Z. 273 |
| R-112 | 6 | 282 | R | Schutzbedarf ist Obergrenze | CLAUDE.md §6 Z. 274 – Wortlaut gestrafft |
| R-113 | 6 | 283 | R | Secrets nicht in eigener Ausgabe; kompromittiert → Stopp, Rotation | CLAUDE.md §6 Z. 275 – Wortlaut gestrafft |
| R-114 | 6 | 284 | R | Reproduzierbarkeit vor Performance; flaky Tests = Blocker | CLAUDE.md §6 Z. 276 |
| R-115 | 6 | 285 | R | Code-Standards verbindlich; Suppressions nur begründet | CLAUDE.md §6 Z. 277 – Wortlaut gestrafft |
| R-116 | 6 | 286 | R | Schutzmechanismen durch erzwungenen Fehler belegen | CLAUDE.md §6 Z. 278 – Wortlaut gestrafft |
| R-117 | 6 | 287 | R | Architektur-Reifegrad respektieren | CLAUDE.md §6 Z. 279 |
| R-118 | 6 | 288 | R | Phasentyp-Disziplin | CLAUDE.md §6 Z. 280 |
| R-119 | 6 | 289 | R | Reaktiv-ADR-Disziplin (STABILISIERUNG, 14-Tage-Korrektur) | CLAUDE.md §6 Z. 281 – Wortlaut gestrafft |
| R-120 | 6 | 290 | R | Logbuch-Einträge proaktiv | CLAUDE.md §6 Z. 282 – Wortlaut gestrafft |
| R-122 | 7 | 297 | R | VERSCHOBEN nennt Ziel-Schritt-ID | CLAUDE.md §7 Z. 289 |
| R-121 | 7 | 298 | R | Status-Marker (9 Marker, max. ein IN ARBEIT) | CLAUDE.md §7 Z. 290 |
| R-123 | 7 | 302 | R | CI-AUSSTEHEND nennt Nachprüf-Schritt mit Frist | CLAUDE.md §7 Z. 294 |
| R-124 | 8 | 309 | R | Stopp 1 Informationslücke | CLAUDE.md §8 Z. 301 |
| R-125 | 8 | 310 | R | Stopp 2 Widerspruch | CLAUDE.md §8 Z. 302 |
| R-126 | 8 | 311 | R | Stopp 3 Freigabebedarf | CLAUDE.md §8 Z. 303 |
| R-127 | 8 | 312 | R | Stopp 4 Dreifach-Fehlschlag | CLAUDE.md §8 Z. 304 |
| R-128 | 8 | 313 | R | Stopp 5 Fremde Modulgrenze | CLAUDE.md §8 Z. 305 |
| R-129 | 8 | 314 | R | Stopp 6 Destruktiver Eingriff | CLAUDE.md §8 Z. 306 |
| R-130 | 8 | 315 | R | Stopp 7 Unklare Testlage | CLAUDE.md §8 Z. 307 |
| R-131 | 8 | 316 | R | Stopp 8 Architektur-Rateschluss → Spike | CLAUDE.md §8 Z. 308 |
| R-132 | 8 | 318 | R | Stopp 9 Phasen-Wucherung (Zählung nach Inhalt, Neuplanung) | CLAUDE.md §8 Z. 310 |
| R-133 | 8 | 330 | R | Form des STOPP-Blocks; kein Umgehen | CLAUDE.md §8 Z. 322 |
| R-134 | 9 | 334 | R | DoD: alle Punkte, Checkliste (19 Punkte) | CLAUDE.md §9 Z. 326 |
| R-135 | 9 | 341 | R | DoD: Sicherheitsänderung durch getrennte Instanz geprüft | CLAUDE.md §9 Z. 333 |
| R-136 | 9 | 350 | R | DoD: Onboarding-Pfad validiert bei Quick-Start-Änderung | CLAUDE.md §9 Z. 342 – Definition „Quick-Start-relevant" verweist auf §17 (dessen Liste die DoD schon vorher auslöste; die engere Aufzählung hier widersprach ihr) |
| R-137 | 9 | 353 | R | DoD: CI grün inkl. Warnungs-Bestand | CLAUDE.md §9 Z. 345 |
| R-138 | 9 | 356 | R | Unvollständige DoD bleibt IN ARBEIT | CLAUDE.md §9 Z. 348 |
| R-139 | 9 | 358 | R | Einzige Ausnahme CI-AUSSTEHEND (3 Bedingungen) | CLAUDE.md §9 Z. 350 |
| R-140 | 10 | 362 | R | Blocker-Protokoll bei dreifachem Scheitern (4 Schritte) | CLAUDE.md §10 Z. 354 |
| R-141 | 10 | 369 | R | Was als derselbe Ansatz zählt | CLAUDE.md §10 Z. 361 |
| R-142 | 11 | 376 | R | Commit-Format mit Fahrplan-Zeile | CLAUDE.md §11 Z. 368 |
| R-143 | 11 | 385 | R | Commit-Regeln (atomar, Imperativ, kein WIP, keine Mix, ADR-Nr.) | CLAUDE.md §11 Z. 377 |
| R-234 | 11 | 393 | R | Hauptbranch-Umbenennung freigabepflichtig | CLAUDE.md §11 Z. 385 |
| R-144 | 11 | 394 | R | Branch-Regeln | CLAUDE.md §11 Z. 386 |
| R-235 | 11 | 395 | R | Push-Regeln in project-context | CLAUDE.md §11 Z. 387 |
| R-145 | 12 | 401 | R | Sessionende 1: Fahrplan aktualisieren | CLAUDE.md §12 Z. 393 |
| R-146 | 12 | 402 | R | Sessionende 2: README synchronisieren | CLAUDE.md §12 Z. 394 – Wortlaut gestrafft |
| R-147 | 12 | 403 | R | Sessionende 3: Inter-Pflicht-Drift-Check | CLAUDE.md §12 Z. 395 – Wortlaut gestrafft |
| R-148 | 12 | 404 | R | Sessionende 4: SESSIONENDE-Eintrag mit Pflichtinhalten | CLAUDE.md §12 Z. 396 |
| R-149 | 12 | 405 | R | Sessionende 5: committen oder begründet uncommitted | CLAUDE.md §12 Z. 397 |
| R-150 | 12 | 406 | R | Sessionende 6: offene Gedanken festhalten | CLAUDE.md §12 Z. 398 |
| R-151 | 12 | 407 | R | Sessionende 7: STOPP-Block im Fahrplan | CLAUDE.md §12 Z. 399 |
| R-152 | 12 | 408 | R | Sessionende 8: Ablaufdaten-Register prüfen | CLAUDE.md §12 Z. 400 |
| R-153 | 12 | 415 | R | Vision-Checkpoint an Phasengrenze (Re-Derivation, ab M requirements) | CLAUDE.md §12 Z. 407 – Wortlaut gestrafft |
| R-154 | 12 | 416 | R | Vision-Checkpoint vor Go-Live | CLAUDE.md §12 Z. 408 |
| R-155 | 12 | 418 | B | Begründung: gegen Quelle statt Spiegel; Traceability-Tabelle zulässig | CLAUDE.md §12 Z. 410 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-156 | 12 | 420 | B | Begründung: an Phasengrenzen statt pro Session | CLAUDE.md §12 Z. 412 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-157 | 12 | 422 | R | Kein Abschluss mit IN ARBEIT ohne Statushinweis; 5-Minuten-Ziel | CLAUDE.md §12 Z. 414 – Wortlaut gestrafft |
| R-158 | 12 | 425 | R | Pflichtfrage: Anlass, nicht verschiebbar | CLAUDE.md §12 Z. 417 |
| R-159 | 12 | 431 | R | Pflichtfrage Ablauf 1–4 (getrennte Instanz, Stellungnahme, Vorlage, ADR) | CLAUDE.md §12 Z. 423 – Wortlaut gestrafft |
| R-160 | 12 | 436 | R | Werkzeug ohne getrennte Session: Mensch stößt an | CLAUDE.md §12 Z. 428 |
| R-161 | 12 | 438 | B | Begründung getrennte Instanz (Pilot) | CLAUDE.md §12 Z. 430 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-162 | 12 | 443 | R | Gate: acht Prüfpunkte, blockierend, eigener Schritt | CLAUDE.md §12 Z. 435 – Wortlaut gestrafft |
| R-163 | 12 | 445 | R | Gate 1 Bedrohungsmodell | templates/gate-deployment.md §1 Z. 16 – verschoben (ADR-018) |
| R-164 | 12 | 446 | R | Gate 2 Sicherheitsniveau per ADR | templates/gate-deployment.md §1 Z. 17 – verschoben (ADR-018) |
| R-165 | 12 | 447 | R | Gate 3 Grundhärtung Host, Prüfung von außen | templates/gate-deployment.md §1 Z. 18 – verschoben (ADR-018) |
| R-166 | 12 | 448 | R | Gate 4 Secrets im Betrieb, Zugriff der KI | templates/gate-deployment.md §1 Z. 19 – verschoben (ADR-018) |
| R-167 | 12 | 449 | R | Gate 5 Backup mit erprobter Wiederherstellung | templates/gate-deployment.md §1 Z. 20 – verschoben (ADR-018) |
| R-168 | 12 | 450 | R | Gate 6 Unabhängige Prüfung | templates/gate-deployment.md §1 Z. 21 – verschoben (ADR-018) |
| R-169 | 12 | 451 | R | Gate 7 Vertretung und Notfall-Handbuch | templates/gate-deployment.md §1 Z. 22 – verschoben (ADR-018) |
| R-170 | 12 | 452 | R | Gate 8 KI im Betrieb (Kontingent, Rückfall, Befehlsliste) | templates/gate-deployment.md §1 Z. 23 – verschoben (ADR-018) |
| R-171 | 12 | 454 | R | Definition getrennte Instanz; Pflicht nach Gate für Kat. 6 | CLAUDE.md §12 Z. 439 |
| R-172 | 12 | 456 | R | Verzicht auf Prüfpunkt nur per ADR Kat. 6 | CLAUDE.md §12 Z. 441 |
| R-173 | 12 | 458 | R | Vor Go-Live externer Blick oder ADR | CLAUDE.md §12 Z. 443 |
| R-174 | 12 | 460 | R | Bereits öffentliche Projekte holen Gate nach | CLAUDE.md §12 Z. 445 |
| R-175 | 12 | 462 | B | Begründung Gate vor erstem Deployment (Pilot) | templates/gate-deployment.md §2 (vollständig); in CLAUDE.md §12 ein Satz („Grund: …", ADR-009) |
| R-176 | 13 | 466 | R | Kommunikationsstil (5 Punkte) | CLAUDE.md §13 Z. 449 |
| R-177 | 14 | 474 | R | Archivierung nach harten Triggern, im Sessionende-Commit | CLAUDE.md §14 Z. 457 |
| R-178 | 14 | 480 | R | Trigger Logbuch > 800 Zeilen | CLAUDE.md §14 Z. 463 |
| R-179 | 14 | 481 | R | Trigger Fahrplan: Phase vollständig erledigt | CLAUDE.md §14 Tabelle, Zeile `docs/fahrplan.md` – erweitert um Klasse K (ADR-018 Teilfrage 5) |
| R-180 | 14 | 482 | R | Trigger Blocker: gelöst > 90 Tage | CLAUDE.md §14 Z. 465 |
| R-181 | 14 | 483 | R | Trigger decisions G/V ab ADR 30; Teil A/C zentral | CLAUDE.md §14 Z. 466 – Wortlaut gestrafft |
| R-182 | 14 | 487 | R | Klassen-Dämpfung K doppelt | CLAUDE.md §14 Z. 470 – Wortlaut gestrafft |
| R-183 | 14 | 493 | R | Verfahren (3 Punkte, Kopfzeile) | CLAUDE.md §14 Z. 478 |
| R-184 | 14 | 495 | R | Logbuch-Verdichtung beim Phasen-Wechsel | CLAUDE.md §14 Z. 480 – letzter Satz (Begründung) gestrichen |
| R-185 | 15 | 509 | R | Sechs Pflichtkategorien je Sprache in CI | CLAUDE.md §15 Z. 494 |
| R-186 | 15 | 513 | R | Kategorie 1 Linter | CLAUDE.md §15 Z. 498 |
| R-187 | 15 | 514 | R | Kategorie 2 Formatter | CLAUDE.md §15 Z. 499 |
| R-188 | 15 | 515 | R | Kategorie 3 Type-Checker strict | CLAUDE.md §15 Z. 500 |
| R-189 | 15 | 516 | R | Kategorie 4 Security-Scanner | CLAUDE.md §15 Z. 501 |
| R-190 | 15 | 517 | R | Kategorie 5 Dependency-Audit | CLAUDE.md §15 Z. 502 |
| R-191 | 15 | 518 | R | Kategorie 6 Test-Runner mit Coverage | CLAUDE.md §15 Z. 503 |
| R-192 | 15 | 520 | R | Nicht anwendbare Kategorie explizit vermerken | CLAUDE.md §15 Z. 505 |
| R-193 | 15 | 528 | R | Skripte A: Header mit Zweck | CLAUDE.md §15 Z. 513 |
| R-194 | 15 | 529 | R | Skripte B: Voraussetzungen im Header | CLAUDE.md §15 Z. 514 |
| R-195 | 15 | 530 | R | Skripte C: Plattform-Matrix | CLAUDE.md §15 Z. 515 |
| R-196 | 15 | 531 | R | Skripte D: Exit-Code-Disziplin | CLAUDE.md §15 Z. 516 |
| R-197 | 15 | 535 | R | Skripte E: Idempotenz-Aussage (Schwelle) | CLAUDE.md §15 Z. 520 |
| R-198 | 15 | 536 | R | Skripte F: Reproduzierbarkeits-Aussage | CLAUDE.md §15 Z. 521 |
| R-199 | 15 | 537 | R | Skripte G: Shell-Linter im Pre-Commit | CLAUDE.md §15 Z. 522 |
| R-200 | 15 | 538 | R | Skripte H: Architektur-Eintrag | CLAUDE.md §15 Z. 523 |
| R-201 | 15 | 540 | R | Nicht-anwendbar-Kennzeichnung für Skripte | CLAUDE.md §15 Z. 525 |
| R-202 | 15 | 544 | R | Pre-Commit-Hook Pflicht | CLAUDE.md §15 Z. 529 |
| R-203 | 15 | 545 | R | CI-Gates Pflicht, rote Gates blockieren | CLAUDE.md §15 Z. 530 |
| R-204 | 15 | 546 | R | Bypass verboten ohne Freigabe | CLAUDE.md §15 Z. 531 |
| R-238 | 15 | 550 | B | Begründung Warnungen mechanisch behandeln | CLAUDE.md §15 Z. 535 – auf einen Satz gekürzt, Wortlaut vorher in ADR-018 |
| R-205 | 15 | 552 | R | Warnungen sind Fehler, wo möglich | CLAUDE.md §15 Z. 537 – Wortlaut gestrafft |
| R-206 | 15 | 553 | R | Warnungs-Bestand mit Obergrenze, nur schrumpfend | CLAUDE.md §15 Z. 538 – Wortlaut gestrafft |
| R-207 | 15 | 554 | R | Warnungsquellen ohne Schalter: Protokoll lesen | CLAUDE.md §15 Z. 539 |
| R-208 | 15 | 555 | R | Jede Abkündigung wird Schritt mit Frist + Register | CLAUDE.md §15 Z. 540 |
| R-209 | 15 | 561 | R | Versionswahl: KI belegt, Mensch bestätigt | CLAUDE.md §15 Z. 546 – Wortlaut gestrafft |
| R-210 | 15 | 562 | R | Versionswahl: ausgereifte Linie (3 Bedingungen) | CLAUDE.md §15 Z. 547 |
| R-211 | 15 | 563 | R | Versionswahl: Nachprüfung über Register | CLAUDE.md §15 Z. 548 |
| R-212 | 15 | 567 | R | Lokale Suppressions nur mit Begründung, engster Scope | CLAUDE.md §15 Z. 552 |
| R-213 | 15 | 573 | R | Pauschale Suppressions freigabepflichtig | CLAUDE.md §15 Z. 558 |
| R-214 | 15 | 577 | R | Naming nach Sprachstandard; Inkonsistenz = Bug | CLAUDE.md §15 Z. 562 |
| R-215 | 16 | 582 | R | README folgt zwei festen Triggern, nie sporadisch | CLAUDE.md §16 Z. 567 – Wortlaut gestrafft |
| R-216 | 16 | 584 | R | Trigger 1: pro nutzerrelevantem Schritt (5 Fälle) | CLAUDE.md §16 Z. 569 |
| R-217 | 16 | 594 | R | Trigger 1: Voraussetzungen und .env.example im selben Commit | CLAUDE.md §16 Z. 579 – Wortlaut gestrafft |
| R-218 | 16 | 596 | R | Trigger 2: Sessionende-Synchronisation (Tabelle) | CLAUDE.md §16 Z. 581 – Einleitung gestrafft |
| R-219 | 16 | 615 | R | Drift README = Bug | CLAUDE.md §16 Z. 598 |
| R-220 | 16 | 617 | R | Inter-Pflicht-Drift-Tabelle (7 Anker) | CLAUDE.md §16 Z. 600 – Einleitung gestrafft (Beispiele gestrichen) |
| R-221 | 16 | 633 | R | Drift zwischen Pflicht-Dokumenten = Bug | CLAUDE.md §16 Z. 616 |
| R-222 | 16 | 635 | R | Trigger 3: Phasen-Abschluss-Re-Validation je Klasse | CLAUDE.md §16 Z. 618 – Klammerzusatz gestrafft |
| R-223 | 16 | 646 | R | Findings → eigener STABILISIERUNG-Schritt | CLAUDE.md §16 Z. 629 |
| R-224 | 16 | 650 | R | Badge-Maximalwerte je Klasse | CLAUDE.md §16 Z. 633 |
| R-225 | 16 | 651 | R | Pflicht-Badges | CLAUDE.md §16 Z. 634 |
| R-226 | 16 | 653 | R | Badges spiegeln reale Zustände | CLAUDE.md §16 Z. 636 |
| R-227 | 16 | 654 | R | Badge-Update im selben Commit | CLAUDE.md §16 Z. 637 – Wortlaut gestrafft |
| R-228 | 16 | 656 | R | Was die README nicht ist (4 Punkte) | CLAUDE.md §16 Z. 639 – vier Punkte in einem Absatz zusammengefasst |
| R-229 | 17 | 665 | R | Definition Onboarding-Pfad (5 Bestandteile) | CLAUDE.md §17 Z. 645 – Wortlaut gestrafft |
| R-230 | 17 | 673 | R | Quick-Start-relevante Artefakte | CLAUDE.md §17 Z. 647 – Einleitung gestrafft, Beispiele bei „Nicht Quick-Start-relevant" gestrichen; gilt jetzt auch als Definition für die DoD (R-136) |
| R-231 | 17 | 687 | R | Validierungs-Form (Mindestform 4 Schritte, erweitert G/V) | CLAUDE.md §17 Z. 661 – Beispiel gekürzt, „Erweiterte Form" verweist auf §16 Trigger 3 |
| R-232 | 17 | 700 | R | Auslöser-Trigger (Wiederholung aus 9 und 16) | gestrichen – reine Wiederholung von R-136 (§9), R-217 (§16 Trigger 1) und R-222 (§16 Trigger 3), die bestehen bleiben |
| R-233 | 17 | 710 | R | Verhältnis README, Runbook, Logbuch; Eintragstyp ONBOARDING-VALIDATION | CLAUDE.md §17 Z. 676 – Wortlaut gestrafft |
| R-236 | Schluss | 714 | R | Hinweis: generische Vorlage, Projektspezifika in docs/ | CLAUDE.md §17 Z. 680 |
