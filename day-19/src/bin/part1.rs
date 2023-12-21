use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::cell::RefCell;

#[derive(Debug)]
struct Workflow<'a> {
    name: String,
    rules: RefCell<Vec<Rule<'a>>>,
}

impl<'a> Workflow<'a> {
    fn add_rule(&self, rule: Rule<'a>) {
        self.rules.borrow_mut().push(rule);
    }

    fn to_dest(&self, part: &Part) -> Destination<'a> {
        self.rules
            .borrow()
            .iter()
            .rev()
            .find_map(|rule| {
                if rule.matches(part) {
                    Some(rule.destination)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    destination: Destination<'a>,
}

impl Rule<'_> {
    fn matches(&self, part: &Part) -> bool {
        match &self.condition {
            None => true,
            Some(condition) => {
                let operand1 = match condition.operand1 {
                    PartRating::X => part.x,
                    PartRating::M => part.m,
                    PartRating::A => part.a,
                    PartRating::S => part.s,
                };
                match condition.operator {
                    '>' => operand1 > condition.operand2,
                    '<' => operand1 < condition.operand2,
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Destination<'a> {
    R,
    A,
    Workflow(&'a Workflow<'a>),
}

impl Destination<'_> {
    fn from<'a>(raw_from: &str, workflows: &'a [Workflow<'a>]) -> Destination<'a> {
        match raw_from {
            "A" => Destination::A,
            "R" => Destination::R,
            workflow_name => Destination::Workflow(
                workflows
                    .iter()
                    .find(|workflow| workflow.name == workflow_name)
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
struct Condition {
    operator: char,
    operand1: PartRating,
    operand2: usize,
}

#[derive(Debug)]
enum PartRating {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn rating_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn part1(input: &str) -> usize {
    let mut input_split = input.split("\n\n");
    let (workflows, raw_rules): (Vec<Workflow>, Vec<String>) = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_chars = line.chars();
            (
                Workflow {
                    name: line_chars.by_ref().take_while(|&c| c != '{').collect(),
                    rules: RefCell::from(Vec::new()),
                },
                line_chars.take_while(|&c| c != '}').collect(),
            )
        })
        .unzip();
    let root_workflow = workflows
        .iter()
        .find(|workflow| workflow.name == "in")
        .unwrap();
    for (workflow, raw_rules) in workflows.iter().zip(raw_rules) {
        // workflow.rules
        let mut raw_rules_split = raw_rules.rsplit(',');
        workflow.add_rule(Rule {
            condition: None,
            destination: Destination::from(raw_rules_split.next().unwrap(), &workflows),
        });
        for raw_rule in raw_rules_split {
            let mut raw_rule_chars = raw_rule.chars();
            let operand1 = match raw_rule_chars.next() {
                Some('x') => PartRating::X,
                Some('m') => PartRating::M,
                Some('a') => PartRating::A,
                Some('s') => PartRating::S,
                _ => unreachable!(),
            };
            let operator = raw_rule_chars.next().unwrap();
            let operand2 = raw_rule_chars
                .by_ref()
                .take_while(|&c| c != ':')
                .fold(0, |sum, c| sum * 10 + c.to_digit(10).unwrap() as usize);
            let raw_from = raw_rule_chars.collect::<String>();
            let raw_from = raw_from.as_str();
            workflow.add_rule(Rule {
                condition: Some(Condition {
                    operator,
                    operand1,
                    operand2,
                }),
                destination: Destination::from(raw_from, &workflows),
            })
        }
    }

    input_split
        .next()
        .unwrap()
        .lines()
        .filter_map(|raw_part_ratings| {
            let mut raw_part_ratings_chars = raw_part_ratings.chars();
            let mut next_part_rating = || {
                raw_part_ratings_chars
                    .by_ref()
                    .skip_while(|&c| !c.is_numeric())
                    .fold_while(0, |num, c| match c.to_digit(10) {
                        Some(n) => Continue(num * 10 + n as usize),
                        None => Done(num),
                    })
                    .into_inner()
            };
            let part = Part {
                x: next_part_rating(),
                m: next_part_rating(),
                a: next_part_rating(),
                s: next_part_rating(),
            };
            let mut current_workflow = root_workflow;
            loop {
                match current_workflow.to_dest(&part) {
                    Destination::Workflow(dest_workflow) => {
                        current_workflow = dest_workflow;
                    }
                    Destination::A => break Some(part.rating_sum()),
                    Destination::R => break None,
                }
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 19114);
    }
}
