.instrument
p1: define piano
p2: define piano
harmonica: define harmonica
guitar: define steelguitar

.song
c_chord:
    push %n0
    push %r0
    mov %n0 #c
    call play_major_chord
    pop %r0
    pop %n0
    ret

g_chord:
    push %n0
    push %r0
    mov %n0 #g
    call play_major_chord
    pop %r0
    pop %n0
    ret

f_chord:
    push %n0
    push %r0
    mov %n0 #f
    call play_major_chord
    pop %r0
    pop %n0
    ret

d7_chord:
    play-for #d %r0
    sharp #f
    play-for #f %r0
    flat #f
    play-for #a %r0
    play-for #c %r0
    ret

guitar1:
    call c_chord
    call bt
    call bt
    call bt

    call g_chord
    call bt
    call bt
    call bt

    call f_chord
    call bt
    call bt
    call bt

    call c_chord
    call bt
    call bt
    call bt
    ret

guitar2:
    call f_chord
    call bt
    call bt
    call bt

    call c_chord
    call bt
    call bt

    call d7_chord
    call bt
    call bt
    call bt

    call g_chord
    call bt
    call bt
    call bt
    ret

guitar3:
    call f_chord
    call bt
    call bt
    call bt

    call g_chord
    call bt
    call bt

    ;call c_chord
    call bt
    call bt
    call bt

    ;call g_chord
    call bt
    call bt
    call bt
    ret

guitar_part:
    lir guitar
    mov %r0 6
    beat

    call guitar1
    call guitar2
    call guitar1
    call guitar3

    exit

piano1:
    play-for #c 2
    call bt
    call c_chord
    call bt
    call c_chord
    call bt

    play-for #b 2
    call bt
    call g_chord
    call bt
    call g_chord
    call bt

    play-for #a 2
    call bt
    call f_chord
    call bt
    call f_chord
    call bt


    play-for #g 2
    call bt
    call c_chord
    call bt
    call c_chord
    call bt
    ret

piano2:
    play-for #f 2
    call bt
    call f_chord
    call bt
    call f_chord
    call bt

    play-for #e 2
    call bt
    call c_chord
    call bt
    call c_chord
    call bt

    play-for #g 2
    call bt
    call d7_chord
    call bt
    call d7_chord
    call bt

    play-for #g 2
    call bt
    call g_chord
    call bt
    call g_chord
    call bt
    ret

piano3:
    lir p1
    play-for #f 3
    lir p2
    call bt
    call f_chord
    call bt
    call f_chord
    call bt

    lir p1
    play-for #g 3
    lir p2
    call bt
    call g_chord
    call bt
    call g_chord
    call bt
    ret

piano4:
    lir p1
    play-for #c 3
    lir p2
    call c_chord
    call bt

    play-for #c 1
    beat
    play-for #e 1
    beat
    play-for #g 1
    beat
    inc %oc
    play-for #c 1
    dec %oc
    beat

    lir p1
    inc %oc
    play-for #g 3
    dec %oc
    lir p2
    play-for #c 2
    play-for #f 2
    inc %oc
    play-for #c 2
    dec %oc

    call bt

    play #c
    beat
    play #f
    beat
    inc %oc
    play #c
    dec %oc
    beat
    play #g
    beat


    lir p1
    inc %oc
    play-for #g 3
    dec %oc
    lir p2
    play-for #c 2
    play-for #g 2
    inc %oc
    play-for #c 2
    dec %oc

    call bt

    play #c
    beat
    play #f
    beat
    inc %oc
    play #c
    dec %oc
    beat
    play #g
    beat

    lir p1
    inc %oc
    play-for #c 2
    dec %oc
    play-for #f 2
    lir p2
    dec %oc
    play-for #f 2
    call bt
    inc %oc

    lir p1
    inc %oc
    play-for #c 2
    dec %oc
    play-for #e 2
    lir p2
    dec %oc
    play-for #e 2
    call bt
    inc %oc

    lir p1
    inc %oc
    play-for #c 2
    dec %oc
    play-for #d 2
    lir p2
    dec %oc
    play-for #d 2
    call bt
    inc %oc
    ret

piano_part:
    lir p1
    mov %r0 2

    call piano1
    call piano2
    call piano1
    call piano3
    call piano4
    call piano4
    ret

bt:
    beat
    beat
    ret

main:
    bpm 356
    time 6 8

    go guitar_part
    call piano_part
    stop
