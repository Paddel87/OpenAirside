# Recherche: Wünsche der Community, die nie umgesetzt wurden

<!-- Recherche der KI vom 2026-10-29 im Netz, Grundlage für den Entwurf
     `neuausrichtung-entwurf.md`. Nicht freigegeben, ersetzt keinen Pflicht-Text. -->

## 0. Vorgehen und Grenzen

- **Quellen:** das Forum von Apog Labs (Vorschläge mit Stimmenzahl, Dev-Blogs, Wunschlisten),
  die Steam-Diskussionen und die Steam-Rezensionen der letzten sechs Monate. Wiedergegeben
  wird jeweils sinngemäß, nicht wörtlich.
- **Grenzen:**
  - Reddit lieferte über die Suche keine Treffer.
  - Das Forum hat keine eigene Kategorie mit Stimmen mehr. Eine belastbare Stimmenzahl gibt
    es nur für einen einzigen Wunsch (Umsteiger).
  - Viele Beiträge stammen aus 2018 bis 2021. Die Stichprobe der Rezensionen umfasst nur
    10 Stück.
  - Die Rangfolge unten ist deshalb eine Einschätzung aus Häufigkeit, Stimmen und Reaktion
    der Entwickler, keine Zählung.

## 1. Wünsche nach Stärke des Belegs

| # | Wunsch | Beleg | Antwort der Entwickler | Stand |
|---|---|---|---|---|
| 1 | **Umsteiger** (Anschlussflüge ohne Gepäckausgabe und Check-in) | Forums-Vorschlag seit 2018, 208 Stimmen im Mai 2020, laut Nutzern einer der meistgewählten; mehrere Doppel-Threads | 2020 abgelehnt: zu viel Komplexität für zu wenig Mehrwert | nie umgesetzt |
| 2 | **Fracht (Cargo)** | wiederkehrend in Steam und Forum; ein Spieler würde dafür mehr zahlen als für jedes andere Add-on | Dev-Blog 158: keine Pläne, dem Team fehle die Lust, es spaßig umzusetzen | nie umgesetzt; Mods mit einfacher Fracht |
| 3 | **Verlässliche Wegfindung und klare Ursachen** | Rezensionen (etwa 5 von 10): Personen bleiben hängen, unlogische Wege, besonders bei mehreren Sicherheitszonen; Wunsch nach Rückmeldung, die die echte Ursache zeigt | keine | ungelöst |
| 4 | **Leistung bei großen Flughäfen, Nutzung mehrerer Kerne** | Steam-Thread 2025: Ruckeln ab 2.000–8.000 Fluggästen, CPU nicht ausgelastet; Rezensionen | Moderator: Das Spiel sei größer geworden als erwartet | ungelöst |
| 5 | **Entwürfe, die beim Ausbau nicht brechen** | Rezensionen (etwa 3): Layouts brechen beim Erweitern oder mit neuen Flugzeugen, Änderungen stören andere Systeme | keine | ungelöst |
| 6 | **Offenheit und Modding** (Quellcode freigeben, weiteres Modding-Kit) | Dev-Blog-Kommentare; Rezension wünscht freigegebenen Code; Wunschliste 2021 nennt Modding „sehr nötig“ | keine Antwort | teilweise (Workshop, Community-Mods) |
| 7 | **Mehr Management-Tiefe** („CEO“ statt reiner Aufbausimulation, Verhandlungen mit Airlines, Läden mit Bestand) | Forums-Vorschlag; Vergleiche mit SimAirport in Steam | keine | nicht umgesetzt |
| 8 | **Bessere Einführung** (Gepäck, Förderbänder, Zonen verbinden) | Rezensionen (etwa 3) | keine | ungelöst |
| 9 | **Bedienkomfort im Großen:** Vorlagen im Flugplaner, Personal in Gruppen einstellen, Fahrzeuge leichter zuordnen | Wunschliste 2021 (Personal in Gruppen mit den meisten Likes) | keine | nicht umgesetzt |
| 10 | **Freiere Räume** (unregelmäßige Form, Zusammenlegen), kleinere Rettungswache für kleine Flughäfen | Wunschliste 2021 | keine | nicht umgesetzt |
| 11 | **Mehr Flugzeuge und Airlines** | Umfragen der Entwickler zu Flugzeugen; Thread „Airline Wishes“ mit über 3.400 Antworten | teilweise per Umfrage umgesetzt | laufend über Mods |
| 12 | **3D-Ansicht** | nur ein Aprilscherz eines Modders nennt sie den häufigsten Wunsch; kein belastbarer Beleg | – | Kamera-Mod |

## 2. Abgleich mit dem Entwurf

| Wunsch | Im Entwurf | Bewertung |
|---|---|---|
| 1 Umsteiger | Säule 3: Umsteigebank, Mindestumsteigezeit | **Volltreffer.** Der am besten belegte Wunsch wurde abgelehnt, weil er Komplexität ohne Spielwert bringe. Der Entwurf macht genau daraus Spielwert (Wellen, Umsteigezeit als Kennzahl). |
| 2 Fracht | Säule 3: Beiladefracht mit den Passagierflügen, Frachtnacht | **Eingearbeitet** (2026-10-29). Fracht ist ein weiterer Strom und füllt die Täler des Tagesrhythmus. |
| 3 Wegfindung, Ursachen | Säule 2: Ströme lesen, Engpass-Diagnose | **Treffer.** Technisch vorbereitet (Wegweiser, Unerreichbarkeit mit Ursache in 3.3). |
| 4 Leistung | Leistungskriterien unverändert | **Treffer.** Kern des bisherigen Projekts. |
| 5 Ausbau bricht Entwürfe | Säule 1: Umbau im laufenden Betrieb | **Treffer.** Der Entwurf macht den Ausbau zur geplanten Spielphase statt zum Risiko. |
| 6 Offenheit, Modding | Open Source, Datenpakete, Mod-Schnittstelle | **Treffer.** |
| 7 Management-Tiefe | Säule 5: Geschäftsmodelle der Airlines, Verträge mit messbaren Zusagen, Gebührenordnung | **Eingearbeitet** (2026-10-29) für die Airlines; Läden mit Bestand bleiben offen. |
| 8 Einführung | Grundsatz „Lernen am eigenen Flughafen“, Lektionen am Engpass, Erfolgskriterium „Einstieg ohne Vorwissen“ | **Eingearbeitet** (2026-10-29). |
| 9 Bedienkomfort im Großen | fehlt | **Lücke**, klein: Vorlagen, Sammelaktionen; gehört zu „sehr große Flughäfen“. |
| 10 Freiere Räume | widerspricht teilweise dem Raster | **Offen:** Unregelmäßige Räume im Raster sind möglich, ohne Kurven. |
| 11 Flugzeuge, Airlines | Datenpakete, Mods | **Treffer** über die Mod-Schnittstelle. |
| 12 3D | Darstellung getrennt von der Simulation | Kein belegter Wunsch; bleibt späterer Kandidat. |

## 3. Folgerung

Die vier Säulen des Entwurfs treffen die fünf am besten belegten Wünsche (1, 3, 4, 5, 6),
die nie umgesetzt wurden. Am stärksten ist der Fall bei den Umsteigern: von vielen gewünscht,
vom Studio ausdrücklich verworfen, und im Entwurf ein Kern des Spiels. Drei Lücken sollte der
Entwurf schließen: Fracht, eine gute Einführung und mehr Management-Tiefe gegenüber den Airlines.
Alle drei sind seit 2026-10-29 eingearbeitet; offen bleiben Bedienkomfort im Großen (9), freiere
Räume (10) und Läden mit Bestand.

## Quellen

- Forum Apog Labs, „Connecting Flight“ (Vorschlag 2018, Stimmen 2020, Antwort der Entwickler): <https://forum.apoglabs.com/t/connecting-flight/7918>
- Forum Apog Labs, „[Duplicated] Transfer passengers“: <https://forum.apoglabs.com/t/duplicated-transfer-passengers/14609>
- Steam, „Transfer Passengers“: <https://steamcommunity.com/app/673610/discussions/0/1643167006305413924>
- Forum Apog Labs, Dev-Blog 158 (Fracht, weitere Wünsche in den Kommentaren): <https://forum.apoglabs.com/t/dev-blog-158-airport-ceo-in-beta/18214>
- Steam, „Are we ever going see Cargo added to this game?“: <https://steamcommunity.com/app/673610/discussions/0/4139437492713793180/>
- Forum Apog Labs, „The Final Things the Game Needs“ (Wunschliste 2021): <https://forum.apoglabs.com/t/the-final-things-the-game-needs/18646>
- Steam, Rezensionen der letzten sechs Monate: <https://steamcommunity.com/app/673610/reviews/?l=english&filterLanguage=default&p=1&browsefilter=trendsixmonths>
- Steam, „Optimization issues?“ (2025): <https://steamcommunity.com/app/673610/discussions/0/598524660653562215/>
- Steam, „Passengers stuck deboarding and at security“: <https://steamcommunity.com/app/673610/discussions/0/2381701715720719894/>
- Forum Apog Labs, „Airport CEO 1.1-3 released“ (Ende der Entwicklung): <https://forum.apoglabs.com/t/airport-ceo-1-1-3-released/22277>
- Forum Apog Labs, „Airport CEO: 3D“ (Aprilscherz eines Modders): <https://forum.apoglabs.com/t/airport-ceo-3d/22111>
- Forum Apog Labs, Umfragen zu Flugzeugen, z. B. <https://forum.apoglabs.com/t/poll-help-pick-the-next-two-wildcard-aircraft-in-airport-ceo/11640>
- Steam, Vergleich mit SimAirport (Management-Tiefe): <https://steamcommunity.com/app/598330/discussions/0/1752399065083021809>
