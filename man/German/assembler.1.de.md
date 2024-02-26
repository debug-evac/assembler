% ASSEMBLER(1) Version 1.2.0 | Handbuch für Dateiformate
---
Datum: Februar 2024
---

# NAME

**assembler** --- assembles code für eine selbst geschriebene RISC-V based CPU

# ÜBERSICHT

**assembler** [*OPTIONS*...] **\-\-input** *file*...\
**assembler -i**|**\-\-input** *file*...\
**assembler -i**|**\-\-input** *file*...
[**-o**|**\-\-output** *file*]\
**assembler -i**|**\-\-input** *file*...
[**-f**|**\-\-format** [**debug**|**mif**|**raw**]]\
**assembler -i**|**\-\-input** *file*... [**-f**|**\-\-format mif**]
[**-c**]

# BESCHREIBUNG

**Assembler** übersetzt Assemblersprache in Maschinencode für eine selbst geschriebene CPU, die auf RISC-V basiert. [assembler-format(5)][] basiert auf einer modifizierten MIPS-Syntax, die Anweisungen und Makros aus RISC-V enthält, insbesondere RV32I und RV32M.

Standardmäßig übersetzt der **Assembler** eine oder mehrere Eingabedateien (*file*) in binäre oder MIF-Ausgabedateien. Die Option **-f** oder **\-\-format** gibt an, welches Format für die Ausgabe verwendet werden soll. Die Ausgabe erfolgt entweder binär (**raw**), im MIF-Format gemäß [src_mif(5)][] (**mif**) oder gar nicht (**debug**), in welchem Fall sie auf [stderr(3)][] ausgegeben wird. Standardmäßig wird das MIF-Format verwendet. Ausgabedateien werden nach den Eingabedateien benannt und im selben Verzeichnis wie diese geschrieben.

Die Option **-o** or **\-\-output** ändert das Verzeichnis und den Dateinamen der Ausgabedateien. Wenn die Eingabedateien .data-Abschnitte enthalten, wird eine zweite Ausgabedatei generiert, die die Daten enthält.

WARNUNG: Parserfehler sind derzeit sehr rudimentär, nicht einfach und nicht hilfreich. Stellen Sie sicher, dass Sie [assembler-format(5)][] korrekt verwenden. Als letzte Möglichkeit können Sie ein Problem in unserem Repository mit ausreichenden Details öffnen.

# DATEIEN

Der Befehl **assembler** erwartet, dass die Eingabe gültigen [Assembler-Format(5)][]
Code ist. Quelldateien haben normalerweise den Namen *name*.asm (z. B. **example.asm**).

Quelldateien müssen im ASCII- oder UTF-8-Format vorliegen. Andere Codierungen wurden nicht getestet und funktionieren möglicherweise nicht.

Beim Zusammenstellen von Ausgabedateien haben die Zieldateinamen entweder das Format a.*ext* für Textausgaben und a.mem.*ext* für Datenausgaben oder *name* für Textausgaben und *name*.mem.*ext* für Datenausgaben, wenn die Option **-o** or **\-\-output** verwendet wird. Daten werden nur generiert, wenn .data-Abschnitte in den Eingabedateien verwendet werden. Wenn eine Ausgabe mit einer Erweiterung angegeben wird, wird die Textausgabe an diesen Ort geschrieben, aber die Daten werden immer in den Stamm des Dateinamens mit der Erweiterung mem.*ext* geschrieben. Zum Beispiel schreibt die Ausführung von **assembler -i example.asm -o test.example** die Textausgabe in **test.example** und die Codeausgabe in **test.mem.mif**.

Diese Optionen steuern das Format, den Speicherort und den Typ der Ausgabe.

**-f**=[**raw**|**mif**|**debug**], **\-\-format**=[**raw**|**mif**|**debug**]

: Legt das Format der Ausgabe fest. Standardmäßig ist es **mif**. **debug** gibt die Ausgabe nur auf [stderr(3)][stderr] aus. Wie der Name andeutet, sollte es nur zu Debugging-Zwecken verwendet werden. Das Ausgabeformat kann sich ändern.

**raw** schreibt den Maschinencode und die Daten binär in die Ausgabedateien.

**mif** schreibt und formatiert den Maschinencode und die Daten als MIF, siehe [src_mif(5)][src_mif] für Details. Das MIF-Format kann mit den Anweisungs-Assembly-Namen mithilfe der Option **-c** oder **\-\-comment** kommentiert werden. Die Speichertiefe (*depth*) und die Wortbreite (*width*) können mit **\-\-depth** bzw. **\-\-width** geändert werden.

**-c**, **\-\-comment**

: Gibt an, dass die MIF-Ausgabe kommentiert werden soll. Standardmäßig ist die MIF-Ausgabe nicht kommentiert. Wenn sie mit **-f**=**mif** or **\-\-format**=**mif** verwendet wird, enthält jede Maschinencode-Anweisung eine menschenlesbare Darstellung als Kommentar. Beachten Sie, dass Pseudoanweisungen oder Makros nicht als solche dargestellt werden und nur Hardwareanweisungen für die Darstellung in Kommentaren verwendet werden.

**\-\-depth**=*depth*

: Legt die Speichertiefe für das MIF-Format fest. Standardmäßig beträgt *depth* 1024. Gültige Werte liegen zwischen 1 und 65535 (einschließlich). Siehe [src_mif(5)][] für Details.

**\-\-width**=[**8**|**32**]

: Legt die Wortbreite für das MIF-Format fest. Standardmäßig beträgt *width* 32 (Bit). Siehe [src_mif(5)][] für Details.

**-o**=*file*, **\-\-output**=*file*

: Schreibt die Ausgabe in *file* anstelle des Standardorts **a.***ext*. Führt keine Aktion aus, wenn -**-f**=**debug** oder **\-\-format**=**debug**. Wenn .data-Abschnitte in Eingabedateien verwendet werden, werden die Datenausgaben in das Verzeichnis und den Stamm des Dateinamens mit der Erweiterung **.mem.***ext* geschrieben. Wenn **-f**=**mif** oder **\-\-format**=**mif**, dann ext=mif, ansonsten *ext*=**bin**.

Diese Optionen steuern, wie der Assemblercode zusammengebaut wird.

**\-\-no-nop-insertion**

: Gibt an, dass keine NOP-Einfügungen durch den Assembler durchgeführt werden sollen. Standardmäßig werden NOPs eingefügt, um Daten-, Steuerungs- und Speicherprobleme zu umgehen. Durch Verwendung dieses Flags können Unterprogramme nicht verwendet werden, da sie Probleme enthalten!

**\-\-no-sp-init**:

: Standardmäßig wird der Stack auf 4096 initialisiert. Dieses Flag verhindert die Initialisierung des Stacks. Beachten Sie, dass der Stack initialisiert werden muss, wenn Stackoperationen verwendet werden.

Eingabeoption:

**-i**=*file*,[*file*]..., **\-\-input**=*file*,[*file*]...

:   Die Liste der Eingabedateien für die Zusammenstellung. Mindestens eine Eingabedatei muss verwendet werden. Es können mehrere Eingabedateien verwendet werden,    die durch Leerzeichen getrennt werden müssen.

    Die Eingabedateien werden in der Reihenfolge der Spezifikation in dieser Option verknüpft. Die zuerst angegebene Datei wird zuerst verarbeitet, die zuletzt angegebene Datei zuletzt.

Sonstige Optionen:

**-h**, **\-\-help**

: Zeigt Optionen an (show help).

**-v**, **\-\-version**

: Zeigt Version und beendet dann.

# UMGEBUNG

**RUST_LOG**

: Wird verwendet, um das Protokollniveau für den Assembler festzulegen. Gültige Protokollniveaus sind [**warn**|**error**|**info**|**debug**|**tracing**]. Unterschiedliche Protokollniveaus für verschiedene Module können auch durch die Verwendung von *module*=*log_level* festgelegt werden. Derzeit werden nur **debug** und niedrigere Niveaus verwendet. Änderungen am Protokollformat sind zu erwarten. Weitere Informationen zur Protokollierung finden Sie unter https://docs.rs/env_logger/0.11.1/env_logger/.

# BEENDIGUNGSSTATUS

Dieser Abschnitt ist in Arbeit (Work in Progress, WIP). Einige Betriebsfehler haben unterschiedliche Fehlercodes. Diese werden hier dokumentiert.

**0**

: Erfolgreiche Programmausführung.

**1**

: Allgemeiner Fehler.

# BEISPIELE

Eine Assemblierung einer Assemblerdatei mit Ausgabe im MIF-Format:

    $ assembler -i example.asm
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Mehrere Assemblerdateien zusammenstellen und Ausgabe im binären Format:

    $ assembler -i example.asm example2.asm example3.asm -f raw
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Eine Assemblerdatei zusammenstellen und Ausgabe im MIF-Format mit Kommentaren und Verwendung einer Wortbreite von 8 Bit:

    $ assembler -i example.asm \-\-width 8 -c
    Assembled a.mif (/pfad/zu/beispiel)
    Fertig [=========================================================] 5/5 Erfolg

Eine Assemblerdatei zusammenstellen, Debug-Nachrichten anzeigen und keine Ausgabe in Dateien schreiben:

    $ RUST_LOG=debug assembler -i example.asm -f debug

# FEHLER

Siehe Gitea-Probleme: https://git.mafiasi.de/Prj-MR/Assembler/issues

# URHEBERRECHT

Urheberrecht (c) 2023 Steven Becker

Dieses Quellcodeformular unterliegt den Bedingungen der Mozilla Public License, Version 2.0. Falls eine Kopie der MPL nicht mit dieser Datei verteilt wurde, können Sie eine unter http://mozilla.org/MPL/2.0/ erhalten.

# AUTOREN

Steven Becker steven.becker@studium.uni-hamburg.de

: Hauptverantwortlicher

Jan Julius jan.julius@studium.uni-hamburg.de

: Betreuer
# SIEHE AUCH

[src_mif(5)][], [stderr(3)][], [assembler-format(5)][]

[src_mif(5)]: https://linux.die.net/man/5/srec_mif
[stderr(3)]: https://linux.die.net/man/3/stderr
[assembler-format(5)]: assembler-format.5.md
