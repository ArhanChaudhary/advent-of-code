fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "10  13  16  21  30  45  <em>68</em>
   3   3   5   9  15  <em>23</em>
     0   2   4   6   <em>8</em>
       2   2   2   <em>2</em>
         0   0   <em>0</em>",
        );
        assert_eq!(result, 114);
    }
}
