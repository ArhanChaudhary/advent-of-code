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
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    destination: Destination<'a>,
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
    operand1: PartAttr,
    operand2: usize,
}

#[derive(Debug)]
enum PartAttr {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

impl PartRange {
    fn from(start: usize, end: usize) -> Self {
        PartRange {
            x: Interval { start, end },
            m: Interval { start, end },
            a: Interval { start, end },
            s: Interval { start, end },
        }
    }

    fn covers_count(&self) -> usize {
        self.x.covers_count()
            * self.m.covers_count()
            * self.a.covers_count()
            * self.s.covers_count()
    }

    fn apply_rule(&mut self, rule: &Rule) -> Self {
        let Some(ref condition) = rule.condition else { return *self };
        let compare_to = condition.operand2;
        let mut destination_part_range = *self;
        let destination_interval = match condition.operand1 {
            PartAttr::X => &mut destination_part_range.x,
            PartAttr::M => &mut destination_part_range.m,
            PartAttr::A => &mut destination_part_range.a,
            PartAttr::S => &mut destination_part_range.s,
        };
        let self_interval = match condition.operand1 {
            PartAttr::X => &mut self.x,
            PartAttr::M => &mut self.m,
            PartAttr::A => &mut self.a,
            PartAttr::S => &mut self.s,
        };
        match condition.operator {
            '>' => {
                self_interval.end = self_interval.end.min(compare_to);
                destination_interval.start = destination_interval.start.max(compare_to + 1);
            }
            '<' => {
                self_interval.start = self_interval.start.max(compare_to);
                destination_interval.end = destination_interval.end.min(compare_to - 1);
            }
            _ => unreachable!(),
        }
        destination_part_range
    }

    fn accepted_count(mut self, root_workflow: &Workflow) -> usize {
        let mut ret = 0;
        for rule in root_workflow.rules.borrow().iter().rev() {
            let destination_part_range = self.apply_rule(rule);
            ret += match rule.destination {
                Destination::A => destination_part_range.covers_count(),
                Destination::R => 0,
                Destination::Workflow(destination_workflow) => {
                    destination_part_range.accepted_count(destination_workflow)
                }
            }
        }
        ret
    }
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn covers_count(&self) -> usize {
        let r = self.end - self.start + 1;
        assert!(r > 0);
        r
    }
}

fn part2(input: &str) -> usize {
    let (workflows, raw_rules): (Vec<Workflow>, Vec<String>) = input
        .lines()
        .take_while(|&line| !line.is_empty())
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
                Some('x') => PartAttr::X,
                Some('m') => PartAttr::M,
                Some('a') => PartAttr::A,
                Some('s') => PartAttr::S,
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
    let root_workflow = workflows
        .iter()
        .find(|workflow| workflow.name == "in")
        .unwrap();
    PartRange::from(1, 4000).accepted_count(root_workflow)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part2_test() {
        let result = part2("\
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
        assert_eq!(result, 167409079868000);
    }
}
