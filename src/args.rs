const USAGE: &str = "Usage: puke [--max-machines=<#>] [--reorder] [--fail-io] [module file.pk]

Falls back to repl if no module is provided.

Options:
    --max-machines=<#>           Maximum concurrency bound [default: number of cores * 1.2].
    --scramble                   Scramble in-flight messages and scheduling orders.
    --fail-io=<#>                Causes random IO failures on 1 out of every N IO operations.
    --fuzz=<corpus path>         Feeds the provided module fuzzed inputs derived from the provided corpus directory.
";

/// Args for the puke `Interpreter`.
#[derive(Debug, Clone)]
pub struct Args {
    pub max_machines: usize,
    pub scramble: bool,
    pub fail_io: Option<usize>,
    pub fuzz: Option<String>,
    pub module: Option<String>,
}

impl Default for Args {
    fn default() -> Args {
        Args {
            max_machines: ((num_cpus::get_physical() as f64) * 1.2) as usize,
            scramble: false,
            fail_io: None,
            fuzz: None,
            module: None,
        }
    }
}

fn parse<'a, I, T>(mut iter: I) -> T
where
    I: Iterator<Item = &'a str>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    iter.next().expect(USAGE).parse().expect(USAGE)
}

impl Args {
    pub fn parse() -> Args {
        let mut args = Args::default();
        for raw_arg in std::env::args().skip(1) {
            let mut splits = raw_arg[2..].split('=');
            match splits.next().unwrap() {
                "max-machines" => args.max_machines = parse(&mut splits),
                "scramble" => args.scramble = true,
                "fail-io" => args.fail_io = Some(parse(&mut splits)),
                "fuzz" => args.fuzz = Some(parse(&mut splits)),
                other => panic!("unknown option: {}, {}", other, USAGE),
            }
        }
        args
    }
}
