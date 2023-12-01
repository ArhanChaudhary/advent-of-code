function aoc-new() {
    cargo new $1;
    mkdir $1/src/bin;
    rm $1/src/main.rs;
    echo "fn main() {
    let input = include_str!(\"./input1.txt\");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    todo!();
}" > $1/src/bin/part1.rs;
    curl --cookie "session=$AOC_COOKIE" "$(echo `date +https://adventofcode.com/%Y/day/%d/input` | sed 's/\/0/\//g')" > $1/src/bin/input1.txt
    cd $1;
}
aoc-new $1;