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

Phase 1 läuft. Schritte 1.1 (ADR-009) und 1.2 (ADR-010) erledigt. Nächster Schritt: 1.3 (parallel möglich: 1.6), in neuer Session.

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

### 2026-10-13 02:25 – [SESSIONENDE]

- **Session-Dauer:** ca. 20 Minuten nach dem `[SESSIONENDE]` von 02:05 (dieselbe Sitzung)
- **Bearbeitet:** Merge von PR Paddel87/OpenAirside#2 auf Anweisung; Einschätzung zur Umsetzbarkeit (keine Dokumentänderung); Testsysteme für 1.5 im Fahrplan vermerkt.
- **Erreicht:** Testsysteme für 1.5 geklärt: Referenz-PC mit AMD Radeon RX 7700 XT (Windows, Linux per Live-USB), Mac M3 Pro mit 18 GB.
- **Offen geblieben:** Untergrenze „integrierte Grafik" ohne schwaches Testgerät nur hochrechenbar; Intel-Teil des macOS-Universal-Builds ungetestet; Treiberunterstützung RX 7700 XT unter Linux in 1.5 gegen Quellen belegen.
- **Nächster Schritt:** 1.3 (parallel 1.6) in neuer Session.
- **Modell-Bilanz:** Entscheidungs-Klasse (`claude-opus-5-5`, eingestellt und bedient laut `get_session`); 0 Schritte oberhalb der Empfehlung; keine Abgaben.
- **Kontextgröße:** 456.458 Token laut `get_session` – **über der Grenze von 400.000**. Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf. **Drift-Prüfung:** nur Fahrplan-Notizen zu 1.5 geändert, keine Auswirkung auf README oder andere Pflicht-Dokumente.

### 2026-10-13 02:05 – [SESSIONENDE]

- **Session-Dauer:** ca. 60 Minuten seit `[SESSIONSTART]` 01:10
- **Bearbeitet:** 1.2 (Prototyp, Messreihe, Entscheidungsvorlage, ADR-010); Auslagerung der ADRs nach `docs/decisions/` (ADR-001: ab zweistelliger Anzahl)
- **Erreicht:** 1.2 `[ERLEDIGT]` nach Freigabe „A, freigegeben, Referenzszenario mit Reserve"; Datenmodell `[OFFEN]` → `[VORLÄUFIG]`; Wegfindung bleibt `[VORLÄUFIG]` bis 1.5; 1.5 um Rastergröße und fünf Etagen ergänzt; Landeplatz für „Suchen ohne Schilder"/Feinwege: 3.3.
- **Offen geblieben:** PR Paddel87/OpenAirside#2 wartet auf Review und Merge (CI auf `2befa46` grün, Code-Jobs bis 2.1 übersprungen); `shellcheck` im Container nicht installiert, `run-all.sh` des Spikes daher ungeprüft (Spike-Skript, nicht in `scripts/`).
- **Nächster Schritt:** 1.3 – Zustandsübergabe und Spielfeld-Darstellung (parallel möglich 1.6), in neuer Session.
- **Modell-Bilanz:** aktive Klasse Entscheidung (`claude-opus-5-5`, eingestellt und bedient laut `get_session` am Sessionende); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben.
- **Kontextgröße:** 440.467 Token laut `get_session` – **über der Grenze von 400.000** (`project-context.md` Abschnitt 6). Hinweis: nächste Arbeit in einer neuen Session beginnen.
- **Ablaufdaten-Prüfung:** kein Eintrag mit erreichtem Vorlauf.
- **Drift-Prüfung:** README (Phase, Architektur-Reife, Nächste Schritte) nachgezogen; ADR-010 verweist auf 1.2, 1.5 (existieren); Reifegrade passen zu ADR-009/010; Reaktiv-Quote 0/10; Phase 1 weiter 7 Schritte.
- **Archivierung:** Teil B von `decisions.md` in Einzeldateien `docs/decisions/ADR-001.md` bis `ADR-010.md` ausgelagert; Logbuch unter 800 Zeilen.

### 2026-10-13 02:00 – [ADR-ANGELEGT]

- **ADR:** ADR-010 „Datenmodell und Parallelisierung im Kern, Übernahme der Wegweiser" (`docs/decisions/ADR-010.md`)
- **Tag:** `[ERKENNTNIS]`; Reaktiv-Quote 0/10
- **Auslöser:** Freigabe „A, freigegeben, Referenzszenario mit Reserve"

### 2026-10-13 02:00 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Datenmodell und Parallelisierung im Kern
- **Wechsel:** `[OFFEN]` → `[VORLÄUFIG]` (Wegfindung bleibt `[VORLÄUFIG]`: fünf Etagen und Rastergröße erst in 1.5)
- **Auslöser:** Spike 1.2, ADR-010
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja

### 2026-10-13 01:50 – [BEOBACHTUNG]

- **Spike 1.2 abgeschlossen bis zur Entscheidung:** Messreihe in `spikes/1.2-wegweiser/results/`, Auswertung in dessen README. Vorlage im Fahrplan 1.2.
- **Inkrementelle Reparatur der Wegweiser** liefert exakt dieselben Distanzen wie die Vollberechnung (Test), ist aber beim Schließen und Öffnen von Türen langsamer, weil Türen Engstellen sind. Erster Versuch scheiterte am Test: Neue Wände verbieten auch diagonale Schritte der Nachbarzellen (Eckregel); Lösung: Nachbarn geänderter Zellen als zusätzliche Wurzeln.
- **Informationslücke:** Rastergröße des Referenzszenarios und der größten Karte ist in keinem Dokument festgelegt; als Frage in die Entscheidungsvorlage aufgenommen. Spike lief mit 2 statt 5 Etagen (BAU-07).
- **Widerspruch erkannt, nicht aufgelöst:** Determinismus (Architektur-Ziel) gegen „Bauen ohne Wartezeit" (Vision) bei langen Neuberechnungen – Teil der Entscheidungsvorlage.

### 2026-10-13 01:10 – [SESSIONSTART]

- **Letzter Stand:** `[SESSIONENDE]` 01:05; PR Paddel87/OpenAirside#1 gemergt (`5813931`), Branch auf `main` vorgespult.
- **Pflichtlektüre:** Die Dokumente auf `main` unterscheiden sich nicht vom Stand des letzten Commits dieser Sitzung (`git diff 1c9da21 HEAD` leer); die in derselben Sitzung gelesenen und selbst geschriebenen Abschnitte sind damit aktuell. Kein neues Lesen nötig.
- **Geplant für diese Session:** Schritt 1.2 – Wegweiser-Prinzip mit 10.000 Personen (Prototyp, Zeitbox zwei Sessions).
- **Vorabprüfung:** 1.2 ist ERKUNDUNG; Eingangskriterium „Betriebsart aus 1.1 entschieden" erfüllt (ADR-009). Wegfindung `[VORLÄUFIG]`, Datenmodell `[OFFEN]` – Gegenstand des Schritts. Keine aktiven Blocker.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (4 vCPU, keine GPU), Branch `claude/gracious-galileo-2reksz`; Fortsetzung derselben Sitzung auf Wunsch des Projektinhabers.
- **Modell:** eingestellt und bedient `claude-opus-5-5` → Entscheidungs-Klasse (Quelle `get_session`, 00:3x). Empfohlene Klasse 1.2: Entscheidung – passt.
- **Kontextgröße zu Beginn:** rund 310.000 Token; die Grenze von 400.000 wird in diesem Schritt voraussichtlich überschritten.

### 2026-10-13 01:05 – [SESSIONENDE]

- **Session-Dauer:** ca. 65 Minuten (ersetzt den vorläufigen Eintrag von 00:45, nach dem die Session für die Freigabe offen blieb)
- **Bearbeitet:** 1.1 (Spike, Messreihe, Entscheidungsvorlage, ADR-009), Q.2 neu angelegt
- **Erreicht:** 1.1 `[ERLEDIGT]`: Kern als napi-rs-Modul im Electron-Utility-Prozess (ADR-009, Regel-006); Betriebsart `[OFFEN]` → `[VORLÄUFIG]`; WebAssembly mit Threads zurückgestellt nach Q.2 (Auslegung vom Projektinhaber bestätigt). CI-Läufe 4 und 5 grün.
- **Offen geblieben:** nichts aus 1.1. Spike-Code wird von der CI bis 2.1 nicht gebaut (nur Dokumentations-Prüfung).
- **Nächster Schritt:** 1.2 – Wegweiser-Prinzip mit 10.000 Personen (Entscheidungs-Klasse, bis zu zwei Sessions); parallel möglich 1.6. Empfehlung: in neuer Session beginnen.
- **Modell-Bilanz:** aktive Klasse Entscheidung (eingestellt und bedient `claude-opus-5-5`, Quelle `get_session` am Sessionende); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben (Probelauf für Unteragenten offen).
- **Kontextgröße:** 307.567 Token laut `get_session` (`context_usage`), unter der Grenze von 400.000; kein Hinweis nötig.
- **Ablaufdaten-Prüfung:** kein Eintrag hat seinen Vorlauf erreicht; neu: Nachprüfung WebAssembly-Threads (2027-01-02, Q.2).
- **Drift-Prüfung:** README (Phase, Architektur-Reife, Nächste Schritte, Blocker 0) synchron; ADR-009 verweist auf existierende Schritte 1.3, 1.5, 1.6, Q.2; Reifegrad Betriebsart passt zu ADR-009; Modulnamen in 1.1/1.3/Q.2 wie in der Modul-Karte; Reaktiv-Quote 0/9; Phase 1 weiter 7 Schritte (Q.2 ist Querschnitt).
- **Archivierung/Budgets:** Logbuch 146 Zeilen, `project-context.md` 352 Zeilen – keine Trigger.
- **Uncommitted:** nichts.

### 2026-10-13 01:00 – [BEOBACHTUNG]

- **CI für Schritt 1.1:** Läufe 4 (`865b25f`) und 5 (`d912570`) grün. Aktiv sind bis Schritt 2.1 nur „Dokumentation (pre-commit)" und „Projektteile erkennen"; die Code-Jobs werden mangels `Cargo.toml`/`package.json` am Repo-Stamm übersprungen, der Spike-Code wird in der CI also nicht gebaut. Einzige Annotation: der bekannte Hinweis zur Umstellung von `ubuntu-latest` auf Ubuntu 26 ab 2026-10-19 (Ablaufdaten-Register, Q.1).

### 2026-10-13 00:55 – [REIFEGRAD-WECHSEL]

- **Bestandteil:** Betriebsart des Rust-Kerns
- **Wechsel:** `[OFFEN]` → `[VORLÄUFIG]`
- **Auslöser:** Spike 1.1 und ADR-009; `[BELASTBAR]` erst nach dem Leistungsnachweis 1.5
- **Datum in `architecture.md` Abschnitt 9 nachgetragen:** ja

### 2026-10-13 00:55 – [ADR-ANGELEGT]

- **ADR:** ADR-009 „Betriebsart des Kerns: napi-rs im Electron-Utility-Prozess“, dazu Regel-006 „Kern frei von Anbindungscode“
- **Tag:** `[ERKENNTNIS]` – klärt einen in ADR-002 offen gelassenen Punkt, ändert ADR-002 nicht; Reaktiv-Quote 0/9
- **Auslöser:** Freigabe des Projektinhabers „A, freigegeben, B perspektivisch“. „Perspektivisch“ ausgelegt als: B wird wieder geprüft, sobald WebAssembly-Threads mit stabilem Rust baubar sind → Landeplatz Q.2, Nachprüfung im Ablaufdaten-Register (erstmals 2027-01-02). Auslegung vom Projektinhaber am 2026-10-13 bestätigt („genau das meinte ich“).
- **Modell:** weiterhin `claude-opus-5-5` (Entscheidungs-Klasse), erneut über `get_session` abgefragt; Kontext 267.791 Token laut Laufzeitumgebung (unter der Grenze von 400.000).

### 2026-10-13 00:45 – [SESSIONENDE] (vorläufig, Session blieb für die Freigabe offen)

- **Session-Dauer:** ca. 45 Minuten
- **Bearbeitet:** Schritt 1.1 (Vergleichsstudie napi-rs gegen WebAssembly)
- **Erreicht:** Spike `spikes/1.1-betriebsart/` mit gemeinsamem Kern, napi-Anbindung, WebAssembly ein- und mehrthreadig und gehärteter Electron-Messumgebung; Messreihe mit 17 Konfigurationen × 2 Wiederholungen plus native Grundlinie; Messprotokoll in `spikes/1.1-betriebsart/README.md`. Entscheidungsvorlage im Fahrplan, Schritt 1.1 auf `WARTET-AUF-FREIGABE`.
- **Offen geblieben:** Freigabe der Betriebsart; danach ADR (ERKENNTNIS), `architecture.md` (Betriebsart `[OFFEN]` → `[VORLÄUFIG]`, S-1 je Betriebsart) und Abschluss von 1.1.
- **Nächster Schritt:** Antwort auf die Entscheidungsvorlage 1.1, danach 1.2.
- **Modell-Bilanz:** aktive Klasse Entscheidung (eingestellt und bedient `claude-opus-5-5`, Quelle `get_session`); 0 Schritte oberhalb der Empfehlung; keine Teilarbeiten abgegeben (Probelauf für Unteragenten offen).
- **Kontextgröße:** nicht feststellbar – `get_session` meldet `context_usage.used_tokens: 0`, offensichtlich nicht aktuell; Hinweis entfällt.
- **Ablaufdaten-Prüfung:** Kein Eintrag hat seinen Vorlauf erreicht (Electron 44: Vorlauf ab 2027-01-02). Zeile „Claude-Kontingent“ in `project-context.md` Abschnitt 8 mit den Angaben der Laufzeitumgebung gefüllt.
- **Drift-Prüfung:** README (Phase, Nächste Schritte) nachgezogen; Modul-Liste, Blocker (0), Reaktiv-Quote (0/8) unverändert; kein neuer ADR; Phasenumfang 7 Schritte unverändert.
- **Hinweis:** Die Session bleibt für die Antwort offen; wird sie fortgesetzt, folgt kein neuer `[SESSIONSTART]`.

### 2026-10-13 00:35 – [BEOBACHTUNG]

- **WebAssembly mit Threads braucht Nightly-Rust.** Stabiles Rust 1.98.1 akzeptiert `-C target-feature=+atomics` nur mit der Warnung „unstable feature“; die mitgelieferte Standardbibliothek ist nicht threadfähig. Die wasm-bindgen-Doku („Parallel Raytracing“) nennt Nightly und `-Z build-std`. Im Spike mit `nightly-2026-10-12` gebaut, nur für die Messung.
- **Determinismus nativ = WebAssembly:** bitgleiche Prüfsummen über nativ (1/2/4 Threads), wasm ein- und mehrthreadig, solange nur IEEE-exakte Operationen vorkommen. Relevant für 1.2 und spätere Spielstände.
- **Abweichung Werkzeugversion:** Der Container hat Node 22.22.0 statt 26.11.1 (`project-context.md` Abschnitt 3); betrifft nur npm und Hilfsskripte des Spikes, Electron bringt eigenes Node 24 mit.
- **Code-Hooks:** Die lokalen pre-commit-Code-Hooks (cargo fmt/clippy am Repo-Wurzel, Prettier/ESLint über die Projekt-`package.json`) setzen Schritt 2.1 voraus. Im Spike liefen `cargo fmt` und `cargo clippy -D warnings` (für `sim-wasm` zusätzlich `undocumented_unsafe_blocks`) direkt im Spike-Verzeichnis, die übrigen Hooks über `pre-commit run`: Dokumentations-Hooks und Prettier grün (Prettier lief als im Container vorhandene 3.8.1, nicht als Projekt-Version 3.9.10, die erst mit 2.1 installiert wird); ESLint ohne Projektkonfiguration nicht anwendbar (wie in der CI bis 2.1 übersprungen).

### 2026-10-13 00:25 – [PROBLEM-GELÖST]

- **Kontext:** napi-Variante in Electron
- **Symptom:** Lauf endete im Timeout ohne Fehlermeldung.
- **Ursache:** Der Renderer registrierte den Listener für den MessagePort erst nach dem Warten auf die Konfiguration; der Hauptprozess schickt den Port aber zuerst, die Nachricht ging verloren.
- **Lösung:** Listener vor dem ersten `await` registrieren (`app/renderer/main.js`).
- **Aufwand:** 5 Minuten
- **Lerneffekt:** In Phase 2 die Port-Übergabe als Handshake mit Bestätigung bauen, nicht auf die Reihenfolge verlassen.

### 2026-10-13 00:15 – [PROBLEM-GELÖST]

- **Kontext:** Build von `sim-napi` mit den in ADR-002 fixierten Versionen (napi 3.12.7, napi-derive 3.6.8)
- **Symptom:** Kompilierfehler in `napi-derive` („`?` couldn't convert the error to `Diagnostic`“).
- **Ursache:** Cargo wählte die neueste Unterabhängigkeit `napi-derive-backend` 6.1.5 (2026-10-05), die mit napi-derive 3.6.8 nicht kompatibel ist (Bruch in einer Patch-Version).
- **Lösung:** `cargo update -p napi-derive-backend --precise 6.1.4` (erschienen am selben Tag wie napi-derive 3.6.8). Begleit-Crate `napi-build` 2.4.4 gewählt (napi 3.12.7 verlangt `^2.4.4`; Linie 2.4 seit 2026-07-28, mit Patches).
- **Aufwand:** 5 Minuten
- **Wiederkehrgefahr:** hoch – napi-rs veröffentlicht mehrmals pro Woche; `Cargo.lock` einchecken und Aktualisierungen bewusst in Q.1 durchführen.

### 2026-10-13 00:07 – [PROBLEM-GELÖST]

- **Kontext:** Electron-Start im Cloud-Container
- **Symptom:** `Running as root without --no-sandbox is not supported`, Abbruch mit SIGTRAP.
- **Ursache:** Der Container läuft als root; Chromium verweigert die OS-Sandbox als root.
- **Lösung:** Messläufe mit `--no-sandbox` (nur Messumgebung; `sandbox: true` des Renderers bleibt gesetzt).
- **Aufwand:** 2 Minuten

### 2026-10-13 00:05 – [SESSIONSTART]

- **Letzter Stand:** Initialisierung abgeschlossen (Commit `e130cd4`), danach CI-Anpassung `b6fb933`. Kein `[SESSIONENDE]`-Eintrag vorhanden (erste reguläre Session).
- **Geplant für diese Session:** Schritt 1.1 – Betriebsart des Rust-Kerns (Vergleichsstudie napi-rs gegen WebAssembly), Ergebnis als Entscheidungsvorlage.
- **Vorabprüfung:** 1.1 ist ERKUNDUNG, keine Eingangskriterien auf `[BELASTBAR]` nötig. Betriebsart `[OFFEN]` – genau das ist Gegenstand des Schritts. Keine aktiven Blocker.
- **Modus/Werkzeug:** Claude Code, Cloud-Session (Linux-Container, 4 CPU-Kerne, 15 GB RAM, keine GPU), Branch `claude/gracious-galileo-2reksz`.
- **Modell:** eingestellt `claude-opus-5-5`, bedient `claude-opus-5-5` → Entscheidungs-Klasse. Quelle: Laufzeitabfrage `get_session` (`configured_model`, `last_served_model`). Empfohlene Klasse für 1.1: Entscheidung – passt.
- **Kontingent:** Laufzeitumgebung meldet Art „five_hour" (rollierendes 5-Stunden-Fenster), Status „allowed", nächstes Zurücksetzen 2026-10-13 03:00 UTC. Quelle: `get_session` (`rate_limit_info`).

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
