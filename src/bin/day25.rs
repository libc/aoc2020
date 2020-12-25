fn main() {
    let mut v = 1;
    let mut i = 0;

    let door_value = 19774466;
    let card_value = 7290641;

    let mut door_loop_size = 0;
    let mut card_loop_size = 0;

    for i in 0..20201227 {
        v = mul_mod64(v, 7, 20201227);

        if v == door_value {
            door_loop_size = i;
            break;
        }

        if v == card_value {
            card_loop_size = i;
        }
    }

    v = 1;
    for _ in 0..=door_loop_size {
        v = mul_mod64(v, card_value, 20201227);
    }

    println!("key: {} {} {}", door_loop_size, card_loop_size, v);
}

// copied from https://stackoverflow.com/questions/45918104/how-to-do-arithmetic-modulo-another-number-without-overflow
fn mul_mod64(mut x: u64, mut y: u64, m: u64) -> u64 {
    let msb = 0x8000_0000_0000_0000;
    let mut d = 0;
    let mp2 = m >> 1;
    x %= m;
    y %= m;

    if m & msb == 0 {
        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if x & msb != 0 {
                d += y;
            }
            if d >= m {
                d -= m;
            }
            x <<= 1;
        }
        d
    } else {
        for _ in 0..64 {
            d = if d > mp2 {
                d.wrapping_shl(1).wrapping_sub(m)
            } else {
                // the case d == m && x == 0 is taken care of
                // after the end of the loop
                d << 1
            };
            if x & msb != 0 {
                let (mut d1, overflow) = d.overflowing_add(y);
                if overflow {
                    d1 = d1.wrapping_sub(m);
                }
                d = if d1 >= m { d1 - m } else { d1 };
            }
            x <<= 1;
        }
        if d >= m {
            d - m
        } else {
            d
        }
    }
}
