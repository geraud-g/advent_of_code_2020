use std::collections::HashMap;


pub fn day_15() {
    let input = [16, 11, 15, 0, 1, 7];

    let solution_a = solve(&input, 2020);
    println!("Part A: {}", solution_a);
    assert_eq!(662, solution_a);

    let solution_b = solve(&input, 30000000);
    println!("Part B: {}", solution_b);
    assert_eq!(37312, solution_b)
}


fn solve(numbers: &[usize], repeat: usize) -> usize {
    let mut last_spoken = *numbers.last().unwrap();
    let mut spoken_numbers: HashMap<usize, Number> = numbers.iter().enumerate()
        .map(|(idx, v)| (*v, Number::new(idx + 1))).collect();

    for turn in (numbers.len() + 1)..=repeat {
        last_spoken = spoken_numbers.get(&last_spoken).unwrap().get_next_number();
        spoken_numbers.entry(last_spoken)
            .and_modify(|s|s.update_turn(turn))
            .or_insert_with(||Number::new(turn));
    }
    last_spoken
}


#[derive(Debug, Hash, Copy, Clone)]
struct Number {
    last_turn: usize,
    last_turn_before: Option<usize>,
}


impl Number {
    fn new(value: usize) -> Self {
        Number { last_turn: value, last_turn_before: None }
    }

    fn get_next_number(&self) -> usize {
        match self.last_turn_before {
            Some(value) => self.last_turn - value,
            None => 0
        }
    }

    fn update_turn(&mut self, turn: usize) {
        self.last_turn_before = Some(self.last_turn);
        self.last_turn = turn;
    }
}
