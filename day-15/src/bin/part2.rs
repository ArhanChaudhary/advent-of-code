use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn hash(to_hash: &str) -> usize {
    to_hash.chars().fold(0, |current_value, c| {
        if c == '\n' {
            return current_value;
        }
        let mut next_value = current_value + c as usize;
        next_value *= 17;
        next_value %= 256;
        next_value
    })
}

#[derive(Debug)]
struct Lens(String, usize);

fn part2(input: &str) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());
    for step in input.split(',') {
        let mut chars = step.chars();
        let lens_label = chars
            .take_while_ref(|c| c.is_alphabetic())
            .collect::<String>();
        let box_ = &mut boxes[hash(&lens_label)];
        let operation = chars.next().unwrap();
        match operation {
            '-' => {
                box_.retain(|lens| lens.0 != lens_label);
            }
            '=' => {
                let focal_length: usize = chars.collect::<String>().parse().unwrap();
                match box_.iter().position(|lens| lens.0 == lens_label) {
                    Some(same_label_index) => {
                        box_[same_label_index].1 = focal_length;
                    }
                    None => {
                        let lens = Lens(lens_label, focal_length);
                        box_.push(lens);
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box_)| {
            box_.iter()
                .enumerate()
                .map(move |(j, lens)| (i + 1) * (j + 1) * lens.1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part2_test() {
        let result = part2("\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(result, 145);
    }
}
