
.instrument
p1: define piano
p2: define flute
p3: define piano

.song

verse:
    play #a
    beat
    play #b
    beat
    play #c
    beat
    play #d
    beat
    play #e
    beat
    play #f

    ret

chorus:
    ret

main:
    bpm 120
    time 3 4

    lir p1
    call verse

    ; from here, all as played are sharp by 1
    sharp #a

    call chorus

    ; from here, all as played are flat by 1.
    ; However, they were already sharp so this
    ; effectively restores the #a
    flat #a

    call verse
    call verse
    call chorus

    stop
