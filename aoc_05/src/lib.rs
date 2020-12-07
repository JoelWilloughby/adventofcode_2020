
pub fn read_line(line: &str) -> usize {
    let bytes = line.bytes();
    assert_eq!(bytes.len(), 10);

    let mut val: usize = 0;
    for i in bytes {
        val <<= 1;
        if i == 'R' as u8 || i == 'B' as u8 {
            val += 1;
        } 
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let file = std::fs::read_to_string(filename).unwrap();

        let mut max = 0;
        let mut seat_field = [false; 0x3ff];
        let seats = file.lines().map(|line| read_line(line)).collect::<Vec<usize>>();
        for seat in seats {
            if seat > max {
                max = seat;
            }
            seat_field[seat] = true;
        }

        let mut down = max;
        while seat_field[down] {
            down -= 1;
        }

        println!("Max val: {}", max);
        println!("Your seat: {}", down);
    }


    #[test]
    fn it_works() {
        drive("res/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/input.txt");
    }
}
