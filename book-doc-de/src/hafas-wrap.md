# Die hafas_wrap-Bibliothek

Die Hauptfunktionalität kommt mit dem `Client`-Objekt, da alle Anfragen damit erledigt werden.

Diese Bibliothek stellt auch einige Profile ([`hafas_profiles`](#hafas-profiles)) für gängige Endpunkte und [Anfrage- und Antwortstrukturen](#methods) bereit, um die Arbeit mit HAFAS-Endpunkten zu vereinfachen.
Das Modul [`util`](#utils) bietet Funktionen zum De- und Codieren von Base64-AES-Strings unter Verwendung eines statisch festgelegten Schlüssels und einer md5-Hashing-Funktion, die für Anfragen benötigt werden.

## Die Anfragefunktion

Es bietet ein einfaches, anpassbares System für den Zugriff auf HAFAS[^hafas].
Es gibt drei allgemeine Anforderungsparameter:
```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:76:80}}
```
- Ein Verweis auf sich selbst, um auf den Anfrageclient [reqwest] zuzugreifen, der zum Stellen von Webanfragen verwendet wird

- Ein Verweis auf eine Instanz des Typs `HafasProfile`.
  
   Ein `HafasProfile` spezifiziert den Endpunkt, eine mögliche Verschlüsselung und zusätzliche Konfigurationsdaten, die mit der Anfrage übergeben werden müssen

- Ein Array, das Instanzen des Typs `RawHafasRequest` enthält

   Der `RawHafasRequest`-Typ (und seine generische Anfrage) müssen `serde::Serialize + Send` implementieren.
   Diese Implementierungen werden benötigt, damit der Typ in eine JSON-Zeichenfolge serialisiert werden kann, die in der POST-Anfrage an HAFAS[^hafas] verwendet wird.

Die `request`-Funktion versucht auch, die resultierenden Request-Bodys in den spezifizierten generischen Typ `O` zu parsen.
Dieser Typ muss `DeserializeOwned` implementieren, damit er von einem JSON-Objekt deserialisiert werden kann.

## Anfrageverfahren
Es gibt zwei Methoden zum Anfordern von Daten: Entweder `request` oder `request_raw`,
wobei `request` hauptsächlich ein Wrapper um die `request_raw`-Methode ist, um das Parsen der Ergebnisse zu erleichtern.

Hier ist eine Zusammenfassung dessen, was die Funktionen tun:

Beide:

1. Kopieren des `HafasProfile` in ein JSON-Objekt,
  
    Serialisieren der Anfragen in ein JSON-Objekt und setzen des `svcReqL`-Wert der Anfrage darauf.
  
    Serialisieren des JSON-Objekt in einen String
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:147:149}}
    ```

2. Optional das erstellen einer Prüfsumme für die GET-Parameter der Anfrage.
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:151:154}}
    ```
   
3. Antworttext einer Webanfrage an HAFAS[^hafas] erstellen, senden und abwarten.
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:156:164}}
    ```

`request_raw` gibt an dieser Stelle den Antwort-String zurück, während `request` mit dem Parsen der Antwort fortfährt:

Nur `Anfrage`:

5. Prüft, ob HAFAS[^hafas] eine Fehlermeldung zurückgegeben hat.
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:85:87}}
    ```

6. Deserialisiert die Anfrage in ein JSON-Objekt und versucht, das Antworttextsegment als Array abzurufen.
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:101:106}}
    ```

7. Deserialisiert den Array von JSON-Objekten in den angegebenen generischen Typ `O`.
    ```rust,ignore
{{#include ../../hafas-wrap/src/lib.rs:107:117}}
    ```

## HAFAS-Profile
Im Modul `hafas_profiles` sind mehrere Konfigurationsvoreinstellungen für verschiedene HAFAS[^hafas]-Endpunkte zu finden.

Diese wurden aus verschiedenen anderen Open-Source-Projekten gesammelt.

## Methoden
Im Modul `Methoden` finden Sie einige Voreinstellungen für Request- und Response-Strukturen. Man muss sie nicht verwenden, aber sie wurden getestet und scheinen für die meisten Endpunkte zu funktionieren.

Derzeit verfügbare Methoden:
- HimSearch
- JourneyMatch
- Reisedetails

## Dienstprogramme
Einige verschiedene Funktionen wie AES-Verschlüsselung mit einem festgelegten Schlüssel oder das Hashen eines Strings mit dem `md5`-Digest-Algorithmus.

[reqwest]: https://crates.io/crates/reqwest
[^hafas]: [Hacon Fahrplan Auskunfts System](https://de.wikipedia.org/wiki/HAFAS)
