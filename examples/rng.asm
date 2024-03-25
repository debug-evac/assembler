; Pseudo random number generator (Pseudo-Zufallszahlgenerator)
; Dieses Programm erzeugt pseudo Zufallszahlen mithilfe des moderneren
; xoroshiro128++ Algorithmus. Der Anfangszustand muss gesetzt werden.
; Der Anfangszustand sind vier 32 Bit Zahlen.
; Derzeitig werden keine Macros des Assemblers verwendet, da diese nicht
; zuverlässig getestet werden können (in Arbeit).
.data
.RNGstate:
    .word   16777216, 33554432, 50331648, 67108864
.text
    call    RNGNext
    call    RNGNext
    call    RNGNext
    call    RNGNext
LOD:
    j       LOD
RNGNext:
    addi    a1, zero, 7
    la	    a7, .RNGstate
    lw	    a0, (a7)
    lw      a2, 12(a7)
    add	    a0, a0, a2
    addi    sp, sp, 8
    sw	    ra, -4(sp)
    sw	    a7, (sp)
    call    .SLR
    lw	    a7, (sp)		; peek top of stack
    lw	    t0, (a7)
    add     a0, a0, t0		; result = rotl(s[0] + s[3], 7) + s[0];
    lw	    t1, 4(a7)
    slli    t5, t1, 9		; t = s[1] << 9;
    lw	    t2, 8(a7)
    lw	    t3, 12(a7)
    xor	    t2, t2, t0		; s[2] ^= s[0];
    xor     t3, t3, t1		; s[3] ^= s[1];
    xor	    t1, t1, t2		; s[1] ^= s[2];
    xor	    t0, t0, t3		; s[0] ^= s[3];
    xor	    t2, t2, t5		; s[2] ^= t;
    sw	    t0, (a7)
    sw	    t1, 4(a7)
    sw	    t2, 8(a7)
    addi    sp, sp, 4
    sw	    a0, (sp)		; store result on top of stack
    mv	    a0, t3
    addi    a1, zero, 11
    call    .SLR            ; s[3] = rotl(s[3], 11);
    mv	    t3, a0
    addi    sp, sp, -12		; load result, address of data and return address from stack
    lw	    a0, 12(sp)
    lw	    a7, 8(sp)
    lw	    ra, 4(sp)
    sw	    t3, 12(a7)
    ret
.SLR:
    addi    a4, zero, 32
    sub	    a4, a4, a1
    sll     a2, a0, a1
    srl     a3, a0, a4
    or      a0, a2, a3
    ret
