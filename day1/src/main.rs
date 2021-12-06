use std::fs;

fn main() {
  let file = fs::read_to_string("./input.txt").unwrap();

  let scan: Vec<u32> = file.lines().map(|line| line.parse().unwrap()).collect();

  dbg!(part_1(&scan));
  dbg!(part_2(&scan));
}

/** Count Increases */
fn part_1(vector: &Vec<u32>) -> u32 {
  let mut it = vector.iter().peekable();
  let mut count = 0;
  for _ in 0..it.len() {
    let one = it.next();
    let two = it.peek();
    match (one, two) {
      (Some(first), Some(&second)) => {
        if second > first {
          count += 1;
        }
      }
      _ => (),
    }
  }
  count
}

/** Count windowed increases */
fn part_2(vector: &Vec<u32>) -> u32 {
  let mut count = 0;
  for items in vector.windows(4) {
    if let [one, _, _, four] = items {
      if four > one {
        count += 1;
      }
    } else {
      panic!("slice wasn't length four?")
    }
  }
  count
}
