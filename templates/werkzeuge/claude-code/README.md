# Kontingent-Warnung für Claude Code (optional)

Warnt im Gespräch, sobald das 7-Tage-Kontingent eines Pro- oder Max-Abos eine Schwelle erreicht (Default 80 % und 95 %). Gehört zu `CLAUDE.md` Abschnitt 0; die Regel dazu steht im Regelwerk werkzeugneutral, diese Umsetzung ist werkzeugspezifisch.

## Wie es funktioniert

1. Die **Statuszeile** von Claude Code erhält bei Pro/Max die Angabe `rate_limits.seven_day` (Verbrauch in Prozent, Zurücksetz-Zeitpunkt). `kontingent-warnung.py statuszeile` speichert sie in `~/.claude/kontingent-stand.json` und zeigt Modell und Wochenverbrauch an.
2. Ein **Hook** auf `UserPromptSubmit` ruft `kontingent-warnung.py hook` auf. Ab der Schwelle gibt er eine Warnung aus; Claude Code fügt die Ausgabe dieses Ereignisses dem Gespräch hinzu.

Hooks selbst erhalten keine Limit-Angaben – deshalb der Umweg über die Statuszeile (Befund S-19 in Dev-Templates).

## Einrichten (lokal, Terminal-Session)

Skript nach `~/.claude/` kopieren, dann in `~/.claude/settings.json` ergänzen:

```json
{
  "statusLine": { "type": "command", "command": "python3 ~/.claude/kontingent-warnung.py statuszeile" },
  "hooks": {
    "UserPromptSubmit": [
      { "hooks": [ { "type": "command", "command": "python3 ~/.claude/kontingent-warnung.py hook" } ] }
    ]
  }
}
```

Schwellen ändern: Umgebungsvariable `KONTINGENT_WARNSCHWELLEN`, z. B. `70,90`.

## Probelauf (Pflicht vor dem Einsatz, `CLAUDE.md` Abschnitt 6 „Schutzmechanismen durch erzwungenen Fehler belegen")

1. `KONTINGENT_WARNSCHWELLEN=0` setzen und Claude Code neu starten.
2. Eine beliebige Eingabe senden. **Erwartet:** Die KI erwähnt die Kontingent-Warnung. Kommt keine Warnung, läuft die Statuszeile nicht oder liefert keine Limit-Angaben – dann bleibt die Warnung inaktiv.
3. Schwelle zurücksetzen, Ergebnis mit Datum in `docs/project-context.md` Abschnitt 6 eintragen.

## Test

`test_kontingent_warnung.py` prüft beide Betriebsarten und wiederholt den erzwungenen Fehlerfall (Schwelle 0 %) automatisch; nur Standardbibliothek:

```bash
python3 -m unittest discover -s <Pfad zu diesem Verzeichnis>
```

Der Test ersetzt den Probelauf oben nicht: Er belegt das Skript, nicht den Zustellweg bis ins Gespräch.

## Grenzen

- **Cloud-Sessions** (Web, App) zeigen keine Statuszeile; dort schweigt der Hook.
- Bei API-Nutzung gibt es kein Abo-Kontingent und keine `rate_limits`-Angabe.
- Der Stand ist so aktuell wie der letzte Lauf der Statuszeile.
