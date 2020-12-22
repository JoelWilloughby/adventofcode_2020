use std::collections::{HashSet,HashMap,VecDeque,hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Game {
    left: VecDeque<usize>,
    right: VecDeque<usize>,
    history: HashSet<(usize, usize)>,
}

impl Game {
    pub fn new(left: &VecDeque<usize>, right: &VecDeque<usize>) -> Self {
        Self {
            left: left.clone(), right: right.clone(), history: HashSet::new(),
        }
    }

    pub fn has_winner(&self) -> bool {
        self.left.is_empty() || self.right.is_empty()
    }

    pub fn get_winner(&self) -> Option<Vec<usize>> {
        if self.has_winner() {
            if self.left.is_empty() {
                Some(self.right.iter().map(|x| *x).collect())
            } else {
                Some(self.left.iter().map(|x| *x).collect())
            }
        } else {
            None
        }
    }

    pub fn turn(&mut self) {
        if self.has_winner() {
            return;
        }

        if self.left[0] > self.right[0] {
            self.win_p1();
        } else {
            self.win_p2();
        }
    }

    fn win_p1(&mut self) {
        let left_front = self.left.pop_front().unwrap();
        let right_front = self.right.pop_front().unwrap();
        self.left.push_back(left_front);
        self.left.push_back(right_front);
    }

    fn win_p2(&mut self) {
        let left_front = self.left.pop_front().unwrap();
        let right_front = self.right.pop_front().unwrap();
        self.right.push_back(right_front);
        self.right.push_back(left_front);
    }

    fn game_hash(&self) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        self.left.hash(&mut hasher);
        let hash_1 = hasher.finish() as usize;
        let mut hasher = DefaultHasher::new();
        self.right.hash(&mut hasher);
        let hash_2 = hasher.finish() as usize;

        (hash_1, hash_2)
    }

    fn check_history(&mut self) -> bool {
        // Check history
        let hash = self.game_hash();
        if self.history.contains(&hash) {
            return true;
        }

        self.history.insert(hash);

        false
    }

    fn run_sub_game(&mut self) -> bool {
        lazy_static! {
            static ref HISTORY: Mutex<HashMap<(usize, usize), bool>> = Mutex::new(HashMap::new());
        };
        let hash = self.game_hash();
        if let Some(val) = HISTORY.lock().unwrap().get(&hash) {
            return *val;
        }

        // Build new left hand
        let mut sub_left = VecDeque::new();
        for val in self.left.iter().skip(1).take(self.left[0]) {
            sub_left.push_back(*val);
        }

        // Build new right hand
        let mut sub_right = VecDeque::new();
        for val in self.right.iter().skip(1).take(self.right[0]) {
            sub_right.push_back(*val);
        }

        // Run the game to done
        let mut sub_game = Self::new( &sub_left, &sub_right );
        while !sub_game.has_winner() {
            sub_game.turn_2();
        }

        HISTORY.lock().unwrap().insert((hash.1, hash.0), sub_game.left.is_empty());
        HISTORY.lock().unwrap().insert(hash, !sub_game.left.is_empty());

        !sub_game.left.is_empty()
    }

    pub fn turn_2(&mut self) {
        if self.has_winner() {
            return;
        }

        // Check history
        if self.check_history() {
            // Player 1 wins
            while !self.has_winner() {
                self.win_p1();
            }
            return;
        }

        if self.left[0] > self.left.len() - 1 || self.right[0] > self.right.len() - 1 {
            // Cannot recurse, do normal check
            self.turn();
        } else {
            // Run a sub game and propogate winner
            if self.run_sub_game() {
                self.win_p1();
            } else {
                self.win_p2();
            }
        }
    }
}

pub fn doit(input: &String) {
    let mut lines = input.lines();

    let player_1 = lines.next().unwrap();
    assert_eq!(player_1, String::from("Player 1:"));

    let mut nums_1 = VecDeque::new();
    let mut nums_2 = VecDeque::new();

    let mut current_player = &mut nums_1;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line == "Player 2:" {
            current_player = &mut nums_2;
            continue;
        }

        current_player.push_back(line.parse::<usize>().unwrap());
    }

    println!("{:?} {:?}", nums_1, nums_2);

    let mut game = Game::new(&nums_1, &nums_2);
    while !game.has_winner() {
        game.turn();
    }
    let mut winning_hand = game.get_winner().unwrap();
    winning_hand.reverse();
    println!("Part 1: {}", winning_hand.iter().enumerate().fold(0, |acc, (i, val)| acc + (i+1) * val));

    let mut game = Game::new(&nums_1, &nums_2);
    while !game.has_winner() {
        game.turn_2();
    }
    let mut winning_hand = game.get_winner().unwrap();
    winning_hand.reverse();
    println!("Part 2: {}", winning_hand.iter().enumerate().fold(0, |acc, (i, val)| acc + (i+1) * val));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drive(filename: &str) {
        let input = std::fs::read_to_string(filename).unwrap();
        doit(&input);
    }

    #[test]
    fn it_works() {
        drive("res/22/input_simple.txt");
    }

    #[test]
    fn water() {
        drive("res/22/input_inf.txt");
    }

    #[test]
    fn test_it() {
        drive("res/22/input.txt");
    }
}