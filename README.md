# Rusty-Search

![Build Status](https://img.shields.io/github/actions/workflow/status/rextorne/rusty-search/ci.yml?branch=main)
![Rust Version](https://img.shields.io/badge/rust-1.90%2B-orange)
![License](https://img.shields.io/badge/license-BSD--3--Clause-blue)

**Rusty-Search** ist ein hochperformantes, multithreaded Kommandozeilen-Tool (CLI) zum Durchsuchen von Dateien, geschrieben in Rust. Es wurde entwickelt, um die Performance von Rust zu demonstrieren und klassische Suchalgorithmen (Boyer-Moore) manuell zu implementieren.

Es ist eine moderne Alternative zu `grep` mit Fokus auf Geschwindigkeit und User Experience.

## Features

* **Multithreading:** Nutzt `rayon`, um Dateien parallel auf allen CPU-Kernen zu durchsuchen.
* **Algorithmen-Wahl:**
    * **Standard (Regex-Mode):** Schnelle Suche nach exakten Teilstrings (String Matching).
    * **Boyer-Moore:** Eigene Implementierung der "Bad Character Heuristic" für effiziente Suche.
* **User Experience:**
    * Integrierter Ladebalken (`indicatif`) für Fortschrittsanzeige bei großen Verzeichnissen.
    * Farbige Ausgabe (`colored`) für bessere Lesbarkeit.
* **Robust:** Überspringt automatisch Binärdateien (via UTF-8 Check) und versteckte Systemdateien.
* **Case Insensitive:** Unterstützt Groß-/Kleinschreibung ignorieren (`-i`).

## Bekannte Einschränkungen

* **Regex-Pattern:** In der aktuellen Version `v1.1.1` führt der Standard-Modus (`--algo regex`) eine **exakte Textsuche** durch (`contains`). Echte reguläre Ausdrücke (wie `\d+`, `^Start`, `[a-z]`) werden momentan als normaler Text behandelt und noch nicht ausgewertet. Die Integration einer vollwertigen Regex-Engine ist für das nächste Update geplant.
* **Testabdeckung:** Die Unit- und Integration-Tests decken derzeit nur die Basisfunktionalität ab und sind noch nicht vollständig.

## Installation

### Option 1: Release Binary herunterladen (Empfohlen für Nutzer)
Wenn du kein Rust installiert hast, kannst du einfach die fertige Programmdatei nutzen:

1.  Öffne die [Releases-Seite](https://github.com/rextorne/rusty-search/releases) dieses Repositories.
2.  Lade die passende Datei für dein System herunter (Linux, macOS oder Windows).
3.  **Linux/macOS:** Mache die Datei ausführbar und verschiebe sie in deinen System-Pfad:
    ```bash
    chmod +x rusty-search
    sudo mv rusty-search /usr/local/bin/
    ```
4.  **Windows:** Platziere die `.exe` Datei in einem Ordner deiner Wahl und füge diesen den Umgebungsvariablen (PATH) hinzu, oder nutze sie direkt.

### Option 2: Aus dem Source Code bauen (Für Entwickler)
Voraussetzung: Eine installierte [Rust-Umgebung (Cargo)](https://rustup.rs/).

1.  **Repository klonen:**
    ```bash
    git clone [https://github.com/DEIN_USERNAME/rusty-search.git](https://github.com/DEIN_USERNAME/rusty-search.git)
    cd rusty-search
    ```

2.  **Release Build erstellen:**
    ```bash
    cargo build --release
    ```

3.  **Installieren (Global verfügbar machen):**
    ```bash
    cargo install --path .
    ```

## Benutzung
Nach der Installation kannst du das Tool mit `rusty-search` (oder dem Namen deines Binaries) aufrufen.

### Syntax
```bash
rusty-search [OPTIONS] <PATTERN> <PATH>

```

### Beispiele

**Einfache Suche:**
Suche nach "Error" im aktuellen Ordner:

```bash
rusty-search "Error" .

```

**Case Insensitive Suche:**
Findet "error", "ERROR", "Error" etc.:

```bash
rusty-search -i "error" ./logs

```

**Boyer-Moore Algorithmus nutzen:**
Besonders effizient bei sehr langen Suchbegriffen:

```bash
rusty-search --algo boyer "Ein_sehr_langes_such_wort" .

```

## Technologie-Stack

Dieses Projekt nutzt folgende Rust-Crates:

* **clap:** Parsing der Kommandozeilen-Argumente.
* **rayon:** Daten-Parallelismus (Work-Stealing Iterator).
* **walkdir:** Rekursives Traversieren des Dateisystems.
* **indicatif:** Fortschrittsbalken.
* **colored:** Terminal-Farben.

## Algorithmen Details

### Regex-Modus (Standard)

Aktuell implementiert als **Substring-Suche** (`String::contains`). Dies ist extrem schnell für einfache Wortsuchen, unterstützt aber noch keine Wildcards.

### Boyer-Moore (Custom Implementation)

Implementiert die **Bad Character Rule**.

* Erstellt eine Tabelle für das letzte Vorkommen jedes Zeichens im Suchmuster.
* Bei einem Mismatch springt der Algorithmus basierend auf dem Zeichen im Text, das nicht passte.
* Kann theoretisch Sub-Linear laufen (d.h. muss nicht jedes Zeichen des Textes anschauen).

## Contributing

Pull Requests sind willkommen! Bitte stelle sicher, dass alle vorhandenen Tests erfolgreich durchlaufen:

```bash
cargo test

```

**Hinweis:** Die aktuelle Testabdeckung ist noch unvollständig. Beiträge, die neue Testfälle hinzufügen oder die bestehenden Tests verbessern, werden besonders geschätzt.

## Lizenz

Dieses Projekt ist unter der [BSD-3-Clause Lizenz](https://www.google.com/search?q=LICENSE) veröffentlicht.
