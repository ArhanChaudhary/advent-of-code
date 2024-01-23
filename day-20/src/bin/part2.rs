use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use utils::lcmm;

#[derive(Debug, PartialEq, Clone, Copy)]
enum SignalType {
    High,
    Low,
}

struct SignalResult {
    signal_type: SignalType,
    to_queue: Vec<ModuleRef>,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: Option<ModuleType>,
    destinations: Option<ModuleDestinations>,
    emitting: SignalType,
}

impl Module {
    fn new(
        name: String,
        type_: Option<ModuleType>,
        destinations: Option<ModuleDestinations>,
    ) -> Self {
        Module {
            name,
            module_type: type_,
            destinations,
            emitting: SignalType::Low,
        }
    }

    fn destination_names(&self) -> Option<&[String]> {
        self.destinations.as_ref().map(|destinations| {
            if let ModuleDestinations::Names(destination_names) = destinations {
                destination_names.as_slice()
            } else {
                panic!();
            }
        })
    }

    fn destinations(&self) -> Option<&[ModuleRef]> {
        self.destinations.as_ref().map(|destinations| {
            if let ModuleDestinations::ModuleRefs(destination_refs) = destinations {
                destination_refs.as_slice()
            } else {
                panic!();
            }
        })
    }
}

type ModuleRef = Rc<RefCell<Module>>;

#[derive(Debug)]
enum ModuleDestinations {
    Names(Vec<String>),
    ModuleRefs(Vec<ModuleRef>),
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug)]
struct Broadcaster;
impl Broadcaster {
    fn new() -> Self {
        Broadcaster
    }
}

#[derive(Debug)]
struct FlipFlop(bool);
impl FlipFlop {
    fn new() -> Self {
        FlipFlop(false)
    }
}

#[derive(Debug)]
struct Conjunction {
    from_signals: Vec<ModuleRef>,
}

impl Conjunction {
    fn new() -> Self {
        Conjunction {
            from_signals: Vec::new(),
        }
    }

    fn add_signal_sender(&mut self, module: ModuleRef) {
        self.from_signals.push(module);
    }
}

fn send_signal(module: ModuleRef) -> Option<SignalResult> {
    let mut module = module.borrow_mut();
    let signal_type = match module.module_type.as_mut()? {
        ModuleType::Broadcaster(_) => SignalType::Low,
        ModuleType::FlipFlop(flipflop) => {
            flipflop.0 = !flipflop.0;
            if flipflop.0 {
                SignalType::High
            } else {
                SignalType::Low
            }
        }
        ModuleType::Conjunction(conjunction) => {
            if conjunction
                .from_signals
                .iter()
                .all(|module| module.borrow().emitting == SignalType::High)
            {
                SignalType::Low
            } else {
                SignalType::High
            }
        }
    };
    let module_destinations = module.destinations().unwrap();
    let to_queue = if signal_type == SignalType::High {
        module_destinations
            .iter()
            .filter_map(|module_destination| {
                if let Some(ModuleType::FlipFlop(_)) = module_destination.borrow().module_type {
                    None
                } else {
                    Some(module_destination.clone())
                }
            })
            .collect()
    } else {
        module_destinations.to_vec()
    };
    module.emitting = signal_type;
    Some(SignalResult {
        signal_type,
        to_queue,
    })
}

const TARGET_MODULE_NAME: &str = "rx";

fn part2(input: &str) -> usize {
    let mut broadcaster: Option<ModuleRef> = None;
    let mut modules: Vec<ModuleRef> = Vec::new();
    let mut before_target_destination_module: Option<ModuleRef> = None;
    for line in input.lines() {
        let mut split = line.split(" -> ");
        let mut module_name = split.next().unwrap();
        let (module_type, is_broadcaster) = match module_name.chars().next().unwrap() {
            '%' => (ModuleType::FlipFlop(FlipFlop::new()), false),
            '&' => (ModuleType::Conjunction(Conjunction::new()), false),
            'b' => (ModuleType::Broadcaster(Broadcaster::new()), true),
            _ => unreachable!(),
        };

        if !is_broadcaster {
            module_name = &module_name[1..];
        }
        let mut has_target_destination = false;
        let mut destination_names = Vec::new();
        for destination_name in split.next().unwrap().split(", ").map(String::from) {
            if destination_name == TARGET_MODULE_NAME {
                has_target_destination = true;
            }
            destination_names.push(destination_name);
        }
        let module = Rc::new(RefCell::new(Module::new(
            module_name.to_owned(),
            Some(module_type),
            Some(ModuleDestinations::Names(destination_names)),
        )));
        if is_broadcaster {
            broadcaster = Some(module.clone());
        }
        if has_target_destination {
            before_target_destination_module = Some(module.clone());
        }
        modules.push(module);
    }

    let broadcaster = broadcaster.unwrap();
    let before_target_destination_module = before_target_destination_module.unwrap();
    let mut before_target_cycle_lengths: HashMap<String, Option<usize>> = HashMap::new();
    for module in modules.iter() {
        let mut destination_refs: Vec<ModuleRef> = Vec::new();
        for destination_name in module.borrow().destination_names().unwrap() {
            let module_destination: ModuleRef = modules
                .iter()
                .find_map(|module_finder| {
                    if module_finder.borrow().name.as_str() == destination_name {
                        Some(module_finder.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    Rc::new(RefCell::new(Module::new(
                        destination_name.to_string(),
                        None,
                        None,
                    )))
                });
            if let Some(ModuleType::Conjunction(conjunction)) =
                &mut module_destination.borrow_mut().module_type
            {
                conjunction.add_signal_sender(module.clone());
            }
            destination_refs.push(module_destination);
            if destination_name == &before_target_destination_module.borrow().name {
                before_target_cycle_lengths.insert(module.borrow().name.clone(), None);
            }
        }
        module.borrow_mut().destinations = Some(ModuleDestinations::ModuleRefs(destination_refs));
    }

    let mut button_presses = 0;
    loop {
        if let Some(all_cycle_lengths) = before_target_cycle_lengths
            .iter()
            .map(|(_, &cycle_length)| cycle_length)
            .collect::<Option<Vec<usize>>>()
        {
            break lcmm(all_cycle_lengths);
        }
        button_presses += 1;
        let mut module_signal_queue: VecDeque<ModuleRef> = VecDeque::from([broadcaster.clone()]);
        loop {
            let Some(to_signal) = module_signal_queue.pop_front() else {
                break;
            };
            let Some(signal_result) = send_signal(to_signal.clone()) else {
                continue;
            };
            for module in signal_result.to_queue {
                if signal_result.signal_type == SignalType::High {
                    if let Some(cycle_count) =
                        before_target_cycle_lengths.get_mut(&to_signal.borrow().name)
                    {
                        *cycle_count = Some(button_presses);
                    }
                }
                module_signal_queue.push_back(module);
            }
        }
    }
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
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        assert_eq!(result, 11687500);
    }
}

mod utils {
    fn gcd(a: usize, b: usize) -> usize {
        let mut max = a;
        let mut min = b;
        if min > max {
            std::mem::swap(&mut max, &mut min);
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / gcd(a, b)
    }

    pub fn lcmm(nums: Vec<usize>) -> usize {
        nums.into_iter().reduce(lcm).unwrap()
    }
}
