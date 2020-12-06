
fn read_line(line: &str) -> usize {
    let bytes = String::from(line).into_bytes();
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
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    fn drive(filename: &str) {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut max = 0;
        let mut seats = [false; 0x3ff];
        for line in reader.lines() {
            let li = line.unwrap();
            let temp = read_line(&li);
            println!("{:?} -> {:b} = {}", li, temp, temp);
            if temp > max {
                max = temp;
            }
            seats[temp] = true;
        }

        let mut down = max;
        while seats[down] {
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
