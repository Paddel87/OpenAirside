# Vision: OpenAirside

<!-- Eingangs-Dokument des Projekts. Erarbeitet in der Konzeptphase (Modus 1) am 2026-10-12.
     Bleibt nach der Initialisierung der anderen Dokumente unverändert als historisches Artefakt erhalten.
     Spätere Erkenntnisse landen in project-context.md, decisions.md, fahrplan.md.
     Verbindliche Anhänge:
       - funktionsinventar.md: Funktionsumfang des Originals (Maßstab für Funktionsgleichheit)
       - modernisierung.md:    entschiedene Abweichungen vom Original (Modernisierungen) -->

## 1. Kernidee

OpenAirside ist ein Open-Source-Nachbau des eingestellten Flughafen-Tycoon-Spiels Airport CEO. Der Spieler plant, baut und betreibt seinen eigenen Flughafen, vom kleinen Regionalflughafen bis zum internationalen Drehkreuz. Spielprinzip und Funktionsumfang folgen dem Original einschließlich seiner Add-ons, die technische Grundlage ist dagegen von Anfang an auf sehr große Flughäfen und heutige Hardware ausgelegt und das Spiel an den heutigen Flughafenbetrieb angepasst.

## 2. Problem und Anlass

- **Welches Problem löst das System?**
  Airport CEO wird nicht mehr weiterentwickelt. Das letzte inhaltliche Update erschien im Oktober 2023, im Januar 2026 folgte nur noch eine Sicherheitskorrektur, und das Studio erklärt die Entwicklung für abgeschlossen. Ein Nachfolger ist nicht angekündigt. Gleichzeitig bricht das Spiel bei großen Flughäfen ein: Das Bild ruckelt, Menüs öffnen sich nicht, Bauaufträge werden erst nach mehreren Sekunden übernommen, Passagiere finden ihre Wege nicht mehr und stauen sich vor dem Check-in, Fahrzeuge blockieren sich an Kurven und Kreuzungen.
- **Wer hat dieses Problem heute?**
  Spieler von Airport CEO, die große Flughäfen bauen wollen, allen voran der Initiator dieses Projekts. Darüber hinaus die Community des Originals, die das Ende der Entwicklung bedauert.
- **Wie wird das Problem heute gelöst (oder nicht)?**
  Gar nicht durch den Hersteller. Die Community behilft sich mit inoffiziellen Mods, etwa einem selbst gebauten Mod-Loader, einem Leistungs-Mod gegen den Speicherverbrauch von Airline-Mods und einem großen Umbau-Mod für Airlines und Verträge. Eine Open-Source-Alternative mit vergleichbarem Umfang existiert nicht.
- **Warum reicht das nicht?**
  Die Probleme liegen in der Architektur des Originals, nicht in einzelnen Fehlern. Die Grundsysteme entstanden 2015 bis 2017 für damals schwache Rechner (Mindestanforderung ein Core 2 Duo). Etagen, Zonen, internationale Bereiche, Notfälle und Forschung kamen erst 2019 und 2020 hinzu, ohne dass die Grundsysteme neu entworfen wurden. Das Studio selbst hielt einen besser aufgebauten Nachfolger für sinnvoller als eine Weiterentwicklung der alten Codebasis. Mods können solche Grenzen nicht beheben. Außerdem nutzt das Original moderne Mehrkern-Prozessoren und Grafikkarten kaum, kennt keine offizielle Mod-Schnittstelle, kein Cargo und läuft nicht unter Linux.

## 3. Zielbild

Der Spieler startet OpenAirside unter Windows, Linux oder macOS, auf Deutsch oder Englisch. Ein integrierter Benchmark hat beim ersten Start einen Flughafen im Zeitraffer wachsen lassen und empfiehlt eine Kartengröße, die der Rechner flüssig schafft. Der Spieler entscheidet frei und wird nur verständlich gewarnt, wenn er größer bauen will. Er baut im rechtwinkligen Raster in der Draufsicht: Start- und Landebahnen, Rollwege mit selbst gezogenen Mittellinien, Terminals über mehrere Etagen, Gepäckanlagen, Straßen und Parkhäuser. Bauaufträge werden sofort übernommen, egal wie groß der Flughafen ist. Tausende Passagiere folgen der Beschilderung und den Bodenmarkierungen, die der Spieler setzt, und finden auch ohne Schilder durch Suchen ans Ziel, nur langsamer. Fahrzeuge und Flugzeuge bewegen sich nach klaren Vorfahrtsregeln, eine automatische Verkehrssteuerung weist Rollrouten zu und löst Konflikte, bevor ein Stillstand entsteht. Wartet etwas, zeigt das Spiel warum. Der Flughafen klingt belebt, mit Stimmengewirr, Schritten, Rollkoffern, Triebwerken und Durchsagen. Abläufe wie Check-in, Sicherheits- und Gepäckkontrolle, Grenzkontrolle und Boarding entsprechen dem heutigen Flughafenbetrieb. Nach vielen Spielstunden läuft das Spiel noch genauso flüssig wie am Anfang.

**Kernumfang über das Original hinaus** (beschlossene Weiterentwicklungen):

- Integrierter Benchmark mit unverbindlicher Empfehlung zur Kartengröße und Warnhinweis bei schwacher Hardware
- Leistungsbewusstsein: Das Spiel erklärt dem Spieler, wann und warum sein Flughafen an Leistungsgrenzen stößt
- Erklärbare Wegfindungsprobleme: Bei fehlendem Weg zeigt das Spiel die Ursache (fehlende Zone, unterbrochene Verbindung, fehlende Station)
- Passagierführung über Schilder und Bodenmarkierungen als sichtbare Spielebene des Wegweiser-Prinzips; ohne Beschilderung finden Passagiere ihr Ziel durch Suchen, brauchen aber länger
- Straßenverkehr nach festen Regeln: eigene Fahrspuren je Richtung, Abbiegen behindert den Gegenverkehr nicht, kein Einfahren in Kreuzungen, die nicht geräumt werden können
- Vorfahrt standardmäßig automatisch, je Kreuzung auf manuell umschaltbar (Vorfahrtsschilder, Stoppstellen, Ampeln)
- Vorrang im Boden- und Rollverkehr: 1. Einsatzfahrzeuge zu einem Flugzeug in Not, 2. Flugzeuge beim Landen, Starten und Rollen (auch geschleppt), 3. übrige Einsatzfahrzeuge im Einsatz, 4. Pushback und Schlepp, 5. Abfertigung, 6. sonstige Betriebsfahrzeuge, 7. öffentlicher Verkehr
- Automatische Verkehrssteuerung in der Rolle von Tower und Rollkontrolle: weist Rollrouten zu, löst Konflikte nach der Rangfolge, greift bei drohender Blockade ein, nachvollziehbar für den Spieler
- Offizielle Mod-Unterstützung von Beginn an, plattformneutral (kein Store-gebundener Workshop)
- Sprachpakete für weitere Sprachen
- 19 Modernisierungen laut `modernisierung.md` (Kategorie „Kern"), u. a. Online-Check-in mit reiner Gepäckabgabe, Self Bag Drop, Kontrollspuren mit Technikstufen, dreistufige Gepäckkontrolle, moderne Grenzkontrolle, Startfreigabe am Gate, „Aufrollen und Warten", Drohnen-, IT- und Stromausfall als Ereignisse

**Übernommen wie im Original** (bewusst bestätigt):

- Rollwege mit vom Spieler gezogenen Mittellinien (Rollweg-Knoten), je Abschnitt einstellbar
- Trennung von ankommenden und abfliegenden Passagieren freiwillig
- Wählbare Kartengrößen zu Spielbeginn

**Beispielszenarien:**

- **Der Großflughafen-Bauer:** Ein Spieler startet auf einer großen Karte und verbringt die ersten Stunden nur mit Planen und Bauen: Start- und Landebahnen, Rollwege, Vorfeld, Terminals mit Check-in, Sicherheitskontrolle und Gates, Gepäckanlagen, Straßen und Bodenverkehrsdienste. Erst wenn alles steht, nimmt er die ersten Flüge an. Der Flughafen wächst über viele Spielstunden zu einem internationalen Drehkreuz mit mehreren Terminals und tausenden Passagieren. Das Spiel bleibt dabei flüssig, Bauaufträge werden sofort übernommen, und die Passagiere finden ihre Wege.
- **Der Einsteiger:** Ein neuer Spieler beginnt mit einem kleinen Regionalflughafen, auf dem Propellermaschinen starten und landen. Er lernt die Grundlagen und baut den Flughafen nach und nach aus.
- **Der Spieler mit schwächerem Rechner:** Ein Spieler mit Laptop und integrierter Grafik startet das Spiel zum ersten Mal. Der Benchmark lässt im Zeitraffer einen Flughafen wachsen und empfiehlt eine mittlere Kartengröße. Der Spieler wählt trotzdem die große Karte und wird verständlich darauf hingewiesen, dass sehr große Flughäfen auf seinem Rechner ruckeln können.

## 4. Erfolgskriterien

- **Gleichwertiges Spielerlebnis:** Das Spiel bietet ein ebenso interessantes und fesselndes Spielerlebnis wie das Original. Maßstab ist der Initiator: Eine Partie über mehrere Stunden, ohne dass Funktionen des Originals fehlen oder Langeweile aufkommt.
- **Funktionsgleichheit:** Alle Funktionen aus `funktionsinventar.md` einschließlich der DLC-Inhalte sind gleichwertig abgebildet, in der modernisierten Form laut `modernisierung.md`.
- **Leistung im Referenzszenario:** Auf der Zielhardware läuft folgender Flughafen flüssig, auch bei höchster Spielgeschwindigkeit, mit Reserve für größere Anlagen:
  - 3 Terminalbereiche in U-Form, 2 Start- und Landebahnen, ca. 17 Gates, mehrere tausend Passagiere gleichzeitig, mehrere Gepäckabfertigungsanlagen
  - Richtwert: mindestens 10.000 gleichzeitig simulierte Personen
- **Bauen ohne Wartezeit:** Bauaktionen werden sofort übernommen, unabhängig von der Flughafengröße.
- **Langzeitstabilität:** Die Leistung bleibt nach vielen Spielstunden gleich und baut nicht schleichend ab.
- **Kein Stillstand:** Im Passagier-, Straßen- und Rollverkehr gibt es keine Blockaden, die sich nicht von selbst auflösen. Ein solcher Stillstand gilt als Fehler, nicht als Spielsituation.
- **Untergrenze:** Auf Rechnern mit integrierter Grafik ist das Spiel lauffähig. Der Benchmark warnt rechtzeitig und nachvollziehbar, bevor die Flughafengröße das Spielerlebnis beeinträchtigt.
- **Machbarkeitstest:** Das Projekt liefert die Erkenntnis, wie weit ein Vorhaben dieser Größenordnung mit KI-gestützter Entwicklung umsetzbar ist. Diese Erkenntnis ist ein eigenständiges Ergebnis, auch wenn die Funktionsgleichheit nicht vollständig erreicht wird.

## 5. Bewusste Abgrenzung

**Was das System ausdrücklich NICHT tun soll:**

- **Keine Kurven beim Bauen.** Gebaut wird im rechtwinkligen Raster. Kurven machen das Spielprinzip komplex und überfordern, wie bei Rollercoaster Tycoon 3 gegenüber Teil 1.
- **Keine Übernahme von Material aus dem Original.** Kein Quellcode, keine Grafiken, Texte oder Töne, kein verwechselbarer Name, kein nachgezeichnetes Aussehen. Orientierung ausschließlich an Funktionen und Spielprinzip.
- **Keine Kompatibilität zum Original.** Weder Mods noch Spielstände des Originals werden unterstützt. Grafiken aus Community-Mods des Originals werden nicht übernommen (unklare Rechtslage, abweichende Auflösungen und Stile, technisch veraltet).
- **Kein Store-Vertrieb vorerst.** Kein Steam, keine anderen Software-Stores. Primärer Vertriebsweg ist GitHub.
- **Kein Fotorealismus.** Optik stilisiert und verspielt.
- **Keine Gewinnabsicht.** Das Spiel ist kostenlos.
- **Nicht im ersten Umfang:** Cargo und eine isometrische oder 3D-Ansicht (siehe Erweiterungen unten).
- **Keine Pflicht-Mechaniken, wo das Original frei war:** Trennung von Ankunft und Abflug bleibt freiwillig, Beschilderung ist hilfreich, aber nicht notwendig.
- **Verworfene Modernisierungen:** Gepäckabholung zu Hause, ferngesteuerter Tower, Slot-System.

**Erweiterungen nach Erreichen der Funktionsgleichheit:**

- **Fest vorgesehen:** Cargo. Die Architektur wird von Beginn an so angelegt, dass Cargo ohne Umbau ergänzt werden kann.
- **Unverbindliche Kandidaten** (ohne Zusage und Reihenfolge):
  - Leitungsbau für Kraftstoff (Hydranten) und Wasser
  - Neue Flugzeugtypen per Mod
  - Planmäßige Flugzeugwartung
  - Fernbahn neben der U-Bahn
  - Mietwagen
  - Nebel und Sichtweite als Mechanik
  - Lärm
  - Isometrische oder 3D-Ansicht als spätere Ausbaustufe (Simulation und Darstellung werden dafür von Anfang an getrennt)
  - Die 33 Modernisierungs-Kandidaten aus `modernisierung.md`, darunter die komplette Kategorie Abfertigung und Vorfeld

## 6. Harte Randbedingungen

- **Technologie:** offen, Entscheidung in der Initialisierung. Codesprache durchgehend Englisch, einschließlich Kommentaren.
- **Plattformen:** Windows, Linux und macOS.
- **Hardware:** Untergrenze sind Rechner mit integrierter Grafik, auch Macs. Ausgelegt und optimiert wird für aktuelle Mehrkern-PCs mit eigener Grafikkarte.
- **Hosting und Vertrieb:** primär GitHub, damit unabhängig von Plattformbetreibern. Software-Stores und die Apple-Signatur werden später gesondert entschieden.
- **Datenschutz/Compliance:** keine besonderen Anforderungen benannt.
- **Lizenzmodell:** Open Source. Code unter MIT. Selbst erstellte Grafiken unter einer freien Creative-Commons-Lizenz (voraussichtlich CC0), damit jeder frei darüber verfügen kann. Ton aus freien Bibliotheken und KI-generierte Sounds (z. B. ElevenLabs) sind erlaubt, abweichende Bedingungen werden im Repository vermerkt.
- **Zeitrahmen:** kein Termin. Hobbyprojekt.
- **Budget für externe Dienste:** Das Spiel ist kostenlos. Für die Entwicklung werden nach Möglichkeit kostenlose Werkzeuge genutzt. Einzelne Ausgaben werden bei Bedarf gesondert entschieden.
- **Sprachen:** zum Start Deutsch und Englisch. Weitere Sprachen modular über Sprachpakete, Übersetzungen durch die Community sind ausdrücklich erwünscht.

## 7. Weiche Präferenzen

- **Orientierung am Original:** Übernommen wird, was sinnvoll ist. Das Original ist Vorbild, nicht Vorschrift.
- **Modernisierungsgrundsatz:** Das Spielprinzip des Originals bleibt erhalten. Wo sich der reale Flughafenbetrieb seit der Konzeption 2015 grundlegend verändert hat, wird die moderne Variante umgesetzt, sofern sie mindestens so verständlich ist.
- **Atmosphäre:** Die Optik muss nicht realistisch sein, der Flughafen soll sich aber lebendig anfühlen, damit sich der Spieler mit seinem Flughafen identifiziert. Ton anfangs aus freien Bibliotheken, später mit mehr Realismus. Intro oder animiertes Hauptmenü können KI-gestützt entstehen.
- **Grafik aus einer Hand:** eigene SVG-Vektorgrafiken mit einem früh festgelegten Stilleitfaden (Palette, Strichstärken, Maßstab, Detailgrad). Sie werden einmal in der Entwicklung erstellt und im Spiel als Bilddateien in mehreren Auflösungen verwendet. Die Vektorquellen liegen im Repository.
- **Einfachheit vor Freiheit:** Das Spielprinzip soll zugänglich bleiben, zusätzliche Komplexität muss sich spielerisch lohnen.
- **Erklären statt still bestrafen:** Wenn etwas nicht funktioniert, zeigt das Spiel die Ursache.
- **Community:** Sobald das Projekt weit genug ist, sollen sich Freiwillige beteiligen können, etwa mit Grafiken, Airlines, Lackierungen und Übersetzungen. Inhalte erstellt die Community selbst.

## 8. Inspirationen und Vorbilder

- **Airport CEO (Apog Labs, 2017–2026):** Vorbild für Spielprinzip und Funktionsumfang, vollständig erfasst in `funktionsinventar.md`. Nicht übernommen werden Material jeder Art und die Architektur-Schwächen: Wegfindung pro Person, teure Neuberechnungen bei Bauänderungen, sich ansammelnde Prozesse über die Spielzeit, kaum genutzte Mehrkern-CPUs und Grafikkarten.
- **Prison Architect:** Vorbild für das rechtwinklige Bauprinzip in der Draufsicht.
- **Rollercoaster Tycoon 3:** bewusstes Gegenbeispiel. Zusätzliche Baufreiheit (Kurven) hat die Bedienung gegenüber Teil 1 überfrachtet.
- **OpenTTD und OpenRCT2:** Vorbild für Open-Source-Nachfolger von Tycoon-Spielen und für eine tragende Community. OpenTTD hat seine freien Grafiken über Freiwillige bekommen.
- **Spiele mit integriertem Benchmark:** Vorbild für die Hardware-Prüfung beim ersten Start.
- **Realer Flughafenbetrieb heute:** Vorbild für die Modernisierungen, u. a. die dreistufige Gepäckkontrolle nach europäischem Standard, gemeinsame Zeitplanung von Airline, Abfertigung, Flughafen und Flugsicherung, Vorrangregeln im Boden- und Rollverkehr, Leitlinien am Boden und Beschilderung zur Passagierführung. Details in `modernisierung.md`.

## 9. Bekannte Risiken und offene Punkte

- **Umfang:** Rund 150 Funktionen des Originals plus Weiterentwicklungen und Modernisierungen. Viele Systeme greifen ineinander (Gepäck, Flugplan, Personal, Bewertungen). Ob ein Projekt dieser Größe als Hobbyprojekt mit KI-gestützter Entwicklung fertig wird, ist offen und Teil des Machbarkeitstests.
- **Leistungsanspruch:** Das zentrale Erfolgskriterium ist zugleich der technisch schwierigste Teil: Wegfindung für zehntausend Personen, Bauänderungen ohne Wartezeit, Verkehr ohne Blockaden, Stabilität über lange Spielzeit.
- **Grafik:** Per Code erzeugte Vektorgrafik wirkt sauber, aber schlicht. Ob sich Grafiker als Freiwillige finden, ist ungewiss.
- **Spielgefühl und Balancing:** Ob das Spiel fesselt, lässt sich nur durch menschliches Testen beurteilen. Viele Formeln und Zahlenwerte des Originals sind nicht dokumentiert und müssen eigenständig gefunden werden.
- **Reichweite:** Ohne Steam erreicht das Projekt deutlich weniger Spieler. Die Community muss es aktiv finden, etwa über Foren und die bestehende Airport-CEO-Community. Davon hängt ab, ob sich Freiwillige finden.
- **Neubeginn bei Inhalten:** Ohne Kompatibilität zum Original fangen Wechsler bei Airlines und Lackierungen von vorn an. Das bremst anfangs die Attraktivität.
- **macOS:** Zwei Prozessorarchitekturen (ARM und Intel) müssen gebaut und getestet werden, ohne eigenen Mac ist das schwierig. Ohne Signatur warnt macOS bei der Installation.
- **Abstand zum Original:** Name, Optik und Beiträge von Freiwilligen müssen frei von Material des Originals bleiben. Reale Flugzeugtypen- und Airline-Namen sind ein Markenthema.
- **Möglicher Nachfolger:** Eine spätere Ankündigung eines Airport CEO 2 könnte die Motivation der Community für einen Open-Source-Nachbau dämpfen.
- **Wissenslücken zum Original:** Einige Mechaniken sind nur unvollständig belegt (siehe `funktionsinventar.md`, Abschnitt 18). Sie werden bei Bedarf in Erkundungsschritten geklärt oder bewusst eigenständig gelöst.

## 10. Was diese Vision nicht ersetzt

Dieses Dokument ist Eingang in die Konzeptphase, nicht ihr Ergebnis.
Es ersetzt **nicht**:

- die Architekturentscheidung (kommt in `architecture.md`)
- die Stack-Entscheidung (kommt in `project-context.md` und `decisions.md`)
- die Roadmap (kommt in `fahrplan.md`)
- die Constraints in operationalisierter Form (kommt in `project-context.md`)

Hinweise für die Initialisierung, die in der Konzeptphase entstanden sind:

- Erst klären, wie die Simulation aufgebaut sein muss, um mindestens 10.000 Personen zu tragen, danach die Engine wählen, nicht umgekehrt.
- Wegweiser-Prinzip statt Wegsuche pro Person prüfen (Aufwand abhängig von der Zahl der Ziele, nicht der Personen).
- Simulation und Darstellung trennen (Voraussetzung für eine spätere andere Ansicht und für den Benchmark).
- Flugzeugtypen, Airlines und Gebäude als Daten statt als Code anlegen (Grundlage für DLC-Umfang und Mods).
- Cargo in der Architektur von Beginn an vorsehen.
- Vektorgrafik als Quellformat, Rasterung in mehreren Auflösungen im Build.

---

**Überführungs-Status:**

- [x] Vision ausgefüllt (im Dialog erarbeitet, 2026-10-12)
- [x] Konzeptphase abgeschlossen (Lücken geschlossen, Optionen entschieden)
- [x] Härtungsphase abgeschlossen (Blocker und Inkonsistenzen geprüft; Blocker #001 gelöst)
- [x] Anforderungen geklärt (Modus 1.5, requirements.md, Anwendungsfälle und Muss-Anforderungen bestätigt am 2026-10-13)
- [x] Vorlagen-Set initialisiert (project-context.md, requirements.md, architecture.md, fahrplan.md, decisions.md, blockers.md, logbuch.md, onboarding-runbook.md, README.md)
- [x] ADR-001 angelegt: Anpassung des Vorlagen-Sets (Klasse G)
- [x] Datum der Initialisierungs-Abschluss: 2026-10-13

**Nach abgeschlossener Initialisierung:** Diese Datei wird nicht mehr verändert.
Spätere Vision-Erweiterungen oder Pivots werden in einem ADR dokumentiert,
nicht in dieser Datei. Bei substantieller Vision-Änderung: neuer ADR mit Verweis hier.
