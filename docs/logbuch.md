# Logbuch

<!-- Chronologischer Flugschreiber von OpenAirside. Ereignisbasierte Einträge, neueste oben.
     Zweck:
       1. Nahtlose Fortsetzung in neuer Session: was war zuletzt los, womit ging es zu Ende?
       2. Wiederfindbarkeit kleiner Lösungen
       3. Selbst-Beobachtung des Projekts

     Abgrenzung zu anderen Dokumenten:
       - fahrplan.md: Was tun wir? (Plan)
       - decisions.md: Warum so? (Begründung)
       - architecture.md: Wie ist es gebaut? (Zustand)
       - blockers.md: Was hindert uns aktuell? (offene Probleme)
       - CHANGELOG.md: Was hat sich für Nutzer geändert? (extern, versionsorientiert)
       - logbuch.md: Was ist während der Arbeit passiert? (intern, chronologisch) -->

<!-- ANCHOR:aktueller-stand -->
## Aktueller Stand

Phase 1 abgeschlossen (ADR-016), Detail-Einträge in `docs/archiv/logbuch-phase-1.md`. Phase 2 (Fundament) abgeschlossen am 2026-10-19 mit „gezielt umbauen“ (ADR-036), Detail-Einträge in `docs/archiv/logbuch-phase-2.md`. Phase 3 (Vertikaler Schnitt „Kleiner Flughafen“) ausgeplant mit 13 Schritten, nächster Schritt 3.7.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-20 16:48 (UTC) – [BEOBACHTUNG]

- **Ergebnisse Mac W-A** (`docs/research/3.8-ergebnisse/mac-m3pro-wa-z1.json`, `mac-m3pro-wa-z2.json`; Vorlage Abschnitt 15):
  - 119,8–120,0 Bilder/s, Bildabstand p99 bis 11,7 ms, Rundlauf p99 bis 2,5 ms.
  - Erstübertragung ganz 123 bzw. 141 ms; Bauaktion bis zum Welt-Bild höchstens 32 ms.
  - Speicher über der Schwelle, Z-1 Spitze +339 MB, Z-2 +620 MB gegenüber 2.10.
- **Sichtbeobachtung Projektinhaber, Lauf `world wb z1` (Referenz-PC, neues Paket, noch laufend):** kein Ruckeln erkennbar. Frage, ob Personen über Möbel laufen dürfen: im Prototyp ja, weil die 20.160 Objekte nur Darstellungsdaten sind und der Kern sie nicht kennt (Objekte im Weltzustand und ihre Wirkung auf die Begehbarkeit kommen mit 3.1).
- **Sichtbeobachtung Projektinhaber (Referenz-PC):** Ruckeln vor allem in der Nahansicht beim Kreisen; deckt sich mit Bildabstand p99 bis 47 ms in den kreisenden Ansichten.
- **Folgerung:** Die späten Bilder des Referenz-PCs sind dort an den Rechner gebunden, nicht an W-A. Der Engpass ist der Speicher. Z-2 scheidet aus.

### 2026-10-20 16:45 (UTC) – [BEOBACHTUNG]

- **Schritt 3.8, Fehler im Prototyp (W-B):** Auf dem Mac blieb `world wb z1` ohne Welt. Ursache: Das erste Bild einer neuen Sitzung kommt vor `sessionStarted`; der Renderer fragte die angekündigten Blöcke noch mit der alten Sitzung ab, der Kern verwarf sie (Vertrag S-7), alle vier Abfrageplätze blieben belegt. Im Container lief es zufällig durch (Reihenfolge der Nachrichten). Behoben in `f23e154` (Abfrage erst mit der neuen Sitzung, hängende Abfragen neu stellen); Container-Probe danach: sichtbarer Ausschnitt 1,1 s, ganze Welt 1,6 s. Neuer Release-Lauf 39459607051. Lehre für den Vertrag: W-B müsste den Sitzungswechsel ausdrücklich regeln.
- **Ergebnisse Referenz-PC W-A** (`docs/research/3.8-ergebnisse/referenz-pc-wa-z1.json`, `referenz-pc-wa-z2.json`; Auswertung in der Vorlage Abschnitt 14):
  - Erstübertragung: sichtbar 0,14 s bzw. im ersten Bild, ganz 0,23 bzw. 0,27 s.
  - Bauaktion bis zum Welt-Bild höchstens 57 ms, in allen 122 Bauaktionen im selben Bild wie der Tick.
  - Bildrate 58,1–60,0 bzw. 58,9–60,0 Bilder/s, Bildabstand p99 bis 47 ms (2.10: 17,3–19,7 ms). Ursache: Rundlauf der Bildanforderung p99 14–31 ms statt 1–3 ms. Herkunft offen, Kern-Prozess oder Renderer.
  - Speicher: Z-1 +90 bis +161 MB, Z-2 bis +369 MB gegenüber 2.10.

### 2026-10-20 15:32 (UTC) – [BEOBACHTUNG]

- **Schritt 3.8, Prototyp:** auf dem nicht gemergten Spike-Branch `erkundung/3-8-messaufbau` (`257a498`, `8f557d2`, `74ceec9`). Kern allein: 4.988 Blöcke des Referenz-Terminals auf Karte 2048² = 2,76 MB, Kodierung 100 ms; W-A in 3 Bildern (langsamstes 47 ms im Kern-Prozess). Container-Probelauf aller drei Kombinationen über SwiftShader: ganze Welt nach 0,74–0,96 s, Bauaktion bis zum Welt-Bild mit W-B 1–2 Bilder später als mit W-A. Bildraten ohne Aussage.
- **Antworten zum Messplan (15:05 UTC, per Auswahl):** Übersicht als Einstellung im Grafikmenü mit Voreinstellung „vereinfacht“; ganze Welt nach höchstens 3 s, sichtbarer Ausschnitt nach höchstens 1 s; drei Läufe je Rechner. Festgehalten in der Vorlage Abschnitt 11.
- **Release-Lauf** 39449159828 auf dem Spike-Branch grün auf drei Plattformen (Bau und Rauchtest der Pakete; Artefakte openairside-Windows, -macOS, -Linux); Messanleitung in `docs/research/3.8-weltdarstellung.md` Abschnitt 13.
- **Reibungen:** (1) PixiJS 8.20.1 setzt `#version 300 es` bei eigenem `GlProgram` nicht selbst; ohne die Zeile scheitert `texelFetch` beim Übersetzen. (2) Ein Block ohne zweiten Halter wird von `Arc::make_mut` an Ort und Stelle geändert; die Erkennung geänderter Blöcke über die Identität braucht deshalb, dass der Sender die übergebenen Blöcke selbst hält. (3) Der Grafik-Prüfer meldet die 512 Testgrafiken (kein Objekt im Basispaket); betrifft nur den Spike-Branch, der Release-Workflow prüft das nicht.

### 2026-10-20 15:01 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.8, Vorlage vor der Messung:** `docs/research/3.8-weltdarstellung.md`. Kandidaten für den Weg der ruhenden Welt (W-A Blöcke im Bild gegen den quittierten Stand, W-B Abruf über S-7) und für das Zeichnen (Z-1 Datentextur mit Shader, Z-2 vorgerenderte Blöcke; Z-3 Sprite je Zelle ohne Messung verworfen: 6,55 Mio. Zellen). Entwurf S-1 Layout 2 (Kopf 48 Byte; Personen 8 Byte mit Gruppe nach ADR-037, Fahrzeuge 12, Flugzeuge 16, Gepäck 8; Welt-Abschnitt nur bei W-A).
- **Vertragsabgleich (Maßnahmen 7–9):** 12 Sätze geprüft. Funde: V-1 Layout 2 erhöht `protocol` auf 2; V-4 W-B bräche „Wirkung im Bild von `tick`“ und das vorläufige Zeichnen; V-5 (Maßnahme 8) der Katalog in S-7 nennt keine feste Reihenfolge, auf die S-1 per Index verweisen kann; V-7 W-B bräuchte eigene Grenzen mit Herkunft.
- **Frage 3 (neue Wegweiser):** im Entwurf beantwortet – die Darstellung zeigt keine Wegweiser und erhält nach `waypointsAdopted` nichts neu; die Messung prüft es mit.
- **Vorgelegt:** drei Fragen (Einzelheiten in der weiten Übersicht, Wartezeit der Erstübertragung, Umfang des Messplans). Messaufbau: Prototyp auf dem nicht gemergten Spike-Branch `erkundung/3-8-messaufbau`, Pakete aus dem bestehenden Release-Workflow.

### 2026-10-20 14:53 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-20 14:52 UTC (Session zu 3.7, Kontext 492.293 Token über der Grenze von 400.000; neue Session vom Projektinhaber gewählt). Merge-Vermerk `8adb074` und Sessionende `3f813e3` nach `project-context.md` Abschnitt 10 von `scp/compassionate-shannon-wz4apd` auf den Branch dieser Session vom aktuellen `main` (`b8b3106`) übernommen (`b23c396`, `6b56ab1`); der alte Branch wird nicht weiterverwendet.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 14:53 UTC → Entscheidungs-Klasse.
- **Werkzeug:** Claude Code in der Cloud; Branch `erkundung/3-8-weltdarstellung-spielfeld` nach `CLAUDE.md` Abschnitt 11.
- **Vorhaben:** Schritt 3.8 Weltdarstellung im Spielfeld und S-1 Layout 2 (Vergleichsstudie mit Messung, Zeitbox zwei Sessions). Empfohlene Klasse: Entscheidung (Auslöser 1, Kategorie 5) – aktive Klasse passt.
- **Vorabprüfung:** Eingangskriterium „3.7 entschieden“ erfüllt (ADR-037); Abhängigkeit 3.7 erledigt. Reaktiv-Quote 7 / 10 über dem Schwellenwert, Reflexion zu ADR-037 liegt vor (Regel-009). Vor dem Vertrag: Vertragsabgleich nach Maßnahmen (7) bis (9).

### 2026-10-20 14:52 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 50 Minuten seit `[SESSIONSTART]` 14:02 UTC.
- **Bearbeitet:**
  - Archivierte Session geprüft: „Weiter mit 3.7“ kam an, die Session wurde 11 Sekunden später archiviert, ohne Ausführung.
  - Schritt 3.7: Messung `spikes/3.7-personengruppen/`, Vorlage, Einwand des Projektinhabers zu Frage 1, ADR-037, Reflexion nach Regel-009.
  - PR #28 eröffnet und nach grüner CI gemergt (`b8b3106`).
- **Erreicht:**
  - 3.7 erledigt.
  - Personengruppen, Flächenarten, Zugangsprofile und Querungen entschieden; Landeplätze in 3.1, 3.2, 3.3, 3.4, 3.5, 3.11, 3.12, 4.1 und 5.6.
  - Maßnahme (10) gilt.
- **Offen geblieben:** Merge-Vermerk `8adb074` und dieser Eintrag liegen auf `scp/compassionate-shannon-wz4apd` (`project-context.md` Abschnitt 10). Die nächste Session übernimmt sie auf ihren Branch vom aktuellen `main`.
- **Nächster Schritt:** 3.8 Weltdarstellung im Spielfeld und S-1 Layout 2, in einer neuen Session; der Projektinhaber hat die neue Session gewählt.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 14:50 UTC). 0 Schritte oberhalb der Empfehlung (3.7: Entscheidung). Abgegeben: keine Teilarbeiten; der Probelauf für Routine und Mechanik bleibt offen.
- **Kontextgröße:** 492.293 Token laut `get_session` 14:50 UTC, **über der Grenze von 400.000**. Empfehlung: die nächste Arbeit in einer neuen Session beginnen; so umgesetzt.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 14:50 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung und Größen:** keine Trigger. Logbuch rund 200 Zeilen (Grenze 800), `project-context.md` 375 Zeilen (Budget 600), `architecture.md` 467 Zeilen (eigene Schwelle 500). Phase 3 läuft noch.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Architektur-Reife, Nächste Schritte, Datum).
  - ADR-037 verweist auf vorhandene Schritte (3.1–3.5, 3.11, 3.12, 4.1, 5.6).
  - Reifegrade passen zu ADR-037 (Datenmodell fortgeschrieben, neue Zeile Aufgabenfelder `[VORLÄUFIG]`).
  - Modulnamen in 3.7 stimmen mit der Modul-Karte überein.
  - FR-052 → 3.7, 3.4.
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 7 / 10 (ADR-028 bis ADR-037) stimmt mit den Tags, gesamt 15 / 37.
  - Phase 3: 13 Schritte bei Plan 13.
- **Beobachtung:** Der Einwand des Projektinhabers (Henne-Ei bei den Sicherheitsbeamten) fand einen Fehler in der ersten Empfehlung, den weder Messung noch Inventar gezeigt hätten. Fragen mit Vision-Bezug lohnen sich auch dann, wenn die Empfehlung klar wirkt.

### 2026-10-20 14:49 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #28 (Schritt 3.7, ADR-037) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `b8b3106`. Vorher grün auf `49312e5`: Push-Lauf 39441426518 und die PR-Läufe CI 39441613876 und Security 39441613790 (39 Prüfungen, CodeQL ohne Meldung). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `scp/compassionate-shannon-wz4apd` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-20 14:40 (UTC) – [ADR-ANGELEGT]

- **ADR-037** `[REAKTIV]` „Personengruppen mit Zugangsprofilen, Flächenarten und Querungen“: freigegeben vom Projektinhaber per Auswahl.
  - Frage 1: zuerst Einwand gegen A. Wenn auch Sicherheitsbeamte durch die Kontrolle müssen, kommt der erste Beamte einer Schicht nie an seinen Arbeitsplatz (Henne-Ei-Problem). Danach A2: Kontrolle für alle außer Sicherheitsbeamten und Bauarbeitern.
  - Frage 2 A: Vorfeldpersonal zu Fuß und per Fahrzeug.
  - Frage 3 A: Walk-out-Querungen geregelt im Wechsel.
  - Frage 4 A: Bauarbeiter überall außer auf Bahnen in Betrieb.
  - Technischer Entwurf wie vorgelegt, mit fünftem Profil für Sicherheitsbeamte.
- **Klassifikation:** reaktiv nach der 14-Tage-Regel (ändert ADR-020 nach 7 Tagen). Reaktiv-Quote 7 / 10 (ADR-028 bis ADR-037). Nach Regel-009 neues Muster: Reflexion mit Stopp in `fahrplan.md` („Ergänzung Reaktiv-Quote, Anlass ADR-037“), zur Kenntnis genommen; Maßnahme (10).
- **Nachgezogen:**
  - `decisions.md` (Teil A, Reaktiv-Quote, Liste) und Status von ADR-020.
  - `architecture.md`: Module Welt & Bau, Wegfindung, Verkehr, Personen; Datenmodell; verworfene Alternativen; Reifegrade. Neue Zeile „Aufgabenfelder“ `[VORLÄUFIG]` (5.6); Datenmodell bleibt `[BELASTBAR]`, fortgeschrieben.
  - `requirements.md` FR-052.
  - Fahrplan: 3.7 erledigt; Landeplätze in 3.1, 3.2, 3.3, 3.4, 3.5, 3.11, 3.12, 4.1, 5.6.
  - README; Vorlage um die Antworten ergänzt.
- **Schritt 3.7:** erledigt (Phasentyp-Kontext ERKUNDUNG, Akzeptanz wissensbasiert). Phase 3 weiter 13 Schritte bei Plan 13; die Landeplätze ergänzen bestehende Schritte und legen keine neuen an.

### 2026-10-20 14:31 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Schritt 3.7:** Messung `spikes/3.7-personengruppen/` (Produktions-Kern als Pfad-Abhängigkeit, Referenz-Terminal 768² × 5 in Welt 2048² mit Vorfeld, 17 Stellplätzen, 6 Walk-out-Gehwegen und 4 Personalräumen; Container, 4 vCPU). Herleitung und Vorlage: `docs/research/3.7-personengruppen.md`.
- **Ergebnisse:**
  - Begehbarkeit für Vorplatz, Gehwege und Stellplätze kostet 10 % mehr Zellen (1,50 → 1,65 Mio.). Ein gemeinsamer Feldsatz für Fluggäste und Personal mit 79 Zielen braucht 130 MB und 2,6 s (heute 36 Ziele: 54 MB, 1,0 s). Getrennte Sätze kosten dasselbe.
  - Bauarbeiter über das ganze Erdgeschoss: 5,1 MB je Feld. Ein Satz für alle Gruppen bräuchte 444 MB und 9–11 s. Zwei Felder (Anlieferstelle, alle Baustellen) kosten 10 MB und 0,6 s.
  - Einzelweg-Suche je Fahrt (A*, gleiche Kosten wie die Wegweiser, 1.000 von 1.000 gleich): im Terminal Median 18–61 ms, p99 170–272 ms; Bauarbeiter p99 1,3–1,7 s. Für Personal ungeeignet.
- **Vorgelegt:** vier Fragen (Personal und Kontrolle, Vorfeldpersonal, Walk-out-Querung, Bauarbeiter und Bahnen) und der technische Entwurf (Flächenart und Personalzone je Zelle, Profil je Ziel, ein Feldsatz für Fluggäste und Personal, einer für Bauarbeiter). Klassifikation `[REAKTIV]` (ändert ADR-020 nach 7 Tagen); nach Regel-009 neue Reflexion mit Stopp, Entwurf in der Vorlage Abschnitt 8.
- **Orientierung am Original:** Wechsel in den Sicherheitsbereich nur über eine Kontrolle, Sicherheitsbeamte und Vertragskräfte ausgenommen (airportceo.wiki.gg/wiki/Security); Vorfeldagenten über Gehwege und Zebrastreifen (forum.apoglabs.com).

### 2026-10-20 14:10 (UTC) – [PROBLEM-GELÖST]

- **Kontext:** Sessionstart in einem frischen Cloud-Container.
- **Symptom:** `.git/hooks/pre-commit` fehlte, `pre-commit` war nicht installiert. Der Commit `aad31a3` (Sessionstart, nur Markdown) lief deshalb ohne Hooks.
- **Lösung:** `pip install --user pre-commit`, `pre-commit install` nach Runbook Schritt 3. Die Hooks liefen danach über die beiden Dateien des Commits, alle grün. `node_modules` fehlt im Container und Node ist 22.22.0 statt 26.11.1; für Markdown und Rust in 3.7 nicht nötig, vor TypeScript-Änderungen `npm ci` mit Node 26.
- **Wiederkehrgefahr:** hoch bei jeder neuen Cloud-Session. Vor dem ersten Commit `ls .git/hooks` prüfen.

### 2026-10-20 14:02 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 2026-10-19 20:03 UTC, danach Merge PR #27 (`119d2b1`) mit Vermerk `d2e6c70` auf `docs/phasenabschluss-2`. Der Vermerk ist nach `project-context.md` Abschnitt 10 auf den Branch dieser Session vom aktuellen `main` übernommen (`0162671`); `docs/phasenabschluss-2` wird nicht weiterverwendet. Die Session „Phasenabschluss Phase 2“ erhielt am 2026-10-20 13:50 UTC „Weiter mit 3.7“ und wurde 11 Sekunden später versehentlich archiviert, ohne dass etwas ausgeführt wurde (Transkript über `list_events` geprüft); 3.7 beginnt hier.
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 3, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 14:01 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Werkzeug:** Claude Code in der Cloud; vom Werkzeug vergebener Branch `scp/compassionate-shannon-wz4apd` (Typ und Schritt-ID im PR-Titel, `CLAUDE.md` Abschnitt 11).
- **Vorhaben:** Schritt 3.7 Personengruppen, Begehbarkeit und Querungen (Entwurf mit Messung, ADR in Ergänzung von ADR-020). Empfohlene Klasse: Entscheidung (Auslöser 1) – aktive Klasse passt.
- **Vorabprüfung:** Eingangskriterium „Datenmodell im Kern `[BELASTBAR]`“ erfüllt (`architecture.md` Abschnitt 9); Abhängigkeit 2.3 erledigt. Reaktiv-Quote 6 / 10 über dem Schwellenwert, Reflexion für Phase 2 liegt vor (Regel-009).

### 2026-10-19 20:10 (UTC) – [BEOBACHTUNG]

- **Merge:** PR #27 (Phasenabschluss 2) nach Anweisung des Projektinhabers gemergt („PR eröffnen und bei grün mergen“), Merge-Commit `119d2b1`. Vorher grün auf `62998bd`: Push-Lauf 39316204108 und die PR-Läufe CI 39316460887 und Security 39316460888 (39 Prüfungen, CodeQL ohne Meldung). Dieser Eintrag bleibt nach `project-context.md` Abschnitt 10 auf `docs/phasenabschluss-2` und geht mit dem PR des nächsten Schritts nach `main`.

### 2026-10-19 20:03 (UTC) – [SESSIONENDE]

- **Session-Dauer:** 23 Minuten seit `[SESSIONSTART]` 19:40 UTC.
- **Bearbeitet:** Phasenabschluss 2: Vision-Abgleich, Pflichtfrage mit zwei getrennten Instanzen, Stellungnahme, Vorlage und Freigabe, ADR-036, Onboarding-Re-Validation, Archivierung und `[PHASEN-WECHSEL]`, Phase 3 im vollen Format.
- **Erreicht:** Phase 2 abgeschlossen. Phase 3 hat einen ursprünglichen Schrittplan von 13 Schritten; Regel-008 und Regel-009 gelten. Abschnitt 4 der Architektur ist ausgelagert, FR-052 neu, FR-016 ergänzt.
- **Offen geblieben:** PR für `docs/phasenabschluss-2` nach `main`, nur auf Anweisung. Rückmeldung an das Vorlagen-Repo Dev-Templates zur Zählregel der Reaktiv-Quote: im Fahrplan vermerkt, nicht gesendet, liegt außerhalb dieses Repos.
- **Nächster Schritt:** 3.7 Personengruppen, Begehbarkeit und Querungen, in einer neuen Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session` 20:03 UTC). 0 Schritte oberhalb der Empfehlung (Phasenabschluss: Entscheidung). Abgegeben: zwei Bewertungen an Unteragenten als getrennte Instanzen (Opus 5.5 und Sonnet 5.5). Das war kein Sparzweck; der Probelauf für die Abgabe an Routine/Mechanik bleibt offen.
- **Kontextgröße:** nicht feststellbar – `get_session` meldet `used_tokens: 0`. Kein Hinweis zur Sessiongröße.
- **Kontingent:** 5-Stunden-Fenster `allowed` laut `get_session` 20:03 UTC.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf ohne Schritt. Electron 44: Vorlauf ab 2027-01-02, Q.1. Runner-Image Ubuntu 26: Q.1.
- **Archivierung:** Fahrplan Phase 2 und Logbuch Phase 2 ausgelagert. Logbuch jetzt rund 130 Zeilen, Architektur 461 Zeilen (unter der Schwelle von 500), `project-context.md` 375 Zeilen.
- **Drift-Prüfung:**
  - README nachgezogen (Phase, Nächste Schritte, Verweise auf `architecture-schnittstellen.md`).
  - ADR-036 verweist auf vorhandene Schritte (3.1–3.13, 4.1, 4.3).
  - Reifegrade passen zu ADR-036 (keine Änderung).
  - Modulnamen der Phase-3-Schritte stimmen mit der Modul-Karte überein.
  - Jede Muss-Anforderung hat einen Schritt (FR-052 → 3.7, 3.4).
  - Keine `[BLOCKIERT]`-Schritte, keine aktiven Blocker.
  - Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036) stimmt mit den Tags.
  - Phase 3: 13 Schritte bei Plan 13.

### 2026-10-19 20:02 (UTC) – [PHASEN-WECHSEL]

- **Phase 2 → Phase 3:** Phase 2 (UMSETZUNG, Fundament) mit 11 von 11 Schritten abgeschlossen; Phase 3 (UMSETZUNG, vertikaler Schnitt) mit ursprünglichem Schrittplan von 13 Schritten ausgeplant (ADR-036). Detail-Einträge von Phase 2: [`docs/archiv/logbuch-phase-2.md`](archiv/logbuch-phase-2.md); Fahrplan: [`docs/archiv/fahrplan-phase-2.md`](archiv/fahrplan-phase-2.md). Die Einträge der Session 1.7 stehen jetzt in [`docs/archiv/logbuch-phase-1.md`](archiv/logbuch-phase-1.md).
- **Reflexion Phase 2:**
  - Gelernt: Fundament mit Reserve – Schritt mit 10.000 Personen 0,13–0,56 ms, Bild an der Grenze des Bildschirms auf Referenz-PC und Mac (2.7, 2.10), Spielstand bitgleich nach Laden auch während einer Neuberechnung (2.5). Getrennte Prüfungen fanden in jedem Schritt mit Kategorie 6 echte Mängel; die Sammelprüfung 2.11 fand 10 Vertragslücken auf einmal.
  - Gekippte Annahmen: Verträge aus 2.9 trugen das Gebaute nicht vollständig (ADR-026, ADR-028 bis ADR-032); „Kern-Module über Weltzustand und interne Ereignisse“ ist noch nicht gebaut (3.9); Neuberechnung im Hintergrund ohne Zusammenfassen der Anfragen trägt Dauerbau nicht (3.1); Linux bleibt ohne Messung auf echter Hardware (7.2).
  - Reifegrade: Takt & Weltzustand, Welt & Bau (Grundraster), Brücke, Plattform, Inhalte & Mods (Lader), Lokalisierung, Grafik-Pipeline `[BELASTBAR]` durch Umsetzung; S-2 bis S-7 bestätigt, S-8 neu; NFR Darstellung im gemessenen Umfang (ADR-034); Weltdarstellung `[OFFEN]` (3.8).
  - ADRs der Phase: ADR-018 bis ADR-034 (dazu ADR-035 im Querschnitt, ADR-036 Phasenabschluss); reaktiv ADR-019, ADR-020, ADR-025, ADR-026, ADR-028 bis ADR-032 – formal durch die 14-Tage-Regel oder additive Vertragsergänzungen, keiner änderte Code in Takt, Welt & Bau, Wegfindung oder Brücke. Maßnahmen: Regel-008, Regel-009.
  - Neue Erkundungen: 3.11 Suchen ohne Schilder und Feinwege, 3.12 Blockadevermeidung, 3.13 Schrittzeit-Spitzen auf Apple-Chips; Entwurf 3.9 Kernstruktur.
  - Mini-Reibungen, die wiederkehren können: Zeitangaben geschätzt statt mit `date -u` geholt (zweimal); Typ-Check des Pre-Commit-Hooks über den ganzen Arbeitsbaum; Prettier formatiert JSON in `docs/research/`; Suppression ohne Begründung (`scenario.rs:76`) blieb in Hooks und Prüfungen unbemerkt.
- **Pflichtfrage:** „gezielt umbauen“ (A), zwei getrennte Instanzen (Opus 5.5, Sonnet 5.5), `docs/research/phasenabschluss-2.md`.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ → FR-016 mit 3.2 und 4.1; MOD-13 Walk-out in den Kern → FR-052 mit 3.7 und 3.4; Erkundungen mit eigener ID 3.11, 3.12; keine `[VERSCHOBEN]`-Zeilen. Keine verwaisten Elemente mehr.
- **Onboarding-Validierung:** siehe `[ONBOARDING-VALIDATION]` 19:56.

### 2026-10-19 20:01 (UTC) – [ADR-ANGELEGT]

- **ADR-036** `[REAKTIV]` „Phasenabschluss 2 – gezielt umbauen vor dem vertikalen Schnitt“: freigegeben vom Projektinhaber per Auswahl (A; MOD-13 in den Kern; FR-016 ergänzen; 4.3 als 3.13 vorziehen; kein Linux-Rechner). Reaktiv, weil er den Landeplatz 4.3 aus ADR-034 (heute) nach 3.13 verlegt. Reaktiv-Quote 6 / 10 (ADR-027 bis ADR-036), Reflexion für Phase 2 im Fahrplan nach Regel-009.
- **Nachgezogen:** `decisions.md` (Teil A, Regel-008, Regel-009), Abschnitt 4 nach `docs/architecture-schnittstellen.md` mit Änderungsprotokoll und Ankern, `architecture.md` (Abschnitte 0, 2, 4, 9, offene Fragen), `requirements.md` (FR-016, FR-052, Schritt-IDs 3.11/3.12, Übersicht 52 Anforderungen), `modernisierung.md` (MOD-13 Kern), Fahrplan (Phase 3 im vollen Format, 4.1, 4.3, Reflexion, Replanning), README.

### 2026-10-19 19:56 (UTC) – [ONBOARDING-VALIDATION]

- **Phasen-Abschluss 2 (Trigger 3, Klasse G):** frischer Worktree von `origin/main` (`d4e457f`) im Container (Linux, 4 vCPU), Runbook Schritte 2 bis 5 wörtlich: `npm ci`, `pre-commit run --all-files` (alle Hooks grün), `npm run build`, Start `npx electron . --no-sandbox` unter `xvfb-run` (40 s ohne Absturz, nur GPU-Meldungen der Software-Grafik), dann alle Gates: rustfmt, clippy, `cargo test` (alle grün, u. a. 134 Kern-Unit-Tests, 26 Datenpaket-, 10 Spielstand-Fälle), cargo-deny, ESLint, Prettier, tsc, Vitest 356 Tests mit 86,6 % Lines, Lizenzen, `check:types`, `check:schema`, `check:graphics`, Benchmarks (Tick: Mittel 0,17 ms, p99 0,47 ms; Wegweiser: Neuberechnung Median 2,9 s bei Grenze 4 s, Schritt p99 1,8 ms), Rauchtest grün (17,1 s).
- **Fund und behoben:** Runbook Schritt 4 nannte für das Basispaket „3 entries“; seit 2.6 zählen die Texte der Sprachdateien mit. Text auf „N entries“ mit Erklärung umgestellt (`a0603e7`).
- **Einschränkungen:** Electron kam aus dem Cache der Container-Einrichtung (`~/.cache/electron`), der Download beim ersten Start ist damit nicht frisch belegt. Neuberechnung 2,9 s liegt über den früher lokal gemessenen 1,1–1,2 s (anderer Container, kein Gate verletzt; Beobachtung für 3.1). Zweite Plattform (Klasse G): Windows und macOS über die CI des Stands (PR #26 auf `0cf1724`, Lauf 39311921712 grün auf drei Plattformen), nicht von Hand.

### 2026-10-19 19:51 (UTC) – [ENTSCHEIDUNG-VORGELEGT]

- **Pflichtfrage Phasenende 2:** zwei getrennte Instanzen ohne Gesprächsverlauf, Logbuch ausgeschlossen: A Opus 5.5 (wie Phase 1), B Sonnet 5.5 (anderes Modell). Beide empfehlen „gezielt umbauen“, Konfidenz mittel, und schließen Neuaufsetzen mit Belegen aus. A: Kernstruktur für die Fachmodule vor 3.1, Dauerbau der Wegweiser, `main.ts` teilen, Architektur auslagern, Zählregel. B: Prozess (Zählregel, Reflexionen), `main.ts`, S-1 Layout 2 in 3.8. Volltext und Stellungnahme: `docs/research/phasenabschluss-2.md`.
- **Nachgeprüft:** `core/src/adoption.rs` (ältere Neuberechnung läuft weiter, wird verworfen), `vitest.config.ts` (Ausnahmen), `core/src/scenario.rs:76` (Suppression ohne Begründung, Verstoß seit 2.3; B hatte sie als begründet gezählt), keine internen Ereignisse im Kern, `architecture.md` über der eigenen Schwelle von 500 Zeilen.
- **Widerspruch der Stellungnahme:** Die Zählregel steht wörtlich in `CLAUDE.md` Abschnitt 6 und ist im Projekt nicht auslegbar. Stattdessen soll die Projektregel „jede Änderung an `[BELASTBAR]` ist freigabepflichtig“ an `CLAUDE.md` Abschnitt 4 Nr. 5 angeglichen werden (additiv ohne ADR) und es soll eine Reflexion je Phase geben.
- **Vision-Abgleich:** Lücke „kein Stillstand im Passagierverkehr“ ohne Akzeptanzkriterium (FR-016 ergänzen, 3.2); MOD-13 Walk-out (Kandidat) steckt in 3.7 und über ADR-020 in 3.4 – Frage an den Projektinhaber; Erkundungen vor 3.3/3.4 ohne Schritt-ID (Regelverstoß, neue Schritte 3.11/3.12). Keine `[VERSCHOBEN]`-Zeilen.
- **Archivierung vorgezogen (unabhängig von der Antwort):** Fahrplan Phase 2 nach `docs/archiv/fahrplan-phase-2.md` mit Phasen-Bilanz; Logbuch-Einträge von Phase 2 nach `docs/archiv/logbuch-phase-2.md`, die Session 1.7 (2026-10-13 18:00–19:05) nach `docs/archiv/logbuch-phase-1.md`. `[PHASEN-WECHSEL]` mit Reflexion folgt nach der Antwort.
- **Drift behoben:** Fahrplan „Aktueller Stand“ nannte 7 / 10, `decisions.md` 6 / 10 (seit ADR-035); jetzt 6 / 10.

### 2026-10-19 19:40 (UTC) – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 18:53 UTC; danach Merge PR #25 (19:11), ADR-035 und Q.3 (19:31), Merge PR #26 (19:37, `d4e457f`). Der Abschluss-Commit `836ec95` nach `project-context.md` Abschnitt 10 auf den neuen Branch `docs/phasenabschluss-2` vom aktuellen `main` übernommen (`e1c2262`); `ci/q-3-rustup-toolchain-install` wird nicht weiterverwendet. Die vorige Session endete mit über 500.000 Token Kontext (Grenze 400.000).
- **Pflichtlektüre:** durchlaufen (project-context vollständig, Logbuch ab letztem `[SESSIONENDE]`, Fahrplan Stand und Phase 2, Architektur 1/2/9, Decisions Teil A/C, aktive Blocker: keine, Requirements-Übersicht).
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` laut `get_session` 19:40 UTC → Entscheidungs-Klasse. Kontingent: 5-Stunden-Fenster `allowed`.
- **Vorhaben:** Phasenabschluss Phase 2 nach `CLAUDE.md` Abschnitt 12: Vision-Re-Derivations-Pass (mit `requirements.md`), Pflichtfrage „Weiterbauen, umbauen oder neu aufsetzen?“ mit Bewertung durch eine getrennte Instanz, Onboarding-Re-Validation (Trigger 3, Klasse G), Archivierung und `[PHASEN-WECHSEL]`. Empfohlene Klasse: Entscheidung (Auslöser 1 und 3) – aktive Klasse passt.

---

<!-- ANCHOR:eintragstypen -->
## Eintragstypen (Übersicht)

| Typ | Wann | Pflicht? |
|---|---|---|
| `[SESSIONSTART]` | Zu Beginn jeder Session | Ja |
| `[SESSIONENDE]` | Vor Sessionabschluss | Ja |
| `[PROBLEM-GELÖST]` | Nach Behebung eines Problems, das Reibung war | Empfohlen, alle Mini-Probleme erfassen |
| `[PROBLEM-OFFEN → BLOCKER]` | Wenn ein Problem zum Blocker eskaliert | Ja, mit Verweis auf `blockers.md` |
| `[BLOCKER-AUFGELÖST]` | Wenn ein Blocker gelöst wurde | Ja, mit Verweis auf den ursprünglichen Logbuch- und Blocker-Eintrag |
| `[REIFEGRAD-WECHSEL]` | Bei jeder Reifegrad-Änderung in `architecture.md` | Ja |
| `[ADR-ANGELEGT]` | Bei Anlage eines neuen ADR | Ja |
| `[BEOBACHTUNG]` | Wenn etwas auffällt, das später nützlich sein könnte | Optional, KI proaktiv |

### Format der Einträge

```text
### YYYY-MM-DD HH:MM – [TYP]

- **Feld:** Inhalt
```

Felder je Typ:

- `[SESSIONSTART]`: Letzter Stand, Geplant für diese Session, Vorabprüfung (Reifegrade, Eingangskriterien), Modus/Werkzeug, Modell (eingestellt und bedient) mit Modellklasse und Quelle (`CLAUDE.md` Abschnitt 0)
- `[SESSIONENDE]`: Session-Dauer, Bearbeitet, Erreicht, Offen geblieben, Nächster Schritt, Aktive Klasse mit Zahl der Schritte oberhalb ihrer Empfehlung und abgegebenen Teilarbeiten, Kontextgröße (mit Hinweis bei Überschreitung der Grenze aus `project-context.md` Abschnitt 6), Ablaufdaten-Prüfung, Stimmung/Beobachtung (optional)
- `[PROBLEM-GELÖST]`: Kontext, Symptom, Ursache, Lösung, Aufwand, Lerneffekt (optional), Wiederkehrgefahr (optional)
- `[PROBLEM-OFFEN → BLOCKER]`: Kontext, Symptom, Versuche bisher, Eskaliert nach (Blocker-Nr.)
- `[REIFEGRAD-WECHSEL]`: Bestandteil, Wechsel, Auslöser, Datum in `architecture.md` Abschnitt 9 nachgetragen
- `[ADR-ANGELEGT]`: ADR, Tag, Auslöser

<!-- ANCHOR:hinweise-zur-pflege -->
## Hinweise zur Pflege

- **Neueste Einträge oben.**
- **Zeitstempel ist Pflicht.** Format: `YYYY-MM-DD HH:MM` (24h, lokale Zeitzone).
- **Detailtiefe lieber zu hoch als zu niedrig.** Wenn unsicher, ob etwas eingetragen werden soll: eintragen.
- **Verweise statt Duplikate** zu ADRs, Blockern und Fahrplan-Schritten.
- **Keine sensiblen Daten.**
- **CI-Ergebnisse:** Bei Problemen aus der CI wird festgehalten, was im Log stand, nicht nur die Ampelfarbe.

<!-- ANCHOR:archivierung -->
## Archivierung

Bei mehr als 800 Zeilen: alte Einträge nach `docs/archiv/logbuch-YYYY-MM.md` auslagern; im aktiven Logbuch bleiben die letzten 4–8 Wochen plus alle Einträge zu offenen Blockern. Auslagerung ist Sessionende-Aktion.
