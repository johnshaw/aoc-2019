fn is_valid(n: &u32) -> bool {
    // Extract digits
    let mut buf = [0u8; 6];
    let mut n = *n;
    for i in (0..6).rev() {
        buf[i] = (n % 10) as u8;
        n /= 10;
    }

    // Non decreasing
    for i in 0..5 {
        if buf[i] > buf[i+1] {
            return false;
        }
    }

    // double digit
    for i in 0..5 {
        if buf[i] == buf[i+1] {
            if i == 4 {
                if buf[i-1] != buf[i] {
                    return true;
                }
            } else if i == 0 {
                if buf[i+2] != buf[i] {
                    return true;
                }
            } else if buf[i-1] != buf[i] && buf[i] != buf[i+2] {
                return true;
            }
        }
    }

    false
}

fn main() {
    let count = (134564..=585159).filter(is_valid).count();
    println!("{}", count);
}
