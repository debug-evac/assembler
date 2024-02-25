% ASSEMBLER-FORMAT(5) Version 1.2.0 | Handbuch für Dateiformate
---
Datum: Februar 2024
---

# NAME

**assembler-format** --- Format von Eingabe-Assemblerdateien

# SYNOPSIS

    ; ist ein einzeiliger Kommentar, hier kann alles geschrieben werden
    .data
    ; Sie können hier* auch * sowie Speicherorte verwenden
    LABEL:
        .byte   1, 2, 3, LABEL      ; 8-Bit-Zahlen
        .half   1, 2, 3, LABEL      ; 16-Bit-Zahlen
        .word   1, 2, 3, LABEL      ; 32-Bit-Zahlen
        .dword  1, 2, 3, LABEL      ; 64-Bit-Zahlen
        .eqv    CONST, 20           ; CONST wird auf 20 gesetzt

    .text
    ; Sie können* auch * hier im .text-Abschnitt verwenden
    ; Schreibt Anweisungen und Makros hier
    START:
        nop
        addi    zero, zero, 0
        addi    t1, zero, 2
        mul     t0, sp, t1
    LOD:
        j       LOD                 ; Sie k*önnen * auch in Makros verwenden

# BESCHREIBUNG

Der Befehl [assembler(1)] konvertiert Assemblerdateien in eine Ausgabe im binären oder MIF-Format. Die Syntax basiert auf der MIPS-Assembler-Syntax und wurde modifiziert, um die Belastung des Programmierers zu verringern und die Kompatibilität mit bereits etablierten Tools und Konventionen zu erhöhen.

# SYNTAX

Im Allgemeinen enthält jede Zeile nichts (Leerzeichen), eine Sektion, ein Label oder ein Label und eine Operation. Kommentare können am Ende jeder Zeile hinzugefügt oder in eine Zeile eingefügt werden, die ansonsten nichts (Leerzeichen) enthalten würde. Der Assembler ist nachsichtig mit der Platzierung dieser Komponenten zueinander. Eine Operation ist eine Anweisung, ein Makro oder eine Direktive.

Eine einzelne Zeile:

    LABEL: <OPERATION> ; COMMENT

Mehrere Zeilen, die in einer einzigen Zeile zusammenfallen:

    LABEL:
                    ; COMMENT
    ; COMMENT

                           <OPERATION>
            ; COMMENT

Ähnlich wie oben, aber in einem anderen Format:

    LABEL: ; COMMENT
        <OPERATION>                     ; COMMENT
        ; COMMENT

Alle drei Beispiele sind gleich, wenn die Operationen gleich sind.

Siehe [OPERATIONS][] und [LABEL DEFINITIONS][] für Details.

# TEXT- UND DATENSEKTIONEN

Sektionen sind geordnet und werden verwendet, um Abschnitte eines bestimmten Typs zu umreißen. Sektionen jeglicher Art können nur einmal definiert werden. Derzeit werden nur Daten- und Textsektionen unterstützt, ohne Pläne zur Unterstützung anderer Sektionen.

Die Verwendung von Sektionen ist optional, und eine Assemblerdatei, die keine expliziten Sektionen enthält, definiert implizit eine Textsektion, die die gesamte Datei umfasst.

Assemblierdateien müssen eine Textsektion enthalten und optional eine nicht leere Datensektion enthalten.

Datensektionen müssen vor Textsektionen stehen.

Gültige Assemblierdatei, die eine Datensektion und eine Textsektion enthält:

    .data                           ; Umreißt eine Datensektion, die bis zur nächsten Textsektion reicht
        <DIREKTIVE>                 ; <DIREKTIVE>
        <DIREKTIVE>

    .text                           ; Umreißt eine Textsektion, die bis zum Ende der Datei reicht
        <ANWEISUNG>                 ; <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblierdatei, die nur eine Textsektion enthält:

    .text
        <ANWEISUNG>
        <ANWEISUNG>

Gültige Assemblierdatei, die implizit eine Textsektion definiert:

    START:
        <ANWEISUNG>
        <ANWEISUNG>

Ungültige Assemblierdatei, da die Datensektion leer ist (dies kann sich in Zukunft ändern):

    .data
    .text
        <ANWEISUNG>

Ungültige Assemblierdatei, da sie nur eine Datensektion enthält:

    .data
        <DIREKTIVE>
        <DIREKTIVE>

Siehe [DIREKTIVEN][], [MAKROS][] und [ANWEISUNGEN][] für Details.

# LABEL-DEFINITIONEN

Label-Definitionen sind Label-Verweise, die mit einem Doppelpunkt e*nden. * müssen mindestens ein Zeichen lang sein und dürfen nur alphanumerische Zeichen enthalten. Das erste Zeichen muss alphabetisch sein.

Korrekte Label-Definitionen umfassen folgende:

    Label0:
    A:
    LabelVeryNice:
    Example215:

Falsche Label-Definitionen sind:

    :
    __TEST:
    0Lol:
    Lol

Beachtt, dass derzeit keine Validierung der Label-Typen und ob der Label-Wert in die Anweisung passt, erfolgt. Fehler für Ersteres und Warnungen für Letzteres sind geplant.

# OPERATIONEN

Operationen sind Definitionen, die verwendet werden, um Anweisungen, Makros und Direktiven zu beschreiben. Die Argumente können Immediate, *Register* und*/oder * sein. Die Argumente sind durch Kommas (**,**) oder Kommas mit einem Leerzeichen (**,** ) getrennt. Der Operationsname und die Argumente sind durch ein oder mehrere Leerzeichen getrennt.

Siehe [ANWEISUNGEN][], [DIREKTIVEN][] und [MAKROS][] für Details.

# *Register*

Einige Anweisungen und Makros erfordern *Register*, um Aktionen auszuführen. Es gibt 31 *Register*, die verwendet werden können. *Register* können entweder durch die *Register*nummer mit einem vorangestellten x referenziert werden, was x0 bis x31 bedeutet, oder durch ihren ABI-Namen.

| *Register*            | ABI Name            | Description                                                                                          | Saver    |
|:-------------------:|:-------------------:|------------------------------------------------------------------------------------------------------|:--------:|
| x0                  | zero                | Unveränderbares *Register*, das immer Null ist.                                                        | —        |
| x1                  | ra                  | Rückgabeadresse                                                                                      | Callee   |
| x2                  | sp                  | Stack-Pointer                                                                                        | Callee   |
| x3                  | gp                  | Allzweck*Register*. Globales Zeiger gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet. | —        |
| x4                  | tp                  | Allzweck*Register*. Thread-Pointer gemäß RISC-V-Spezifikation, aber hier nicht als solches verwendet.  | —        |
| x5-x7               | t0-t2               | Temporäres *Register*.                                                                                 | Caller   |
| x8                  | s0/fp               | Gespeichertes *Register* oder Frame-Pointer.                                                           | Callee   |
| x9                  | s1                  | Gespeichertes *Register*.                                                                              | Callee   |
| x10-x11             | a0-a1               | *Register* für Rückgabewerte und Funktionsargumente.                                                   | Caller   |
| x12-x17             | a2-a7               | *Register* für Funktionsargumente.                                                                     | Caller   |
| x18-x27             | s2-s11              | Gespeichertes *Register*.                                                                              | Callee   |
| x28-x31             | t3-t6               | Temporäres *Register*.                                                                                 | Caller   |

# IMMEDIATES

Einige Anweisungen, Makros und Direktiven erfordern Immediates, um Aktionen auszuführen. Immediates sind entweder im Dezimal-, Binär- oder Hexadezimalformat. Standardmäßig wird das Dezimalformat angenommen. Um Immediates als binär oder hexadezimal zu interpretieren, muss das Präfix 0b bzw. 0x verwendet werden. Optional können binäre oder hexadezimale Immediates als vorzeichenbehaftete Zahlen interpretiert werden, indem das Immediates mit s oder S als Suffix versehen wird.

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

Für einige Direktiven ist das Argument ein String, der von Anführungszeichen begrenzt ist. Ansonsten gelten hier auch allgemeine Regeln.

Die Reihenfolge der Direktiven in der Assemblierdatei bestimmt die Reihenfolge der Daten im Speicher. Daten beginnen bei Adresse 0 und wachsen nach oben. Die erste Direktive wird an Adresse 0 geschrieben.

Derzeit werden die folgenden Direktiven unterstützt:

**.byte** *register*|*label*,[*register*|*label*]...

: Die *Register* und * werden als 8-Bit-Werte im Speicher gespeichert.

**.half** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 16-Bit-Werte im Speicher gespeichert.

**.word** *register*|*label*,[*register*|*label*]...

: Die *Register* und *Labels* werden als 32-Bit-Werte im Speicher gespeichert.

**.dword** *register*|*label*,[*register*|*label*]...

: Die *Register** und *Labels* werden als 64-Bit-Werte im Speicher gespeichert.

**.space** *dezimal*

: Reserviere Platz für Daten. Das *dezimal* gibt den reservierten Platz in Bytes an. Es muss eine Dezimalzahl sein und kann nicht negativ sein.

**.ascii "***string***"**

: Der *string* wird als aufeinanderfolgende 8-Bit-Werte gespeichert. Der *string* sollte nur ASCII-Zeichen enthalten. Alle Zeichen werden in ihren ASCII-Code übersetzt. Der *string* ist nicht nullterminiert.

**.asciz "***string***"**

: Wie **.ascii**, aber der *string* ist nullterminiert.

**.string "***string***"**

: Alias für **.asciz**.

**.eqv** *label*, *immediate*

: Der Wert des *label* ist *immediate*. Ein auf diese Weise emittiertes *label* ist eine Konstante, die nicht im Speicher geschrieben wird und wie ein Immediate verwendet werden kann.

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

Das erste *Register* für Makros, die solche haben, ist immer das *Register*, in das geschrieben wird.

Derzeit werden die folgenden Makros unterstützt:

**srr** *register*, *register*, *immediate*

: Shift right rotate. Dies ist als Unterprogramm implementiert, daher ist das Speichern von *Register*n erforderlich. Das Speichern von *Register*n erfolgt nicht automatisch!

**slr** *register*, *register*, *immediate*

: Shift left rotate. Dies ist als Unterprogramm implementiert, daher ist das Speichern von *Register*n erforderlich. Das Speichern von *Register*n erfolgt nicht automatisch!

**li** *register*, *immediate*|*label*

: Load *immediate*. *register* wird auf das *immediate* oder *label* gesetzt.

**la** *register*, *immediate*|*label*

: Load address. *register* wird entweder auf das *immediate* oder auf die Adresse des *label* gesetzt.

**call** *immediate*|*label*

: Springt zu einem weit entfernten *Label* und behandelt es als Unterprogramm. Die Rückgabeadresse wird in das *Register* **ra** geschrieben. Eine Rückkehr ist möglich, indem das Makro **ret** oder die entsprechende **jal**-Anweisung verwendet wird.

**tail** *immediate*|*label*

: Springt zu einem weit entfernten *Label*. Die Rückgabeadresse wird ungültig. Eine Rückkehr ist nicht möglich.

**push** *register*, [*register*]...

: Speichert den Inhalt dieser *Register* auf dem Stack. Die Initialisierung des Stack-Pointer-*Registers* **sp** ist erforderlich. Es können mehrere *Register* angegeben werden, um den Subtraktionsaufwand zu reduzieren. Die *Register* werden in der angegebenen Reihenfolge gespeichert. Das erste *Register* wird am unteren Ende, das letzte *Register* am oberen Ende des Stacks gespeichert.

**pop** *register*, [*register*]...

: Ladt den Inhalt des Stacks in die *Register*. Die Initialisierung des Stack-Pointer-*Registers* **sp** ist erforderlich. Es können mehrere *Register* angegeben werden, um den Additionsaufwand zu reduzieren. Der Inhalt wird in den *Register*n in der angegebenen Reihenfolge geladen. Das erste *Register* erhält den Inhalt des obersten Stacks, das letzte *Register* des untersten.

**rep** *decimal*, *instruction*|*macro*

: Wiederholt die *instruction* oder *macro* *decimal* Mal. Die Dezimalzahl muss positiv und größer als 0 sein. Wiederholungen können nicht verschachtelt werden, d. h. eine Wiederholung kann keine weitere Wiederholung enthalten.

**mv** *register*, *register*

: Kopiert den Inhalt des letzten *Registers* in das erste *Register*. Dies wird entweder zur Anweisung **addi** oder **add** abgebildet.

**nop**

: Keine Operation. Es bewirkt nichts. Dies wird entweder zur Anweisung **addi zero, zero, 0** oder **add zero, zero, zero** abgebildet.

**ret**

: Wird verwendet, um aus einem Unterprogramm zurückzukehren. Dies wird zur Anweisung **jalr zero, ra, 0** abgebildet.

**j** *immediate*|*label*

: Springt zum *label* oder *immediate*. Dies wird zur Anweisung **jal zero,** *offset* abgebildet.

**jal** *immediate*|*label*

: Springt und verknüpft mit dem *label* oder *immediate*. Dies wird zur Anweisung **jal ra,** *offset* abgebildet.

**jr** *register*

: Springt zur Adresse im *register*. Dies wird zur Anweisung **jalr zero,** *register***, 0** abgebildet.

**jalr** *register*

: Springt und verknüpft mit der Adresse im *Register*. Dies wird zur Anweisung **jalr ra,** *register***, 0** abgebildet.

Siehe [RISC-V Shortened Spec][] für Details.


# ANWEISUNGEN

Eine Anweisung ist Maschinencode in menschenlesbarer Form. Die Syntax ähnelt der von [MAKROS][]. Alle Anweisungen der RV32I- und RV32M-Erweiterungen werden unterstützt.

Das erste *Register* für Anweisungen, die sie benötigen, ist immer das *Register*, in das geschrieben wird. Eine Einschränkung dieser Regel sind Sprunganweisungen.

Diese Anweisungen werden verwendet, um arithmetische, logische und Verschiebeoperationen mit *Register*n durchzuführen:

**add** *register*, *register*, *register*

: Addition der beiden letzten *Register*.

**sub** *register*, *register*, *register*

: Subtraktion. Der Minuend ist der Inhalt des zweiten *Registers*, der Subtrahend ist der Inhalt des letzten *Registers*.

**xor** *register*, *register*, *register*

: Logisches bitweises exklusives Oder des zweiten und dritten *Registers*.

**or** *register*, *register*, *register*

: Logisches bitweises Oder des zweiten und dritten *Registers*.

**and** *register*, *register*, *register*

: Logisches bitweises Oder des zweiten und dritten *Registers*.

**sll** *register*, *register*, *register*

: Logische Linksverschiebung des zweiten *Registers* um das dritte *Register*.

**srl** *register*, *register*, *register*

: Logische Rechtsverschiebung des zweiten *Registers* um das dritte *Register*.

**sra** *register*, *register*, *register*

: Arithmetische Rechtsverschiebung des zweiten *Registers* um das dritte *Register*.

**slt** *register*, *register*, *register*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist.

**sltu** *register*, *register*, *register*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das letzte *Register* ist. Der Inhalt der verglichenen *Register* wird als vorzeichenlose Zahlen interpretiert.

**xnor** *register*, *register*, *register*

: Logisches bitweises negiertes exklusives Oder des zweiten und dritten *Registers*. Beachtt, dass dies nicht in der RISC-V-Standarddefiniert ist.

**equal** *register*, *register*, *register*

: Vergleicht die zweiten und dritten *Register* und setzt das erste *Register* auf eins (1), wenn sie gleich sind. Beachtt, dass dies nicht in der RISC-V-Standarddefiniert ist.

**mul** *register*, *register*, *register*

: Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die unteren 32 Bits des Ergebnisses gesetzt.

**mulh** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die höheren 32 Bits des Ergebnisses gesetzt. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**mulhu** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die höheren 32 Bits des Ergebnisses gesetzt. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

**mulhsu** *register*, *register*, *register*

: Hohe Multiplikation des zweiten und dritten *Registers*. Das erste *Register* wird auf die höheren 32 Bits des Ergebnisses gesetzt. Der Inhalt des zweiten *Registers* wird als vorzeichenbehaftete Zahl interpretiert, der Inhalt des dritten *Registers* als vorzeichenlose Zahl.

**div** *register*, *register*, *register*

: Division des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**divu** *register*, *register*, *register*

: Division des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

**rem** *register*, *register*, *register*

: Modulo-Operation des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenbehaftete Zahlen interpretiert.

**remu** *register*, *register*, *register*

: Modulo-Operation des zweiten und dritten *Registers*. Das zweite *Register* ist der Dividend und das dritte *Register* ist der Divisor. Der Inhalt beider *Register* wird als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um arithmetische, logische und Verschiebeoperationen mit *Immediate* Werten durchzuführen:

Verschiebeoperationen können nur *Immediate* Werte von 4 Bits verwenden.

Beachtt, dass einige Anweisungen *keine * verwenden können. Dies ist in Arbeit.

**addi** *register*, *register*, *immediate*|*label*

: Addition des zweiten *Registers* und des *Immediate** oder *.

**xori** *register*, *register*, *immediate*

: Logisches bitweises exklusives Oder des zweiten *Registers* und des *Immediate*.

**ori** *register*, *register*, *immediate*

: Logisches bitweises Oder des zweiten *Registers* und des *Immediate*.

**andi** *register*, *register*, *immediate*

: Logisches bitweises Oder des zweiten *Registers* und des *Immediate*.

**slli** *register*, *register*, *immediate*|*label*

: Logische Linksverschiebung des zweiten *Registers* um das *Immediate* oder *Label*.

**srli** *register*, *register*, *immediate*|*label*

: Logische Rechtsverschiebung des zweiten *Registers* um das *Immediate* oder *Label*.

**srai** *register*, *register*, *immediate*|*label*

: Arithmetische Rechtsverschiebung des zweiten *Registers* um das *Immediate* oder *Label*.

**slti** *register*, *register*, *immediate*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das *Immediate* ist.

**sltiu** *register*, *register*, *immediate*

: Das erste *Register* wird auf eins (1) gesetzt, wenn das zweite *Register* kleiner als das *Immediate* ist. Der Inhalt der verglichenen *Register* wird als vorzeichenlose Zahlen interpretiert.

Diese Anweisungen werden verwendet, um den Speicherinhalt zu manipulieren:

Das Zielbyte und die Hälfte sind immer die LSBs des Zielregisters.

**lb** *register*, *register*, *immediate*|*label*

: Lädt ein Byte aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

**lh** *register*, *register*, *immediate*|*label*

: Lädt eine Hälfte (16 Bit) aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

**lw** *register*, *register*, *immediate*|*label*

: Lädt ein Wort (32 Bit) aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

**lbu** *register*, *register*, *immediate*|*label*

: Lädt ein Byte aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist. Das Byte wird auf 32 Bits erweitert.

**lhu** *register*, *register*, *immediate*|*label*

: Lädt eine Hälfte aus dem Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist. Die Hälfte wird auf 32 Bits erweitert.

**sb** *register*, *register*, *immediate*|*label*

: Speichert ein Byte in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

**sh** *register*, *register*, *immediate*|*label*

: Speichert ein halbes Wort in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

**sw** *register*, *register*, *immediate*|*label*

: Speichert ein Wort in den Speicher an der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist.

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

: "Jump and link" zu der Adresse, die die Summe des Programmzählers und des *Immediate** oder *Labels* ist. Die Rückkehradresse wird in das *Register* geschrieben.

**jalr** *register*, *register*, *immediate*|*label*

: "Jump and link" zu der Adresse, die die Summe des zweiten *Registers* und des *Immediate* oder *Labels* ist. Die Rückkehradresse wird in das *Register* geschrieben.

Diese Anweisungen können nicht kategorisiert werden:

**lui** *register*, *immediate*|*label*

: Lädt das obere *Immediate*. Die oberen 20 Bits des *Registers* werden auf das *Immediate** oder *Label* gesetzt. Die unteren 12 Bits sind null.

**auipc** *register*, *immediate*|*label*

: Addiert das obere *Immediate* auf den Programmzähler. Die oberen 20 Bits des *Immediate* oder *Label* werden zum Programmzähler addiert und das Ergebnis wird in das *Register* geschrieben.

Siehe [RISC-V Shortened Spec][] für weitere Details.
# SIEHE AUCH

[assembler(1)], [RISC-V Shortened Spec][]

[assembler(1)]: assembler.1.md
[RISC-V Shortened Spec]: https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf
[MACROS]: #macros
[LABEL DEFINITIONS]: #label-definitions
[OPERATIONS]: #operations
[DIRECTIVES]: #directives
[INSTRUCTIONS]: #instructions
