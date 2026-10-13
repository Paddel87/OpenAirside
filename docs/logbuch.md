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

Phase 1 läuft. Schritt 1.1 erledigt: Kern als napi-rs-Modul im Electron-Utility-Prozess (ADR-009); WebAssembly mit Threads zurückgestellt nach Q.2. Nächster Schritt: 1.2 (parallel möglich: 1.6).

---

<!-- ANCHOR:eintraege -->
## Einträge (neueste oben)

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
- **Auslöser:** Freigabe des Projektinhabers „A, freigegeben, B perspektivisch“. „Perspektivisch“ ausgelegt als: B wird wieder geprüft, sobald WebAssembly-Threads mit stabilem Rust baubar sind → Landeplatz Q.2, Nachprüfung im Ablaufdaten-Register (erstmals 2027-01-02).
- **Modell:** weiterhin `claude-opus-5-5` (Entscheidungs-Klasse), erneut über `get_session` abgefragt; Kontext 267.791 Token laut Laufzeitumgebung (unter der Grenze von 400.000).

### 2026-10-13 00:45 – [SESSIONENDE]

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
