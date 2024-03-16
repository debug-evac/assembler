% ASSEMBLER(1) Version 1.2.0 | Handbuch für generelle Befehle
---
Datum: Februar 2024
---

# NAME

**assembler** --- assembliert Code für eine selbst geschriebene auf RISC-V basierende CPU

# ÜBERSICHT

**assembler** [*OPTIONS*...] **\-\-input** *Datei*...\
**assembler -i**|**\-\-input** *Datei*...\
**assembler -i**|**\-\-input** *Datei*...
[**-o**|**\-\-output** *Datei*]\
**assembler -i**|**\-\-input** *Datei*...
[**-f**|**\-\-format** [**debug**|**mif**|**raw**]]\
**assembler -i**|**\-\-input** *Datei*... [**-f**|**\-\-format mif**]
[**-c**]

# BESCHREIBUNG

**Assembler** übersetzt Assemblerdateien in Maschinencode für eine selbst geschriebene CPU, die auf RISC-V basiert.
[assembler-format(5)][] basiert auf einer modifizierten MIPS-Syntax, die Anweisungen und Makros aus RISC-V enthält, insbesondere RV32I und RV32M.

Standardmäßig übersetzt der **Assembler** eine oder mehrere Eingabedateien
(*file*s) in binäre oder MIF-Ausgabedateien.  Die Option **-f** oder
**\-\-format** gibt an, welches Format für die Ausgabe verwendet werden
soll.  Die Ausgabe erfolgt entweder binär (**raw**), im MIF-Format gemäß
[src_mif(5)][] (**mif**) oder gar nicht (**debug**), wobei sie dann auf
[stderr(3)][] ausgegeben wird.  Standardmäßig wird das MIF-Format
verwendet.  Ausgabedateien werden nach den Eingabedateien benannt und im
selben Verzeichnis wie diese geschrieben.

Die Option **-o** or **\-\-output** ändert das Verzeichnis und den
Dateinamen der Ausgabedateien.  Wenn die Eingabedateien .data-Abschnitte
enthalten, wird eine zweite Ausgabedatei generiert, die die Daten enthält.

WARNUNG: Parserfehler sind derzeit sehr rudimentär, nicht einfach und nicht
hilfreich.  Stellen Sie sicher, dass Sie [assembler-format(5)][] korrekt
verwenden.  Als letzte Möglichkeit können Sie ein Problem in unserem
Repository öffnen, bei dem Sie das Problem detailliert darstellen sollten.

# DATEIEN

Der Befehl **assembler** erwartet, dass die Eingabe gültiger Code im
[assembler- format(5)][] ist.  Quelldateien haben normalerweise den Namen
*name*.asm (z. B. **example.asm**).

Quelldateien müssen im ASCII- oder UTF-8-Format vorliegen.  Andere
Codierungen wurden nicht getestet und funktionieren möglicherweise nicht.

Die Ausgabedateien werden entweder a.*ext* für Maschinencode-Instruktionen
und a.mem.*ext* für Daten oder *name* für Maschinencode-Instruktionen und
*name*.mem.*ext* für Daten genannt, ausgehend davon, ob die Option **-o**
oder **\-\-output** verwendet wird.  Daten werden nur generiert, wenn
.data-Abschnitte in den Eingabedateien verwendet werden.  Wenn eine Ausgabe
mit einer Erweiterung angegeben wird, wird die Textausgabe an diesen Ort
geschrieben, aber die Daten haben den Stamm des Dateinamens mit der
Erweiterung mem.*ext* als Namen.  Zum Beispiel schreibt das Ausführen des
Befehls **assembler -i example.asm -o test.example** die
Maschinencode-Instruktionen in **test.example** und die Daten in
**test.mem.mif**.

# OPTIONEN

Diese Optionen legen das Format, den Speicherort und den Typ der Ausgabe
fest.

**-f**=[**raw**|**mif**|**debug**], **\-\-format**=[**raw**|**mif**|**debug**]

:   Legt das Format der Ausgabe fest.
    Standardmäßig ist es **mif**.
    **debug** gibt die Ausgabe nur auf [stderr(3)][] aus.
    Wie der Name andeutet, sollte es nur zu Debug-Zwecken verwendet werden.
    Das Ausgabeformat kann sich jederzeit ändern.

    **raw** schreibt den Maschinencode und die Daten binär in die Ausgabedateien.

    **mif** schreibt und formatiert den Maschinencode und die Daten als MIF, siehe [src_mif(5)][] für Details.
    Jede Instruktion kann mithilfe der Option **-c** oder **\-\-comment** mit dessen Repräsentation in Assemblersyntax automatisch kommentiert werden.
    Die Anzahl der Speicheradressen (*depth*) und die Wortbreite (*width*) kann mithilfe der Optionen **\-\-depth** und **\-\-width** respektive verändert werden.

**-c**, **\-\-comment**

:   Gibt an, dass die MIF-Ausgabe kommentiert werden soll.
    Standardmäßig ist die MIF-Ausgabe nicht kommentiert.
    Wenn sie mit **-f**=**mif** or **\-\-format**=**mif** verwendet wird, enthält jede Maschinencode-Anweisung eine lesbare Darstellung als Kommentar.
    Beachten Sie, dass Pseudoanweisungen oder Makros nicht als solche dargestellt werden und nur Anweisungen, die die Hardware implementiert, für die Darstellung in Kommentaren verwendet werden.

**\-\-depth**=*depth*

:   Legt die Speicherlänge für das MIF-Format fest.
    Standardmäßig beträgt Speicherlänge (*depth*) 1024.
    Gültige Werte liegen zwischen 1 und 65535 (einschließlich).
    Siehe [src_mif(5)][] für Details.

**\-\-width**=[**8**|**32**]

:   Legt die Wortbreite (*width*) für das MIF-Format fest.
    Standardmäßig beträgt die Wortbreite (*width*) 32 (Bit).
    Siehe [src_mif(5)][] für Details.

**-o**=*file*, **\-\-output**=*file*

:   Schreibt die Ausgabe in *file* anstelle des Standardorts **a.***ext*.
    Diese Option wird ignoriert, wenn die Option **-f**=**debug** oder **\-\-format**=**debug** angegeben sind.
    Wenn .data-Abschnitte in Eingabedateien verwendet werden, werden die Datenausgaben in das Verzeichnis und den Stamm des Dateinamens mit der Erweiterung **.mem.***ext* geschrieben.
    Wenn die Option **-f**=**mif** oder **\-\-format**=**mif** angegeben sind, dann ist *ext*=**mif**, ansonsten gilt *ext*=**bin**.

Diese Optionen legen fest, wie der Assemblercode zusammengesetzt wird.

**\-\-no-nop-insertion**

:   Gibt an, dass keine NOP-Einfügungen durch den Assembler durchgeführt werden sollen.
    Standardmäßig werden NOPs eingefügt, um Probleme bei Daten-, Kontrollfluss- und Speicherabhängigkeiten zu umgehen.
    Durch Verwendung dieser Flag können Unterprogramme nicht verwendet werden, da diese Datenabhängigkeiten enthalten!

**\-\-no-sp-init**:

:   Standardmäßig wird der Stack auf 4096 initialisiert.
    Diese Flag verhindert die Initialisierung des Stacks.
    Beachten Sie, dass der Stack initialisiert werden muss, wenn Stackoperationen verwendet werden.

Eingabeoption:

**-i**=*file*,[*file*]..., **\-\-input**=*file*,[*file*]...

:   Die Liste der Eingabedateien für die Assemblierung.
    Mindestens eine Eingabedatei muss angegeben werden.
    Es können mehrere Eingabedateien angegeben werden, die durch ein Leerzeichen getrennt werden müssen.

    Die Eingabedateien werden in der Reihenfolge der Spezifikation in dieser Option verknüpft.
    Die zuerst angegebene Datei wird zuerst verarbeitet, die zuletzt angegebene Datei zuletzt.

Sonstige Optionen:

**-h**, **\-\-help**

:   Zeigt die verwendbaren Optionen an (show help).

**-v**, **\-\-version**

:   Zeigt die Version an und beendet dann.

# UMGEBUNG

**RUST_LOG**

:   Wird verwendet, um das Protokollniveau für den Assembler festzulegen.
    Gültige Protokollniveaus sind [**warn**|**error**|**info**|**debug**|**tracing**].
    Unterschiedliche Protokollniveaus für verschiedene Module können auch durch die Verwendung von *module*=*log_level* festgelegt werden.
    Derzeit werden nur **debug** und niedrigere Niveaus verwendet.
    Das Protokollformat kann sich jederzeit ändern.
    Weitere Informationen zur Protokollierung finden Sie unter <https://docs.rs/env_logger/0.11.1/env_logger/>.

# BEENDIGUNGSSTATUS

Dieser Abschnitt ist in Arbeit (Work in Progress, WIP).  Einige
Betriebsfehler haben andere Fehlercodes.  Diese werden hier dokumentiert.

**0**

:   Erfolgreiche Programmausführung.

**1**

:   Allgemeiner Fehler.

# BEISPIELE

Eine Assemblierung einer Assemblerdatei mit Ausgabe im MIF-Format:

    $ assembler -i example.asm
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Mehrere Assemblerdateien assemblieren und in binären Format in einer Datei
ausgeben:

    $ assembler -i example.asm example2.asm example3.asm -f raw
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Eine Assemblerdatei assemblieren und im MIF-Format mit Kommentaren unter
Verwendung einer Wortbreite von 8 Bit in eine Datei ausgeben:

    $ assembler -i example.asm \-\-width 8 -c
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Eine Assemblerdatei assemblieren, Debug-Nachrichten anzeigen und Dateien
erstellen:

    $ RUST_LOG=debug assembler -i example.asm -f debug

# FEHLER

Siehe Gitea-Probleme:
<https://git.mafiasi.de/Prj-MR/assembler-crates/issues>

# URHEBERRECHT

Urheberrecht (c) 2023 Steven Becker

Dieser Quellcode unterliegt den Bedingungen der Mozilla Public License,
Version 2.0.  Falls eine Kopie der MPL nicht mit dieser Datei verteilt
wurde, können Sie eine unter <http://mozilla.org/MPL/2.0/> erhalten.

# AUTOREN

Steven Becker <steven.becker@studium.uni-hamburg.de>

:   Hauptverantwortlicher

Jan Julius <jan.julius@studium.uni-hamburg.de>

:   Mitverantwortlicher

# SIEHE AUCH

[src_mif(5)][], [stderr(3)][], [assembler-format(5)][]


[src_mif(5)]: https://linux.die.net/man/5/srec_mif

[stderr(3)]: https://linux.die.net/man/3/stderr

[assembler-format(5)]: assembler-format.5.md
