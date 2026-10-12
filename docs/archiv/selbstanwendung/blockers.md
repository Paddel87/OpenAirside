# Blockers – Dev-Templates

<!-- Arbeitsdokument von Dev-Templates selbst (Selbstanwendung, ADR-001).
     Die Vorlage für Ziel-Projekte liegt unter templates/docs/blockers.md. -->

<!-- ANCHOR:blocker-erkennung -->
## Blocker-Erkennung (vor dem Dreifach-Versuch)

Ein Problem ist **sofort** als Blocker zu behandeln, ohne drei Versuche abzuwarten, wenn eines dieser Muster zutrifft:

1. **Informationslücke:** Eine für die Lösung nötige Angabe fehlt in allen Pflicht-Dokumenten.
2. **Widerspruch:** Zwei Dokumente geben unvereinbare Vorgaben und kein ADR löst den Konflikt auf.
3. **Fremde Modulgrenze:** Die Lösung würde Änderungen in einem Bestandteil erfordern, der nicht Teil des aktuellen Fahrplan-Schritts ist.
4. **Freigabebedarf:** Die Lösung fällt in eine Kategorie aus `CLAUDE.md` Abschnitt 4.
5. **Nicht-deterministisches Verhalten:** Das Problem tritt nicht reproduzierbar auf.

Für alle anderen Fälle gilt die Dreifach-Regel aus `CLAUDE.md` Abschnitt 10.

---

<!-- ANCHOR:aktive-blocker -->
## Aktive Blocker

Keine aktiven Blocker.

Die drei offenen Punkte aus `docs/project-context.md` Abschnitt 11 (Lizenz, Markdown-Linter, CI-Pipeline) sind **keine Blocker**: Sie hindern keinen Fahrplan-Schritt an der Fortsetzung, sondern sind selbst als Schritte S-4 bis S-6 geführt. Sie werden hier bewusst nicht dupliziert.

---

<!-- ANCHOR:geloeste-blocker -->
## Gelöste Blocker

Noch keine.
