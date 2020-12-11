pub fn do_it(nums: &mut Vec<usize>) -> usize {
    nums.sort();
    let mut diff_counts: [usize; 4] = [0, 1, 0, 1];

    for i in 1..nums.len() {
        diff_counts[nums[i] - nums[i-1]] += 1;
    }

    println!("{:?}", diff_counts);
    diff_counts[1] * diff_counts[3]
}

pub fn hard_mode(nums: &mut Vec<usize>) -> usize {
    nums.sort();

    //Sculpting input here...
    let fib3_nums = [1, 1, 1, 2, 4, 7, 13];

    let mut working: usize = if nums[0] == 1 {
        2
    } else {
        1
    };

    let mut seqs: Vec<usize> = vec![];
    for i in 1..nums.len() {
        let jump = nums[i] - nums[i-1];
        if jump == 1 {
            working += 1;
        } else if jump == 3 {
            seqs.push(working);
            working = 1;
        }
    }

    seqs.push(working);

    println!("seqs {:?}", seqs);
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

        nums.sort();
        println!("{:?}", nums);
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
