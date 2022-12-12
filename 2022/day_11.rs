use std::collections::VecDeque;
use std::num::ParseIntError;

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<i64>,
    operation: (String, String),
    test: i64,
    true_monkey: usize,
    false_monkey: usize,
    times_inspected: i64,
}

impl Monkey {
    fn new(
        items: VecDeque<i64>,
        op: String,
        op_val: String,
        test_num: i64,
        true_monkey_idx: usize,
        false_monkey_idx: usize,
    ) -> Self {
        Self {
            starting_items: items,
            operation: (op, op_val),
            test: test_num,
            true_monkey: true_monkey_idx,
            false_monkey: false_monkey_idx,
            times_inspected: 0,
        }
    }

    fn add_item(&mut self, item: i64) {
        self.starting_items.push_back(item);
    }

    fn pass_item(worry: i64, test: i64, true_idx: usize, false_idx: usize) -> usize {
        if worry % test == 0 {
            return true_idx;
        }
        false_idx
    }

    fn perform_operation(item: i64, op: &str, op_val: &str) -> i64 {
        let old = match op_val.parse::<i64>() {
            Ok(val) => val,
            Err(_) => item,
        };

        match op {
            "+" => (old + item) / 3,
            "-" => (old - item) / 3,
            "*" => (old * item) / 3,
            "/" => (old / item) / 3,
            _ => item,
        }
    }

    fn inspect_item(&mut self) -> Option<(i64, usize)> {
        if let Some(item) = self.starting_items.pop_front() {
            self.times_inspected += 1;
            let worry = Self::perform_operation(item, &self.operation.0, &self.operation.1);
            let idx = Self::pass_item(worry, self.test, self.true_monkey, self.false_monkey);
            return Some((worry, idx));
        }
        None
    }
}

fn init_monkeys(input: &str) -> Result<Vec<Monkey>, ParseIntError> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut items: VecDeque<i64> = VecDeque::new();
    let mut op: String = String::from("");
    let mut op_val: String = String::from("");
    let mut test_num: i64 = 0;
    let mut true_monkey_idx: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let attributes: Vec<&str> = line.trim().split(':').collect();

        match attributes[..] {
            ["Starting items", values] => {
                for i in values.split(',') {
                    let num = i.trim().parse::<i64>()?;
                    items.push_back(num);
                }
            }
            ["Operation", values] => {
                let operations: Vec<&str> = values.split_whitespace().collect();
                op_val = operations[4].to_string();
                op = operations[3].to_string();
            }
            ["Test", values] => {
                let divisible: Vec<&str> = values.split_whitespace().collect();
                test_num = divisible[2].parse::<i64>()?;
            }
            ["If true", values] => {
                let true_monkey: Vec<&str> = values.split_whitespace().collect();
                true_monkey_idx = true_monkey[3].parse::<usize>()?;
            }
            ["If false", values] => {
                let false_monkey: Vec<&str> = values.split_whitespace().collect();
                let false_monkey_idx = false_monkey[3].parse::<usize>()?;

                monkeys.push(Monkey::new(
                    items,
                    op,
                    op_val,
                    test_num,
                    true_monkey_idx,
                    false_monkey_idx,
                ));

                items = VecDeque::new();
                op = String::from("");
                op_val = String::from("");
                test_num = 0;
                true_monkey_idx = 0;
            }
            _ => {}
        }
    }

    Ok(monkeys)
}

fn main() {
    let mut monkeys = init_monkeys(
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
    )
    .unwrap();

    for _round in 0..19 {
        for idx in 0..monkeys.len() {
            while let Some((item, monkey_idx)) = monkeys[idx].inspect_item() {
                monkeys[monkey_idx].add_item(item);
            }
        }
    }

    let mut max = 0;
    let mut second_max = 0;

    for idx in 0..monkeys.len() {
        while let Some((item, monkey_idx)) = monkeys[idx].inspect_item() {
            monkeys[monkey_idx].add_item(item);
        }

        if monkeys[idx].times_inspected > max {
            second_max = max;
            max = monkeys[idx].times_inspected;
        } else if monkeys[idx].times_inspected > second_max {
            second_max = monkeys[idx].times_inspected;
        }
    }

    println!("Result: {}", max * second_max);
}

//================================================================================================

