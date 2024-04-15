//! Generated by Snowball 2.2.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::Among;
use crate::snowball::SnowballEnv;

static A_0: &'static [Among<Context>; 3] = &[
    Among("arsen", -1, -1, None),
    Among("commun", -1, -1, None),
    Among("gener", -1, -1, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("'", -1, 1, None),
    Among("'s'", 0, 1, None),
    Among("'s", -1, 1, None),
];

static A_2: &'static [Among<Context>; 6] = &[
    Among("ied", -1, 2, None),
    Among("s", -1, 3, None),
    Among("ies", 1, 2, None),
    Among("sses", 1, 1, None),
    Among("ss", 1, -1, None),
    Among("us", 1, -1, None),
];

static A_3: &'static [Among<Context>; 13] = &[
    Among("", -1, 3, None),
    Among("bb", 0, 2, None),
    Among("dd", 0, 2, None),
    Among("ff", 0, 2, None),
    Among("gg", 0, 2, None),
    Among("bl", 0, 1, None),
    Among("mm", 0, 2, None),
    Among("nn", 0, 2, None),
    Among("pp", 0, 2, None),
    Among("rr", 0, 2, None),
    Among("at", 0, 1, None),
    Among("tt", 0, 2, None),
    Among("iz", 0, 1, None),
];

static A_4: &'static [Among<Context>; 6] = &[
    Among("ed", -1, 2, None),
    Among("eed", 0, 1, None),
    Among("ing", -1, 2, None),
    Among("edly", -1, 2, None),
    Among("eedly", 3, 1, None),
    Among("ingly", -1, 2, None),
];

static A_5: &'static [Among<Context>; 24] = &[
    Among("anci", -1, 3, None),
    Among("enci", -1, 2, None),
    Among("ogi", -1, 13, None),
    Among("li", -1, 15, None),
    Among("bli", 3, 12, None),
    Among("abli", 4, 4, None),
    Among("alli", 3, 8, None),
    Among("fulli", 3, 9, None),
    Among("lessli", 3, 14, None),
    Among("ousli", 3, 10, None),
    Among("entli", 3, 5, None),
    Among("aliti", -1, 8, None),
    Among("biliti", -1, 12, None),
    Among("iviti", -1, 11, None),
    Among("tional", -1, 1, None),
    Among("ational", 14, 7, None),
    Among("alism", -1, 8, None),
    Among("ation", -1, 7, None),
    Among("ization", 17, 6, None),
    Among("izer", -1, 6, None),
    Among("ator", -1, 7, None),
    Among("iveness", -1, 11, None),
    Among("fulness", -1, 9, None),
    Among("ousness", -1, 10, None),
];

static A_6: &'static [Among<Context>; 9] = &[
    Among("icate", -1, 4, None),
    Among("ative", -1, 6, None),
    Among("alize", -1, 3, None),
    Among("iciti", -1, 4, None),
    Among("ical", -1, 4, None),
    Among("tional", -1, 1, None),
    Among("ational", 5, 2, None),
    Among("ful", -1, 5, None),
    Among("ness", -1, 5, None),
];

static A_7: &'static [Among<Context>; 18] = &[
    Among("ic", -1, 1, None),
    Among("ance", -1, 1, None),
    Among("ence", -1, 1, None),
    Among("able", -1, 1, None),
    Among("ible", -1, 1, None),
    Among("ate", -1, 1, None),
    Among("ive", -1, 1, None),
    Among("ize", -1, 1, None),
    Among("iti", -1, 1, None),
    Among("al", -1, 1, None),
    Among("ism", -1, 1, None),
    Among("ion", -1, 2, None),
    Among("er", -1, 1, None),
    Among("ous", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ent", -1, 1, None),
    Among("ment", 15, 1, None),
    Among("ement", 16, 1, None),
];

static A_8: &'static [Among<Context>; 2] = &[Among("e", -1, 1, None), Among("l", -1, 2, None)];

static A_9: &'static [Among<Context>; 8] = &[
    Among("succeed", -1, -1, None),
    Among("proceed", -1, -1, None),
    Among("exceed", -1, -1, None),
    Among("canning", -1, -1, None),
    Among("inning", -1, -1, None),
    Among("earring", -1, -1, None),
    Among("herring", -1, -1, None),
    Among("outing", -1, -1, None),
];

static A_10: &'static [Among<Context>; 18] = &[
    Among("andes", -1, -1, None),
    Among("atlas", -1, -1, None),
    Among("bias", -1, -1, None),
    Among("cosmos", -1, -1, None),
    Among("dying", -1, 3, None),
    Among("early", -1, 9, None),
    Among("gently", -1, 7, None),
    Among("howe", -1, -1, None),
    Among("idly", -1, 6, None),
    Among("lying", -1, 4, None),
    Among("news", -1, -1, None),
    Among("only", -1, 10, None),
    Among("singly", -1, 11, None),
    Among("skies", -1, 2, None),
    Among("skis", -1, 1, None),
    Among("sky", -1, -1, None),
    Among("tying", -1, 5, None),
    Among("ugly", -1, 8, None),
];

static G_aeo: &'static [u8; 2] = &[17, 64];

static G_v: &'static [u8; 4] = &[17, 65, 16, 1];

static G_v_WXY: &'static [u8; 5] = &[1, 17, 65, 208, 1];

static G_valid_LI: &'static [u8; 3] = &[55, 141, 2];

#[derive(Clone)]
struct Context {
    b_Y_found: bool,
    i_p2: i32,
    i_p1: i32,
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.b_Y_found = false;
    let v_1 = env.cursor;
    'lab0: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"'") {
            break 'lab0;
        }
        env.ket = env.cursor;
        if !env.slice_del() {
            return false;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    'lab1: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"y") {
            break 'lab1;
        }
        env.ket = env.cursor;
        if !env.slice_from("Y") {
            return false;
        }
        context.b_Y_found = true;
        break 'lab1;
    }
    env.cursor = v_2;
    let v_3 = env.cursor;
    'lab2: loop {
        'replab3: loop {
            let v_4 = env.cursor;
            'lab4: for _ in 0..1 {
                'golab5: loop {
                    let v_5 = env.cursor;
                    'lab6: loop {
                        if !env.in_grouping(G_v, 97, 121) {
                            break 'lab6;
                        }
                        env.bra = env.cursor;
                        if !env.eq_s(&"y") {
                            break 'lab6;
                        }
                        env.ket = env.cursor;
                        env.cursor = v_5;
                        break 'golab5;
                    }
                    env.cursor = v_5;
                    if env.cursor >= env.limit {
                        break 'lab4;
                    }
                    env.next_char();
                }
                if !env.slice_from("Y") {
                    return false;
                }
                context.b_Y_found = true;
                continue 'replab3;
            }
            env.cursor = v_4;
            break 'replab3;
        }
        break 'lab2;
    }
    env.cursor = v_3;
    return true;
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if env.find_among(A_0, context) == 0 {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            'golab3: loop {
                'lab4: loop {
                    if !env.in_grouping(G_v, 97, 121) {
                        break 'lab4;
                    }
                    break 'golab3;
                }
                if env.cursor >= env.limit {
                    break 'lab0;
                }
                env.next_char();
            }
            'golab5: loop {
                'lab6: loop {
                    if !env.out_grouping(G_v, 97, 121) {
                        break 'lab6;
                    }
                    break 'golab5;
                }
                if env.cursor >= env.limit {
                    break 'lab0;
                }
                env.next_char();
            }
            break 'lab1;
        }
        context.i_p1 = env.cursor;
        'golab7: loop {
            'lab8: loop {
                if !env.in_grouping(G_v, 97, 121) {
                    break 'lab8;
                }
                break 'golab7;
            }
            if env.cursor >= env.limit {
                break 'lab0;
            }
            env.next_char();
        }
        'golab9: loop {
            'lab10: loop {
                if !env.out_grouping(G_v, 97, 121) {
                    break 'lab10;
                }
                break 'golab9;
            }
            if env.cursor >= env.limit {
                break 'lab0;
            }
            env.next_char();
        }
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true;
}

fn r_shortv(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.out_grouping_b(G_v_WXY, 89, 121) {
                break 'lab1;
            }
            if !env.in_grouping_b(G_v, 97, 121) {
                break 'lab1;
            }
            if !env.out_grouping_b(G_v, 97, 121) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !env.out_grouping_b(G_v, 97, 121) {
            return false;
        }
        if !env.in_grouping_b(G_v, 97, 121) {
            return false;
        }
        if env.cursor > env.limit_backward {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor;
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor;
}

fn r_Step_1a(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if env.find_among_b(A_1, context) == 0 {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        if !env.slice_del() {
            return false;
        }
        break 'lab0;
    }
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("ss") {
                return false;
            }
        }
        2 => 'lab1: loop {
            let v_2 = env.limit - env.cursor;
            'lab2: loop {
                if !env.hop_back(2) {
                    break 'lab2;
                }
                if !env.slice_from("i") {
                    return false;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_2;
            if !env.slice_from("ie") {
                return false;
            }
            break 'lab1;
        },
        3 => {
            if env.cursor <= env.limit_backward {
                return false;
            }
            env.previous_char();
            'golab3: loop {
                'lab4: loop {
                    if !env.in_grouping_b(G_v, 97, 121) {
                        break 'lab4;
                    }
                    break 'golab3;
                }
                if env.cursor <= env.limit_backward {
                    return false;
                }
                env.previous_char();
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_Step_1b(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("ee") {
                return false;
            }
        }
        2 => {
            let v_1 = env.limit - env.cursor;
            'golab0: loop {
                'lab1: loop {
                    if !env.in_grouping_b(G_v, 97, 121) {
                        break 'lab1;
                    }
                    break 'golab0;
                }
                if env.cursor <= env.limit_backward {
                    return false;
                }
                env.previous_char();
            }
            env.cursor = env.limit - v_1;
            if !env.slice_del() {
                return false;
            }
            env.ket = env.cursor;
            env.bra = env.cursor;
            let v_3 = env.limit - env.cursor;
            among_var = env.find_among_b(A_3, context);
            match among_var {
                1 => {
                    if !env.slice_from("e") {
                        return false;
                    }
                    return false;
                }
                2 => {
                    let v_4 = env.limit - env.cursor;
                    'lab2: loop {
                        if !env.in_grouping_b(G_aeo, 97, 111) {
                            break 'lab2;
                        }
                        if env.cursor > env.limit_backward {
                            break 'lab2;
                        }
                        return false;
                    }
                    env.cursor = env.limit - v_4;
                }
                3 => {
                    if env.cursor != context.i_p1 {
                        return false;
                    }
                    let v_5 = env.limit - env.cursor;
                    if !r_shortv(env, context) {
                        return false;
                    }
                    env.cursor = env.limit - v_5;
                    if !env.slice_from("e") {
                        return false;
                    }
                    return false;
                }
                _ => (),
            }
            env.cursor = env.limit - v_3;
            env.ket = env.cursor;
            if env.cursor <= env.limit_backward {
                return false;
            }
            env.previous_char();
            env.bra = env.cursor;
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_Step_1c(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"y") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !env.eq_s_b(&"Y") {
            return false;
        }
        break 'lab0;
    }
    env.bra = env.cursor;
    if !env.out_grouping_b(G_v, 97, 121) {
        return false;
    }
    'lab2: loop {
        if env.cursor > env.limit_backward {
            break 'lab2;
        }
        return false;
    }
    if !env.slice_from("i") {
        return false;
    }
    return true;
}

fn r_Step_2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_5, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            if !env.slice_from("tion") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("ence") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("ance") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("able") {
                return false;
            }
        }
        5 => {
            if !env.slice_from("ent") {
                return false;
            }
        }
        6 => {
            if !env.slice_from("ize") {
                return false;
            }
        }
        7 => {
            if !env.slice_from("ate") {
                return false;
            }
        }
        8 => {
            if !env.slice_from("al") {
                return false;
            }
        }
        9 => {
            if !env.slice_from("ful") {
                return false;
            }
        }
        10 => {
            if !env.slice_from("ous") {
                return false;
            }
        }
        11 => {
            if !env.slice_from("ive") {
                return false;
            }
        }
        12 => {
            if !env.slice_from("ble") {
                return false;
            }
        }
        13 => {
            if !env.eq_s_b(&"l") {
                return false;
            }
            if !env.slice_from("og") {
                return false;
            }
        }
        14 => {
            if !env.slice_from("less") {
                return false;
            }
        }
        15 => {
            if !env.in_grouping_b(G_valid_LI, 99, 116) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_Step_3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_6, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            if !env.slice_from("tion") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("ate") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("al") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("ic") {
                return false;
            }
        }
        5 => {
            if !env.slice_del() {
                return false;
            }
        }
        6 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_Step_4(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"s") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"t") {
                    return false;
                }
                break 'lab0;
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_Step_5(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !r_R2(env, context) {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !r_R1(env, context) {
                    return false;
                }
                let v_2 = env.limit - env.cursor;
                'lab2: loop {
                    if !r_shortv(env, context) {
                        break 'lab2;
                    }
                    return false;
                }
                env.cursor = env.limit - v_2;
                break 'lab0;
            }
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.eq_s_b(&"l") {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_exception2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_9, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if env.cursor > env.limit_backward {
        return false;
    }
    return true;
}

fn r_exception1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    among_var = env.find_among(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    if env.cursor < env.limit {
        return false;
    }
    match among_var {
        1 => {
            if !env.slice_from("ski") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("sky") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("die") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("lie") {
                return false;
            }
        }
        5 => {
            if !env.slice_from("tie") {
                return false;
            }
        }
        6 => {
            if !env.slice_from("idl") {
                return false;
            }
        }
        7 => {
            if !env.slice_from("gentl") {
                return false;
            }
        }
        8 => {
            if !env.slice_from("ugli") {
                return false;
            }
        }
        9 => {
            if !env.slice_from("earli") {
                return false;
            }
        }
        10 => {
            if !env.slice_from("onli") {
                return false;
            }
        }
        11 => {
            if !env.slice_from("singl") {
                return false;
            }
        }
        _ => (),
    }
    return true;
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !context.b_Y_found {
        return false;
    }
    'replab0: loop {
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            'golab2: loop {
                let v_2 = env.cursor;
                'lab3: loop {
                    env.bra = env.cursor;
                    if !env.eq_s(&"Y") {
                        break 'lab3;
                    }
                    env.ket = env.cursor;
                    env.cursor = v_2;
                    break 'golab2;
                }
                env.cursor = v_2;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            if !env.slice_from("y") {
                return false;
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_Y_found: false,
        i_p2: 0,
        i_p1: 0,
    };
    'lab0: loop {
        let v_1 = env.cursor;
        'lab1: loop {
            if !r_exception1(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = v_1;
        'lab2: loop {
            let v_2 = env.cursor;
            'lab3: loop {
                if !env.hop(3) {
                    break 'lab3;
                }
                break 'lab2;
            }
            env.cursor = v_2;
            break 'lab0;
        }
        env.cursor = v_1;
        r_prelude(env, context);
        r_mark_regions(env, context);
        env.limit_backward = env.cursor;
        env.cursor = env.limit;
        let v_5 = env.limit - env.cursor;
        r_Step_1a(env, context);
        env.cursor = env.limit - v_5;
        'lab4: loop {
            let v_6 = env.limit - env.cursor;
            'lab5: loop {
                if !r_exception2(env, context) {
                    break 'lab5;
                }
                break 'lab4;
            }
            env.cursor = env.limit - v_6;
            let v_7 = env.limit - env.cursor;
            r_Step_1b(env, context);
            env.cursor = env.limit - v_7;
            let v_8 = env.limit - env.cursor;
            r_Step_1c(env, context);
            env.cursor = env.limit - v_8;
            let v_9 = env.limit - env.cursor;
            r_Step_2(env, context);
            env.cursor = env.limit - v_9;
            let v_10 = env.limit - env.cursor;
            r_Step_3(env, context);
            env.cursor = env.limit - v_10;
            let v_11 = env.limit - env.cursor;
            r_Step_4(env, context);
            env.cursor = env.limit - v_11;
            let v_12 = env.limit - env.cursor;
            r_Step_5(env, context);
            env.cursor = env.limit - v_12;
            break 'lab4;
        }
        env.cursor = env.limit_backward;
        let v_13 = env.cursor;
        r_postlude(env, context);
        env.cursor = v_13;
        break 'lab0;
    }
    return true;
}
