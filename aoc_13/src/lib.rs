pub fn do_it(target: usize, nums: &Vec<usize>) -> (usize, usize) {
    let mut min_train_id = 0;
    let mut min_train_time = usize::MAX;

    for &x in nums.iter() {
        if x - (target % x) < min_train_time {
            min_train_time = x - (target % x);
            min_train_id = x;
        }
    }

    (target + min_train_time, min_train_id)
}

pub fn do_it_2(trains: &Vec<(usize, usize)>) -> (usize, usize) {
    // this will be the initial position
    let mut acc: usize = 0;
    let mut jump: usize = 1;

    for i in 0..(trains.len()-1) {
        let current_train = trains[i];
        let next_train = trains[i+1];
        jump *= current_train.0;            

        while (acc + next_train.1) % next_train.0 != 0 {
            acc += jump;
        }
    }

    (acc, jump)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.lines();
        let target = lines.next().unwrap().parse::<usize>().unwrap();

        let trains: Vec<usize> = lines.next().unwrap()
            .split(",").filter_map(|s| s.parse::<usize>().ok())
            .collect();

        let (final_time, train_id) = do_it(target, &trains);
        println!("{} {} ==> {}", final_time, train_id, (final_time - target) * train_id);
    }

    fn drive_2(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.lines();
        lines.next();

        let trains: Vec<(usize, usize)> = lines.next().unwrap()
            .split(",").enumerate().filter_map(|(i, s)| {
                s.parse::<usize>().ok().map(|x| (x, i))
            }).collect();

        let (acc, jump) = do_it_2(&trains);
        println!("{} {}", acc, jump);
    }

    #[test]
    fn it_works() {
        drive("res/input_simple.txt");
        drive_2("res/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/input.txt");
        drive_2("res/input.txt");
    }
}
