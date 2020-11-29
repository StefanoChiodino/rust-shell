use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

struct Context {
    current_directory: String,
    current_user: String,
    args: String,
}

fn main() {
    let context = Context {
        current_directory: "~".to_string(),
        current_user: "Stefano".to_string(),
        args: String::new(),
    };
    let mut tools: HashMap<&str, fn(&Context) -> String> = HashMap::new();
    tools.insert("pwd", |c: &Context| format!("{}", c.current_directory));

    let mut s = String::new();
    loop {
        print!("{}:{}$ ", context.current_user, context.current_directory);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        print!("{}", s);
        match tools.get(s.trim_end_matches("\n")) {
            Some(x) => print!("{}", x(&context)),
            None => print!("What?\n")
        }
    }
}
