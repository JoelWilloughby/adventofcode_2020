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

// This algorithm is pretty nifty. The problem gives you a sliding
// window of size |your input|. This means you can represent 
// this window with a (start, size) pair. It also gives you a set 
// divisors that have multiples in that interval.
// I.e., if you were given 
// 10 vals, and 6 x's: 7,x,x,11,x,x,13,x,17,x.
// You are looking for a start value v s.t. (v + 0) % 7 == 0, 
// (v + 3) % 11 == 0, (v + 6) % 13 == 0, and (v + 8) % 17 == 0. 
// 
// Finding a v by brute force quickly shows that this is possible 
// for the simple case, but painfully slow on the real input.
//
// The aha! moment for me was that we should just pretend we had 
// been given the first restriction, i.e. (v + 0) % 7 == 0. Thats
// easy, thats just all the multiples of 7. So, we can count by 7
// rather than 1. Big whoop. But, it means every count we make from
// here on _must factor in the 7_, no matter what constraints we have
// left. So, we start by finding the first value that satisfies the
// next constraint: (v + 3) % 11. For that, we start at 0 and jump 
// by 7 until we hit 63. (63 + 3) % 11 == 0. Cool. From now on,
// because of nice relative prime properties in our divisors, 63
// will work, but so will 63 + (n) * (7 * 11), for all n >= 0. 
// What's more, these are the _only_ values that will work (I think).
// So, what happens when we add the next group? Well, start at 63 and
// jump by 77 until you find a v that satisfies (v + 6) % 13 = 0.
// That value happens to be 371. Then, because of the same reasoning
// above, our solution is narrowed to 371 + n * (7 * 11 * 13).
// Our jump is quickly growing, yielding a combound interest effect.
// From there, you find that resulting value is 11382 (which is
// when n = 11). Since there are no more constraints, problem done. 
pub fn do_it_2(trains: &Vec<(usize, usize)>) -> usize {
    // Start at 0.
    let mut acc: usize = 0;
    // Jump at 1 because it will grow via multiplication
    let mut jump: usize = 1;

    // Each iteration of the loop satisfies the i+1'th train
    // The 0'th train is the base case and will always be
    // satisfied.
    for i in 0..(trains.len()-1) {
        // We are jumping by everything under this ith train
        let current_train = trains[i];
        // We are looking for the first value starting from acc
        // counting by jump that statifes this constraint
        let next_train = trains[i+1];

        // jump = product train[0..i]
        jump *= current_train.0;      
        println!("Current jump: {} {}", jump, acc);      

        // Search for the constraint (acc + n * jump) % train == 0 
        while (acc + next_train.1) % next_train.0 != 0 {
            acc += jump;
        }
    }

    acc
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

        let acc = do_it_2(&trains);
        println!("{}", acc);
    }

    #[test]
    fn it_works() {
        drive("res/13/input_simple.txt");
        drive_2("res/13/input_simple.txt");
    }

    #[test]
    fn test_it() {
        drive("res/13/input.txt");
        drive_2("res/13/input.txt");
    }

    #[test]
    fn example() {
        drive_2("res/13/input_example.txt");
    }
}
