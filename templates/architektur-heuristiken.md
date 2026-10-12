# Architektur-Heuristiken

<!-- Projektübergreifendes Begleitdokument zur Methodik (CLAUDE.md).
     Zweck: Bei Vision-Driven Development bringt der Mensch die Vision, nicht das
     Architektur-Fachwissen. Dieses Dokument ist die Prothese für das fehlende
     Architektur-Urteil – es diszipliniert die Empfehlung der KI UND macht jede
     technische Wahl für einen nicht-fachlichen Treiber beurteilbar.

     WICHTIG – Lade-Disziplin:
       - Dieses Dokument gehört NICHT zur Mindest-Lektüre (CLAUDE.md Abschnitt 2).
       - Es wird ausschließlich ON-DEMAND geladen, wenn eine Architekturentscheidung
         ansteht (Vertiefung-Trigger in CLAUDE.md Abschnitt 2).
       - Es ist projektübergreifend und wird, wie CLAUDE.md, unverändert übernommen.
         Projektspezifische Übersetzungen wachsen in Teil 2 mit; der Rahmen bleibt.

     Stabile Sprung-Anker pro Hauptabschnitt (nicht umbenennen, nicht entfernen –
     siehe CLAUDE.md Abschnitt 2 „Graceful Degradation an Tool-Grenzen"). -->

<!-- ANCHOR:gilt-wann -->
## 0. Wann dieses Dokument gilt

Geladen wird es **nur**, wenn eine Entscheidung aus CLAUDE.md Abschnitt 4 Kategorie 1–2 ansteht (Architekturänderung, neues Modul) oder eine Muster-/Kommunikations-Wahl getroffen wird. Es ist **kein Gate bei jedem Commit** – der reguläre Flow bleibt unberührt.

Die drei Teile haben getrennte Adressaten:

- **Teil 1** richtet sich an die **KI** – sie diszipliniert die eigene Empfehlung, bevor sie formuliert wird.
- **Teil 2** richtet sich an den **Menschen** – er übersetzt die technische Wahl in eine Vision-Frage, die ein nicht-fachlicher Treiber beantworten kann.
- **Teil 3** ist die **Ehrlichkeits-Pflicht** der KI – sie legt offen, wann sie rät.

<!-- ANCHOR:entwurfs-urteil -->
## 1. Entwurfs-Urteil disziplinieren (für die KI)

**Default-Bias über allem:** Bei nicht-fachlichem Treiber gibt es keinen menschlichen Experten, der unnötige Komplexität abfängt. Deshalb gilt durchgehend: **Die einfachere Option gewinnt, bis das Gegenteil belegt ist.** „Wäre robuster/cooler/zukunftssicherer" ist kein Beleg. Belegt ist eine Komplexität nur durch eine konkrete Anforderung aus der Vision.

### 1.1 Zerlegungs-Heuristik (Wie schneide ich Module?)

Frageliste, kein Dogma:

- **Änderungsrate:** Was sich gemeinsam ändert, gehört zusammen; was sich unabhängig ändert, wird getrennt. Module entlang von „ändert sich aus demselben Grund" schneiden.
- **Fachliche Sprache vor technischer Schicht:** Wenn Fachlichkeit der dominante Komplexitätstreiber ist, nach fachlichen Begriffen (Bounded Context) schneiden, nicht nach „Frontend/Backend/DB".
- **Dichtheits-Test:** *Könnte ich die öffentliche Schnittstelle eines Moduls beschreiben, ohne seine interne Struktur zu nennen?* Wenn nein → die Grenze ist undicht, der Schnitt ist falsch.

### 1.2 Kopplung/Kohäsion-Checkliste

- Ruft Modul A mehr als 2–3 **verschiedene** Operationen von B auf? → Verdacht auf falschen Schnitt.
- Muss A **fast immer mit**, wenn sich B ändert? → zu hohe Kopplung, Grenze überdenken.
- Hat ein Modul mehr als **eine** Änderungs-Ursache? → Kohäsion prüfen (Single Responsibility auf Modulebene).
- Zeigt die Modul-Karte einen **Zyklus** (A→B→A)? → Architekturbruch, immer auflösen.

### 1.3 Muster-Defaults (Entscheidungsbaum, knapp)

- **Monolith vs. Services:** *Ein Team, ein Deploy-Ziel, Komplexität ist fachlich?* → **Modularer Monolith** als Default. Getrennte Services erst, wenn unabhängige Skalierung **oder** unabhängige Deploybarkeit durch die Vision *belegt* gefordert ist.
- **Synchron vs. asynchron:** Default **synchron** (leichter zu verstehen und zu debuggen). Asynchron nur, wenn (a) zeitliche Entkopplung fachlich gefordert ist oder (b) Lastspitzen gepuffert werden müssen.
- **Eigene Lösung vs. Fremd-Service:** Default die Option mit **weniger Abhängigkeiten**, sofern keine harte Anforderung (Sensibilität der Daten, Time-to-Market) das Gegenteil belegt.
- **Jetzt entscheiden vs. Spike:** Bei niedriger Konfidenz und teurer Umkehrbarkeit (siehe Teil 3) → **Spike**, nicht Rateschluss.

### 1.4 Architektur-Smells (Warnsignale, immer prüfen)

- „Gott-Modul", das alle anderen kennt.
- Zyklische Abhängigkeiten in der Modul-Karte.
- Eine Schnittstelle, die bei **jedem** neuen Feature wächst.
- NFRs, die nirgends gemessen, nur behauptet werden.
- Eine `[OFFEN]`- oder `[VORLÄUFIG]`-Architektur, auf der bereits Code aufbaut (verstößt gegen CLAUDE.md Abschnitt 6).

<!-- ANCHOR:uebersetzung -->
## 2. Übersetzungs-Tabelle: Technik → Vision-Folge (für den Menschen)

Das Herzstück bei Vision-Driven Development. Jede technische Wahl wird auf **die eine Frage** reduziert, die ein nicht-fachlicher Treiber beantworten **kann** – weil sie in seiner Vision liegt, nicht in der Technik. Die Tabelle wächst projektübergreifend mit; neue Zeilen werden ergänzt, bestehende nicht gelöscht.

| Technische Wahl | Was die einfache Option spart / kostet | Die *eine* Frage an dich |
|---|---|---|
| Monolith vs. Services | Einfach: 1 Baustein, billiger Betrieb, schneller fertig. Grenze: spätere Umbaukosten bei echtem Massen-Wachstum | „Ist große, **unabhängige** Skalierung Teil deiner Vision – oder ein ‚wäre nett'?" |
| Synchron vs. asynchron | Einfach: synchron, leichter zu verstehen und zu reparieren. Async: robuster bei Lastspitzen, aber spürbar komplexer | „Müssen Aufgaben im Hintergrund weiterlaufen, während der Nutzer schon weitermacht?" |
| Eigene DB vs. Fremd-Service | Selbst: volle Kontrolle, mehr Pflegeaufwand. Service: schneller live, aber Abhängigkeit + laufende Kosten | „Sind die Daten so sensibel oder zentral, dass du sie nicht aus der Hand geben willst?" |
| Jetzt bauen vs. Spike | Jetzt: schneller sichtbares Ergebnis, aber auf meiner Vermutung gebaut. Spike: kostet Zeit, liefert aber Gewissheit | „Reicht dir mein ‚wahrscheinlich', oder willst du erst einen kleinen Test sehen?" |
| Generisch vs. konkret bauen | Konkret: schneller, exakt für deinen Fall. Generisch: flexibler, aber mehr Aufwand und mehr Fehlerquellen | „Wird es realistisch noch andere Anwendungsfälle geben – oder bauen wir für genau diesen einen?" |

**Pflege-Regel:** Taucht eine wiederkehrende Architekturwahl auf, die hier fehlt, wird eine Zeile ergänzt – im selben Schritt, in dem die Entscheidung getroffen wird.

<!-- ANCHOR:konfidenz -->
## 3. Konfidenz-Selbstprüfung (Ehrlichkeits-Pflicht der KI)

Ein nicht-fachlicher Treiber kann eine **selbstbewusst vorgetragene Fehlentscheidung nicht erkennen**. Dasselbe Fehlermuster wie bei „Keine Erfolgsmeldungen ohne Verifikation" (CLAUDE.md Abschnitt 6), nur auf Architektur-Ebene – und ohne rohen Output, der es abfangen könnte. Deshalb muss die KI **vor jeder Empfehlung** offenlegen, wie sicher sie ist.

Drei Prüf-Fragen vor jeder Empfehlung:

1. Stützt sich die Empfehlung auf eine **Heuristik aus Teil 1** – oder auf ein Bauchgefühl?
2. Habe ich diese Art Entscheidung schon im **belegten Kontext** gesehen, oder extrapoliere ich?
3. Wäre die Entscheidung in 6 Monaten **teuer** rückgängig zu machen?

Ergebnis → Konfidenz **hoch / mittel / niedrig**, plus eine Begründung in einem Satz.

**Harte Folge:** Konfidenz *niedrig* **und** teure Umkehrbarkeit → **keine** Entscheidung erzwingen, sondern einen ERKUNDUNG-Schritt (Spike) im Fahrplan vorschlagen. Das ist ein Stopp-Kriterium (CLAUDE.md Abschnitt 8).

<!-- ANCHOR:anwendung -->
## 4. Anwendung im `ENTSCHEIDUNG ERFORDERLICH`-Block

Diese Heuristiken sind kein Selbstzweck – sie speisen den Vorschlags-Block aus CLAUDE.md Abschnitt 4:

- **Teil 1** begründet die Zeile `Empfehlung` und `Trade-off`.
- **Teil 2** liefert die Zeilen `Was es für deine Vision bedeutet` und `DIE FRAGE AN DICH`.
- **Teil 3** liefert die Zeilen `Konfidenz` und `Umkehrbarkeit`.

Die verworfene Option ist das Nebenprodukt einer angewandten Heuristik und wandert nach `docs/architecture.md` Abschnitt 8 („Verworfene Alternativen").
