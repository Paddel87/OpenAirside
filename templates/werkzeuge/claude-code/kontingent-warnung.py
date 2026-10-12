#!/usr/bin/env python3
# Zweck: Warnt im Gespräch, wenn das 7-Tage-Kontingent eines Pro/Max-Abos eine Schwelle
#   erreicht (CLAUDE.md Abschnitt 0; Entscheidung in docs/decisions.md, ADR zu S-22).
#   Zwei Modi: „statuszeile" speichert die Limit-Angaben der Statuszeile in eine Datei,
#   „hook" liest sie bei jeder Eingabe (UserPromptSubmit) und gibt ab der Schwelle eine Warnung aus.
# Voraussetzungen: Python 3.8+ (nur Standardbibliothek), Claude Code mit Statuszeile.
#   ENV (optional): KONTINGENT_WARNSCHWELLEN (Default „80,95"), KONTINGENT_STATUSDATEI
#   (Default ~/.claude/kontingent-stand.json).
# Plattformen: Linux, macOS, Windows (Python im PATH). Nur lokale Terminal-Sessions:
#   Cloud-Sessions ohne Statuszeile liefern keine Daten – dann schweigt der Hook.
# Exit-Codes: immer 0 – ein Fehler dieses Hilfsskripts darf keine Eingabe blockieren;
#   Fehler landen auf stderr (bewusste Abweichung von „Abbruch bei Fehler").
# Idempotenz: „statuszeile" überschreibt die Statusdatei mit dem neuesten Stand, „hook" liest nur.
#   Wiederholter Aufruf mit derselben Eingabe hat denselben Effekt.
# Reproduzierbarkeit: Die Ausgabe von „hook" hängt von der Statusdatei und der Uhrzeit ab (nach dem
#   Zurücksetzen des Fensters schweigt er). Zwei Aufrufe liefern dasselbe, solange dazwischen keine
#   Statuszeile neue Werte geschrieben hat und das Fenster nicht zurückgesetzt wurde.
import json
import os
import sys
import time

STATUSDATEI = os.environ.get(
    "KONTINGENT_STATUSDATEI", os.path.expanduser("~/.claude/kontingent-stand.json")
)


def schwellen():
    roh = os.environ.get("KONTINGENT_WARNSCHWELLEN", "80,95")
    return sorted(float(w) for w in roh.split(",") if w.strip())


def statuszeile():
    daten = json.load(sys.stdin)
    fenster = (daten.get("rate_limits") or {}).get("seven_day")
    if fenster:
        os.makedirs(os.path.dirname(STATUSDATEI), exist_ok=True)
        with open(STATUSDATEI, "w", encoding="utf-8") as f:
            json.dump(fenster, f)
    modell = (daten.get("model") or {}).get("display_name", "?")
    anteil = f"{fenster['used_percentage']:.0f} % Woche" if fenster else "Woche: –"
    print(f"[{modell}] {anteil}")


def hook():
    try:
        with open(STATUSDATEI, encoding="utf-8") as f:
            fenster = json.load(f)
    except (OSError, ValueError):
        return  # keine Daten: schweigen
    if fenster.get("resets_at", 0) <= time.time():
        return  # Fenster schon zurückgesetzt: Angabe veraltet
    anteil = float(fenster.get("used_percentage", 0))
    erreicht = [s for s in schwellen() if anteil >= s]
    if erreicht:
        t = time.localtime(fenster["resets_at"])
        tag = ("Mo", "Di", "Mi", "Do", "Fr", "Sa", "So")[t.tm_wday]
        zurueck = f"{tag} {time.strftime('%d.%m. %H:%M', t)}"
        print(
            f"KONTINGENT-WARNUNG: {anteil:.0f} % des 7-Tage-Kontingents verbraucht "
            f"(Schwelle {erreicht[-1]:.0f} %), Zurücksetzung {zurueck}. "
            "Dem Menschen melden; Arbeit nach CLAUDE.md Abschnitt 0 sparsam planen."
        )


if __name__ == "__main__":
    try:
        {"statuszeile": statuszeile, "hook": hook}[sys.argv[1]]()
    except Exception as fehler:  # noqa: BLE001 – Hilfsskript darf die Sitzung nie blockieren (S-22)
        print(f"kontingent-warnung: {fehler}", file=sys.stderr)
    sys.exit(0)
