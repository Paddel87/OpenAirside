# Funktionsinventar: Airport CEO (Original, Stand 1.1-3)

<!-- Anhang zur vision.md. Referenz für das Erfolgskriterium „Funktionsgleichheit mit dem Original".
     Erfasst WAS das Original kann (Funktionen, Regeln, Systeme), nicht WIE es aussieht oder umgesetzt ist.
     Alle Beschreibungen sind eigene Formulierungen. Zahlenwerte dienen als Orientierung für das Balancing,
     nicht als Pflichtvorgabe. Erstellt 2026-10-12 aus Wiki, offiziellen Patch Notes, Store-Seiten und Forum. -->

## Lesehinweise

- **IDs** (z. B. `PAX-03`) dienen als Referenz für Fahrplan und Funktionsgleichheits-Prüfung.
- **(unsicher)**: Quelle widersprüchlich, veraltet oder nur aus Spielerberichten.
- **[DLC]**: nur mit Add-on im Original.
- Eine Funktion gilt als „abgebildet", wenn das Spielprinzip gleichwertig umgesetzt ist, nicht wenn sie identisch aussieht.
- Abschnitt 17 listet, was es im Original **nicht** gab (Kandidaten für Erweiterungen). Abschnitt 18 listet offene Lücken der Recherche.

---

## 1. Spielstart, Meta und Einstellungen

| ID | Funktion |
|---|---|
| META-01 | Neues Spiel: Standort auf Weltkarte wählen. Bestimmt Land, Kontinent, Koordinaten, angebotene Routen, Inlands-/Auslandsanteil der Flüge und das Klima. |
| META-02 | Flughafenname, IATA-Code und Logo festlegen (Logos auch per Mod). |
| META-03 | Schwierigkeitsgrad (mindestens Leicht bis Extrem): steuert Startkapital und Häufigkeit von Notfällen. Genaue Stufen (unsicher). |
| META-04 | Kartengröße: normal oder groß (größere Karte nur bei neuem Spiel). |
| META-05 | Sandbox-Modus mit konfigurierbaren Regeln (u. a. unbegrenztes Geld, sofortige Forschung). Sandbox deaktiviert Erfolge. |
| META-06 | Gesperrte Kartenabschnitte, die gekauft/freigeschaltet werden müssen, bevor dort gebaut werden kann (Anzahl und Preise unsicher). |
| META-07 | Spielerbaute Weltzugangs-Tunnel am Kartenrand (bis zu 10), Richtung einstellbar. |
| META-08 | Zeitsteuerung: Pause, 1×, 2×, 3×. Nachtsprung 18:00→06:00, nur wenn kein Passagier im Terminal und kein Flugzeug in Bewegung. Option: bei Notfall auf Normaltempo zurückfallen. Option: Spiel pausiert laden. |
| META-09 | Kalender: ein Spieljahr = 12 Spieltage (relevant für Jahresauszeichnung). |
| META-10 | Speichersystem: manuelle Spielstände, Schnellspeichern, Autosave (Intervall einstellbar, Rotation), Vorschaubild je Spielstand. |
| META-11 | Einstellungen: Einheiten metrisch/imperial, Währung (15), 12/24-h-Uhr, Passagiere pro Flug 0–100 % (auch Leistungsregler), Kantenscrollen, Zoom-/Scrolltempo, Tastenbelegung, Zonen-Overlay-Deckkraft, Grafikmenü, saisonale Dekoration. |
| META-12 | Spielregel-Schalter: Schengen als Inland, „realistische Stellplätze" (international auf jeder Größe), realistische GA-Kraftstoffe, Boarding-Schalter automatisch mit Stellplätzen verbinden, neue Mitarbeiter automatisch Terminals zuweisen. |
| META-13 | Tutorial als Aufgabenkette mit Abschluss-Hinweisen und Willkommens-E-Mail mit Tipps. |
| META-14 | 12 Oberflächensprachen, keine Sprachausgabe. |
| META-15 | Konsole und Debug-Panel (Cheats, Testpassagiere erzeugen, Bausimulation abschalten). Nutzung deaktiviert Erfolge. |
| META-16 | Erfolge (36), siehe Abschnitt 16. |

## 2. Bausystem und Werkzeuge

| ID | Funktion |
|---|---|
| BAU-01 | Bauen im rechtwinkligen Raster; Fundamente, Böden, Wände, Fenster (mehrere Größen), Türen, Innenobjekte. |
| BAU-02 | Planungsmodus: Objekte als Plan platzieren, später bauen lassen. Ohne ausreichendes Geld keine Umsetzung. |
| BAU-03 | Simulierter Bau: Bauarbeiter (Vertragsfirma) tragen Material und errichten jedes Objekt; Material kommt per Lkw zur Anlieferstelle. Abladeplatz und Materialanlieferung mit Straßenanbindung nötig. Bauarbeiter werden angefordert und entlassen. |
| BAU-04 | Abriss und Versetzen gebauter Objekte mit Teilerstattung; Entplanen erstattet weniger als Abriss. |
| BAU-05 | Vorlagen-Werkzeug: Objektgruppen kopieren, einfügen, speichern (auch per Workshop teilen). Kopieren des Objekts unter dem Mauszeiger. |
| BAU-06 | Mehrfachauswahl, Auswahlrahmen, Pinsel-Werkzeug zum Flächenmalen, alternative Zieh-Methode. |
| BAU-07 | Mehrere Etagen: 5 Ebenen (2 oberirdisch, Erdgeschoss, 2 unterirdisch). Freischaltung per Forschung. Dachdarstellung. |
| BAU-08 | Vertikale Verbindungen: Treppe, schmale Treppe, Rolltreppe, Aufzug; dürfen nicht ins Leere führen, Zonen müssen auf beiden Ebenen passen. |
| BAU-09 | Laufbänder (Walkalator) mit Richtung. |
| BAU-10 | Terminal-Überführungen; unterirdische Straßen, Tunnel, unterirdische Gepäckbänder, unterirdische Depots/Parkplätze. |
| BAU-11 | Dekoration: Statuen, Blumenbeete, Pflanzen, Hecken, Bänke, Sofas, Steine, baubares Wasser (inkl. zufälliger Teiche), Zäune und Zauntore, platzierbare Logos und Aufkleber. |
| BAU-12 | Mülleimer; Passagiere erzeugen Müll, Mülleimer verändern das Verhalten. |
| BAU-13 | Kein Bau auf Wasser; Mindestgröße Terminal (16×16, unsicher); kein Terminal auf Stellplätzen/Enteisungsflächen. |

## 3. Zonen, Räume und Overlays

| ID | Funktion |
|---|---|
| ZON-01 | Zonen werden auf Flächen gemalt; Entfernen über eigenes Werkzeug. |
| ZON-02 | Personalzone: nur Personal; Personalräume automatisch Personalzone. Fehlerhafte Nutzung blockiert Passagierwege. |
| ZON-03 | Sicherheitszone: Abflug findet hier statt; Wechsel nur über Kontrolle (rein) bzw. Sicherheitsausgang (raus). Zebrastreifen können zoniert werden, damit die Zone eine Straße quert. |
| ZON-04 | Internationale Zone: nur innerhalb der Sicherheitszone; Wechsel nur über Passkontrolle (Richtung Ein-/Ausreise). Internationale Stellplätze brauchen Boarding-Schalter in dieser Zone. |
| ZON-05 | Trennung von abfliegenden und ankommenden Passagieren möglich. |
| ZON-06 | Terminalbereiche (Mehrterminal-Betrieb): Malwerkzeug für Terminalgebiete inkl. Stellplätze; Personal, Fahrzeuge und Stellplätze werden Terminals zugewiesen. Aufgaben im Terminal nur durch zugewiesenes Personal. |
| ZON-07 | Räume: Fläche markieren, Pflichtobjekte platzieren, Raum aktivieren. Typen: Personalraum, Gastronomie, Laden, Toilette, Lounge, Gepäckausgabe. Raumpanel für Vertrag, Airline, Zugang. |
| ZON-08 | Overlays: Zonen, Räume, Terminals, Gepäck, Warteschlangen, Begehbarkeit, Rollwege. |
| ZON-09 | Analyse-Werkzeuge: Weg- und Zonenanalyse, Heatmaps für Passagierankunft/-abflug und Flugzeugwege, Hervorhebung: Klick auf Objekt zeigt, wer es nutzt. |
| ZON-10 | Kamera folgt Person oder Fahrzeug. |

## 4. Passagiere

| ID | Funktion |
|---|---|
| PAX-01 | Ablauf Abflug: Ankunft am Verkehrsmittel → optional Annehmlichkeiten → Check-in (sobald Fenster offen) → Sicherheitskontrolle → Annehmlichkeiten im Sicherheitsbereich → Boarding → Abflug. |
| PAX-02 | Ablauf Ankunft: Aussteigen → ggf. Annehmlichkeiten (wenn nicht getrennt) → Gepäckausgabe → Sicherheitsausgang → Verkehrsmittel → verlassen. |
| PAX-03 | Bedürfnisse: Zufriedenheit, Blase (Toilette), Müdigkeit (Sitzplätze), Hunger (Gastronomie, Automaten), Unterhaltung/Einkauf (Läden). Unerfüllte Wünsche senken Zufriedenheit; zufriedenere Passagiere geben mehr aus. |
| PAX-04 | Individuelle Passagiere mit Namen, Persönlichkeit und Flug. |
| PAX-05 | Business-Class-Anteil je Flug mit Lounge-Berechtigung. |
| PAX-06 | Warteschlangen an allen Servicepunkten; Sitzplatzsuche auch über Etagen. Leerlauf: herumlaufen oder stehen. |
| PAX-07 | Passagiere können sich verirren; Infoschalter und Fluginformationsanzeigen (FIDS) helfen (Mechanik unsicher). |
| PAX-08 | Sicherheitsgefühl: Vorfälle senken Zufriedenheit. |
| PAX-09 | Besucher ohne Flug (Marketing-Forschung) nutzen Läden und Gastronomie landseitig. |
| PAX-10 | Passagierstatistiken. |

## 5. Landseite und Verkehr

| ID | Funktion |
|---|---|
| LAND-01 | Öffentliche Straßen (Asphalt, Beton), Einbahnstraßen, Tunnel, Zebrastreifen (einzige Querungsstelle), Gehwege. |
| LAND-02 | Haltestellen: Bus (fester Fahrplan), Auto (Bringen/Abholen), Taxi (inkl. Kleinbus), U-Bahn-Station (Forschung, ohne Straße, bei Abreise bevorzugt). Spiegeln möglich. Überfüllte Haltestelle meldet sich ab. Verteilung nach Gesamtkapazitäten. |
| LAND-03 | Öffentlicher Parkplatz: Kapazität 28, auch unterirdisch; Einstellungen: an/aus, Passagiere oder Personal, Langzeitparken (günstiger, weiter weg), Name. Gebühren kurz/lang getrennt. |
| LAND-04 | Personalparkplätze; Personal reist ebenfalls mit Verkehrsmitteln an. |
| LAND-05 | Fahrzeugkontrollpunkt als Übergang öffentliche Straße → Betriebsstraße; neue Fahrzeuge und Lieferungen fahren hindurch. |

## 6. Check-in

| ID | Funktion |
|---|---|
| CHK-01 | Schaltertypen: klein bemannt (1 Agent), mittel bemannt (1–2 Agenten, mit Gepäckband), Self-Check-in-Automat (braucht Gepäckabgabe in der Nähe), automatische Gepäckabgabe. Automaten per Forschung. |
| CHK-02 | Nicht im Sicherheitsbereich erlaubt. |
| CHK-03 | Jeder Flug braucht einen Schalter für sein Check-in-Fenster (30 min); ein Schalter bedient einen Flug gleichzeitig, viele über den Tag. Ohne freien Schalter ist der Flug nicht planbar. |
| CHK-04 | Schalterpanel: aktueller/nächster Flug, aktivieren, Personal zuweisen, Warteschlange bauen/verlängern, logische Verbindung zur Gepäckstation (zusätzlich physisches Band nötig). |
| CHK-05 | Personal geht nur bei offenem Check-in an den Schalter. |

## 7. Sicherheit

| ID | Funktion |
|---|---|
| SEC-01 | Sicherheitskontrollen klein/mittel/groß (1/4/5 Beamte, langsam bis schnell), optionale Warteschlangen. Jede Kontrolle nutzbar, nicht an Check-in gebunden. |
| SEC-02 | Sicherheitsausgang (auch personalzoniert). |
| SEC-03 | Passkontrolle manuell (1 Beamter) oder automatisch (Forschung), mit Richtung, optionale Warteschlangen. |
| SEC-04 | Schmuggelware/gefährliches Gepäck: Wahrscheinlichkeit sinkt mit Können und Zufriedenheit der Beamten und Gepäckscanstufe; Durchrutschen → Vorfall, Strafe, Sicherheitsbewertung sinkt. |
| SEC-05 | Sicherheitsstreifen zu Fuß und per Fahrzeug mit geplanten Routen; erhöhen Bewertung, Fahrzeugstreifen vertreiben Tiere. |
| SEC-06 | Flughafen-Polizeiwache mit Streifenwagen. |

## 8. Gepäck

| ID | Funktion |
|---|---|
| GEP-01 | Gepäcksystem per Forschung; Pflicht für mittlere und große Flüge, bei kleinen Stellplätzen nicht nötig. Global abschaltbar (Passagiere tragen Gepäck selbst). |
| GEP-02 | Passagiere geben Gepäck am Check-in ab, behalten Handgepäck. |
| GEP-03 | Förderbänder (normal, Hochgeschwindigkeit), nur 90°-Winkel, frei zusammenführbar, begehbar, einfärbbar; Gepäckrolltreppen auf/ab. |
| GEP-04 | Kipptablett-Weiche mit Routing-Modi (zum Ziel, geradeaus+links, geradeaus+rechts, links+rechts, geradeaus). |
| GEP-05 | Scanstufen: Stufe I allgemeiner Scanner (automatisch), Stufe II spezifische Scanner (Drogen, Waffen, Sprengstoff, Organisches, Geld), Stufe III manuelle Station (Beamter, Fehlerquote), Gepäckvernichter. Jeder Scanner mit Eingang, Annahme- und Ablehnungsausgang. Stufen per Forschung. Ohne Scan: Strafen, Bewertung sinkt. |
| GEP-06 | Gepäckstation: 12 Punkte (klein 1, mittel 1,5, groß 2), 2–16 Vorfeldagenten, Ankunfts- und Abflugseite, Betriebsstraßen ringsum. Verknüpfung Schalter→Station und Stellplatz→Station bestimmt Zuordnung. |
| GEP-07 | Gepäckwagen (klein, groß, groß mit Dach) mit Anhängern; bedienen jede Stellplatzgröße. |
| GEP-08 | Belader: Bandlader (mittlere Flugzeuge), ULD-Lader mit Containerpacken (große Flugzeuge). |
| GEP-09 | Gepäckausgabe-Raum: mit Station verknüpft, geschlossene Bandschleife (≥ 3 Bänder), im oder außerhalb des Sicherheitsbereichs auf dem Ausgangsweg; Ankunftsgepäck ungescannt. |
| GEP-10 | Gepäckstatistiken. |

## 9. Gates, Boarding und Stellplätze

| ID | Funktion |
|---|---|
| GATE-01 | Boarding-Schalter manuell (Agenten) oder automatisch (Forschung, schneller); mehrere je Stellplatz; Geschwindigkeit hängt von Können und Zustand ab. |
| GATE-02 | Stellplätze klein (5×7, GA oder Linie), mittel (10×10), groß (16×16, optional mittlere Flugzeuge). Oberflächen Gras/Asphalt/Beton je nach Größe. |
| GATE-03 | Fluggastbrücke als Upgrade für mittel/groß (an Etage 1), sonst Treppenfahrzeug; manche Typen mit eigener Treppe. |
| GATE-04 | Stellplatz-Einstellungen: GA/Linie, Airline-Bindung, international/national, Nachtflüge, Auto-Planer an/aus, Terminal-Zuordnung, Name, öffnen/schließen (storniert Flüge), Abriss, Reparatur, Flugzeug zwangsweise abfertigen. |
| GATE-05 | Stellplatz-Anforderungen: Betriebsstraße, Weg zur Sicherheit, Fahrzeugdepot/-parkplatz, Rollweg zur Bahn; Linie zusätzlich Boarding-Schalter, Gepäckstation, Weg im Sicherheitsbereich. |
| GATE-06 | Außenpositionen: beliebige Größe abseits des Terminals; Vorfeldbus-Haltestelle am Terminal (auch nur Ankunft), Servicewagen-Haltestelle für Vorfeldagenten, Vorfeldbus und Servicewagen; Flugplaner schlägt 45 min auf. |
| GATE-07 | Stellplatzgebunden zuweisbare Fahrzeuge: Pushback, Band-/ULD-Lader, Treppen, Vorfeldbusse (Größenregeln). |

## 10. Läden, Gastronomie, Lounges, Toiletten

| ID | Funktion |
|---|---|
| SHOP-01 | Läden (Franchise): Raum markieren, Vertrag wählen, Anforderungen erfüllen (Fläche, Regale, Kassen), Vertrag unterzeichnen. Land-, Sicherheits- oder internationaler Bereich. |
| SHOP-02 | Einnahmen: Miete je Fläche, Provision je Verkauf, Bonus bei Umsatzziel; Ziel 5 Tage verfehlt → Vertragsende. Angebot hängt vom Flughafen-Ruf ab. |
| SHOP-03 | Franchise-Personal kommt per Vertrag, Schichtwechsel; weite Wege schließen den Laden kurzzeitig. |
| SHOP-04 | Ladenprodukte in Regalen (per Mod erweiterbar). |
| FOOD-01 | Cafés und Restaurants (Forschung) in Gastronomieräumen, Anforderungen u. a. Fläche, Kühlschränke, Kassen, Kochstationen; Vertragslogik wie Läden. |
| FOOD-02 | Verkaufsautomaten mit Preisregler. |
| LOUNGE-01 | Airline-Lounges (Forschung): Sicherheits- oder internationaler Bereich, Buffet und Infoschalter Pflicht, Sitzplätze und eigene Toiletten üblich; einer oder allen Airlines zugeordnet; nur Berechtigte. |
| WC-01 | Toiletten: Pflicht, Mindestgröße 3×3, Toilette/Waschbecken/Urinal/Händetrockner; Geschlecht (m/w/alle), Zugang (Personal+Passagiere, nur Passagiere, nur Personal, nur Führung); optionale Nutzungsgebühr; Reinigung und Reparatur. |
| SEAT-01 | Sitzplätze (Gates, Gepäckausgabe, Wartebereiche). |

## 11. Flugbetrieb: Bahnen, Rollwege, Flugsicherung

| ID | Funktion |
|---|---|
| RWY-01 | Start-/Landebahnen klein/mittel/groß (mittel/groß per Forschung), Oberfläche Gras/Asphalt/Beton (Beton per Forschung), Oberfläche nachträglich änderbar, Größe nicht. Kreuzende Bahnen möglich. Richtung bei inaktiver Bahn umkehrbar. |
| RWY-02 | Bahnzugänge an beiden Enden (Einfahrt, Ausfahrt, beides), zusätzliche Abrollwege für kleinere Flugzeuge. |
| RWY-03 | Bahn-Einstellungen: nur Landung/nur Start/beides, erlaubter Verkehr (klein/mittel/groß, GA/Linie), Name, öffnen/schließen, Upgrades, Reparatur. Anzeige: Status, Kosten, Flüge in Warteschleife. |
| RWY-04 | Flugzeuge brauchen Bahn eigener Größe oder größer. Abflüge warten, bis Bahn frei ist (kein „Line-up and wait"). |
| RWY-05 | Warteschleife bei voller Bahn oder belegtem Stellplatz; Kapazität hängt von Towergröße ab. |
| RWY-06 | Globale Bahnzuweisung: gleichmäßig verteilen oder nächste Bahn; Regeln je Flugzeugtyp. |
| RWY-07 | Wind bestimmt Landerichtung; Durchstarten wahrscheinlicher bei Wind, seltener mit Anflugbefeuerung. |
| RWY-08 | Verschleiß durch Starts/Landungen (bessere Oberfläche hält länger); schlechter Zustand → Reifenplatzer-Notfall. Reparatur manuell oder automatisch (COO). |
| RWY-09 | Befeuerung: PAPI, Anflugbefeuerung (Forschung), Bahnendbefeuerung, ILS-Objekt (unsicher), Flutlicht; Windsack (optisch). Lichtschaltzeiten einstellbar. |
| TWY-01 | Rollweg-Fundament (Gras/Beton/Asphalt) plus Rollweg-Knoten als Wegenetz (horizontal, vertikal, diagonal), Kreuzungen automatisch. |
| TWY-02 | Knoten-Einstellungen: Einbahn, nur Ankunft/Abflug, Gewichtsklassen-Beschränkung. Keine automatische Flügelfreiheitsprüfung. |
| TWY-03 | Flugzeuge reservieren Wegabschnitte und warten bei Belegung; Betriebsstraßen dürfen Rollwege kreuzen. |
| ATC-01 | Tower klein/mittel/groß Pflicht für jeden Flug; nur der größte zählt; Tageslimit für Flüge (ca. 25/75/200, unsicher für 1.0). |
| ATC-02 | Radar mittel/groß erhöht Tageslimit (unsicher: +50/+100). Sandbox ohne Limit. |
| ATC-03 | Wetterstationen klein/groß: ohne keine Wetterdaten; große zeigt Unwetter-Vorhersage (bis 10 Tage). |

## 12. Flüge, Planung und Airlines

| ID | Funktion |
|---|---|
| FLT-01 | Flugarten: GA (ungeplant, kommt bei freiem Stellplatz, parkt Stunden, zahlt Standgebühr je Stunde; Schalter „GA erlauben"; GA-Bewertung steuert Menge), Linienflüge (aus Airline-Verträgen), Notfall-/Ambulanzflüge, Hubschrauberflüge [DLC]. |
| FLT-02 | Voraussetzungen Linienflug: Vertrag, Tower, Lizenz-Forschung, passender verbundener Stellplatz, besetzte Check-in-, Sicherheits- und Boarding-Stationen. |
| FLT-03 | Flugplaner: Flüge per Ziehen auf Zeitleiste je Stellplatz legen, bestätigen/sperren; mind. 3 h Vorlauf (Stellplatzgröße 1–3 h); Ansicht heute + 6 Tage; wöchentlich wiederkehrende Flüge; Einzelflug stornierbar; Filter nach Größe. |
| FLT-04 | Ohne Nachtflug-Forschung Landungen ab 05:00, Abflüge bis 22:00. |
| FLT-05 | Auto-Planer (braucht COO und CIO), je Stellplatz schaltbar. |
| FLT-06 | Verspätungsdialoge mit automatischen Reaktionen; manuelles Abfertigen. |
| FLT-07 | Flugmonitor: Zeile je Flug mit Airline, Größe, Stellplatz, Status (unterwegs, verspätet, gelandet, Boarding, Rollen, Pushback …), Passagiere ab/an, Zeiten, Live-Zähler (angereist, eingecheckt, Sicherheit, ausgestiegen, eingestiegen) und Service-Fortschritt (Service-Runde, Gepäck ab/an, Treibstoff, Catering, Reinigung); Warnungen bei Engpässen. |
| AIR-01 | Airlines mit Sterneklasse 1–5 (1–2 Sterne ohne große, 4–5 Sterne ohne kleine Flugzeuge), Flotte nach Typen, Flugpräfix, Lackierungen (mehrere/seltene je Typ). Ca. 26 Basis-Airlines plus DLC-, Ambulanz- und Mod-Airlines. |
| AIR-02 | Airline-Vertrag: Zahlung je Flug, Provision je Passagier, Servicepreise, teils Aufgaben mit Frist und Bonus. |
| AIR-03 | Beziehung zur Airline: steigt bei Pünktlichkeit, sinkt bei Ausfall/Verspätung; höhere Beziehung → mehr Flugangebote. Erreichbare Sterneklasse hängt von Flughafenbewertung und angebotenen Services ab. |
| AIR-04 | Vertragsverhandlung: bestehende Verträge nachverhandeln (Verhandlungspunkte, Details unsicher). |
| AIR-05 | Flugrouten aus realen Verkehrsdaten, abhängig vom Standort; Inlands-/Auslandsflüge nach Land, Schengen-Regel. |

## 13. Abfertigung und Versorgung (Turnaround)

| ID | Funktion |
|---|---|
| TURN-01 | Service je Flugzeugklasse: Betankung und Enteisung (alle), Service-Runde und Gepäck (Linie), Catering, Kabinenreinigung, Pushback (mittel/groß). Jeder Service global schaltbar, mit eigener Gebühr (CFO). |
| TURN-02 | Reihenfolge: Reinigung nach Aussteigen, Boarding nach Reinigung. Fahrzeug ohne Vorrat → automatischer Folgeauftrag. |
| TURN-03 | Service-Runde: Vorfeldagenten inspizieren (klein 2, mittel 4, groß 6–8); dabei „nicht flugtauglich"-Notfall möglich. |
| FUEL-01 | Zwei Kraftstoffe (Avgas, Jet A-1), Zuordnung nach Typ (realistische GA-Option). |
| FUEL-02 | Tanklager klein/mittel je Kraftstoff, Zusatztanks an mittleren Lagern (bis 6), Forschungskette; auch unterirdisch. |
| FUEL-03 | Lieferantenvertrag (ein aktiver, Grundgebühr + Literpreis, Klassen mit Liefermengen); automatische Nachbestellung mit Schwelle und Ziel-Füllstand, abschaltbar. |
| FUEL-04 | Tankwagen in Varianten (Kraftstoff × Marke × Solo/Anhänger) mit Preis, Betriebskosten und Ladezeit; große Flugzeuge brauchen mehrere Fahrten. |
| FUEL-05 | Treibstoffkrise: Preisspitze für 2–5 Tage; Vorrat in Tanks als Gegenmittel. |
| ICE-01 | Enteisung bei Frost: Flüssigkeitsvertrag → Lieferung ins Depot → Enteisungsfahrzeug → Enteisungsfläche klein/mittel/groß (1/2/4 Fahrzeuge) auf dem Abflugweg. Abflug ohne Enteisung → Vorfall. |
| CAT-01 | Catering (Forschung): Lebensmittellieferant → Depot wandelt in Mahlzeiten (Tempo-Regler gegen Kosten) → Cateringwagen holt genau die Mahlzeiten des Flugs → Stellplatz. Nur mittel/groß; große Stellplätze 2 Wagen. |
| CLEAN-01 | Kabinenreinigung (Forschung): Reinigungswagen sammelt Kabinenmüll, leert ins Abfalldepot; volles Depot wird von städtischer Müllabfuhr geleert. Schwellen-Regler. |
| PUSH-01 | Pushback klein (mittlere Stellplätze/Hangars) und groß (große), Richtung einstellbar; kleine Flugzeuge drehen selbst. |
| VEH-01 | Fahrzeugkauf mit Preis, Unterhalt und Lieferzeit; Lagerung in Fahrzeugdepot oder Betriebsparkplatz (max. 7); Verkauf; Zuweisung zu Parkplätzen/Terminals; individuelle Lackierung. |
| VEH-02 | Betriebsstraßen (Gras/Asphalt/Beton), nur Betriebsfahrzeuge, Einschränkung je Fahrzeugtyp, Einbahn. |

## 14. Hangars, Notfälle, Wetter, Tiere

| ID | Funktion |
|---|---|
| HAN-01 | Hangars klein/mittel/groß: Reparatur beschädigter Flugzeuge (15–30 h, Gebühr) oder Zerlegen von Wracks auf eigene Kosten; Einstellungen GA/Linie, Airline, große auch für mittlere. Rollweg, bei mittel/groß Betriebsstraße und Pushback. Keine planmäßige Wartung. |
| EMG-01 | Notfälle erst nach Lizenz-Forschung und einigen Spielstunden; Häufigkeit nach Schwierigkeit; bis zu 24 Typen. |
| EMG-02 | Aktive Notfälle Ankunft (annehmen/ablehnen, Stellplatz binnen 30 min): Triebwerksausfall, Geräteausfall, Wetterwarnung, Treibstoffwarnung, medizinischer Notfall, Ambulanzflug (binnen 48 h, Gate 2 h), Sicherheitsproblem, Triebwerksbrand (unlöschbar → Totalverlust). |
| EMG-03 | Aktive Notfälle Abflug: nicht flugtauglich, Vogelschlag beim Start, Reifenplatzer. |
| EMG-04 | Weitere: Überspannung (alle IT-Geräte defekt), Sanitärausfälle, Passagiervorfälle, Streik. |
| EMG-05 | Passive Ereignisse: Treibstoff-/Ölkrise, Wirtschaftskrise, Pandemie, Vulkanausbruch (Asche, Flugverschiebung), Unwetter, U-Bahn-Signalstörung. |
| EMG-06 | Nachträgliche Vorfälle: gefährliches Gepäck an Bord, fehlende Enteisung, schwache Sicherheitsbeamte → Strafe, Bewertung, Behörden-E-Mail. |
| EMG-07 | Bewertung A–F je Notfall; Erfolg: Sicherheitsbewertung, Airline-Beziehung, Geld; Misserfolg/Ablehnung: Strafe, Bewertungsverlust. Zeitlimit je Notfall. Bericht-E-Mail und Untersuchungspanel. |
| EMG-08 | Rettungswache (Feuerwehr, Rettungswagen; Zufahrt zu jeder Bahn), Polizeiwache (Streifenwagen). |
| WX-01 | Klima nach Standort (polar, gemäßigt, tropisch, Wüste): Wetter, Bodentexturen, Tierarten. Temperatur nach Tageszeit und Jahreszeit. |
| WX-02 | Simuliert: Temperatur, Wind (Stärke, Richtung), Niederschlag, Schnee am Boden; Sonnenlicht nach Wetter und Tageszeit; saisonale Bodenübergänge. |
| WX-03 | Auswirkungen: Landerichtung, Durchstarten, Enteisung, Unwetter-Verspätungen bis Ausfall; mit Vorhersage frühzeitig stornierbar (Stärke der Effekte unsicher). |
| ANI-01 | Bodentiere (Füchse, Kühe) vom Kartenrand auf Rollwege/Bahnen, blockieren Flugzeuge; Zäune, Kontrollpunkte und Straßen halten auf; Streifen vertreiben. |
| ANI-02 | Vögel verursachen Vogelschlag; Vogelabwehranlage an Start- und Landezonen. |

## 15. Management, Personal, Wirtschaft, Forschung

| ID | Funktion |
|---|---|
| STAFF-01 | Personaltypen: Passagierservice-Agent, Sicherheitsbeamter, Vorfeldagent, Verwaltungsmitarbeiter (Forschung, Bürozeiten, Schreibtisch im Personalraum), Reinigungskraft, Servicetechniker. Dazu Franchise-Personal und Bauarbeiter per Vertrag. |
| STAFF-02 | Einstellen aus Bewerberliste (Name, Aussehen, Gehaltswunsch, Können), Filter; Schulung (Können ↑, Gehalt ↑, Einmalkosten); Entlassen; Person auf Karte finden. |
| STAFF-03 | Können beeinflusst Arbeitstempo und Passagierzufriedenheit; Müdigkeit senkt es. Schichtlänge als Abwägung (genaue Steuerung unsicher). |
| STAFF-04 | Bedürfnisse: Pausen im Personalraum, Toiletten, Personalparkplätze. Zufriedenheitswert je Mitarbeiter. |
| STAFF-05 | Gewerkschaft: Gehaltsverhandlungen, Streiks mit Abklingzeit. |
| STAFF-06 | Aufgabenverteilung nach Entfernung, ausgeruhte Mitarbeiter bevorzugt; Angebots-/Nachfrage-Panel der Aufgaben. |
| EXEC-01 | Führungskräfte (anstellbar, wirken nur bei Anwesenheit): CFO (Gebühren-Tab, tägliche Finanzberichte), COO (automatische Reparatur, Betriebsberichte, Auto-Planer mit CIO), CIO (Auto-Planer, Programm-Effizienz +5 %), HR-Direktor (Gehalts-Tab, Produktivität), Strategiedirektor (Pflicht für Franchise-Forschung, Forschungstempo), Einkaufsdirektor (Beschaffung). |
| ECO-01 | Einnahmen: Passagierausgaben, Landung/Start, Aus-/Einsteigen, Servicegebühren, Standgebühren (Hauptquelle), Treibstoffverkauf, Hangargebühren, Franchise-Miete/-Provision, Parken, Toilettengebühr, Automaten. |
| ECO-02 | Ausgaben: Bau, Bauarbeiter, Unterhalt je Kachel, Reparaturen, Gehälter, Fahrzeuge, Kredite, Steuern, Strafen, Lieferanten. |
| ECO-03 | Gebühren-Regler (mit CFO) beeinflussen Zufriedenheit und Bewertung („passend zur lokalen Wirtschaft"). Gehalts-Regler je Personaltyp (mit HR). |
| ECO-04 | Kredite: drei Angebote (Betrag, Zins, Laufzeit), Rate je Tick, vorzeitige Tilgung (nicht bei negativem Saldo). |
| ECO-05 | Budgetübersicht mit Kategorien, Stundensaldo, Nettowert, tägliche Berichte per E-Mail. |
| ECO-06 | Beschaffung: Produkte mit Preis, Unterhalt, Lieferzeit, Abhängigkeiten; Freischaltungen über Führungskräfte. |
| RD-01 | Forschung: Projekt wählen, Verwaltungsmitarbeiter zuweisen (Min/Max je Projekt), Dauer je Projekt; parallele Projekt-Slots per Forschung (bis 3). Ziel: alles erforschen. |
| RD-02 | Projektbaum (Auszug): Linienlizenz → Mittel-/Großflugzeug-Genehmigung, Bandlader, Nachtflüge → Anflugbefeuerung, Etagenbau, Außenpositionen, Erweiterte Lizenz (mittlerer Tower, Lounges) → Ultimative Lizenz (großer Tower), U-Bahn, Kabinenreinigung, Catering, Enteisung, Mehrterminal, Automatisierung. Gepäck → Kipptablett, Hochgeschwindigkeitsband, Scanstufen 1–3. Jet A-1, Tankwagen-Kette, Tanklager → Zusatztanks. Franchise-Läden/-Cafés → Restaurants. Bautechnik (Beton). Programmgruppen 1–3, proaktive Wartung. Rettungsdienste, Hangars. Hubschrauber [DLC]. |
| RD-03 | Programme (Dauerboni, max. 2 je Gruppe, Effizienz baut sich auf): Gruppe 1 Unterhalt/Gehälter/Reparatur −5 %, Bautempo +10 %; Gruppe 2 Forschung +10 %, Passagierzufriedenheit +5 %, Personalkönnen +8 %, Marketing +5 %; Gruppe 3 Airline-Zufriedenheit +5 %, Sicherheitsrisiko −5 %, Lieferantenrabatt −5 %, Notfallrisiko −5 %. |
| OPS-01 | Betriebspanel: Flughafen offen/zu, GA, Nachtflüge, Lichtzeiten, Bahnzuweisung, Gepäck an/aus, proaktive Reinigung/Reparatur, Services an/aus, Schwellen für Müll und Catering, Bau-Tab, Terminalzuweisung. |
| RATE-01 | Flughafenbewertung mit Teilwerten (u. a. Sicherheit, GA), Tooltip mit Erklärung und Verbesserungstipps; Gesamtwert bestimmt erreichbare Vertragsklasse. Einflussfaktoren u. a. Preise, Verspätungen, Angebot an Läden/Gastronomie, Warteschlangen, Verkehrsanbindung, Wege, Sitzplätze/Toiletten, Gepäckscan, Streifen, Dekoration, Personalzufriedenheit. |
| RATE-02 | Auszeichnung „Flughafen des Jahres" einmal pro Spieljahr nach Durchschnittszufriedenheit, Top 10, Preisgeld 2 Mio. |
| MAIL-01 | Posteingang: Bürgermeister, Lieferungen, Tagesberichte, Notfallberichte, Verhandlungsanfragen, Auszeichnung. Benachrichtigungen in der Spielwelt. |

## 16. Inhalte, Modding und Erfolge

| ID | Funktion |
|---|---|
| ACFT-01 | Flugzeugklassen: GA (9 Typen), klein Linie (6), mittel (23), groß (16). Jeder Typ mit Passagierkapazität, Kraftstoff, Größenklasse. |
| ACFT-02 | [DLC] Vintage: 7 Typen (2 klein, 5 mittel) mit eigenen Vintage-Airlines; Vintage-Stellplatz-Optik. |
| ACFT-03 | [DLC] Beasts of the East: 4 Typen (3 mittel, 1 groß), 2 Airlines. |
| ACFT-04 | [DLC] Supersonic (kostenlos): 2 große Überschallflugzeuge in bestehenden Airlines, eigene Sounds und Animationen. |
| ACFT-05 | [DLC] Hubschrauber: 5 Typen (3 klein, 2 mittel), Hubschrauberlandeplätze klein/mittel (ohne Fluggastbrücke), reiner Hubschrauber-Flughafen oder gemischt, 2 Forschungsprojekte, eigene Flugerzeugung, Planung, Abfertigung, Vorfälle, 2 Airlines. |
| MOD-01 | Integrierte Mod-Verwaltung (lokal und Workshop), Mod-Menü, Upload aus dem Spiel. Mod-Typen: Airlines (Logo, Flotte, Präfix, Farben, Sterne, Lackierungen), Lieferanten, Franchises, Baufirmen, Banken, Ladenprodukte, Flughafenlogos, Vorlagen, Aufkleber, Spielstände. Keine neuen Flugzeugtypen, keine Code-Mods offiziell. |
| ACH-01 | Erfolge u. a.: Abfertigungen pro Tag, Abflüge, Passagierzahlen, Passagiere mit hoher Bewertung, Personalzufriedenheit, Gepäck-Umleitungen und manuelle Scans, alle Forschungen, Gesamtbewertung ≥ 95 %, Flughafen des Jahres, Kredit getilgt, Nettowert, jeden Flugzeugtyp gelandet, Abfertigung mit allen Services. |
| AUDIO-01 | Musik (ca. 1 h Soundtrack), Soundeffekte für Flugzeuge (Triebwerke, Überschall), Umgebung, Oberfläche; Hubschrauber-Sounds [DLC]. |

## 17. Nicht im Original enthalten (Erweiterungskandidaten)

- **Cargo/Fracht:** nie umgesetzt, meistgewünschte Funktion. In unserer Vision: Erweiterung nach Funktionsgleichheit, Architektur von Beginn an vorbereitet.
- Lärmsystem, Nebel/Sichtweite als Mechanik, Treibstoff-Pipelines/Hydranten, planmäßige Flugzeugwartung, Mietwagen, Fernbahn (nur U-Bahn).
- Offizielle Code-Mod-Schnittstelle und neue Flugzeugtypen per Mod.
- Linux-Unterstützung (Original: Windows, macOS).
- Kurven beim Bauen: im Original nicht vorhanden, in unserer Vision bewusst ausgeschlossen.

## 18. Offene Lücken der Recherche

- Genaue Formeln: Passagierbedürfnisse, Bewertungen, Verhandlungspunkte, Streikauslöser.
- Genaue Werte: Tower-/Radar-Limits in 1.0, Preise vieler Gebäude und Fahrzeuge, Kapazitäten von Catering-/Reinigungs-/Tankwagen, Bahnlängen der Größenklassen, Kreditwerte, Steuersatz, Kartenabschnitt-Preise.
- Unklare Mechaniken: Infoschalter, Verkaufsautomaten, Schichtsteuerung, Zoll-Schalter (nur Store-Text), Wirkung von PAPI/ILS, Stärke der Wettereffekte.
- Widersprüche: Personal und internationale Zone (frei vs. Kontrolle), Einführungszeitpunkt Parkplätze.

Diese Lücken sind kein Hindernis für die Vision. Sie werden bei Bedarf in Erkundungsschritten geklärt oder bewusst eigenständig gelöst.

## Quellen

- Airport CEO Wiki (airportceo.wiki.gg), insbesondere Passengers, Check-In, Baggage, Security, Zones, Rooms, Ground Transportation, Runway, Taxiway, Stand, Turnaround Services, Hangars, Flights, Flight Planner, Flight Monitor, Aircraft, Weather, Emergencies, Animals, Contracts, Economy, Staff, R&D Panel, Modding, Achievements, Patch Notes
- Offizielle Release Notes im Apog-Labs-Forum (Alpha 23 bis 1.1-3), Dev Blogs auf airportceo.com
- Steam-Store-Seiten des Spiels und der DLCs, Steam-News, SteamDB-Patch-Notes
- Steam-Leitfaden zur Forschung (Stand Alpha 35.5), Spielerdiskussionen auf Steam und im Forum für einzelne Detailfragen
