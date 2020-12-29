
pub fn do_it(n: usize, m: usize) -> usize {
    let subject = 7;
    let base = 20201227;

    let mut m_count = 0;
    let mut n_count = 0;
    let mut acc = 1;
    let mut count = 1;
    while m_count == 0 || n_count == 0 {
        acc *= subject;
        acc %= base;
        if acc == n {
            assert_eq!(n_count, 0);
            n_count = count;
        }
        if acc == m {
            assert_eq!(m_count, 0);
            m_count = count;
        }
        count += 1;
    }

    acc = m;
    for _ in 1..n_count {
        acc *= m;
        acc %= base;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(n: usize, m: usize) {
        let val = do_it(n, m);
        println!("Part 1: {}", val);
    }

    #[test]
    fn it_works() {
        drive(5764801,17807724);
    }

    #[test]
    fn test_it() {
        drive(16616892, 14505727);
    }
}