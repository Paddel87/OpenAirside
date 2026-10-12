# Gate vor dem ersten öffentlichen Deployment

<!-- Projektübergreifender Anhang zu CLAUDE.md Abschnitt 12 (ausgelagert mit ADR-018).
     Lade-Disziplin: gehört NICHT zur Mindest-Lektüre. Die KI lädt ihn Pflicht-gemäß,
     sobald ein Fahrplan-Schritt ein öffentliches Deployment vorsieht (CLAUDE.md
     Abschnitt 12 und Abschnitt 2, „Vertiefung auf Anforderung").
     Auslöser, Blockierwirkung, getrennte Instanz, Verzicht und Nachholpflicht stehen
     weiter in CLAUDE.md Abschnitt 12. Die Regeln hier gelten verbindlich wie
     CLAUDE.md; ausgelagert ist nur der Ort. -->

<!-- ANCHOR:pruefpunkte -->
## 1. Die acht Prüfpunkte

Anforderungen je Prüfpunkt. Wann das Gate greift und dass es blockiert, regelt CLAUDE.md Abschnitt 12.

1. **Bedrohungsmodell** für das Gesamtsystem – nicht nur für einzelne Module – in `docs/architecture.md` Abschnitt 6, mindestens `[VORLÄUFIG]`.
2. **Sicherheitsniveau** per ADR festgelegt, z. B. eine Stufe des OWASP ASVS.
3. **Grundhärtung des Hosts:** Firewall aktiv, SSH nur mit Schlüssel, automatische Sicherheitsupdates, interne Dienste (Datenbank, Cache, Verwaltungsoberflächen) von außen nicht erreichbar. Belegt durch eine Prüfung von außen, nicht nur durch Ansehen der Konfiguration.
4. **Secrets im Betrieb und Zugriff der KI:** Ablageort und Rotationsweg jedes Secrets sind festgehalten. In `docs/project-context.md` Abschnitt 8 steht, worauf die KI in der Produktion zugreifen darf; der Zugriff ist auf das für ihre Aufgabe Nötige beschränkt.
5. **Backup mit erprobter Wiederherstellung:** Mindestens eine Wiederherstellung aus einem echten Backup ist vollständig durchgelaufen (CLAUDE.md Abschnitt 6, „Schutzmechanismen durch erzwungenen Fehler belegen").
6. **Unabhängige Prüfung:** Authentifizierung, Autorisierung, Umgang mit Secrets und personenbezogene Datenflüsse wurden von einer getrennten Instanz geprüft (Definition: CLAUDE.md Abschnitt 12, „Getrennte Instanz"); ihre Befunde sind behoben oder als Fahrplan-Schritt mit Frist geführt.
7. **Vertretung und Notfall-Handbuch:** Eine zweite Person ist benannt, die im Notfall eingreifen kann – oder der Verzicht darauf ist per ADR mit benanntem Restrisiko festgehalten. Ein Notfall-Handbuch (Abschnitt „Notfall" in `docs/onboarding-runbook.md`, bei Klasse K ohne Runbook `docs/notfall-handbuch.md`) erlaubt einem Menschen **ohne KI**, das System anzuhalten, eine Sicherung zu ziehen und es wiederherzustellen.
8. **KI im Betrieb:** In `docs/project-context.md` Abschnitt 8 stehen Konto bzw. Bezugsmodell der KI, ihr Kontingent mit Zurücksetz-Zeitpunkt (bei API-Nutzung: Budget und Nutzungsgrenzen) und der Rückfallweg ohne KI. Liegt ein kritischer Termin (Deployment, Pilot, Go-Live) im selben Kontingent-Fenster wie kontingentintensive Vorbereitung, wird die Vorbereitung ins vorige Fenster gelegt oder Kontingent für den Termin ausdrücklich zurückgehalten; beides steht im Fahrplan. Handelt die KI **unbeaufsichtigt** auf der Produktion, gelten zusätzlich eine feste Befehlsliste ohne löschende und ohne Secret-lesende Befehle und ein vorheriger Probelauf nach Abschnitt 6 („Schutzmechanismen durch erzwungenen Fehler belegen").

<!-- ANCHOR:begruendung -->
## 2. Begründung

**Warum vor dem ersten öffentlichen Deployment und nicht erst vor Go-Live:** Ein Testsystem im Internet ist genauso angreifbar wie ein Produktivsystem. Im Pilotprojekt war das System rund zweieinhalb Monate öffentlich erreichbar, bevor ein Bedrohungsmodell für das Gesamtsystem existierte; Host-Härtung, Secrets und Backups kamen erst Wochen nach dem ersten Deployment. Ein nicht-fachlicher Treiber kann diese Lücke nicht selbst bemerken – deshalb liegt die Prüfung bei der KI und dem Gate, nicht beim Menschen.
