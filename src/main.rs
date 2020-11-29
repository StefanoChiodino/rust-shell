use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::env;

struct Context {
    current_directory: String,
    current_user: String,
    args: String,
}

fn main() {
    let context = Context {
        current_directory: env::current_dir().unwrap().display().to_string(),
        current_user: "Stefano".to_string(),
        args: String::new(),
    };
    let mut tools: HashMap<&str, fn(&Context) -> String> = HashMap::new();
    tools.insert("pwd", |c: &Context| format!("{}", c.current_directory));

    loop {
        print!("{}:{}$ ", context.current_user, context.current_directory);
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        match tools.get(s.trim_end_matches("\n")) {
            Some(x) => print!("{}\n", x(&context)),
            None => print!("What?\n")
        }
    }
}
