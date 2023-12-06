function part1() {
    N=$(printf "day-%02d" $1)
    cargo new $N
    mkdir $N/src/bin
    rm $N/src/main.rs
    BASE=`date +https://adventofcode.com/%Y/day/$1`;
    # add --cookie "session=$AOC_COOKIE" to access part 2
    BASE_CONTENTS=`curl "$BASE"`
    # we need to echo BASE_CONTENTS in quotes and do the strange \n stuff because \n is a line terminator so we do some weird workarounds
    TEST_CASES=`echo "$BASE_CONTENTS" | tr '\n' '\r' | sed -e 's|.*<pre><code>\(.*\)\r</code></pre>.*|\1|' -e 's/\r/\n/g'`
    TEST_RESULT=`echo "$BASE_CONTENTS" | tr '\n' '\r' | sed -e 's|.*<code><em>\(.*\)</em></code>.*|\1|'`
    echo "fn main() {
    let input = include_str!(\"./input.txt\");
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
        let result = part1(
            \"$TEST_CASES\",
        );
        assert_eq!(result, $TEST_RESULT);
    }
}" > $N/src/bin/part1.rs;
    curl --cookie "session=$AOC_COOKIE" $BASE/input > $N/src/bin/input.txt
    cd $N
    cargo build
    unset N
    unset BASE
    unset BASE_CONTENTS
    unset TEST_CASES
    unset TEST_RESULT
}

function part2() {
    N=$(printf "day-%02d" $1)
    BASE=`date +https://adventofcode.com/%Y/day/$1`;
    BASE_CONTENTS=`curl --cookie "session=$AOC_COOKIE" "$BASE"`;
    # we need to echo BASE_CONTENTS in quotes and do the strange \n stuff because \n is a line terminator so we do some weird workarounds
    TEST_CASES=`echo "$BASE_CONTENTS" | tr '\n' '\r' | sed -e 's|.*<pre><code>\(.*\)\r</code></pre>.*|\1|'`
    TEST_RESULT=`echo "$BASE_CONTENTS" | tr '\n' '\r' | sed -e 's|.*<code><em>\(.*\)</em></code>.*|\1|'`
    cat $N/src/bin/part1.rs | sed -e 's/part1/part2/g' | tr '\n' '\r' | sed -E "s/(.+)part2\(\r +".+",\r +\);\r +assert_eq\!\(result, .+\);(.*)/\1part2(\r            \"$TEST_CASES\",\r        );\r        assert_eq!(result, $TEST_RESULT);\2/g" | tr '\r' '\n' > $N/src/bin/part2.rs
    unset N
    unset BASE
    unset BASE_CONTENTS
    unset TEST_CASES
    unset TEST_RESULT
}

"$@"
