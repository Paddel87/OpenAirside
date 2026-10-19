# Stilleitfaden für Grafiken

<!-- Schritt 2.10 (ADR-008, ADR-033). Maßgeblich für jede SVG-Quelle des Spiels.
     Die Regeln mit Prüfzeichen ✓ prüft `npm run check:graphics` (scripts/assets.mjs, Tests in
     scripts/assets.test.mjs) bzw. das Werkzeug tools/assets beim Bauen. -->

Grafiken von OpenAirside sind eigene Vektorgrafiken (SVG), einmal gezeichnet und im Build zu Bilddateien gerastert (ADR-008). Sie stehen unter CC0 (`project-context.md` Abschnitt 6). Nichts aus Airport CEO oder dessen Mods übernehmen, nachzeichnen oder als Vorlage nehmen (Regel-005).

<!-- ANCHOR:ablage-und-namen -->
## 1. Ablage und Namen

- Objektgrafiken des Basispakets: `assets/graphics/base/<eintrag>.svg`, eine Datei je Objekt; `<eintrag>` ist die ID des Objekts aus `content/base/objects/` (`bench.svg` für `base:bench`). ✓
- Im Atlas trägt das Bild die volle Objekt-ID (`base:bench`, ADR-032). ✓
- Jedes Objekt des Basispakets hat eine Grafik, jede Grafik gehört zu einem Objekt. ✓ Objekte ohne Bild (künftig: Mods bis 6.1) zeigen die neutrale Ersatzgrafik `assets/missing.svg` (Name im Atlas `missing`, nie eine Objekt-ID, weil diese immer `:` enthält).
- Programmsymbol: `assets/icon.svg`, quadratisch. ✓ Der Build rastert es für Installer und Pakete (1024 px, `build/icon.png`) und das Fenster (256 px).
- Die Quellen liegen außerhalb der Paketordner unter `content/`: Sie werden nicht ausgeliefert, und das Paketformat S-4 bleibt unverändert. Grafiken in Mods regelt 6.1.

<!-- ANCHOR:massstab-und-ansicht -->
## 2. Maßstab und Ansicht

- **Draufsicht** senkrecht von oben, ohne Perspektive, ohne Schlagschatten außerhalb des Objekts.
- **Maßstab:** 1 Zelle = 2 m (ADR-020) = **32 SVG-Einheiten**; 1 Einheit = 6,25 cm.
- **Größe:** `width`, `height` und `viewBox="0 0 W H"` in ganzen Einheiten. ✓ Ein Objekt mit Platzierung `single` ist genau so groß wie seine Grundfläche bei Drehung 0: Breite `width` in Metern ÷ 2 × 32 Einheiten entlang x, Tiefe `depth` ÷ 2 × 32 entlang y (Bank 4 m × 2 m → 64 × 32). ✓ Flächenobjekte (`area`: Böden, Wände, später Zonen) sind eine Kachel von 32 × 32, die sich nahtlos aneinanderlegt. ✓
- **Ausrichtung bei Drehung 0:** Vorderseite (Sitzseite, Schalter, Ausgang) zeigt nach unten (+y).
- **Grenze der Pipeline:** Ein Bild darf höchstens 2.046 px je Kante haben (Atlasseite 2.048 px mit Rand), bei 32 px je Zelle also höchstens 63 Zellen; das Werkzeug meldet größere Grafiken als Fehler. ✓

<!-- ANCHOR:auflosungen -->
## 3. Auflösungen im Atlas

Der Build rastert jede Grafik in drei Stufen: **8, 16 und 32 px je Zelle** (Faktor 0,25, 0,5 und 1 auf die SVG-Einheiten). Das Spielfeld wählt die kleinste Stufe, die bei der aktuellen Vergrößerung und dem Pixelverhältnis des Bildschirms mindestens ein Bildpunkt je Bildschirmpunkt liefert, höchstens 32. Atlasseiten sind höchstens 2.048 × 2.048 px (WebGL2 garantiert diese Größe); jedes Bild hat einen Rand von 1 px aus wiederholten Kantenpixeln, damit beim Filtern keine Nachbarbilder durchscheinen.

<!-- ANCHOR:palette -->
## 4. Palette

Nur diese Farben, in Kleinbuchstaben als `#rrggbb`; `none` ist erlaubt. ✓ Maßgeblich ist `assets/palette.json`; jede Farbe dort muss in dieser Tabelle stehen. ✓

| Name | Farbe | Verwendung |
|---|---|---|
| `outline` | `#262a30` | Kontur aller Objekte |
| `shadow` | `#4a515c` | Innenlinien, Kanten, Schattierung im Objekt |
| `metal` | `#8a929e` | Metall, Gestelle; Ersatzgrafik |
| `metal_light` | `#c4c9d0` | helles Metall, Geräte |
| `white` | `#eef0f2` | Markierungen, Flugzeugrumpf, Papier |
| `asphalt` | `#3b3f45` | Straßen, Vorfeld, Bahnen |
| `concrete_floor` | `#b3b0a8` | Betonboden |
| `concrete_wall` | `#7d7a74` | Betonwand |
| `wood` | `#a8784a` | Holz |
| `wood_dark` | `#7a5434` | dunkles Holz, Maserung |
| `fabric_blue` | `#3f6fa8` | Polster, Stoff |
| `glass` | `#9cc7e0` | Glas, Fenster |
| `plant` | `#5e9c4f` | Pflanzen, Grünflächen |
| `signal_yellow` | `#e8b730` | Warn- und Bodenmarkierungen |
| `signal_red` | `#c8433b` | Notfall, Sperrung |
| `brand_blue` | `#2a5b8c` | Programmsymbol, Akzente |

Neue Farben kommen mit der ersten Grafik, die sie braucht, in Palette und Tabelle (im selben Commit). Zustände (ausgewählt, gesperrt, Zonenfarben) färbt das Spielfeld beim Zeichnen ein, nicht die Grafik.

<!-- ANCHOR:linien-und-detailgrad -->
## 5. Linien und Detailgrad

- **Strichstärken:** nur 0,5, 1 oder 2 Einheiten. ✓ Kontur 1 (bei kleinen Teilen 0,5), Innenlinien 0,5, Markierungen 1–2. Linienenden und -ecken bevorzugt rund (`stroke-linejoin="round"`), wo es die Form zulässt.
- **Detailgrad:** Lesbar bei 16 px je Zelle; Details unter 2 Einheiten (12,5 cm) fallen in der kleinsten Stufe weg und tragen keine Bedeutung. Formen flach gefüllt, keine Verläufe, Filter, Masken oder Muster.
- **Kein Text in Grafiken** – Texte kommen aus den Sprachpaketen (ADR-031). ✓

<!-- ANCHOR:erlaubte-elemente -->
## 6. Erlaubte SVG-Elemente und -Attribute

- Elemente: `svg`, `g`, `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `path`. ✓
- Attribute: Lage und Form (`x`, `y`, `width`, `height`, `rx`, `ry`, `cx`, `cy`, `r`, `x1` … `y2`, `d`, `points`, `transform`, `viewBox`, `xmlns`), Füllung und Linie (`fill`, `fill-rule`, `fill-opacity`, `stroke`, `stroke-width`, `stroke-linecap`, `stroke-linejoin`, `stroke-opacity`, `opacity`), Werte in doppelten Anführungszeichen. ✓
- Nicht erlaubt: `text`, `image`, `use`, `script`, `style`, `foreignObject`, Verweise (`href`), DOCTYPE und Entitäten. ✓ Das Werkzeug liest keine Datei außerhalb der Quelle.
- Kommentare sind erlaubt; jede Datei nennt in einem Kommentar Objekt-ID, Maße und „CC0".
