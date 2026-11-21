# Drizzle-Erweiterung für VS Code

Drizzle hebt die Syntax von `.zz`-, `.drizzle`- und `.rd`-Dateien in VS Code hervor.

![Drizzle](../../images/logo_drizzle_language.png)

## Demo

![Demo](../../images/drizzle_demo.png)

## Beispiel für eine Zugsteuerung

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## Compilerausgabe

![Compilerausgabe](../../output.png)

Formen, Verbindungsmarkierungen und Layoutoptionen beschreibt die [Drizzle-Syntaxreferenz](./README.md#drizzle-syntax).

## Erweiterung entwickeln und testen

Öffnen Sie den Ordner `drizzle/` in VS Code. Das Manifest `package.json` gibt den Ort der Grammatik an. Die Datei `syntaxes/drizzle.tmLanguage.json` enthält die TextMate-Grammatik. `language-configuration.json` definiert Kommentare und Klammern.

Drücken Sie **F5**, um den Extension Development Host zu öffnen. Öffnen Sie im Host ein Beispiel und prüfen Sie Syntaxhervorhebung und Spracheinstellungen. Starten Sie den Host nach Änderungen neu oder laden Sie sein Fenster neu.

Installieren Sie die Erweiterung aus dem Marketplace oder über **Extensions: Install from VSIX...** mit einer VSIX-Datei. Informationen zur Entwicklung und Veröffentlichung stehen in der [VS-Code-Erweiterungsdokumentation](https://code.visualstudio.com/api).

## Versionshinweise zur Erweiterung

### 1.1.3

- Bild-URLs für das Logo, die Syntaxdemo und die Ausgabe im Marketplace korrigiert.
- Direkten Verweis auf die vollständige Drizzle-Anleitung in der Repository-README ergänzt.

### 1.1.0 – 1.1.2

- Pest-Grammatik um benannte Knoten, Vorwärtsreferenzen, UTF-8-Beschriftungen, Maskierung, Zeilenumbrüche und Diagnosen mit Zeile und Spalte erweitert. Anonyme Ziele, Risiken und Agenten sowie Ganzzeilenkommentare bleiben möglich.
- Objekte, Aktionen, Notizen, Wolkenanmerkungen, beschreibende Trapeze und Rauten ergänzt. Blattknoten wie Anforderungen und Annahmen können dicke Konturen haben.
- Durchgezogene und gestrichelte Verbindungen, optionale Pfeilspitzen an einem oder beiden Enden, Beschriftungen, Schleifen und gemeinsame Verzweigungspunkte ergänzt. Dazu kommen offene und gefüllte Kreise, Konfliktzickzack und Hindernis-Querstriche.
- Vier Layoutrichtungen, explizite Zeilen- und Spaltenpositionen, rechtwinklige Pfade und äußere Umwege ergänzt. Parallele Verbindungen verlaufen getrennt und Pfeile setzen am Formrand an.
- VS-Code-Grammatik und Erweiterungs-README aktualisiert, GIF-Demo durch PNG ersetzt und Drizzle 1.1.0 als VSIX hinzugefügt.

### 1.0.0

Erste Veröffentlichung von Drizzle.
