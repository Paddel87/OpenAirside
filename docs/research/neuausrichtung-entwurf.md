# Entwurf: OpenAirside als eigenes Spiel

<!-- Entwurf der KI vom 2026-10-29; Antworten des Projektinhabers vom selben Tag in Abschnitt 9 eingearbeitet. Noch nicht freigegeben. Ersetzt weder `docs/vision.md` noch
     einen anderen Pflicht-Text. Wird er angenommen, folgt ein ADR mit Tag [STRATEGISCH]
     (Vision-Pivot, CLAUDE.md Abschnitt 0 und 12); erst danach ändern sich Anforderungen
     und Fahrplan. -->

## 0. Worum es geht

Bisher ist OpenAirside als Nachbau eines bestimmten Spiels angelegt: Maßstab ist die
Funktionsgleichheit mit dessen Funktionsinventar. Dieser Entwurf beschreibt eine Fortsetzung
als **eigenes Flughafen-Aufbauspiel**. Es nutzt das Gebaute weiter, folgt aber einem eigenen
Entwurf mit eigenen Schwerpunkten und eigenen Erfolgskriterien. Wo es sich dadurch von
anderen Spielen des Genres unterscheidet, ist das gewollt.

Grundlage für die Schwerpunkte sind auch die unerfüllten Wünsche der Community des Genres
(Recherche in [`community-wuensche.md`](community-wuensche.md)).

## 1. Kernidee (Vorschlag)

**Ein Flughafen ist ein System aus Strömen: Menschen, Gepäck, Fracht, Fahrzeuge, Flugzeuge.
Der Spieler baut ihn, liest seine Ströme und baut ihn um, während er läuft – für Airlines,
die jeweils etwas anderes von ihm wollen.**

Das Spiel dreht sich nicht um das Abhaken von Einrichtungen. Es dreht sich um das Verstehen
und Verbessern eines lebenden Betriebs: Wo staut es sich, warum, und wie behebe ich das,
ohne den Betrieb anzuhalten?

## 2. Säulen

1. **Umbau im laufenden Betrieb.** Bauen kostet Zeit, Platz und Nerven. Baustellen sperren
   Flächen, Bauarbeiter brauchen Wege, Passagiere werden umgeleitet. Ein Terminal zu
   erweitern, während der Abendflug boardet, ist die eigentliche Aufgabe. Grundlage liegt
   vor: Bauaufträge, Anlieferstellen, Bauarbeiter, Gelände (3.1c).
2. **Ströme lesen.** Das Spiel macht sichtbar, was im Betrieb passiert: Dichte, Wartezeit,
   Wege, Engpässe. „Warum wartet das?“ ist kein Hilfetext, sondern ein Werkzeug, mit dem
   man spielt. Grundlage liegt vor: Wegweiser-Prinzip, erklärbare Unerreichbarkeit,
   Leistungsbewusstsein.
3. **Rhythmus statt Dauerlast.** Ein Flughafentag hat Wellen: Morgenspitze, Umsteigebank,
   Nachtruhe. Der Spieler plant gegen den Rhythmus, nicht gegen eine gleichmäßige Last.
   Wer ein Drehkreuz baut, gestaltet Umsteigewellen und Mindestumsteigezeiten. **Fracht** ist
   der Strom, der die Täler füllt: Beiladefracht reist mit den Passagierflügen über Vorfeld
   und Bänder, reine Frachtflüge belegen die Nacht. Dieselbe Bahn und dasselbe Vorfeld dienen
   zwei Rhythmen; Lärm und Nachtruhe setzen die Grenze. Grundlage liegt vor: Ladungsart
   „Fracht“ an den Zielen (FR-046), Feldsätze je Gruppe.
4. **Störung und Erholung.** Gewitter, IT-Ausfall, Drohne, gesperrte Bahn. Gewonnen wird
   nicht dadurch, dass nichts passiert, sondern dadurch, wie schnell der Betrieb sich erholt.
   Die vorhandenen Modernisierungen (Drohnen-, IT- und Stromausfall) passen hierher.
5. **Airlines als Partner mit eigenen Zielen.** Eine Airline ist kein Kunde, der nur Gates
   braucht. Jede hat ein Geschäftsmodell und misst den Flughafen daran: Der Billigflieger
   will kurze Bodenzeiten und niedrige Gebühren, der Netzcarrier kurze Umsteigezeiten und
   Lounges, die Frachtairline Nachtbetrieb und schnelle Umschlagzeiten. Verträge enthalten
   Gebühren und **messbare Zusagen** (etwa Bodenzeit oder Pünktlichkeit) mit Bonus und
   Abschlag. Der Spieler verhandelt, welche Zusagen er geben kann, und baut so, dass er sie
   hält. Hier verbinden sich Management und Bauen: Jede Zusage ist eine Anforderung an
   die Ströme.

Daneben bleiben die bisherigen Grundsätze: rechtwinkliges Raster, sehr große Flughäfen,
Erklären statt still bestrafen, offene Mod-Schnittstelle, Datenpakete. Neu als Grundsatz:

- **Lernen am eigenen Flughafen.** Die Einführung ist kein Text vorweg, sondern Teil der
  Kernschleife: Ein System wird erklärt, wenn es zum ersten Mal gebraucht wird, und
  zwar am Engpass, den der Spieler gerade sieht. Der Einstieg ist ein kleiner
  Bestandsflughafen mit einem ersten, lösbaren Problem; Gepäck, Zonen und Fracht kommen
  in späteren Lektionen dazu, jede mit eigenem Overlay.

## 3. Kernschleife

1. **Planen und bauen:** Bauaufträge setzen, Bauphasen festlegen, Sperrungen und Umleitungen planen.
2. **Betreiben:** Flüge laufen nach Tagesrhythmus, Ströme füllen den Flughafen.
3. **Lesen:** Overlays und Kennzahlen zeigen Engpässe und deren Ursache.
4. **Umbauen:** Im laufenden Betrieb verbessern; jede Baustelle stört, bis sie fertig ist.
5. **Bewerten und verhandeln:** Airlines messen den Flughafen an ihren Zusagen, Fluggäste an
   ihrem Erlebnis; gehaltene Zusagen bringen Bonus, neue Strecken und bessere Verträge,
   gebrochene kosten Geld und Vertrauen.

## 4. Eigene Mechaniken

| Mechanik | Säule | Kurzbeschreibung | Grundlage im Code |
|---|---|---|---|
| Bauphasen und Sperrungen | 1 | Bauaufträge werden zu Phasen gebündelt; eine Phase sperrt Flächen und legt Umleitungen fest, bevor gebaut wird | Aufträge, Feldsätze, Zugangsprofile |
| Baustellenlogistik | 1 | Anlieferung kreuzt Wege von Fluggästen und Fahrzeugen; Material muss durch, ohne den Betrieb zu blockieren | Anlieferstellen, Satz der Bauarbeiter |
| Störwirkung von Baustellen | 1 | Lärm und Enge an Baustellen senken das Erlebnis der Fluggäste in der Nähe; Stärke über die Schwierigkeit einstellbar | neu |
| Strom-Overlays | 2 | Dichte, Wartezeit, Herkunft und Ziel je Zelle; Vergleich vorher/nachher eines Umbaus | S-1, Wegweiser |
| Engpass-Diagnose | 2 | Das Spiel benennt die drei schlimmsten Engpässe mit Ursache (Kapazität, Weg, Personal) | erklärbare Unerreichbarkeit (3.3) |
| Tagesrhythmus | 3 | Flugplan in Wellen; Spitzen sind planbar und sichtbar | neu (Flugplan 3.6) |
| Umsteigebank | 3 | Mindestumsteigezeit als Kennzahl des Flughafens; Airlines wählen Drehkreuze danach | neu |
| Betriebstag mit Störung | 4 | Szenarien mit Störung am Morgen; bewertet wird die Erholung bis zum Abend | Ereignisse |
| Kennzahlen statt Sterne | 5 | Pünktlichkeit, Wartezeit, Umsteigequote, Gepäck-Fehlleitungen als Währung gegenüber Airlines | neu |
| Geschäftsmodelle der Airlines | 5 | Billigflieger, Netzcarrier, Regional, Fracht: je eigene Kennzahlen, Gebührenempfindlichkeit und Wünsche an Bauten | Datenpakete (Airlines als Daten) |
| Verträge mit Zusagen | 5 | Gebühren, Laufzeit und messbare Zusagen mit Bonus und Abschlag; Nachverhandeln, wenn sich der Flughafen ändert | neu |
| Gebührenordnung | 5 | Landeentgelt, Passagierentgelt, Nachtzuschlag, Rabatt für Umsteiger; jede Stellschraube zieht andere Airlines an | neu |
| Slots in Verträgen | 3, 5 | Knappe Zeiten in der Spitze werden als Slots in den Verträgen vergeben; Spitzenzeiten werden wertvoll | neu |
| Beiladefracht | 3 | Fracht reist mit Passagierflügen, nutzt Vorfeld und Gepäckbänder mit; Umschlag beeinflusst die Bodenzeit | Ladungsart „Fracht“ (FR-046) |
| Frachtnacht (später) | 3, 5 | Frachtflüge in der Nacht mit eigenem Vorfeld und Frachthalle; Lärmgrenzen und Nachtzuschlag setzen den Rahmen; nach der Beiladefracht | neu |
| Bestandsflughafen | 1, 2 | Startform „ein alter Flughafen auf Gelände“ zum Modernisieren, wählbar neben der leeren Karte; Einstieg und Szenarien | Gelände, Szenarien |
| Lektionen am Engpass | Grundsatz | Einführung je System beim ersten Gebrauch, am Engpass des Spielers, mit eigenem Overlay | erklärbare Unerreichbarkeit (3.3) |
| Bedienkomfort im Großen | Grundsatz | Vorlagen im Flugplaner, Personal in Gruppen einstellen, Sammelaktionen über viele Objekte | neu |
| Freiere Raumformen | 1 | Unregelmäßige Räume im Raster und Zusammenlegen von Räumen, ohne Kurven | Raster, Zonen |
| Läden mit Bestand | 5 | Läden und Gastronomie mit Lager, Lieferungen über Landseite oder Vorfeld und eigener Wirtschaft; Lieferungen sind ein weiterer Strom | neu |

Alle Zeilen sind vom Projektinhaber gewählt (2026-10-29). Die Reihenfolge der Umsetzung legt der neue Fahrplan fest.

## 5. Was aus dem Bestand bleibt

- **Technik:** Kern in Rust, Raster in Blöcken mit Etagen, Zonen und Flächenarten,
  Wegweiser-Prinzip mit Feldsätzen je Gruppe, Bauaufträge mit Bauarbeitern, Spielstände,
  Datenpakete, Lokalisierung, Grafik-Pipeline, Benchmarks als CI-Gate.
- **Genre-Grundlagen:** Terminals mit Check-in, Sicherheit, Gate und Boarding, Gepäck,
  Landseite, Vorfeld, Rollwege und Bahnen. Das gehört zu jedem Flughafenspiel und ist kein
  Nachbau.
- **Moderner Betrieb:** Die Modernisierungen aus `docs/modernisierung.md` bleiben Ideenquelle,
  sofern sie eine der Säulen tragen.

## 6. Was wegfällt oder neu entschieden wird

- **Funktionsgleichheit** als Ziel und Erfolgskriterium entfällt.
- **`docs/funktionsinventar.md`** ist nicht mehr Maßstab und wird nicht mehr als Anhang der
  Vision geführt. Es bleibt als eine Ideenquelle unter mehreren liegen (Entscheidung des
  Projektinhabers); eine Idee daraus kommt nur ins Spiel, wenn sie eine Säule trägt, und wird
  dann aus dem eigenen Entwurf begründet, nicht mit „das Vorbild hatte es“.
- **„Übernommen wie im Original“** (Vision Abschnitt 3) entfällt als Kategorie; jede dieser
  Funktionen wird nach dem eigenen Entwurf neu begründet oder gestrichen.
- **Anforderungen:** `docs/requirements.md` wird aus der neuen Vision abgeleitet. Muss-Anforderungen,
  die nur der Gleichheit dienen, werden neu bewertet.
- **Slot-System:** In der alten Vision verworfen; jetzt aufgenommen als Teil der Verträge
  (Säulen 3 und 5), Entscheidung des Projektinhabers vom 2026-10-29.
- **Fracht:** In der alten Vision eine Erweiterung nach der Funktionsgleichheit. Hier gehört
  sie zum Kern (Säule 3), mindestens als Beiladefracht; reine Frachtflüge können später kommen.
- **Positionierung:** Das Spiel ist ein eigenes Spiel im Genre, kein Ersatz für ein bestimmtes
  anderes. Spieler des Genres sind willkommen, auch die bisherige Zielgruppe.

## 7. Erfolgskriterien (Vorschlag)

- **Fesselnde Kernschleife:** Ein Spieler erkennt in seiner ersten Stunde einen Engpass,
  versteht die Ursache und behebt ihn durch einen Umbau im laufenden Betrieb. Maßstab bleibt
  der Initiator.
- **Umbau ohne Stillstand:** Eine Baustelle im Terminal führt zu Umleitungen und Wartezeit,
  nie zu einem Stillstand, der sich nicht selbst auflöst.
- **Lesbarkeit:** Jede Wartezeit über einer Schwelle ist mit Ursache im Spiel abrufbar.
- **Einstieg ohne Vorwissen:** Ein neuer Spieler betreibt nach der ersten Lektion einen
  kleinen Flughafen mit Gepäck, ohne außerhalb des Spiels nachzulesen; jedes System hat
  eine Lektion, die beim ersten Gebrauch erscheint.
- **Airlines mit Charakter:** Zwei Airlines mit verschiedenen Geschäftsmodellen verlangen
  erkennbar verschiedene Flughäfen; ein Spieler kann sagen, warum die eine bleibt und die
  andere geht.
- **Zwei Rhythmen:** Passagier- und Frachtverkehr teilen Bahn und Vorfeld, ohne Stillstand,
  der sich nicht selbst auflöst.
- **Leistung:** unverändert (Referenzszenario, 10.000 Personen, Bauen ohne Wartezeit,
  Langzeitstabilität).
- **Machbarkeitstest:** unverändert.

## 8. Auswirkungen auf Ablauf und Fahrplan

1. **Entscheidung:** Du wählst Kernidee, Säulen und Mechaniken (Abschnitte 1–4) oder
   formulierst sie um. Ergebnis wird ein ADR mit Tag `[STRATEGISCH]` und Verweis in
   `docs/vision.md`.
2. **Neuableitung:** Anforderungen und offene Fahrplan-Schritte von Phase 3 werden aus der
   neuen Vision abgeleitet (Vision-Abgleich nach `CLAUDE.md` Abschnitt 12).
3. **3.2 neu zuschneiden:** Vorschlag „Kleines Terminal und Passagierstrom“ mit der ersten
   Stufe der Säulen 1 und 2: Ein Terminal wird im laufenden Betrieb erweitert, eine Baustelle
   sperrt Fläche, Fluggäste werden umgeleitet, das Spiel zeigt die Wartezeit und ihre Ursache.
   Dieses Stück ist zugleich die erste Lektion des Einstiegs.
4. **Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“** durch eine getrennte Instanz,
   bevor Phase 3 weiterläuft.

## 9. Antworten des Projektinhabers (2026-10-29)

| Frage | Antwort |
|---|---|
| Kernidee | trifft genau, bleibt so |
| Säulen | alle fünf |
| Airlines (Säule 5) | voll: Geschäftsmodelle, Verträge mit Zusagen, Gebührenordnung |
| Fracht | zuerst Beiladefracht, Frachtnacht später |
| Startform | beides wählbar: Bestandsflughafen und leere Karte |
| Störwirkung von Baustellen | einstellbar über die Schwierigkeit |
| Slots | ja, als Teil der Verträge |
| Weitere Wünsche | Bedienkomfort im Großen, freiere Raumformen, Läden mit Bestand |
| Name | OpenAirside bleibt |
| `funktionsinventar.md` | bleibt als Ideenquelle, ohne Maßstab |
| Schritt 3.2 | neu zuschneiden wie in Abschnitt 8 |
| Ablauf | strategisches ADR, dann Neuableitung und Bewertung durch eine getrennte Instanz |
