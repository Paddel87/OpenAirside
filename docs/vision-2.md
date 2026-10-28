# Vision 2: OpenAirside

<!-- Eingangs-Dokument nach dem Vision-Pivot vom 2026-10-29 (ADR-046). Ersetzt `docs/vision.md`,
     die als historisches Dokument unverändert bleibt. Abgeleitet aus
     `docs/research/neuausrichtung-entwurf.md` mit den Antworten des Projektinhabers; gestützt auf
     `docs/research/community-wuensche.md`. Wird wie die erste Vision nicht fortgeschrieben:
     spätere Pivots in einem ADR. Ideenquellen ohne Maßstab: `funktionsinventar.md`,
     `modernisierung.md` (Regel-010). -->

## 1. Kernidee

OpenAirside ist ein freies Flughafen-Aufbauspiel. Ein Flughafen ist darin ein System aus Strömen:
Menschen, Gepäck, Fracht, Fahrzeuge und Flugzeuge. Der Spieler baut ihn, liest seine Ströme und
baut ihn um, während er läuft – für Airlines, die jeweils etwas anderes von ihm wollen.

Das Spiel dreht sich nicht um das Abhaken von Einrichtungen, sondern um das Verstehen und
Verbessern eines lebenden Betriebs: Wo staut es sich, warum, und wie behebe ich das, ohne den
Betrieb anzuhalten?

## 2. Anlass

- **Was fehlt Spielern heute?** Flughafen-Aufbauspiele brechen bei großen Flughäfen ein: Das Bild
  ruckelt, Bauaufträge brauchen Sekunden, Personen bleiben hängen, Fahrzeuge blockieren sich. Wenn
  etwas nicht funktioniert, sieht der Spieler selten, warum. Ist ein Flughafen fertig gebaut, bleibt
  oft nur noch Zuschauen.
- **Was wünschen sich Spieler des Genres?** Die Recherche (`docs/research/community-wuensche.md`)
  zeigt unerfüllte Wünsche: Umsteiger, Fracht, verlässliche Wege mit erkennbarer Ursache bei
  Problemen, Leistung im Großen, Entwürfe, die beim Ausbau nicht brechen, Offenheit für Mods, mehr
  Tiefe im Management, eine bessere Einführung.
- **Für wen?** Spieler von Aufbau- und Wirtschaftssimulationen, die große Flughäfen bauen und
  betreiben wollen. Maßstab für das Spielerlebnis ist der Projektinhaber.

## 3. Zielbild

Der Spieler startet OpenAirside unter Windows, Linux oder macOS, auf Deutsch oder Englisch, und
wählt einen Bestandsflughafen zum Modernisieren oder eine leere Karte. Er baut im rechtwinkligen
Raster in der Draufsicht: Bahnen, Rollwege, Terminals über mehrere Etagen, Gepäckanlagen, Straßen.
Bauen kostet Zeit: Bauarbeiter tragen Material zu den Baustellen, Baustellen sperren Flächen, und
Fluggäste werden umgeleitet. Der Flughafentag läuft in Wellen; in der Umsteigebank zählen Minuten.
Fracht reist mit den Passagierflügen mit. Overlays zeigen Dichte, Wartezeit und Engpässe mit ihrer
Ursache. Airlines mit verschiedenen Geschäftsmodellen schließen Verträge mit messbaren Zusagen und
messen den Flughafen daran. Ein Gewitter oder ein IT-Ausfall wirft den Betrieb zurück, und der
Spieler bringt ihn bis zum Abend wieder in Takt. Nach vielen Spielstunden läuft das Spiel noch
genauso flüssig wie am Anfang.

### Säulen

1. **Umbau im laufenden Betrieb.** Bauphasen mit Sperrungen und Umleitungen, Baustellenlogistik,
   Störwirkung von Baustellen auf Fluggäste in der Nähe (Stärke über die Schwierigkeit einstellbar),
   freiere Raumformen im Raster.
2. **Ströme lesen.** Overlays für Dichte, Wartezeit, Herkunft und Ziel; Vergleich vor und nach einem
   Umbau; Engpass-Diagnose mit Ursache (Kapazität, Weg, Personal).
3. **Rhythmus statt Dauerlast.** Flugplan in Wellen, Umsteigebank mit Mindestumsteigezeit als
   Kennzahl, Slots für knappe Zeiten in der Spitze. Fracht füllt die Täler: zuerst als
   Beiladefracht mit den Passagierflügen, später reine Frachtflüge in der Nacht.
4. **Störung und Erholung.** Wetter, IT-, Strom- und Drohnenausfall, gesperrte Bahn; bewertet wird,
   wie schnell sich der Betrieb erholt.
5. **Airlines als Partner mit eigenen Zielen.** Geschäftsmodelle (Billigflieger, Netzcarrier,
   Regional, Fracht) mit eigenen Kennzahlen; Verträge mit Gebühren, Laufzeit, Slots und messbaren
   Zusagen mit Bonus und Abschlag; Gebührenordnung; Läden und Gastronomie mit Bestand und
   Lieferungen.

### Grundsätze

- **Lernen am eigenen Flughafen:** Jedes System wird beim ersten Gebrauch erklärt, am Engpass, den
  der Spieler gerade sieht, mit eigenem Overlay. Der Einstieg ist ein kleiner Bestandsflughafen.
- **Bedienkomfort im Großen:** Vorlagen im Flugplaner, Personal in Gruppen, Sammelaktionen über
  viele Objekte.
- **Erklären statt still bestrafen:** Wenn etwas nicht funktioniert, zeigt das Spiel die Ursache.
- **Einfachheit vor Freiheit:** zusätzliche Komplexität muss sich spielerisch lohnen.

### Beispielszenarien

- **Die Einsteigerin:** Sie übernimmt einen kleinen Regionalflughafen, an dem sich morgens die
  Sicherheitskontrolle staut. Das Spiel zeigt den Stau und seine Ursache; sie baut eine zweite Spur
  im laufenden Betrieb und lernt dabei Baustellen und Umleitungen kennen.
- **Der Drehkreuz-Bauer:** Er baut über viele Stunden ein Drehkreuz. Ein Netzcarrier bietet einen
  Vertrag mit einer Zusage zur Umsteigezeit; er plant Wellen und Wege so, dass er sie hält, und
  erweitert dafür das Terminal, ohne die Abendbank zu stören.
- **Der Krisenmanager:** Ein Gewitter sperrt am Morgen die Bahn. Am Mittag stauen sich Gäste,
  Gepäck und Flugzeuge; bis zum Abend bringt der Spieler den Plan wieder in Takt.

## 4. Erfolgskriterien

- **Fesselnde Kernschleife:** Ein Spieler erkennt in seiner ersten Stunde einen Engpass, versteht die
  Ursache und behebt ihn durch einen Umbau im laufenden Betrieb. Maßstab ist der Projektinhaber:
  eine Partie über mehrere Stunden ohne Langeweile.
- **Umbau ohne Stillstand:** Eine Baustelle führt zu Umleitungen und Wartezeit, nie zu einem
  Stillstand, der sich nicht selbst auflöst.
- **Lesbarkeit:** Jede Wartezeit über einer Schwelle ist mit Ursache im Spiel abrufbar.
- **Einstieg ohne Vorwissen:** Ein neuer Spieler betreibt nach der ersten Lektion einen kleinen
  Flughafen mit Gepäck, ohne außerhalb des Spiels nachzulesen.
- **Airlines mit Charakter:** Zwei Airlines mit verschiedenen Geschäftsmodellen verlangen erkennbar
  verschiedene Flughäfen; ein Spieler kann sagen, warum die eine bleibt und die andere geht.
- **Zwei Rhythmen:** Passagier- und Frachtverkehr teilen Bahn und Vorfeld ohne Stillstand, der sich
  nicht selbst auflöst.
- **Leistung im Referenzszenario:** 3 Terminalbereiche, 2 Bahnen, ca. 17 Gates, mehrere
  Gepäckanlagen, mindestens 10.000 gleichzeitig simulierte Personen, flüssig auch bei höchster
  Spielgeschwindigkeit.
- **Bauen ohne Wartezeit:** Bauaufträge erscheinen sofort, unabhängig von der Flughafengröße; die
  Bauzeit ist Spiel, keine Rechenzeit.
- **Langzeitstabilität:** Die Leistung bleibt nach vielen Spielstunden gleich.
- **Kein Stillstand:** Im Passagier-, Straßen- und Rollverkehr gibt es keine Blockaden, die sich
  nicht von selbst auflösen.
- **Untergrenze:** Auf Rechnern mit integrierter Grafik lauffähig; der Benchmark warnt rechtzeitig.
- **Machbarkeitstest:** Das Projekt zeigt, wie weit ein Vorhaben dieser Größe mit KI-gestützter
  Entwicklung umsetzbar ist.

## 5. Bewusste Abgrenzung

- **Kein Nachbau eines bestimmten Spiels.** OpenAirside folgt seinem eigenen Entwurf; es wird nicht
  an der Funktionsliste eines anderen Spiels gemessen und nicht als Ersatz für ein anderes Spiel
  positioniert. Spieler des Genres sind willkommen.
- **Kein Material anderer Spiele** (Regel-005): kein Quellcode, keine Grafiken, Texte oder Töne,
  kein verwechselbarer Name, kein nachgezeichnetes Aussehen; keine Kompatibilität zu Mods oder
  Spielständen anderer Spiele.
- **Keine Kurven beim Bauen.** Gebaut wird im rechtwinkligen Raster.
- **Kein Store-Vertrieb vorerst**, primärer Weg ist GitHub.
- **Kein Fotorealismus.** Optik stilisiert und verspielt.
- **Keine Gewinnabsicht.** Das Spiel ist kostenlos.
- **Nicht im ersten Umfang:** reine Frachtflüge (Frachtnacht), isometrische oder 3D-Ansicht.

## 6. Harte Randbedingungen

Unverändert aus `docs/vision.md` Abschnitt 6: Codesprache Englisch; Windows, Linux und macOS;
Untergrenze Rechner mit integrierter Grafik; Vertrieb primär über GitHub; Open Source, Code unter
MIT, eigene Grafiken unter CC0, Ton aus freien Bibliotheken oder KI-generiert mit Vermerk; kein
Termin; kostenlose Werkzeuge nach Möglichkeit (BDR-001, BDR-002); Deutsch und Englisch zum Start,
weitere Sprachen über Sprachpakete.

## 7. Weiche Präferenzen

- **Realer Flughafenbetrieb heute als Vorbild**, wo er spielerisch trägt und mindestens so
  verständlich ist; `modernisierung.md` ist dafür eine Ideenquelle.
- **Atmosphäre:** Der Flughafen soll sich lebendig anfühlen und klingen.
- **Grafik aus einer Hand:** eigene SVG-Grafiken nach `docs/stilleitfaden.md`.
- **Community:** Freiwillige beteiligen sich mit Grafiken, Airlines, Übersetzungen und Mods.

## 8. Inspirationen

- **Flughafen- und Aufbauspiele des Genres**, darunter Airport CEO und SimAirport: Spielprinzip
  Flughafen in der Draufsicht. Ideen sind willkommen, wenn sie eine Säule tragen (Regel-010);
  übernommen wird kein Material.
- **Prison Architect:** rechtwinkliges Bauprinzip in der Draufsicht.
- **OpenTTD und OpenRCT2:** freie Spiele mit tragender Community.
- **Realer Flughafenbetrieb:** Wellen an Drehkreuzen, Mindestumsteigezeiten, Slot-Vergabe,
  Entgeltordnungen, Bauen im laufenden Betrieb, Erholung nach Störungen.

## 9. Risiken und offene Punkte

- **Spielspaß der Kernschleife:** Ob Umbau im Betrieb und Ströme lesen fesseln, zeigt erst ein
  spielbares Stück (neuer Schritt 3.2) und das Testen durch Menschen.
- **Umfang:** Fünf Säulen mit 20 Mechaniken sind viel für ein Hobbyprojekt; die Reihenfolge legt
  der Fahrplan fest, jede Mechanik muss sich einzeln lohnen.
- **Leistung:** Zusätzliche Ströme (Fracht, Lieferungen) und Feldsätze kosten Rechenzeit; die Grenze
  des Wegweiser-Benchmarks (4 s) hat auf dem Runner 1,1–2,1 s Abstand.
- **Balancing der Verträge und Gebühren:** Werte müssen eigenständig gefunden werden.
- **Reichweite:** Ohne Store muss die Community das Spiel aktiv finden.
- **Grafik:** Ob sich Grafiker als Freiwillige finden, ist ungewiss.

## 10. Was diese Vision nicht ersetzt

Architektur (`architecture.md`), Stack (`project-context.md`, `decisions.md`), Anforderungen
(`requirements.md`, neu abgeleitet nach ADR-046) und Fahrplan (`fahrplan.md`, Phase 3 neu geplant).
