# Stellungnahme der bauenden Session zum Prüfbericht Bündel #34

- **Datum:** 2026-09-24
- **Bezug:** `docs/pruefberichte/2026-09-24-pflichtfrage-buendel-34.md` (getrennte Instanz). Der Bericht bleibt unverändert; diese Stellungnahme steht daneben (`CLAUDE.md` Abschnitt 12, Pflichtfrage, Schritt 2).
- **Verfasst von:** der Session, die S-13 bis S-22 gebaut hat (`claude-opus-5-5`, Entscheidungs-Klasse). Befangenheit ist Absicht dieser Rolle und hier offen benannt.

## Kurzfassung

1. **Ich stimme der Empfehlung „gezielt umbauen" zu.** Ich habe keinen Befund gefunden, der am Repo nicht stimmt.
2. **Nachgeprüft und bestätigt:** `CLAUDE.md` 79.934 Byte; eingecheckte `.pyc`-Datei (inzwischen entfernt, mein Fehler beim Commit nach dem Probelauf); „Externe Abhängigkeiten: keine" (`docs/project-context.md` Zeile 33) gegen `markdownlint-cli2`/`pre-commit`; „Keine Hilfsskripte" (`docs/architecture.md` Zeile 117) gegen das neue Skript; „Claude Code" und „Claude" als Handelnder in `CLAUDE.md` (Zeilen 3, 163, 219, 254, 307).
3. **Zwei Befunde muss ich als eigenes Versäumnis bestätigen:** Die Selbstausnahme der Methodik-ADRs von der Reaktiv-Quote (W3) habe ich selbst vorgeschlagen, und bei ADR-014 habe ich eine Option auch deshalb verworfen, weil sie den Wucherungs-Stopp ausgelöst hätte (W4). Beides ist genau das Muster aus #34 Punkt 4.
4. **Ergänzung aus dem Gesprächsverlauf, die der Bericht nicht sehen konnte:** In allen acht Entscheidungen dieses Tages hat der Eigentümer die empfohlene Option gewählt (S-13 B, S-14 A, S-15 B, S-16 C, S-20 B, S-21 D, S-22 A, Auflösung des Stopps A). Das beweist kein Abnicken, stützt aber den Befund 3.5 stärker, als der Bericht es ohne Logbuch konnte.

## Ergänzungen

- **Zu W1 (Fertig-Status):** Neun PRs (#35 bis #43) sind ohne CI-Lauf auf `main`. Der Nachlauf S-17 wartet auf den Kontingent-Reset, dessen Datum weiterhin offen ist. Das Risiko ist klein (nur Markdown-Linter, lokal grün), der Präzedenzfall ist es nicht.
- **Zu W6/W7 (Neutralitäts-Suche):** Ich habe den Grep in S-15, S-16 und S-22 nur über die jeweils neuen Zeilen laufen lassen. Die Regel in `docs/project-context.md` verlangt die ganze Datei. Das ist ein Verfahrensfehler von mir, kein Formulierungsproblem der Regel.
- **Weitere Landeplatz-Verstöße dieser Session** (im Logbuch belegt, dem Bericht unzugänglich): Der CI-Nachlauf stand drei Sessionenden lang nur als „offen geblieben" im Logbuch, bevor er S-17 wurde; der dritte Vorschlag aus #34 Punkt 9 hatte bis S-14 keinen Schritt. Beides stützt U3.
- **Messfehler, selbst korrigiert:** Die in drei Sessionende-Einträgen genannten Kostenzählerstände waren wegen verzögerter Aktualisierung keine Messwerte (Korrektur im Logbuch 20:50). Das betrifft die Glaubwürdigkeit von Zahlen in dieser Session und spricht für U4 (ehrlicher Reifegrad).

## Wo ich widerspreche oder einschränke

- **Zu B2 (90.000 Token Pflichtlektüre):** Die Schätzung beruht auf einem Verhältnis von rund 1,9 Byte je Token aus einer einzigen Lesung. Für deutschen Text liegt das eher bei 3 bis 4 Byte je Token; `CLAUDE.md` wäre dann eher 20.000 bis 27.000 Token statt 43.000. Die Richtung des Befunds (Regelwerk wächst ohne Grenze, Lektüre wird teurer) bleibt richtig; die Zahl ist unsicher. Messen lässt sie sich über die Kontextgröße einer frischen Session direkt nach der Mindest-Lektüre.
- **Zu B4 (Begründungen in die ADRs verschieben):** Einschränkung, keine Ablehnung. Teil B der ADRs gehört nicht zur Mindest-Lektüre. Einige „Warum"-Absätze verhindern, dass ein Agent eine Regel für überflüssig hält und sie aufweicht (Beispiel: „Warum kein Ersatz-Vorgehen mehr", Lehre aus ADR-005). Vorschlag: je Regel höchstens ein Satz Begründung im Regeltext, der Rest in den ADR.
- **Zu A3 (zwei Stopps für eine Entscheidung):** Trifft zu, wenn der Alltag auf der Routine-Klasse läuft. In diesem Repo lief jede Session auf der Entscheidungs-Klasse; dort gibt es nur einen Stopp. Die Zusammenlegung (U2) bleibt trotzdem richtig.
- **Zur Reihenfolge:** Die reinen Korrekturen der Selbstanwendung aus U3 (Abhängigkeiten, Skript-Eintrag, `.pyc`, Verweisfehler W8) brauchen keine Freigabe und können sofort erledigt werden. Freigabepflichtig sind U1, U2, die inhaltlichen Teile von U3 (Fertig-Status, Phasentyp, Reaktiv-Regel, Bündel-Zählung) und U4.

## Grenzen dieser Prüfung

- Die getrennte Instanz lief auf derselben Modell-Linie (`claude-opus-5-5`). Die Forderung „möglichst ein anderes Modell" ist nicht erfüllt; gleichartige blinde Flecken sind möglich.
- Der Eigentümer wurde am selben Tag bereits acht Mal um Grundsatzentscheidungen gebeten. Die Entscheidung über diesen Bericht ist die neunte.
