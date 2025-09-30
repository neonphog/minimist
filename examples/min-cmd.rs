use minimist::Minimist;

fn get_args() -> Minimist {
    let mut args = Minimist::parse(std::env::args_os().skip(1));

    // if not in args, get cmd from env
    args.set_default_env(Minimist::POS, "MIN_CMD");
    // if still not set, default to help
    args.set_default(Minimist::POS, "help");

    args.set_default_env("name", "MIN_NAME");
    args.set_default_env("fruit", "MIN_FRUIT");

    if args.as_flag("v") {
        args.set_default("verbose", "true");
    }

    args
}

fn help() {
    println!("usage min-cmd [cmd] [options]");
    println!("(use MIN_CMD= to set cmd via environment variable)");
    println!("");
    println!("-v --verbose      : print extra info");
    println!("");
    println!("help              : print this help");
    println!("");
    println!("speak             : say something to stdout");
    println!("  --name  <NAME>  : specify the name to use  (env: MIN_NAME=)");
    println!("  --fruit <FRUIT> : specify the fruit to use (env: MIN_FRUIT=)");
}

fn speak(args: Minimist) {
    let name = args
        .to_one_str("name")
        .expect("min-cmd: --name (or env: MIN_NAME) is required");
    let fruit = args
        .to_one_str("fruit")
        .expect("min-cmd: --fruit (or env: MIN_FRUIT) is required");

    println!("Hello, {name}, have a(n) {fruit}!");
}

fn main() {
    let args = get_args();

    if args.as_flag("verbose") {
        println!("{args:#?}");
    }

    match args.to_one_str(Minimist::POS).unwrap().as_ref() {
        "speak" => speak(args),
        "help" => help(),
        cmd => {
            eprintln!("unknown command: {cmd}");
            help();
        }
    }
}
