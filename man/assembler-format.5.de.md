% ASSEMBLER-FORMAT(5) Version 1.2.0 | Handbuch für Dateiformate
---
Datum: Februar 2024
---

# NAME

**assembler-format** --- Format von Eingabe-Assemblerdateien

# SYNOPSIS

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
    ; Schreibt Anweisungen und Makros hier hin
    START:
        nop
        addi    zero, zero, 0
        addi    t1, zero, 2
        mul     t0, sp, t1
    LOD:
        j       LOD                 ; Sie können Labels auch in Makros verwenden

# BESCHREIBUNG

Der Befehl [assembler(1)] konvertiert Assemblerdateien in eine Ausgabe im binären oder MIF-Format. Die Syntax basiert auf der MIPS-Assembler-Syntax und wurde modifiziert, um den Aufwand des Programmierers zu verringern und die Kompatibilität mit bereits etablierten Tools und Konventionen zu erhöhen.

# SYNTAX

Im Allgemeinen enthält jede Zeile nichts (Leerzeichen), eine Sektion, ein Label oder ein Label und eine Operation. Kommentare können am Ende jeder Zeile hinzugefügt oder in eine Zeile eingefügt werden, die ansonsten nichts (Leerzeichen) enthalten würde. Der Assembler ist nachsichtig mit der Platzierung dieser Komponenten zueinander. Eine Operation ist eine Anweisung, ein Makro oder eine Direktive.

Eine einzelne Zeile:

    LABEL: <OPERATION> ; KOMMENTAR

Mehrere Zeilen, die in einer einzigen Zeile zusammenfallen:

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

Siehe [OPERATIONS][] und [LABEL DEFINITIONS][] für Details.

# TEXT- UND DATENSEKTIONEN

Sektionen sind geordnet und werden verwendet, um Abschnitte eines bestimmten Typs zu umfassen. Sektionen jeglicher Art können nur einmal definiert werden. Derzeit werden nur Daten- und Textsektionen unterstützt. Die Unterstützung anderer Sektionen ist nicht geplant.

Die Verwendung von Sektionen ist optional, und eine Assemblerdatei, die keine expliziten Sektionen enthält, definiert implizit eine Textsektion, die die gesamte Datei umfasst.

Assemblerdateien müssen eine Textsektion und optional eine nicht leere Datensektion enthalten.

Datensektionen müssen vor Textsektionen stehen.

Gültige Assemblerdatei, die eine Datensektion und eine Textsektion enthält:

    .data                           ; Definiert eine Datensektion, die bis zur nächsten Textsektion reicht
        <DIREKTIVE>                 ; <DIREKTIVE>
        <DIREKTIVE>

    .text                           ; Definiert eine Textsektion, die bis zum Ende der Datei reicht
        <ANWEISUNG>                 ; <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblerdatei, die nur eine Textsektion enthält:

    .text
        <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblerdatei, die implizit eine Textsektion definiert:

    START:
        <ANWEISUNG>
        <ANWEISUNG>

Ungültige Assemblerdatei, da die Datensektion leer ist (dies kann sich in Zukunft ändern):

    .data
    .text
        <ANWEISUNG>

Ungültige Assemblerdatei, da sie nur eine Datensektion enthält:

    .data
        <DIREKTIVE>
        <DIREKTIVE>

Siehe [DIREKTIVEN][], [MAKROS][] und [ANWEISUNGEN][] für Details.

# LABEL-DEFINITIONEN

Label-Definitionen sind Label-Verweise, die mit einem Doppelpunkt enden. Labels müssen mindestens ein Zeichen lang sein und dürfen nur alphanumerische Zeichen enthalten. Das erste Zeichen muss alphabetisch sein.

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

Beachten Sie, dass derzeit keine Validierung der Label-Typen erfolgt und nicht überprüft wird, ob der Label-Wert in die Anweisung passt. Fehler für Ersteres und Warnungen für Letzteres sind geplant.

# OPERATIONEN

Anweisungen, Makros und Direktiven werden unter dem Oberbegriff Operationen zusammengefasst. Die Argumente können Immediates, Register und/oder Labels sein. Die Argumente sind durch Kommas (**,**) oder Kommas mit einem Leerzeichen (**,** ) getrennt. Der Operationsname und die Argumente sind durch ein oder mehrere Leerzeichen getrennt.

Siehe [ANWEISUNGEN][], [DIREKTIVEN][] und [MAKROS][] für Details.

# Register

Einige Anweisungen und Makros erfordern Register, um Aktionen auszuführen. Es gibt 31 Register, die verwendet werden können. Register können entweder durch die Registernummer mit einem vorangestellten x referenziert werden, was **x0** bis **x31** bedeutet, oder durch ihren ABI-Namen.

| Register            | ABI Name            | Description                                                                                          | Saver    |
|:-------------------:|:-------------------:|------------------------------------------------------------------------------------------------------|:--------:|
| x0                  | zero                | Unveränderbares Register, das immer Null ist.                                                        | —        |
| x1                  | ra                  | Rücksprungadresse                                                                                    | Callee   |
| x2                  | sp                  | Stack-Pointer                                                                                        | Callee   |
| x3                  | gp                  | Allzweckregister. Globales Zeiger gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet. | —        |
| x4                  | tp                  | Allzweckregister. Thread-Pointer gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet.  | —        |
| x5-x7               | t0-t2               | Temporäres Register.                                                                                 | Caller   |
| x8                  | s0/fp               | Gespeichertes Register oder Frame-Pointer.                                                           | Callee   |
| x9                  | s1                  | Gespeichertes Register.                                                                              | Callee   |
| x10-x11             | a0-a1               | Register für Rückgabewerte und Funktionsargumente.                                                   | Caller   |
| x12-x17             | a2-a7               | Register für Funktionsargumente.                                                                     | Caller   |
| x18-x27             | s2-s11              | Gespeichertes Register.                                                                              | Callee   |
| x28-x31             | t3-t6               | Temporäres Register.                                                                                 | Caller   |

# IMMEDIATES

Einige Anweisungen, Makros und Direktiven erfordern Immediates, um Aktionen auszuführen. Immediates sind entweder im Dezimal-, Binär- oder Hexadezimalformat. Standardmäßig wird das Dezimalformat angenommen. Um Immediates als binär oder hexadezimal zu interpretieren, muss das Präfix 0b bzw. 0x verwendet werden. Optional können binäre oder hexadezimale Immediates als vorzeichenbehaftete Zahlen interpretiert werden, indem ein s oder S als Suffix rangehängt wird.

Die folgenden Immediates sind gültig:

    0b10          ; 2
    0b10s         ; -2
    0x14          ; 20
    0x14s         ; 20
    205
    -12

Die folgenden Immediates sind ungültig:

    0.1           ; Gleitkommazahlen werden (noch) nicht unterstützt
    b100
    x1516
    02x3
    50-20         ; Ausdrücke werden nicht unterstützt

# DIREKTIVEN

Direktiven werden in Datensektionen verwendet und sind immer mit einem Punkt (.) vorangestellt. Einige häufig verwendete Direktiven werden unterstützt, hauptsächlich solche, die verwendet werden können, um Daten im Datensektion zu speichern.

Für einige Direktiven ist das Argument ein String, der zwischen Anführungszeichen steht. Ansonsten gelten hier die Konventionen, die generell gelten.

Die Reihenfolge der Direktiven in der Assemblerdatei bestimmt die Reihenfolge der Daten im Speicher. Daten beginnen bei Adresse 0 und wachsen nach oben. Die erste Direktive wird an Adresse 0 geschrieben.

Derzeit werden die folgenden Direktiven unterstützt:

**.byte** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 8-Bit-Werte im Speicher gespeichert.

**.half** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 16-Bit-Werte im Speicher gespeichert.

**.word** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 32-Bit-Werte im Speicher gespeichert.

**.dword** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 64-Bit-Werte im Speicher gespeichert.

**.space** *dezimal*

: Reserviere Platz für Daten. Das *dezimal* gibt den reservierten Platz in Bytes an. Es muss eine Dezimalzahl sein und kann nicht negativ sein.

**.ascii "***string***"**

: Der *string* wird als aufeinanderfolgende 8-Bit-Werte gespeichert. Der *string* sollte nur ASCII-Zeichen enthalten. Alle Zeichen werden in ihren ASCII-Code übersetzt. Der *string* ist nicht nullterminiert.

**.asciz "***string***"**

: Wie **.ascii**, aber der *string* ist nullterminiert.

**.string "***string***"**

: Alias für **.asciz**.

**.eqv** *label*, *immediate*

: Der Wert des *label* ist *immediate*. Ein auf diese Weise erstelltes *label* ist eine Konstante, die nicht im Speicher geschrieben wird und wie ein Immediate verwendet werden kann.

Diese sind gültige Direktiven:

    .byte   1, 3,2,LABEL
    .half 20, 15, LABEL
    .space 10
    .eqv                          LABEL,30

Diese sind ungültige Direktiven:

    .non                  ; unbekannte Direktive
    .byte                 ; keine Argumente
    .half 30 15
    .space 0b10           ; Argument kann nur Dezimalzahl sein
    .asciz "STRING        ; fehlende abschließende Anführungszeichen

# MAKROS

Makros sind Pseudo-Anweisungen, die nicht direkt in Maschinencode übersetzt werden können. Die Syntax ist ähnlich wie bei [ANWEISUNGEN][]. Einige der gängigen Makros werden unterstützt.

Makros werden erweitert und auf Anweisungen abgebildet, die maschinenübersetzbar sind. Makros können nicht von Programmierern definiert werden.

Das erste Register für Makros (die welche haben) ist immer das Register, in das geschrieben wird.

Derzeit werden die folgenden Makros unterstützt:

**srr** *register*, *register*, *immediate*

: Shift right rotate (Bitweise rechts rotieren). Dies ist als Unterprogramm implementiert, daher ist das Speichern von Registern erforderlich. Das Speichern von Registern erfolgt nicht automatisch!

**slr** *register*, *register*, *immediate*

: Shift left rotate (Bitweise links rotieren). Dies ist als Unterprogramm implementiert, daher ist das Speichern von Registern erforderlich. Das Speichern von Registern erfolgt nicht automatisch!

**li** *register*, *immediate*|*label*

: Load immediate (Lade den Immediate). *register* wird auf das *immediate* oder *label* gesetzt.

**la** *register*, *immediate*|*label*

: Load address (Lade die Speicheradresse). *register* wird entweder auf das *immediate* oder auf die Adresse des *label* gesetzt.

**call** *immediate*|*label*

: Springt zu einem weit entfernten *Label* und behandelt es als Unterprogramm. Die Rücksprungadresse wird in das Register **ra** geschrieben. Eine Rücksprung ist mithilfe des Makros **ret** oder der entsprechenden **jal**-Anweisung möglich.

**tail** *immediate*|*label*

: Springt zu einem weit entfernten *Label*. Die Rücksprungadresse wird nicht gespeichert. Ein Rücksprung ist nicht möglich.

**push** *register*, [*register*]...

: Speichert den Inhalt dieser Register auf dem Stack. Die Initialisierung des Stack-Pointer-Registers **sp** ist erforderlich. Es können mehrere Register angegeben werden, um den Subtraktionsaufwand zu reduzieren. Die Register werden in der angegebenen Reihenfolge gespeichert. Das erste Register wird am Ende und das letzte Register am Anfang des Stacks gespeichert.

**pop** *register*, [*register*]...

: Lädt den Inhalt des Stacks in die Register. Die Initialisierung des Stack-Pointer-Registers **sp** ist erforderlich. Es können mehrere Register angegeben werden, um den Additionsaufwand zu reduzieren. Der Inhalt wird in den Registern in der angegebenen Reihenfolge geladen. Das erste Register erhält den obersten Inhalt vom Stack, das letzte Register den am weitesten untenliegenden Inhalt, der erreichbar ist.

**rep** *decimal*, *instruction*|*macro*

: Die *instruction* oder *macro* wird *decimal* mal wiederholt. Die Dezimalzahl muss positiv und größer als 0 sein. Wiederholungen können nicht verschachtelt werden, d. h. eine Wiederholung kann keine weitere Wiederholung enthalten.

**mv** *register*, *register*

: Kopiert den Inhalt des letzten Registers in das erste Register. Dies wird entweder auf die Anweisung **addi** oder **add** abgebildet.

**nop**

: Keine Operation. Es bewirkt nichts. Dies wird entweder auf die Anweisung **addi zero, zero, 0** oder **add zero, zero, zero** abgebildet.

**ret**

: Wird verwendet, um aus einem Unterprogramm zurückzukehren. Dies wird auf die Anweisung **jalr zero, ra, 0** abgebildet.

**j** *immediate*|*label*

: Springt zum *Label* oder *Immediate*. Dies wird auf die Anweisung **jal zero,** *offset* abgebildet.

**jal** *immediate*|*label*

: "Jump and Link" (Springen und verknüpfen) mit dem *label* oder *immediate*. Dies wird auf die Anweisung **jal ra,** *offset* abgebildet.

**jr** *register*

: Springt zur Adresse im *register*. Dies wird auf die Anweisung **jalr zero,** *register***, 0** abgebildet.

**jalr** *register*

: "Jump and Link" (Springen und verknüpfen) mit der Adresse im *Register*. Dies wird auf die Anweisung **jalr ra,** *register***, 0** abgebildet.

Siehe [RISC-V Shortened Spec][] für Details.


# ANWEISUNGEN

Eine Anweisung ist Maschinencode in menschenlesbarer Form. Die Syntax ähnelt der von [MAKROS][]. Alle Anweisungen der RV32I- und RV32M-Erweiterungen werden unterstützt.

Bei Anweisungen, die sie benötigen, ist das erste Register immer das Register, in das geschrieben wird. Eine Einschränkung dieser Regel sind Sprunganweisungen.

Diese Anweisungen werden verwendet, um arithmetische, logische und Schiebeoperationen mit Registern durchzuführen:

**add** *register*, *register*, *register*

: Addition der beiden letzten *Register*.

**sub** *register*, *register*, *register*

: Subtraktion. Der Minuend ist der Inhalt des zweiten *Registers*, der Subtrahend ist der Inhalt des letzten *Registers*.

**xor** *register*, *register*, *register*

: Logisches bitweises exklusives Oder des zweiten und dritten *Registers*.

**or** *register*, *register*, *register*

: Logisches bitweises Oder des zweiten und dritten *Registers*.

**and** *register*, *register*, *register*

: Logisches bitweises Und des zweiten und dritten *Registers*.

**sll** *register*, *register*, *register*

: Logisches linksschieben des zweiten *Registers* um das dritte *Register*.

**srl** *register*, *register*, *register*

: Logisches rechtsschieben des zweiten *Registers* um das dritte *Register*.

**sra** *register*, *register*, *register*

: Arithmetisches rechtsschieben des zweiten *Registers* um das dritte *Register*.

**slt** *register*, *register*, *register*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist.

**sltu** *register*, *register*, *register*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist. Der Inhalt der verglichenen *Register* wird als vorzeichenlose Zahlen interpretiert.

**xnor** *register*, *register*, *register*

: Logisches bitweises negiertes exklusives Oder des zweiten und dritten *Registers*. Dies ist nicht im RISC-V-Standard definiert.

**equal** *register*, *register*, *register*

: Vergleicht das zweite und dritte *Register* und setzt das erste *Register* auf eins (1), wenn sie gleich sind. Dies ist nicht im RISC-V-Standard definiert.

**mul** *register*, *register*, *register*

: Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die unteren 32 Bits des Ergebnisses gesetzt.

**mulh** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**mulhu** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

**mulhsu** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die oberen 32 Bits des Ergebnisses gesetzt. Der Inhalt des zweiten *Registers* wird als vorzeichenbehaftete Zahl interpretiert, der Inhalt des dritten *Registers* als vorzeichenlose Zahl.

**div** *register*, *register*, *register*

: Division des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**divu** *register*, *register*, *register*

: Division des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

**rem** *register*, *register*, *register*

: Modulo-Operation des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**remu** *register*, *register*, *register*

: Modulo-Operation des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um arithmetische, logische und Schiebeoperationen mit Immediatewerten durchzuführen:

Schiebeoperationen können nur Immediatewerte von 4 Bits verwenden.

Beachte, dass einige Anweisungen keine Labels verwenden können. Dies ist in Arbeit.

**addi** *register*, *register*, *immediate*|*label*

: Addition des zweiten *Registers* und des *Immediates* oder Labels.

**xori** *register*, *register*, *immediate*

: Logisches bitweises exklusives Oder des zweiten *Registers* und des *Immediates*.

**ori** *register*, *register*, *immediate*

: Logisches bitweises Oder des zweiten *Registers* und des *Immediates*.

**andi** *register*, *register*, *immediate*

: Logisches bitweises Und des zweiten *Registers* und des *Immediates*.

**slli** *register*, *register*, *immediate*|*label*

: Logisches linksschieben des zweiten *Registers* um das *Immediate* oder *Label*.

**srli** *register*, *register*, *immediate*|*label*

: Logisches rechtsschieben des zweiten *Registers* um das *Immediate* oder *Label*.

**srai** *register*, *register*, *immediate*|*label*

: Arithmetisches rechtsschieben des zweiten *Registers* um das *Immediate* oder *Label*.

**slti** *register*, *register*, *immediate*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das *Immediate* ist.

**sltiu** *register*, *register*, *immediate*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das *Immediate* ist. Der Inhalt der verglichenen *Register* wird als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um den Speicherinhalt zu manipulieren:

Das Zielbyte und das halbe Wort sind immer die LSBs (least significant bit/ ) des Zielregisters.

**lb** *register*, *register*, *immediate*|*label*

: Lädt ein Byte aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

**lh** *register*, *register*, *immediate*|*label*

: Lädt ein halbes Wort (16 Bit) aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

**lw** *register*, *register*, *immediate*|*label*

: Lädt ein Wort (32 Bit) aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

**lbu** *register*, *register*, *immediate*|*label*

: Lädt ein Byte aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist. Das Byte wird auf 32 Bits erweitert.

**lhu** *register*, *register*, *immediate*|*label*

: Lädt ein halbes Wort (16 Bit) aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist. Das halbe Wort wird auf 32 Bits erweitert.

**sb** *register*, *register*, *immediate*|*label*

: Speichert ein Byte in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

**sh** *register*, *register*, *immediate*|*label*

: Speichert ein halbes Wort in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

**sw** *register*, *register*, *immediate*|*label*

: Speichert ein Wort in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediates* oder *Labels* ist.

Diese Anweisungen werden verwendet, um den Logikfluss des Programms zu steuern:

Für alle Anweisungen außer jalr wird eine PC-relative Adressierung verwendet. Für jalr wird eine absolute Adressierung verwendet.

**beq** *register*, *register*, *immediate*|*label*

: Springt, wenn die *Register* gleich sind.

**bne** *register*, *register*, *immediate*|*label*

: Springt, wenn die *Register* nicht gleich sind.

**blt** *register*, *register*, *immediate*|*label*

: Springt, wenn der Inhalt des ersten *Registers* kleiner ist als der Inhalt des letzten *Registers*. Der Inhalt der *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**bltu** *register*, *register*, *immediate*|*label*

: Springt, wenn der Inhalt des ersten *Registers* kleiner ist als der Inhalt des letzten *Registers*. Der Inhalt der *Register* wird als vorzeichenlose Zahlen interpretiert.

**bge** *register*, *register*, *immediate*|*label*

: Springt, wenn der Inhalt des ersten *Registers* größer oder gleich dem Inhalt des letzten *Registers* ist. Der Inhalt der *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**bgeu** *register*, *register*, *immediate*|*label*

: Springt, wenn der Inhalt des ersten *Registers* größer oder gleich dem Inhalt des letzten *Registers* ist. Der Inhalt der *Register* wird als vorzeichenlose Zahlen interpretiert.

**jal** *register*, *immediate*|*label*

: "Jump and link" (Springt und Verknüpft) zu der Adresse, die die Summe des Programmzählers und des *Immediate** oder *Labels* ist. Die Rücksprungadresse wird in das *Register* geschrieben.

**jalr** *register*, *register*, *immediate*|*label*

: "Jump and link" (Springt und Verknüpft) zu der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist. Die Rücksprungadresse wird in das erste *Register* geschrieben.

Diese Anweisungen können nicht kategorisiert werden:

**lui** *register*, *immediate*|*label*

: Lädt den oberen Teil des *Immediates*. Die oberen 20 Bits des *Registers* werden auf das *Immediate* oder *Label* gesetzt. Die unteren 12 Bits sind nullen.

**auipc** *register*, *immediate*|*label*

: Addiert den oberen Teil des *Immediate* auf den Programmzähler. Die oberen 20 Bits des *Immediates* oder *Labels* werden zum Programmzähler addiert und das Ergebnis wird in das *Register* geschrieben.

Siehe [RISC-V Shortened Spec][] für weitere Details.
# SIEHE AUCH

[assembler(1)], [RISC-V Shortened Spec][]

[assembler(1)]: assembler.1.md
[RISC-V Shortened Spec]: https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf
[MAKROS]: #macros
[LABEL-DEFINITIONEN]: #label-definitions
[OPERATIONEN]: #operations
[DIREKTIVEN]: #directives
[ANWEISUNGEN]: #instructions
