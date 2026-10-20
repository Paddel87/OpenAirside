#!/usr/bin/env node
// Spike 3.8 (Wegwerf-Code): erzeugt 512 Testgrafiken (1 × 1 bis 4 × 4 Zellen) als
// assets/graphics/base/spike_g_NNN.svg für „Atlanten mit vielen Grafiken“ (ADR-034).
// Voraussetzungen: Node 26.11.1. Plattformen: Linux, macOS, Windows.
// Idempotenz: überschreibt dieselben 512 Dateien mit demselben Inhalt.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

const palette = Object.values(JSON.parse(readFileSync("assets/palette.json", "utf8")));
const dir = "assets/graphics/base";
mkdirSync(dir, { recursive: true });
for (let n = 0; n < 512; n += 1) {
  const w = 1 + (n % 4);
  const h = 1 + (Math.floor(n / 4) % 4);
  const [W, H] = [w * 32, h * 32];
  const c = (k) => palette[(n * 7 + k * 3) % palette.length];
  const shapes = [];
  for (let k = 0; k < 6; k += 1) {
    const x = (n * 13 + k * 29) % (W - 8);
    const y = (n * 17 + k * 23) % (H - 8);
    shapes.push(
      k % 2 === 0
        ? `<rect x="${x}" y="${y}" width="${Math.max(4, W / 3)}" height="${Math.max(4, H / 4)}" fill="${c(k)}" stroke="${palette[0]}" stroke-width="0.5" />`
        : `<circle cx="${x + 4}" cy="${y + 4}" r="${Math.max(2, Math.min(W, H) / 6)}" fill="${c(k)}" stroke="${palette[0]}" stroke-width="1" />`,
    );
  }
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}">
  <!-- spike 3.8 test graphic ${n}, ${w} × ${h} cells; CC0. -->
  <rect x="1" y="1" width="${W - 2}" height="${H - 2}" fill="${c(9)}" stroke="${palette[0]}" stroke-width="1" />
  ${shapes.join("\n  ")}
</svg>
`;
  writeFileSync(`${dir}/spike_g_${String(n).padStart(3, "0")}.svg`, svg);
}
console.log("gen-graphics: 512 graphics written");
