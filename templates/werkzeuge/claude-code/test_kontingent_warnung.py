"""Tests für kontingent-warnung.py – wiederholen den erzwungenen Fehlerfall aus S-22 automatisch.

Voraussetzungen: Python 3.8+ (nur Standardbibliothek).
Aufruf: python -m unittest discover -s templates/werkzeuge/claude-code
"""

import importlib.util
import io
import json
import os
import subprocess
import sys
import tempfile
import time
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

SKRIPT = Path(__file__).with_name("kontingent-warnung.py")


def lade_modul(statusdatei, schwellen=None):
    """Lädt das Skript frisch, weil es die Statusdatei beim Import festlegt."""
    umgebung = {"KONTINGENT_STATUSDATEI": str(statusdatei)}
    if schwellen is not None:
        umgebung["KONTINGENT_WARNSCHWELLEN"] = schwellen
    with mock.patch.dict(os.environ, umgebung):
        spec = importlib.util.spec_from_file_location("kontingent_warnung", SKRIPT)
        modul = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(modul)
    return modul, umgebung


class KontingentWarnungTest(unittest.TestCase):
    def setUp(self):
        self.verzeichnis = tempfile.TemporaryDirectory()
        self.statusdatei = Path(self.verzeichnis.name) / "unter" / "stand.json"

    def tearDown(self):
        self.verzeichnis.cleanup()

    def schreibe_stand(self, anteil, resets_at):
        self.statusdatei.parent.mkdir(parents=True, exist_ok=True)
        self.statusdatei.write_text(
            json.dumps({"used_percentage": anteil, "resets_at": resets_at}), encoding="utf-8"
        )

    def hook_ausgabe(self, schwellen=None):
        modul, umgebung = lade_modul(self.statusdatei, schwellen)
        puffer = io.StringIO()
        with mock.patch.dict(os.environ, umgebung), redirect_stdout(puffer):
            modul.hook()
        return puffer.getvalue()

    def test_statuszeile_speichert_stand(self):
        modul, _ = lade_modul(self.statusdatei)
        eingabe = {
            "model": {"display_name": "Test"},
            "rate_limits": {"seven_day": {"used_percentage": 42.4, "resets_at": 1}},
        }
        puffer = io.StringIO()
        with mock.patch.object(sys, "stdin", io.StringIO(json.dumps(eingabe))):
            with redirect_stdout(puffer):
                modul.statuszeile()
        self.assertEqual(puffer.getvalue().strip(), "[Test] 42 % Woche")
        gespeichert = json.loads(self.statusdatei.read_text(encoding="utf-8"))
        self.assertEqual(gespeichert["used_percentage"], 42.4)

    def test_statuszeile_ohne_limits_schreibt_nichts(self):
        modul, _ = lade_modul(self.statusdatei)
        puffer = io.StringIO()
        with mock.patch.object(sys, "stdin", io.StringIO("{}")), redirect_stdout(puffer):
            modul.statuszeile()
        self.assertEqual(puffer.getvalue().strip(), "[?] Woche: –")
        self.assertFalse(self.statusdatei.exists())

    def test_hook_ohne_statusdatei_schweigt(self):
        self.assertEqual(self.hook_ausgabe(), "")

    def test_hook_mit_veraltetem_fenster_schweigt(self):
        self.schreibe_stand(99, time.time() - 60)
        self.assertEqual(self.hook_ausgabe(), "")

    def test_hook_unter_schwelle_schweigt(self):
        self.schreibe_stand(79, time.time() + 3600)
        self.assertEqual(self.hook_ausgabe(), "")

    def test_hook_warnt_mit_hoechster_erreichter_schwelle(self):
        self.schreibe_stand(96, time.time() + 3600)
        ausgabe = self.hook_ausgabe()
        self.assertIn("KONTINGENT-WARNUNG: 96 %", ausgabe)
        self.assertIn("Schwelle 95 %", ausgabe)

    def test_erzwungener_fehlerfall_schwelle_null(self):
        """Der Probelauf aus S-22: Schwelle 0 % muss bei jedem Stand warnen."""
        self.schreibe_stand(1, time.time() + 3600)
        self.assertIn("KONTINGENT-WARNUNG", self.hook_ausgabe(schwellen="0"))

    def test_fehler_blockiert_nie(self):
        for argumente, eingabe in ((["statuszeile"], "kein json"), (["unbekannt"], "")):
            with self.subTest(argumente=argumente):
                # Fester Aufruf: eigener Interpreter, eigenes Skript, keine fremde Eingabe (S-24).
                ergebnis = subprocess.run(  # noqa: S603
                    [sys.executable, str(SKRIPT), *argumente],
                    input=eingabe,
                    capture_output=True,
                    text=True,
                    env={**os.environ, "KONTINGENT_STATUSDATEI": str(self.statusdatei)},
                    check=False,
                )
                self.assertEqual(ergebnis.returncode, 0)
                self.assertIn("kontingent-warnung:", ergebnis.stderr)


if __name__ == "__main__":
    unittest.main()
