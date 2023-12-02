function aoc-new() {
    cargo new $1;
    mkdir $1/src/bin;
    rm $1/src/main.rs;
    BASE=$(echo `date +https://adventofcode.com/%Y/day/%d` | sed 's/\/0/\//g');
    BASE_CONTENTS=`curl "$BASE"`;
    # we need to echo BASE_CONTENTS in quotes and do the strange \n stuff because \n is a line terminator so we do some weird workarounds
    TEST_CASES=`echo "$BASE_CONTENTS" | tr '\n' '|' | sed -e 's|.*<pre><code>\(.*\)</code></pre>.*|\1|' | sed 's/|/\n/g'`
    TEST_RESULT=`echo "$BASE_CONTENTS" | tr '\n' '|' | sed -e 's|.*<code><em>\(.*\)</em></code>.*|\1|' | sed 's/|/\n/g'`
    echo "fn main() {
    let input = include_str!(\"./input1.txt\");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(\"$TEST_CASES\");
        assert_eq!(result, $TEST_RESULT);
    }
}" > $1/src/bin/part1.rs;
    curl --cookie "session=$AOC_COOKIE" $BASE/input > $1/src/bin/input1.txt
    cd $1;
    unset TEST_CASES;
    unset TEST_RESULT;
}
aoc-new $1;
