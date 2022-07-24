
.instrument
p1: define piano

.song

all_chords:
    mov %n0 #a
    call %r1
    beat
    mov %n0 #b
    call %r1
    beat
    mov %n0 #c
    call %r1
    beat
    mov %n0 #d
    call %r1
    beat
    mov %n0 #e
    call %r1
    beat
    mov %n0 #f
    call %r1
    beat
    mov %n0 #g
    call %r1
    beat
    ret

main:
    lir p1

    mov %r1 play_major_chord
    call all_chords
    mov %r1 play_minor_chord
    call all_chords
    stop
