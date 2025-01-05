use std::env;
use std::io;
use std::io::Write;

use jitcalc_rs::{Jit, Soln1, Soln2};

const USAGE: &str = "usage: ./jitcalc-rs [--soln1|--soln2]";

/// Basically just runs `run(&jit(input))` in a loop
fn repl<Interpreter: Jit>() -> Result<(), io::Error> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        // Read user input
        input.clear();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "exit" {
            break;
        }

        // JIT-compile and run the user-supplied program
        match Interpreter::jit(input) {
            Ok(code) => match Interpreter::run(&code) {
                Ok(result) => println!("{result}"),
                Err(e) => eprintln!("error: {e}"),
            },
            Err(e) => eprintln!("error: {e}"),
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let soln = args.get(1).expect(USAGE);

    if soln == "--soln1" {
        repl::<Soln1>()?;
    } else if soln == "--soln2" {
        repl::<Soln2>()?;
    }

    Ok(())
}
