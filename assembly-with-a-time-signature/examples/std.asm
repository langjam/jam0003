
.text
; Resets all accidentals to 0
reset_accidental:
    mov %a#a 0
    mov %a#b 0
    mov %a#c 0
    mov %a#d 0
    mov %a#e 0
    mov %a#f 0
    mov %a#g 0
    ret

; make a note in register %n0 sharp
sharp_note:
    je %n0 #a sn_a
    je %n0 #b sn_b
    je %n0 #c sn_c
    je %n0 #d sn_d
    je %n0 #e sn_e
    je %n0 #f sn_f
    je %n0 #g sn_g
    ret
    sn_a:
        sharp #a
        ret
    sn_b:
        sharp #b
        ret
    sn_c:
        sharp #c
        ret
    sn_d:
        sharp #d
        ret
    sn_e:
        sharp #e
        ret
    sn_f:
        sharp #f
        ret
    sn_g:
        sharp #g
        ret

; make a note in register %n0 flat
flat_note:
    je %n0 #a fn_a
    je %n0 #b fn_b
    je %n0 #c fn_c
    je %n0 #d fn_d
    je %n0 #e fn_e
    je %n0 #f fn_f
    je %n0 #g fn_g
    ret
    fn_a:
        flat #a
        ret
    fn_b:
        flat #b
        ret
    fn_c:
        flat #c
        ret
    fn_d:
        flat #d
        ret
    fn_e:
        flat #e
        ret
    fn_f:
        flat #f
        ret
    fn_g:
        flat #g
        ret

; make a major chord based on the note in %n0
; The value in %r0 dictates the length of the chord. If
; %r0 is zero, 1 is used as the length
play_major_chord:
    jg %r0 0 maj_c_not_null
    mov %r0 1
maj_c_not_null:
    play-for %n0 %r0
    call sharp_note
    call sharp_note
    call sharp_note
    call sharp_note
    play-for %n0 %r0
    call sharp_note
    call sharp_note
    call sharp_note
    play-for %n0 %r0
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    ret


; make a minor chord based on the note in %n0
; The value in %r0 dictates the length of the chord. If
; %r0 is zero, 1 is used as the length
play_minor_chord:
    jg %r0 0 min_c_not_null
    mov %r0 1
min_c_not_null:
    play-for %n0 %r0
    call sharp_note
    call sharp_note
    call sharp_note
    play-for %n0 %r0
    call sharp_note
    call sharp_note
    call sharp_note
    call sharp_note
    play-for %n0 %r0
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    call flat_note
    ret
