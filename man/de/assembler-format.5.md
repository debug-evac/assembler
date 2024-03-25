% ASSEMBLER-FORMAT(5) Version 2.0.0 | Handbuch für Dateiformate
---
date: März 2024
---

# NAME

**assembler-format** --- Format von Eingabe-Assemblerdateien

# ÜBERSICHT

    ; ist ein einzeiliger Kommentar, hier kann alles geschrieben werden
    .data
    ; Sie können sowohl hier als auch für Plätze im Speicher Labels verwenden
    LABEL:
        .byte   1, 2, 3, LABEL      ; 8-Bit-Zahlen
        .half   1, 2, 3, LABEL      ; 16-Bit-Zahlen
        .word   1, 2, 3, LABEL      ; 32-Bit-Zahlen
        .dword  1, 2, 3, LABEL      ; 64-Bit-Zahlen
        .eqv    CONST, 20           ; CONST wird auf 20 gesetzt

    .text
    ; Sie können auch hier im .text-Abschnitt Labels verwenden
    ; Schreiben Sie Anweisungen und Makros hier hin
    START:
        nop
        addi    zero, zero, CONST   ; äquivalent zu addi zero, zero, 20        addi    t1, zero, 2
        mul     t0, sp, t1
    LOD:
        j       LOD                 ; Sie können Labels auch in Makros verwenden

# BESCHREIBUNG

Der Befehl [assembler(1)] konvertiert Assemblerdateien in eine Ausgabe im
binären oder MIF-Format.  Die Syntax basiert auf der MIPS-Assembler-Syntax
und wurde modifiziert, um den Aufwand des Programmierers zu verringern und
die Kompatibilität mit bereits etablierten Tools und Konventionen zu
erhöhen.

# SYNTAX

Im Allgemeinen enthält jede Zeile nichts (Leerzeichen), eine Sektion, ein
Label oder ein Label und eine Operation.  Kommentare können am Ende jeder
Zeile hinzugefügt oder in eine Zeile eingefügt werden, die ansonsten nichts
(Leerzeichen) enthalten würde.  Der Assembler ist nachsichtig mit der
Platzierung dieser Komponenten zueinander.  Eine Operation ist eine
Anweisung, ein Makro oder eine Direktive.

Eine einzelne Zeile:

    LABEL: <OPERATION> ; KOMMENTAR

Mehrere Zeilen, die in einer einzigen Zeile zusammenfasst werden:

    LABEL:
                    ; KOMMENTAR
    ; KOMMENTAR

                           <OPERATION>
            ; KOMMENTAR

Ähnlich wie oben, aber in einem anderen Format:

    LABEL: ; KOMMENTAR
        <OPERATION>                     ; KOMMENTAR
        ; KOMMENTAR

Alle drei Beispiele sind gleich, wenn die Operationen gleich sind.

Siehe [OPERATIONEN][] und [LABEL-DEFINITIONEN][] für Details.

# TEXT- UND DATENSEKTIONEN

Sektionen sind geordnet und werden verwendet, um Operationen, die sich in
den Sektionen befinden, einer bestimmten Speicherstelle oder Speicherart
zuzuweisen. Sektionen jeglicher Art können nur einmal definiert
werden. Derzeit werden nur Daten- und Textsektionen unterstützt ohne Pläne
für die Unterstützung anderer Sektionen.

Die Verwendung von Sektionen ist optional, und eine Assemblerdatei, die
keine expliziten Sektionen enthält, definiert implizit eine Textsektion, die
die gesamte Datei umfasst.

Assemblerdateien müssen eine Textsektion und optional eine nicht leere
Datensektion enthalten.

Datensektionen müssen vor Textsektionen stehen.

Gültige Assemblerdatei, die eine Datensektion und eine Textsektion enthält:

    .data                           ; Definiert eine Datensektion, die bis zur nächsten Textsektion reicht
        <DIREKTIVE>
        <DIREKTIVE>

    .text                           ; Definiert eine Textsektion, die bis zum Ende der Datei reicht
        <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblerdatei, die nur eine Textsektion enthält:

    .text
        <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblerdatei, die implizit eine Textsektion definiert:

    START:
        <ANWEISUNG>
        <ANWEISUNG>

Ungültige Assemblerdatei, da die Datensektion leer ist (dies kann sich in
Zukunft ändern):

    .data
    .text
        <ANWEISUNG>

Ungültige Assemblerdatei, da sie nur eine Datensektion enthält:

    .data
        <DIREKTIVE>
        <DIREKTIVE>

Siehe [DIREKTIVEN][], [MAKROS][] und [ANWEISUNGEN][] für Details.

# LABEL-DEFINITIONEN

Label-Definitionen sind Label-Verweise, die mit einem Doppelpunkt enden.
Labels müssen mindestens ein Zeichen lang sein und dürfen nur
alphanumerische Zeichen enthalten.  Das erste Zeichen muss alphabetisch
sein.

Korrekte Label-Definitionen:

    Label0:
    A:
    LabelVeryNice:
    Example215:

Falsche Label-Definitionen:

    :
    __TEST:
    0Lol:
    Lol

Beachten Sie, dass derzeit keine Validierung der Label-Typen erfolgt und
nicht überprüft wird, ob der Label-Wert in die Anweisung passt.  Fehler für
Ersteres und Warnungen für Letzteres sind geplant.

# OPERATIONEN

Anweisungen, Makros und Direktiven werden unter dem Oberbegriff Operationen
zusammengefasst.  Die Argumente können Immediates, Register und/oder Labels
sein.  Die Argumente sind durch Kommas (**,**) oder Kommas mit einem
Leerzeichen (**,** ) getrennt.  Der Operationsname und die Argumente sind
durch ein oder mehrere Leerzeichen getrennt.

Siehe [ANWEISUNGEN][], [DIREKTIVEN][] und [MAKROS][] für Details.

# REGISTER

Einige Anweisungen und Makros erfordern Register, um Aktionen auszuführen.
Es gibt 31 Register, die verwendet werden können.  Register können entweder
durch die Registernummer mit einem vorangestellten **x**, was **x0** bis
**x31** bedeutet, oder durch ihren ABI-Namen referenziert werden.

| Register            | ABI Name            | Beschreibung                                                                                         | Saver    |
|:-------------------:|:-------------------:|------------------------------------------------------------------------------------------------------|:--------:|
| x0                  | zero                | Unveränderbares Register, das immer Null ist.                                                        | —        |
| x1                  | ra                  | Rücksprungadresse                                                                                    | Callee   |
| x2                  | sp                  | Stack-Pointer                                                                                        | Callee   |
| x3                  | gp                  | Allzweckregister. Globaler Pointer gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet.| —        |
| x4                  | tp                  | Allzweckregister. Thread-Pointer gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet.  | —        |
| x5-x7               | t0-t2               | Temporäres Register.                                                                                 | Caller   |
| x8                  | s0/fp               | Gespeichertes Register oder Frame-Pointer.                                                           | Callee   |
| x9                  | s1                  | Gespeichertes Register.                                                                              | Callee   |
| x10-x11             | a0-a1               | Register für Rückgabewerte und Funktionsargumente.                                                   | Caller   |
| x12-x17             | a2-a7               | Register für Funktionsargumente.                                                                     | Caller   |
| x18-x27             | s2-s11              | Gespeichertes Register.                                                                              | Callee   |
| x28-x31             | t3-t6               | Temporäres Register.                                                                                 | Caller   |

# IMMEDIATES

Einige Anweisungen, Makros und Direktiven erfordern Immediates, um Aktionen
auszuführen.  Immediates sind entweder als Dezimal-, Binär- oder
Hexadezimalzahl formatiert.  Standardmäßig wird das Dezimalformat
angenommen.  Um Immediates als binär oder hexadezimal zu interpretieren,
muss das Präfix **0b** bzw. **0x** verwendet werden.  Optional können binäre
oder hexadezimale Immediates als vorzeichenbehaftete Zahlen interpretiert
werden, indem ein **s** oder **S** als Suffix hinzugefügt wird.

Immediates und Symbole sind synonym und können beide für Operationen
verwendet werden, die Immediates als Argument annehmen. Symbole sind
Konstanten, die mittels der Direktive **.eqv** definiert wurden. Genauere
Informationen zu dieser Direktive sind unter [DIREKTIVEN][] zu finden.

Die folgenden Immediates sind gültig:

    0b10          ; 2
    0b10s         ; -2
    0x14          ; 20
    0x14s         ; 20
    SYMBOL        ; Gültig, wenn das Symbol in einer .eqv-Direktive definiert wurde
    205
    -12

Die folgenden Immediates sind ungültig:

    0.1           ; Gleitkommazahlen werden (noch) nicht unterstützt
    b100
    x1516
    SYMBOL        ; Ungültig, wenn das kein Symbol ist, welches mithilfe einer .eqv-Direktive definiert wurde
    02x3
    50-20         ; Ausdrücke werden nicht unterstützt

# DIREKTIVEN

Direktiven werden in Datensektionen verwendet und sind immer mit einem Punkt
(**.**) vorangestellt.  Einige häufig verwendete Direktiven werden
unterstützt, hauptsächlich solche, die zur Speicherung von Daten in der
Datensektion verwendet werden.

Für einige Direktiven ist das Argument ein String, der zwischen
Anführungszeichen steht.  Ansonsten gelten hier die Konventionen, die
generell gelten.

Die Reihenfolge der Direktiven in der Assemblerdatei bestimmt die
Reihenfolge der Daten im Speicher.  Daten beginnen bei Adresse 0 und wachsen
nach oben.  Die erste Direktive wird an Adresse 0 geschrieben.

Derzeit werden die folgenden Direktiven unterstützt:

**.byte** *immediate*|*label*,[*immediate*|*label*]...

:   Die Immediates und Label werden als 8-Bit-Werte im Speicher gespeichert.

**.half** *immediate*|*label*,[*immediate*|*label*]...

:   Die Immediates und Label werden als 16-Bit-Werte im Speicher gespeichert.

**.word** *immediate*|*label*,[*immediate*|*label*]...

:   Die Immediates und Label werden als 32-Bit-Werte im Speicher gespeichert.

**.dword** *immediate*|*label*,[*immediate*|*label*]...

:   Die Immediates und Label werden als 64-Bit-Werte im Speicher gespeichert.

**.space** *dezimal*

:   Reserviere Platz für Daten. Die *Dezimal*zahl gibt den reservierten Platz in Bytes an. Es muss eine Dezimalzahl sein und kann nicht negativ sein.

**.ascii "***string***"**

:   Der *string* wird als aufeinanderfolgende 8-Bit-Werte gespeichert. Der *string* sollte nur ASCII-Zeichen enthalten. Alle Zeichen werden in ihren ASCII-Code übersetzt. Der *string* ist nicht nullterminiert.

**.asciz "***string***"**

:   Wie **.ascii**, aber der *string* ist nullterminiert.

**.string "***string***"**

:   Alias für **.asciz**.

**.eqv** *symbol*, *immediate*

:   Der Wert des *symbol*s ist gleich *immediate*. Ein auf diese Weise erstelltes *symbol* ist eine Konstante, die nicht in den Speicher geschrieben wird und wie ein Immediate verwendet werden kann.

Diese sind gültige Direktiven:

    .byte   1, 3,2,LABEL
    .half 20, 15, LABEL
    .space 10
    .eqv                          LABEL,30

Diese sind ungültige Direktiven:

    .non                  ; unbekannte Direktive
    .byte                 ; keine Argumente
    .half 30 15
    .space 0b10           ; Argument kann nur eine Dezimalzahl sein
    .asciz "STRING        ; fehlendes abschließendes Anführungszeichen

# MAKROS

Makros sind Pseudo-Anweisungen, die nicht direkt in Maschinencode übersetzt
werden können.  Die Syntax ist ähnlich wie bei [ANWEISUNGEN][].  Einige der
gängigen Makros werden unterstützt.

Makros werden erweitert und auf Anweisungen abgebildet, die zu Maschinencode
übersetzbar sind.  Makros können nicht von Programmierern definiert werden.

Das erste Register für Makros (die welche haben) ist immer das Register, in
das geschrieben wird.

Derzeit werden die folgenden Makros unterstützt:

**srr** *register*, *register*, *immediate*

:   Shift right rotate (Bitweise rechts rotieren). Dies ist als Unterprogramm implementiert, daher ist das Speichern von Registern erforderlich. Das Speichern von Registern erfolgt nicht automatisch!

**slr** *register*, *register*, *immediate*

:   Shift left rotate (Bitweise links rotieren). Dies ist als Unterprogramm implementiert, daher ist das Speichern von Registern erforderlich. Das Speichern von Registern erfolgt nicht automatisch!

**li** *register*, *immediate*

:   Load immediate (Lade den Immediate). *register* wird auf das *immediate* gesetzt.

**la** *register*, *label*

:   Load address (Lade die Speicheradresse). *register* wird auf die Adresse des *label* gesetzt.

**call** *label*

:   Springt zu einem weit entfernten *Label* und behandelt es als Unterprogramm. Die Rücksprungadresse wird in das Register **ra** geschrieben. Ein Rücksprung ist mithilfe des Makros **ret** oder der entsprechenden **jal**-Anweisung möglich.

**tail** *label*

:   Springt zu einem weit entfernten *Label*. Die Rücksprungadresse wird nicht gespeichert. Ein Rücksprung ist nicht möglich.

**push** *register*, [*register*]...

:   Speichert den Inhalt dieser Register auf dem Stack. Die Initialisierung des Stack-Pointer-Registers **sp** ist erforderlich. Es können mehrere Register angegeben werden, um nur eine Subtraktion auszuführen. Die Register werden in der angegebenen Reihenfolge gespeichert. Das erste Register wird am Ende und das letzte Register am Anfang des Stacks gespeichert.

**pop** *register*, [*register*]...

:   Lädt den Inhalt des Stacks in die Register. Die Initialisierung des Stack-Pointer-Registers **sp** ist erforderlich. Es können mehrere Register angegeben werden, um nur eine Addition auszuführen. Der Inhalt wird in den Registern in der angegebenen Reihenfolge geladen. Das erste Register erhält den Inhalt des ersten Elementes auf dem Stack. Das letzte Register erhält den Inhalt des n-ten Elementes, welches sich ausgehend von dem Anfang des Stacks auf dem Stack befindet. N steht für die Anzahl der Register, die bei der Anweisung angegeben wurde.

**rep** *dezimal*, *Anweisung*|*Makro*

:   Die *Anweisung* oder *Makro* wird *dezimal* mal wiederholt. Die Dezimalzahl muss größer als 0 sein. Wiederholungen können nicht verschachtelt werden, d. h. eine Wiederholung kann keine weitere Wiederholung enthalten.

**mv** *register*, *register*

:   Kopiert den Inhalt des letzten Registers in das erste Register. Dies wird entweder auf die Anweisung **addi** oder **add** abgebildet.

**nop**

:   Nulloperation. Die Operation inkrementiert nur den Programmzähler um 4 und nutzt Prozessorzyklen. Dies wird entweder auf die Anweisung **addi zero, zero, 0** oder **add zero, zero, zero** abgebildet.

**ret**

:   Wird verwendet, um aus einem Unterprogramm zurückzukehren. Dies wird auf die Anweisung **jalr zero, ra, 0** abgebildet.

**j** *label*

:   Springt zum *Label*. Dies wird auf die Anweisung **jal zero,** *offset* abgebildet.

**jal** *label*

:   "Jump and Link" (Springt zu einer Subroutine und speichert die Rücksprungadresse) mit dem *label*. Dies wird auf die Anweisung **jal ra,** *offset* abgebildet.

**jr** *register*

:   Springt zur Adresse im *Register*. Dies wird auf die Anweisung **jalr zero,** *register***, 0** abgebildet.

**jalr** *register*

:   "Jump and Link Register" (Springt zu einer Subroutine, dessen Adresse im *Register* steht, und speichert die Rücksprungadresse) mit der Adresse im Register **ra**. Dies wird auf die Anweisung **jalr ra,** *register***, 0** abgebildet.

**lb** *register*, *immediate*
: \
**lb** *register*, *label*
: \

**lb** *register*, **%lo(***label***)**, **(***register***)**

:   Loads a byte from memory at the address which is either a *immediate*, a *label* or the sum of the lower 12 bits of a *label* and a *register*.

**lh** *register*, *immediate*
: \
**lh** *register*, *label*
: \

**lh** *register*, **%lo(***label***)**, **(***register***)**

:   Loads a half (16 bits) from memory at the address which is either a *immediate*, a *label* or the sum of the lower 12 bits of a *label* and a *register*.

**lw** *register*, *immediate*
: \
**lw** *register*, *label*
: \

**lw** *register*, **%lo(***label***)**, **(***register***)**

:   Loads a word (32 bits) from memory at the address which is either a *immediate*, a *label* or the sum of the lower 12 bits of a *label* and a *register*.

**lbu** *register*, *immediate*
: \

**lbu** *register*, *label*

:   Loads a byte from memory at the address which is either a *immediate* or a *label*. The byte is zero extended to 32 bits.

**lhu** *register*, *immediate*
: \

**lhu** *register*, *label*

:   Loads a half from memory at the address which is either a *immediate* or a *label*. The half is zero extended to 32 bits.

**sb** *register*, *immediate*
: \
**sb** *register*, *immediate*, *register*
: \

**sb** *register*, *label*, *register*

:   Stores a byte into memory at the address which is either a *immediate* or *label*. For *immediate*s over 12 bits and *label*s, a temporary *register* must be specified.

**sh** *register*, *immediate*
: \
**sh** *register*, *immediate*, *register*
: \

**sh** *register*, *label*, *register*

:   Stores a half into memory at the address which is either a *immediate* or *label*. For *immediate*s over 12 bits and *label*s, a temporary *register* must be specified.

**sw** *register*, *immediate*
: \
**sw** *register*, *immediate*, *register*
: \

**sw** *register*, *label*, *register*

:   Stores a word into memory at the address which is either a *immediate* or *label*. For *immediate*s over 12 bits and *label*s, a temporary *register* must be specified.

Siehe [RISC-V Shortened Spec][] für weitere Details.

# ANWEISUNGEN

Eine Anweisung ist Maschinencode in lesbarer Form.  Die Syntax ähnelt der
von [MAKROS][].  Alle Anweisungen der RV32I- und RV32M-Erweiterungen werden
unterstützt.

Bei Anweisungen, auf die das zutrifft, ist das erste Register immer das
Register, in das geschrieben wird.  Eine Ausnahme dazu bilden
Sprunganweisungen.

Diese Anweisungen werden verwendet, um arithmetische, logische und
Schiebeoperationen mit Registern durchzuführen:

**add** *register*, *register*, *register*

:   Addition der beiden letzten *Register*.

**sub** *register*, *register*, *register*

:   Subtraktion. Der Minuend ist der Inhalt des zweiten *Register*s, der Subtrahend ist der Inhalt des letzten *Register*s.

**xor** *register*, *register*, *register*

:   Logisches bitweises exklusives Oder des zweiten und dritten *Register*s.

**or** *register*, *register*, *register*

:   Logisches bitweises Oder des zweiten und dritten *Register*s.

**and** *register*, *register*, *register*

:   Logisches bitweises Und des zweiten und dritten *Register*s.

**sll** *register*, *register*, *register*

:   Logisches Linksschieben des zweiten *Register*s um das dritte *Register*.

**srl** *register*, *register*, *register*

:   Logisches Rechtsschieben des zweiten *Register*s um das dritte *Register*.

**sra** *register*, *register*, *register*

:   Arithmetisches Rechtsschieben des zweiten *Register*s um das dritte *Register*.

**slt** *register*, *register*, *register*

:   Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist.

**sltu** *register*, *register*, *register*

:   Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist. Die Inhalte der verglichenen *Register* werden als vorzeichenlose Zahlen interpretiert.

**xnor** *register*, *register*, *register*

:   Logical bitwise negated exclusive or of the second and third *register*. Note that this is not defined in the RISC-V standard. Deprecated, do not use!

**equal** *register*, *register*, *register*

:   Compares the second and third *register*s and sets the first *register* to one (1), if they are equal. Note that this is not defined in the RISC-V standard. Deprecated, do not use!

**mul** *register*, *register*, *register*

:   Multiplikation des zweiten und dritten *Register*s. Das erste *Register* wird auf die unteren 32 Bits des Ergebnisses gesetzt.

**mulh** *register*, *register*, *register*

:   Hohe Multiplikation des zweiten und dritten *Register*s. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Die Inhalte beider *Register* werden als vorzeichenbehaftete Zahlen interpretiert.

**mulhu** *register*, *register*, *register*

:   Hohe Multiplikation des zweiten und dritten *Register*s. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Die Inhalte beider *Register* werden als vorzeichenlose Zahlen interpretiert.

**mulhsu** *register*, *register*, *register*

:   Hohe Multiplikation des zweiten und dritten *Register*s. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Der Inhalt des zweiten *Register*s wird als vorzeichenbehaftete Zahl interpretiert, der Inhalt des dritten *Register*s als vorzeichenlose Zahl.

**div** *register*, *register*, *register*

:   Division des zweiten und dritten *Register*s. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**divu** *register*, *register*, *register*

:   Division des zweiten und dritten *Register*s. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

**rem** *register*, *register*, *register*

:   Modulo-Operation des zweiten und dritten *Register*s. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**remu** *register*, *register*, *register*

:   Modulo-Operation des zweiten und dritten *Register*s. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um arithmetische, logische und
Schiebeoperationen mit Immediates durchzuführen:

Schiebeoperationen können nur Immediatewerte von 4 Bits verwenden.

Beachten Sie, dass in einigen Anweisungen keine Labels verwenden werden
können.  Dies befindet sich in Arbeit und wird zukünftig geändert.

**addi** *register*, *register*, *immediate*|**%lo(***label***)**

:   Addition des zweiten *Register*s und des *Immediate*s oder *Label*s.

**xori** *register*, *register*, *immediate*

:   Logisches bitweises exklusives Oder des zweiten *Register*s und des *Immediate*s.

**ori** *register*, *register*, *immediate*

:   Logisches bitweises Oder des zweiten *Register*s und des *Immediate*s.

**andi** *register*, *register*, *immediate*

:   Logisches bitweises Und des zweiten *Register*s und des *Immediate*s.

**slli** *register*, *register*, *immediate*

:   Logical left shift of the second *register* by the *immediate*.

**srli** *register*, *register*, *immediate*

:   Logical right shift of the second *register* by the *immediate*.

**srai** *register*, *register*, *immediate*

:   Arithmetical right shift of the second *register* by the *immediate*.

**slti** *register*, *register*, *immediate*

:   Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als die *Immediate* ist.

**sltiu** *register*, *register*, *immediate*

:   Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als die *Immediate* ist. Die Inhalte der verglichenen *Register* werden als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um den Speicherinhalt zu manipulieren:

Das Zielbyte und das halbe Wort sind immer die LSBs (niederwertigsten Bits)
des Zielregisters.

**lb** *register*, [*immediate*]**(***register***)**

:   Loads a byte from memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

**lh** *register*, [*immediate*]**(***register***)**

:   Loads a half (16 bits) from memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

**lw** *register*, [*immediate*]**(***register***)**

:   Loads a word (32 bits) from memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

**lbu** *register*, [*immediate*]**(***register***)**

:   Loads a byte from memory at the address which is the sum of the second *register* and the *immediate*. The byte is zero extended to 32 bits. This instruction has a few Macros, see [MACROS][] for details.

**lhu** *register*, [*immediate*]**(***register***)**

:   Loads a half from memory at the address which is the sum of the second *register* and the *immediate*. The half is zero extended to 32 bits. This instruction has a few Macros, see [MACROS][] for details.

**sb** *register*, [*immediate*]**(***register***)**

:   Stores a byte into memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

**sh** *register*, [*immediate*]**(***register***)**

:   Stores a half into memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

**sw** *register*, [*immediate*]**(***register***)**

:   Stores a word into memory at the address which is the sum of the second *register* and the *immediate*. This instruction has a few Macros, see [MACROS][] for details.

Diese Anweisungen werden verwendet, um den Kontrollfluss des Programms zu
steuern:

Für alle Anweisungen außer **jalr** wird eine PC-relative Adressierung
verwendet.  Für **jalr** wird absolute Adressierung verwendet.

**beq** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt der *Register* gleich ist.

**bne** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt der *Register* nicht gleich ist.

**blt** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt des ersten *Register*s kleiner ist als der Inhalt des letzten *Register*s. Der Inhalt der *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**bltu** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt des ersten *Register*s kleiner ist als der Inhalt des letzten *Register*s. Der Inhalt der *Register* wird als vorzeichenlose Zahlen interpretiert.

**bge** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt des ersten *Register*s größer oder gleich dem Inhalt des letzten *Register*s ist. Der Inhalt der *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**bgeu** *register*, *register*, *immediate*|*label*

:   Springt, wenn der Inhalt des ersten *Register*s größer oder gleich dem Inhalt des letzten *Register*s ist. Der Inhalt der *Register* wird als vorzeichenlose Zahlen interpretiert.

**jal** *register*, *immediate*|*label*

:   "Jump and link" (Springt zu einer Subroutine und speichert die Rücksprungadresse) zu der Adresse, die die Summe aus dem Programmzähler und der *Immediate* oder dem Wert des *Label*s ist. Die Rücksprungadresse wird in das *Register* geschrieben.

**jalr** *register*, *register*, *immediate*

:   Jump and link to the address which is the sum of the second *register* and the *immediate*. The return address is written to the first *register*.

Diese Anweisungen können nicht kategorisiert werden:

**lui** *register*, *immediate*|**%hi(***label***)**

:   Lädt den oberen Teil des *Immediate*s. Die oberen 20 Bits des *Register*s werden auf das *Immediate* oder *Label* gesetzt. Die unteren 12 Bits sind Nullen.

**auipc** *register*, *immediate*

:   Add upper immediate to program counter. The *immediate* is shifted by 12 bits, added to the program counter and the result is written to the *register*.

Siehe [RISC-V Shortened Spec][] für weitere Details.

# SIEHE AUCH

[assembler(1)], [RISC-V Shortened Spec][]

[assembler(1)]: assembler.1.md
[RISC-V Shortened Spec]: https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf
[MAKROS]: #makros
[LABEL-DEFINITIONEN]: #label-definitionen
[OPERATIONEN]: #operationen
[DIREKTIVEN]: #direktiven
[ANWEISUNGEN]: #anweisungen
