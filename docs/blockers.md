# Blockers

<!-- Ungelöste Probleme und gescheiterte Ansätze von OpenAirside.
     Wird befüllt, wenn ein Arbeitsschritt nach drei Versuchen nicht gelöst werden konnte
     (CLAUDE.md Abschnitt 10) oder ein Muster der Blocker-Erkennung zutrifft. -->

<!-- ANCHOR:blocker-erkennung -->
## Blocker-Erkennung (vor dem Dreifach-Versuch)

Ein Problem ist **sofort** als Blocker zu behandeln, ohne drei Versuche abzuwarten, wenn eines dieser Muster zutrifft:

1. **Informationslücke:** Eine für die Lösung nötige Angabe fehlt in allen Pflicht-Dokumenten.
2. **Widerspruch:** Zwei Dokumente geben unvereinbare Vorgaben und kein ADR löst den Konflikt auf.
3. **Fremde Modulgrenze:** Die Lösung würde Änderungen in einem Modul erfordern, das nicht Teil des aktuellen Fahrplan-Schritts ist.
4. **Freigabebedarf:** Die Lösung fällt in eine Kategorie aus CLAUDE.md Abschnitt 4.
5. **Nicht-deterministisches Verhalten:** Das Problem tritt nicht reproduzierbar auf. Nicht-Reproduzierbarkeit ist selbst ein Blocker, keine akzeptierte Eigenschaft.

In diesen Fällen: direkt Eintrag hier anlegen, ohne Dreifach-Versuch.

Für alle anderen Fälle gilt die Dreifach-Regel aus CLAUDE.md Abschnitt 10.

---

<!-- ANCHOR:aktive-blocker -->
## Aktive Blocker

Keine aktiven Blocker.

---

<!-- ANCHOR:geloeste-blocker -->
## Gelöste Blocker

### Blocker #001: Leistungsziele nicht messbar – GELÖST 2026-10-13

- **Ursprüngliche Beschreibung:** Im Härtungs-Schritt der Initialisierung (Modus 2, Schritt 3) festgestellt: Die Vision fordert „flüssig" auf „Zielhardware", ohne Bildrate, Simulationstempo und Referenzhardware zu beziffern. Das zentrale Erfolgskriterium wäre damit nicht prüfbar. Blocker-Typ: Informationslücke.
- **Lösung:** Entscheidung des Projektverantwortlichen: Referenzhardware Mittelklasse-PC (8 CPU-Kerne, 16 GB RAM, Mittelklasse-Grafikkarte der aktuellen Generation); Zielwert mindestens 60 Bilder pro Sekunde im Referenzszenario, Simulation ohne Rückstand auch bei höchster Spielgeschwindigkeit. Eingetragen in `project-context.md` Abschnitt 6.
- **ADR:** keiner (Präzisierung eines Constraints; Grundlage für ADR-002)
- **Abgeleitete Regel:** keine
