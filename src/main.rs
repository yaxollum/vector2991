use rustyline::error::ReadlineError;
use rustyline::Editor;
use vector2991::run;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match run(&line) {
                    Ok(ast) => {
                        print!("{}", ast);
                    }
                    Err(e) => {
                        eprintln!("SyntaxError: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("PromptError: {:?}", err);
                break;
            }
        }
    }
}
