# Phasenabschluss 1: Weiterbauen, umbauen oder neu aufsetzen

<!-- Pflichtfrage am Phasenende (CLAUDE.md Abschnitt 12). Teil 1: Bewertung der getrennten Instanz,
     unverändert übernommen. Teil 2: Stellungnahme der bauenden KI. Teil 3: Vision-Abgleich.
     Entscheidung und ADR folgen nach Freigabe durch den Projektinhaber (Schritt 1.7). -->

- **Datum:** 2026-10-13
- **Getrennte Instanz:** Unteragent ohne Gesprächsverlauf, Modell der Entscheidungs-Klasse (`claude-opus-5-5`, dasselbe Modell wie die bauende KI – eine andere Klasse wäre schwächer gewesen); Quellen: Vision, Architektur, Entscheidungen, Fahrplan, Code und Ergebnisse unter `spikes/`, Workflows; Logbuch bewusst ausgeschlossen; nur lesend.

## Teil 1: Bewertung der getrennten Instanz (unverändert)

### Befund (F = belegt, E = Einschätzung)

1. **Kern und Übergabe tragen mit großer Reserve.** (F) Ein Tick mit 10.000 Personen auf 5 Etagen dauert 0,33–0,84 ms. Bei 180 Ticks/s stehen 5,5 ms zur Verfügung. Gemessen: 60 bzw. 120 Bilder/s, Speicher unter 750 MB ohne Anstieg über 10 min, S-1-Rundlauf 0,6–2,8 ms (Quellen: `spikes/1.5-leistung/README.md`, ADR-014). (E) Für ADR-003, ADR-009 und ADR-010 ist `[BELASTBAR]` gerechtfertigt.
2. **Gemessen wurde weniger als das Zielszenario.** (F) Im Spike gab es nur Fußgänger auf einem Gebäuderaster mit 36 Wegweisern. Es fehlen Fahrzeuge, Flugzeuge, Gepäck, Personal, Bahnen und Vorfeld (ADR-013, Konsequenzen). Das Spielfeld zeichnet je Etage eine Canvas-Textur und Punkte im `ParticleContainer` (`spikes/1.5-leistung/app/renderer/main.js` Z. 95–277). Objekt-Sprites, Atlanten aus der SVG-Pipeline und Overlays kommen nicht vor. (E) Dass `architecture.md` §6/§9 die „NFR Performance Gesamtsystem" als `[BELASTBAR]` führt, ist zu weit gefasst. Belegt sind nur Personen-Simulation, Übergabe und eine sehr einfache Darstellung.
3. **Die Auflagen haben Landeplätze, aber späte.** (F) Schwache integrierte Grafik kommt in 5.8, echte 8 Kerne in 4.1, Linux in 7.2 (ADR-014 Auflagen 3–5). Der Leistungstest als CI-Gate beginnt erst in Phase 4 (`project-context.md` §7). (E) Damit laufen zwei Umsetzungsphasen ohne automatischen Leistungsschutz. Das Linux-Treiberrisiko, im ADR-002 der Grund gegen Tauri, bleibt bis kurz vor Version 1.0 offen.
4. **Die reaktiven ADRs betreffen nur Randbereiche.** (F) ADR-011, ADR-012 und ADR-015 betreffen Testlogistik und Verpackung, keiner den Kern, die Brücke oder das Datenmodell (`fahrplan.md`, Iterations-Reflexion). (E) Ich stimme der Reflexion zu: kein Symptom im Fundament. ADR-011 verschiebt allerdings eine ausdrückliche Bedingung aus ADR-002.
5. **Eine fragile Festsetzung und verkettete Spikes.** (F) napi-derive-backend 6.1.4 ist nur über `Cargo.lock` festgesetzt (ADR-009; `project-context.md` Z. 57). Ein frischer Produktions-Workspace löst 6.1.5 auf und bricht den Build. Die Spikes hängen über relative Pfade aneinander (1.5 → 1.4 → `1.3/bridge-core` → 1.2), Code für 1.5 existiert nur als Erweiterung dieser Crates. Die Ad-hoc-Signatur steht nur in `spikes/1.5-leistung/app/package.json`, `release.yml` hat sie nicht.
6. **Bauaktion im nächsten Bild nicht erreicht.** (F) Gemessen 2–6 Bilder bzw. 16–69 ms, weil das Fenster erst nach Bestätigung durch den Kern zeichnet. (E) Für das Spielgefühl gilt das schon als „sofort" im Sinne der Vision; die NFR „nächstes Bild" ist strenger. Die Lösung (vorläufige Vorschau) liegt in der Präsentation. Für Phase 2 zählt nur der Vertrag S-2/S-3: Befehlskennung und Bestätigung bzw. Ablehnung mit Bezug.
7. **Die Methodik würde Phase 2 sofort stoppen.** (F) Phase 2 ist UMSETZUNG. CLAUDE.md §6 verlangt `[BELASTBAR]` für alle berührten Bestandteile. Die Schritte 2.2–2.7 berühren S-2, S-3 und S-6 (`[VORLÄUFIG]`), S-4 und S-5 (`[OFFEN]`) und Module, die alle `[VORLÄUFIG]` sind (`architecture.md` §9).
8. **Informationslücken für Phase 2.** (F) Offen sind die Zellgröße in Metern (ADR-013), die Tickrate bei 1× (in 1.5 nur als Annahme) und die Lage von Bahnen, Vorfeld und Straßen im Raster. Die Tabellendaten liefen in 1.4 über eine eigene Anfrage (`readTable`); in `architecture.md` §4 gibt es dafür keine Schnittstelle. (E) Bahnen sind lang; die 768² × 5 (ADR-013) bemessen nur das Gebäuderaster ohne Luftseite. Liegen Bahnen im selben Raster, ist ADR-013 neu zu bewerten oder die Wegweiser werden nur über bebauten Blöcken gespeichert.
9. **Veraltete Stellen in der Architektur.** (F) In `architecture.md` stehen noch: §2 „ECS … `[OFFEN]` bis Schritt 1.2", offene Fragen bei Welt & Bau (Schritt 1.2) sowie bei Brücke und S-1 (Schritt 1.3), außerdem „rayon (vorgesehen)".
10. **Zwei Kernstücke der Vision sind unerprobt.** (F) „Suchen ohne Schilder" (vor 3.3) und die Blockadevermeidung (vor 3.4) hatten keinen Spike. (E) Ersteres kann die Grundannahme aus ADR-004 berühren, dass der Aufwand von der Zahl der Ziele abhängt, nicht von der Zahl der Personen.

### Option 1: Weiterbauen

- **Kosten:** Die 7 Schritte, schätzungsweise 8–12 Sessions.
- **Risiko:** hoch für den Ablauf, gering für die Technik. Spätestens 2.2 stoppt am Eingangskriterium `[BELASTBAR]` (Befund 7). 2.2 und 2.3 stoßen auf die Lücken aus Befund 8; die Folge wären reaktive ADRs mitten in der UMSETZUNG, bei einer Reaktiv-Quote, die mit 30 % schon über dem Schwellenwert liegt. Ein neuer Workspace bricht am napi-Pin (Befund 5).

### Option 2: Gezielt umbauen (Plan und Entscheidungen, nicht Stack oder Pattern)

- **Was geändert wird:** Vor 2.2 einen Entwurfs- bzw. Erkundungsblock einschieben, mit ADRs zu Zellgröße und Weltmaßstab samt Ebenen für Bahnen, Vorfeld und Straßen (ADR-013 neu bewerten, gegebenenfalls Wegweiser in Blöcken), Tick- und Zeitmodell (1×-Rate, Spielsekunde je Tick), Verträgen für S-2/S-3 mit Vorschau und Bestätigung, neuer Schnittstelle „Abfrage/Tabellen", S-4 und S-5 im Entwurf. Ein ADR regelt, wie Module, die `[VORLÄUFIG]` sind, in Phase 2 starten dürfen. Reifegrad „NFR Performance Gesamtsystem" per ADR präzisieren: Kern belastbar, Darstellung vorläufig.
- **Kosten:** etwa 3–4 zusätzliche Schritte, rund 2–4 Sessions.
- **Risiko:** gering. Die Messwerte bleiben gültig, betroffen sind nur Plan und Dokumente.

### Option 3: Neu aufsetzen

- **Kosten:** Phase 1 im Kern wiederholen (Godot mit Rust-Kern oder Bevy), schätzungsweise 6–10 Sessions, dazu die verlorene Spike-Erkenntnis.
- **Risiko:** hoch.
- **Belege:** Keine Messung gibt dafür Anlass. Die Simulation braucht ein Fünfzehntel des Budgets, die Bildrate liegt an der Displaygrenze, und Regel-006 hält Godot ohnehin als Rückfall offen (ADR-014).

### Empfehlung der Prüf-Instanz

**Option 2. Konfidenz: mittel.** Gegen ein Neuaufsetzen sprechen die Messungen eindeutig. Ob der Umbau des Plans genügt, hängt aber an der ungemessenen realistischen Darstellungslast und am Verkehr.

### Hinweise für die Ausplanung von Phase 2

- **Erster Schritt (ERKUNDUNG/Entwurf):** Zellgröße, Kartenausdehnung mit 2 Bahnen, Lage der Luftseite, Tickrate, S-2/S-3 (Kennung, Bestätigung, Vorschau), S-4/S-5 und eine Abfrage-Schnittstelle für Tabellen. Die Sortierung großer Tabellen bekommt den Landeplatz 3.6; die Schnittstelle wird für eine Sortierung im Kern offengehalten.
- **Übernahmeliste Spike → Produktion je Schritt:** Wegweiser (`1.2/src/field.rs`), Dreifachpuffer und Bildformat (`1.3/bridge-core`), Electron-Härtung mit CSP und MessagePort-Handshake (`1.5/app/main.mjs`), ParticleContainer, arm64-Build mit Ad-hoc-Signatur (ADR-015) in `release.yml`. Jedes Stück wird neu geschrieben und bekommt Tests; die Verwechslung „inkrementell statt voll" zwischen 1.3/1.4 und 1.5 zeigt, wie leicht hier etwas verloren geht.
- **In 2.1 lösen:** napi-derive-backend bewusst festsetzen oder auf eine verträgliche napi-derive-Version gehen (Freigabe nach Q.1-Regeln). Außerdem entscheiden, ob die Spike-Workflows eingefroren oder entfernt werden.
- **Mess-Infrastruktur schon in Phase 2:** Kern-Benchmarks (Tick, Wegweiser 768² × 5) laufen headless in der CI als Trend mit Schwelle, nicht erst in 4.1. Die CI-Runner liefern für die Grafik keine aussagekräftigen Werte, für den Kern schon. Phase 2 als Abschlusskriterium: Das 1.5-Szenario läuft auf Produktionscode mit gleichwertigen Werten.
- **Determinismus:** Prüfmodus mit festem Übernahme-Tick in 2.2. FR-004 („Laden ergibt identischen Zustand") muss in 2.5 Wegweiser, die gerade neu berechnet werden, mit abdecken.
- **2.6 aufteilen:** Lokalisierung, Stilleitfaden und SVG-Pipeline sind drei Dinge. Der SVG-Rasterer ist eine neue Abhängigkeit (freigabepflichtig). Dazu eine Messung der Darstellung mit echten Atlanten und Objekt-Sprites.
- **Früher messen:** Das Testprogramm aus 1.5 jetzt für Mitwirkende veröffentlichen (Linux, schwache integrierte Grafik), statt bis 5.8/7.2 zu warten. ADR-011 Option B erlaubt das schon.
- **Erkundungen vorziehen:** „Suchen ohne Schilder" vor 3.2 statt vor 3.3, weil das Personenmodell davon abhängen kann. Die Blockadevermeidung vor 3.4 so belassen; 2.3 sieht aber schon Graphen für Straßen und Rollwege im Weltzustand vor.
- **Coverage-Gate:** In `ci.yml` gilt `--fail-under-lines 85` für alle Rust-Crates, einschließlich der Brücke. `project-context.md` §7 sieht 70 % global und 85 % für den Kern vor; das Gate auf den Kern beschränken.
- **Veraltete Stellen** aus Befund 9 bereinigen.

## Teil 2: Stellungnahme der bauenden KI

Nachgeprüft am Repository: Befunde 5 (Pin nur in `Cargo.lock`), 7 (alle Module und S-2/S-3/S-6 `[VORLÄUFIG]`, S-4/S-5 `[OFFEN]`), 8 (Tickrate 1× in `spikes/1.5-leistung/README.md` Z. 17 als Annahme), 9 (`architecture.md` Z. 88, 116, 192) und Coverage-Gate (`.github/workflows/ci.yml` Z. 118) treffen zu.

**Zustimmung:**

- Option 3 ist nicht begründbar; Option 2 ist richtig. Der Umbau betrifft Plan, Verträge und Dokumente, nicht Stack oder Pattern.
- Befund 7 benennt einen echten **Widerspruch** zwischen `CLAUDE.md` Abschnitt 6 („berührte Bestandteile `[BELASTBAR]` vor UMSETZUNG") und `architecture.md` Abschnitt 0 („Module bleiben `[VORLÄUFIG]`, bis ihre eigene Umsetzung sie belegt"; `[VORLÄUFIG]` „darf in der Implementierung verfeinert werden"). Wörtlich genommen könnte kein Modul je erstmals gebaut werden. Ohne ADR wäre das ein Stopp nach `CLAUDE.md` Abschnitt 8 Kriterium 2.
- Befund 2: „NFR Performance Gesamtsystem" ist zu weit gefasst. ADR-014 nennt seine Bedingungen, die Reifegrad-Zeile aber nicht die Grenze „nur Personen, einfache Darstellung". Eine Präzisierung ist eine Rückstufung eines Teils und braucht nach Abschnitt 0 einen ADR.
- Kern-Benchmarks headless in der CI schon in Phase 2 (Befund 3): billig, schützt genau das, was Phase 1 belegt hat.

**Ergänzung:**

- Der Vision-Abgleich (Teil 3) findet zusätzlich einen Muss-Punkt ohne Umsetzungsschritt (FR-039) und zwei Erfolgskriterien ohne Schritt-ID.
- Der Entwurfsschritt sollte die **Maßeinheit der Welt zuerst** klären (Zellgröße in Metern), weil Kartengröße, Lage der Luftseite und Bahnlängen davon abhängen; erst danach ist ADR-013 neu bewertbar.
- Die Darstellungslast mit echten Objekt-Sprites (Befund 2) gehört in den Schritt der SVG-Pipeline, nicht in einen eigenen Spike: Erst die Pipeline erzeugt die Atlanten, die gemessen werden können.

**Widerspruch:**

- „Früher messen durch Mitwirkende": Es gibt noch keine Mitwirkenden, das Projekt ist nicht angekündigt. Der Nutzen ist heute gering; die Auflagen-Landeplätze 4.1, 5.8 und 7.2 bleiben. Ein öffentlicher Aufruf wäre zudem eine eigene Entscheidung (Reichweite, Erwartungen der Community).
- Kosten von Option 1: Der Stopp aus Befund 7 ist kein technisches Risiko, sondern ein Regelwiderspruch, der unter jeder Option per ADR aufgelöst werden muss. Option 1 bleibt trotzdem schlechter wegen Befund 8.
- „Suchen ohne Schilder" vor 3.2 statt vor 3.3: Zustimmung im Grundsatz, aber nur, wenn 3.2 das Personenmodell festlegt; das entscheidet die Verfeinerung von Phase 3, nicht diese Vorlage.

## Teil 3: Vision-Abgleich an der Phasengrenze

Geprüft gegen `docs/vision.md` (Abschnitte 3–6) und `docs/requirements.md` Abschnitt 5.

| Befund | Art | Vorschlag |
|---|---|---|
| FR-039 „Große Tabellen-Panels" (Muss) hängt nur an Spike 1.4; kein Umsetzungsschritt | verwaist (Bug) | Landeplatz in Phase 3 (Flugplaner 3.6) und 5.6 (Personal, Finanzen); Sortierung im Kern dort |
| FR-017 in `requirements.md` nur mit 1.2, Fahrplan führt es auch in 3.3 | Drift | `requirements.md` auf „1.2, 3.3" |
| Erfolgskriterium „Gleichwertiges Spielerlebnis" (Spieltest des Initiators über mehrere Stunden) ohne Schritt-ID | verwaist (Bug) | eigener Schritt 7.3 |
| Erfolgskriterium „Machbarkeitstest" (Erkenntnis zur KI-gestützten Entwicklung) ohne Schritt-ID | verwaist (Bug) | Bilanz in 7.3 |
| Übrige Elemente (Plattformen, Sprachen, Benchmark, Raster, Etagen, Rollweg-Knoten, Passagierführung, Verkehr, Vorrang, Audio, Mods, Modernisierungen, Langzeit, Untergrenze, Cargo vorbereitet, SVG, Signatur) | Schritt-ID vorhanden | – |
| `[VERSCHOBEN]`-Zeilen | keine im Fahrplan | – |
