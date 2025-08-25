pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// TODO: Define and implement `Filter`.

struct Filter<L, P> {
    logger: L,
    predicate: P,
}

impl<L, P> Filter<L, P>
where
    L: Logger,
    P: Fn(u8, &str) -> bool
{
    fn new(logger: L, predicate: P) -> Self {
        Self {
            logger, predicate
        }
    }
}
impl<L, P> Logger for Filter<L, P>
where
    L: Logger,
    P: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        if (self.predicate)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

fn apply_and_log(
    func: impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("Calling {func_name}({input}): {}", func(input))
}

pub fn main() {
    let double_it = |n| n*2;
    dbg!(double_it(50));

    let add_1f32 = |x: f32| -> f32 { x + 1.0 };
    dbg!(add_1f32(10.));

    // Let’s play with mutability
    // let mut max_value = 5;
    // let mut clamp = move |v| {
    //     // the `move` keyword forces the closure to move values instead of referencing them (borrowing them)
    //     // By default, closures will capture each variable from an outer scope by the least demanding form of access they can
    //     // (by shared reference if possible, then exclusive reference, then by move).
    //     // The move keyword forces capture by value.
    //     max_value += 1;
    //     dbg!(max_value);
    //     if v > max_value { max_value } else { v }
    // };

    // dbg!(max_value);

    // dbg!(clamp(1));
    // dbg!(clamp(3));
    // dbg!(clamp(5));
    // dbg!(clamp(7));
    // dbg!(clamp(10));

    // dbg!(max_value);

    let suffix = "-itis";
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appendix");

    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");

    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");

    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}