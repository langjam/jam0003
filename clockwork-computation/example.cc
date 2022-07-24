component not {  // define a resuable component not
    input x: rod {  // define x, a rod that is an input
        x -> my_gear {
            gearOffset: 0 // attach x to the first tooth of my_gear
        }
    }

    my_gear: gear { // define my_gear, a gear with two teeth, i.e a spinning rod.
        teeth: 2
        my_gear -> y {
            gearOffset: 1 // attach y to the second tooth of the gear
        }
    }

    output y: rod {} // define y, an output rod
}

component or {
    input x: rod {
        x -> joiner {
            rodAttachment: push // If x is pushed, it pushes joiner, but if x is pulled, joiner remains where it is
        }
    }

    input y: rod {
        y -> joiner {
            rodAttachment: push
        }
    }

    output joiner: rod {
        spring: pull // If nothing else pushes or pulls on joiner, a spring pulls it to the pull state. 
    }
}

component nor {
    input x: rod {}
    input y: rod {}
    
    use or(x, y) -> p
    p: rod {}
    use not(p) -> q

    output q: rod {}
}

component and {
    input x: rod {}
    input y: rod {}

    use not(x) -> p
    use not(y) -> q

    p: rod{}
    q: rod{}

    use nor(p, q) -> z

    output z: rod {}
}

component xor {
    input x: rod{}
    input y: rod{}

    use and(x, y) -> p
    use not(p) -> q
    use or(x, y) -> r

    p: rod{}
    q: rod{}
    r: rod{}

    use and(q, r) -> z
    output z: rod{}
}

component half_adder {
    input x: rod{}
    input y: rod{}
    
    use and(x, y) -> p
    use xor(x, y) -> q

    output p: rod{}
    output q: rod{}
}

component full_adder {
    input cin: rod{}
    input b: rod{}
    input a: rod{}
    
    use half_adder(a, b) -> p, q
    use half_adder(q, cin) -> r, sum
    use xor(p, r) -> cout

    p: rod{}
    q: rod{}
    r: rod{}
    s: rod{}
    
    output cout: rod{}
    output sum: rod{}
}

component eight_bit_adder {
    input a7: rod{}
    input a6: rod{}
    input a5: rod{}
    input a4: rod{}
    input a3: rod{}
    input a2: rod{}
    input a1: rod{}
    input a0: rod{}

    input b7: rod{}
    input b6: rod{}
    input b5: rod{}
    input b4: rod{}
    input b3: rod{}
    input b2: rod{}
    input b1: rod{}
    input b0: rod{}
    input cin: rod{}

    use full_adder(cin, a0, b0) -> c0, sum0
    use full_adder(c0, a1, b1) -> c1, sum1
    use full_adder(c1, a2, b2) -> c2, sum2
    use full_adder(c2, a3, b3) -> c3, sum3
    use full_adder(c3, a4, b4) -> c4, sum4
    use full_adder(c4, a5, b5) -> c5, sum5
    use full_adder(c5, a6, b6) -> c6, sum6
    use full_adder(c6, a7, b7) -> cout, sum7
    
    c0: rod{}
    c1: rod{}
    c2: rod{}
    c3: rod{}
    c4: rod{}
    c5: rod{}
    c6: rod{}

    output cout: rod{}
    output sum7: rod{}
    output sum6: rod{}
    output sum5: rod{}
    output sum4: rod{}
    output sum3: rod{}
    output sum2: rod{}
    output sum1: rod{}
    output sum0: rod{}
}

component analog_eight_bit_adder {
    input a: gear {
        teeth: 256
        a -> main_rod {}
    }

    input b: gear {
        teeth: 256
        b -> main_rod {}
    }

    main_rod: gear {
        teeth: 512
        main_rod -> sum {}
    }

    output sum: gear {
        teeth: 512
    }
}

component four_bit_digital_to_analog {
    input a3: rod {
        a3 -> a3_gear{
            gearOffset: 0
        }
    }
    input a2: rod {
        a2 -> a2_gear{
            gearOffset: 0
        }
    }
    input a1: rod {
        a1 -> a1_gear{
            gearOffset: 0
        }
    }
    input a0: rod {
        a0 -> a0_gear {
            gearOffset: 0
        }
    }

    a0_gear: gear {
        teeth: 2
        a0_gear -> main_rod{}
    }
    a1_gear: gear {
        teeth: 4
        a1_gear -> main_rod{}
    }
    a2_gear: gear {
        teeth: 8
        a2_gear -> main_rod{}
    }
    a3_gear: gear {
        teeth: 16
        a3_gear -> main_rod{}
    }

    main_rod: gear {
        teeth: 32
        main_rod -> final {}
    }

    output final: gear {
        teeth: 32
    }
}

component four_bit_analog_to_digital {
    input in: gear {
        teeth: 32
        in -> main_rod {}
    }

    main_rod: gear {
        teeth: 32
        main_rod -> a3_gear {}
        main_rod -> a2_gear {}
        main_rod -> a1_gear {}
        main_rod -> a0_gear {}
    }

    a0_gear: gear {
        teeth: 2
        a0_gear -> a0 {
            gearOffset: 0
        }
    }

    a1_gear: gear {
        teeth: 4
        a1_gear -> a1 {
            gearOffset: 0
        }
    }

    a2_gear: gear {
        teeth: 8
        a2_gear -> a2 {
            gearOffset: 0
        }
    }

    a3_gear: gear {
        teeth: 16
        a3_gear -> a3 {
            gearOffset: 0
        }
    }

    output a0: rod {}
    output a1: rod {}
    output a2: rod {}
    output a3: rod {}
}