# Wirtschaftlichkeit der Modellklassen

<!-- Projektübergreifender Anhang zu CLAUDE.md Abschnitt 0 (ausgelagert mit ADR-018).
     Lade-Disziplin: gehört NICHT zur Mindest-Lektüre. Geladen wird er nach
     CLAUDE.md Abschnitt 2 („Vertiefung auf Anforderung"), wenn die KI Arbeit an
     einen Unteragenten abgeben will, ein Probelauf ansteht oder die Kontingent-Warnung
     eingerichtet wird. Die Sessiongröße regelt
     CLAUDE.md Abschnitt 0 vollständig.
     Die Regeln hier gelten verbindlich wie CLAUDE.md; ausgelagert ist nur der Ort. -->

<!-- ANCHOR:arbeit-oberhalb-der-empfohlenen-klasse -->
## 1. Arbeit oberhalb der empfohlenen Klasse

Läuft Arbeit auf einer höheren Klasse als nötig, ist das eine Frage der Wirtschaftlichkeit, nicht der Korrektheit – ein Stopp wäre unverhältnismäßig (`docs/decisions.md` Regel-001, Ausnahme). Ein bloßer Hinweis an den Menschen hat sich im Pilotprojekt aber als wirkungslos erwiesen. Deshalb wirkt die Regel über einen Mechanismus, der kein Zutun des Menschen braucht:

1. **Abgabe an einen Unteragenten.** Kennt das Werkzeug Unteragenten mit fest eingestelltem Modell, gibt die KI Arbeit der Routine- und Mechanik-Klasse selbständig an einen Unteragenten dieser Klasse ab – insbesondere Logbuch-Einträge, README-Synchronisation, Drift-Prüfung, Archivierung und Fahrplan-Status-Updates. Der Unteragent bekommt einen abgegrenzten Auftrag mit den nötigen Quellen; die KI prüft sein Ergebnis vor dem Commit und verantwortet es. Welche Werkzeug-Einstellung das leistet, steht in `docs/project-context.md` Abschnitt 6. Die Abgabe gilt nur für Klassen und Aufgabenarten mit bestandenem Probelauf. Ein Unteragent lädt seinen Kontext neu; kleine Teilarbeiten, die mit dem bereits geladenen Kontext der aktiven Klasse erledigt sind, bleiben deshalb dort. **Abgegeben wird nur, was tatsächlich spart:** an eine Klasse, deren Preis für das Lesen aus dem Cache niedriger ist als der der aktiven Klasse, oder Arbeit, die überwiegend Ausgabe erzeugt (Entwürfe, längere Texte). Lesearbeit an eine Klasse mit gleichem Cache-Lesepreis spart nichts. Die Preise je Klasse stehen in `docs/project-context.md` Abschnitt 6.
2. **Warnung zu Beginn des Schritts.** Läuft ein Schritt trotzdem oberhalb seiner empfohlenen Klasse (keine Abgabe möglich, oder der ganze Schritt ist Routine), sagt die KI das einmal zu Beginn des Schritts – mit Bezug auf die knappe Ressource aus `docs/project-context.md` Abschnitt 6 (Kontingent oder Geld). Kein Stopp.
3. **Bilanz am Sessionende.** Der `[SESSIONENDE]`-Eintrag nennt die aktive Klasse, die Zahl der Schritte oberhalb ihrer Empfehlung und die abgegebenen Teilarbeiten.

<!-- ANCHOR:probelauf -->
**Probelauf.** Mechanik- und Routine-Klasse werden für die Abgabe erst aktiv, wenn ein Probelauf belegt, dass sie typische Aufgaben ihrer Liste in diesem Projekt korrekt ausführen: dieselbe Aufgabe einmal auf der Probe-Klasse und einmal auf einer höheren Klasse, Abweichungen bewertet, Ergebnis mit Datum in `docs/project-context.md` Abschnitt 6. Scheitert der Probelauf, bleibt die Klasse inaktiv und ihre Arbeit läuft eine Klasse höher. Nach einem Wechsel des Modells, das die Klasse besetzt, wird der Probelauf wiederholt (CLAUDE.md Abschnitt 6, „Schutzmechanismen durch erzwungenen Fehler belegen", sinngemäß).

<!-- ANCHOR:kontingent-warnung -->
## 2. Kontingent-Warnung

Meldet die Laufzeitumgebung den Verbrauch eines Nutzungskontingents, kann eine Warnung ab einer Schwelle eingerichtet werden (werkzeugspezifische Umsetzung und Stand in `docs/project-context.md` Abschnitt 6). Sie ist erst aktiv, wenn ein Probelauf mit absichtlich gesenkter Schwelle die Warnung tatsächlich bis ins Gespräch gebracht hat (CLAUDE.md Abschnitt 6, „Schutzmechanismen durch erzwungenen Fehler belegen").
