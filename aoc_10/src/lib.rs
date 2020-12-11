pub fn do_it(nums: &mut Vec<usize>) -> usize {
    let mut diff_counts: [usize; 4] = [0, 0, 0, 0];

    for i in 1..nums.len() {
        diff_counts[nums[i] - nums[i-1]] += 1;
    }

    assert_eq!(diff_counts[0], 0);
    assert_eq!(diff_counts[2], 0);
    diff_counts[1] * diff_counts[3]
}

pub fn hard_mode(nums: &mut Vec<usize>) -> usize {
    // Sculpting input here. We never see groups of more than 5 in a row
    // In general, number of ways to span such groups of size n, f(n), is
    //   f(n-1) + f(n-2) + f(n-3)
    //   f(0) = 0, f(1) = 1, f(2) = 1
    let fib3_nums = [0, 1, 1, 2, 4, 7];
    let mut working: usize = 1;
    let mut seqs: Vec<usize> = vec![];

    for i in 1..nums.len() {
        match nums[i] - nums[i-1] {
            1 => {
                working += 1;
            },
            3 => {
                seqs.push(working);
                working = 1;
            },
            _ => {}
        }
    }

    seqs.iter().fold(1, |acc, x| acc * fib3_nums[*x])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut nums: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        nums.push(0);
        nums.push(nums.iter().max().unwrap() + 3);
        nums.sort();

        println!("{}", do_it(&mut nums));
        println!("{}", hard_mode(&mut nums));
    }

    #[test]
    fn water() {
        drive("res/input_trivial.txt");
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
