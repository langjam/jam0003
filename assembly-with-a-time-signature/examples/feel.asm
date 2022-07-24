.song
lh: define piano
rh: define piano

minor:
    mov %r0 3
    call play_minor_chord
    beat
    beat
    beat
    ret

major:
    mov %r0 3
    call play_major_chord
    beat
    beat
    beat
    ret

intro_loop:
    lir lh
    push %oc
    dec %oc
    play-for #d 16
    dec %oc
    play-for #d 16
    pop %oc
    beat
    

    lir rh
    beat
    mov %n0 #d
    call minor
    call minor
    call minor
    mov %n0 #a
    call minor
    call minor

    lir lh
    push %oc
    dec %oc
    play-for #d 16
    dec %oc
    play-for #d 16
    pop %oc
    beat
    
    lir rh
    beat
    mov %n0 #f
    call major
    call major
    call major

    mov %n0 #g
    call major
    call major
    ret

main:
    bpm 380
    time 4 4
    
    call intro_loop
    call intro_loop


    stop

