use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::env;
use regex::Regex;

struct Context {
    current_directory: String,
    current_user: String,
    args: String,
}

fn cd(context: &mut Context) -> Option<String> {
    let regex = Regex::new(r"[\\/][^\\/]+$").unwrap();
    context.current_directory = regex.replace(&context.current_directory, "").to_string();
    None
}

fn main() {
    let mut context = Context {
        current_directory: env::current_dir().unwrap().display().to_string(),
        current_user: "Stefano".to_string(),
        args: String::new(),
    };
    let mut tools: HashMap<&str, fn(&mut Context) -> Option<String>> = HashMap::new();
    tools.insert("pwd", |c: &mut Context| Some(format!("{}", c.current_directory)));
    tools.insert("cd", cd);

    loop {
        print!("{}:{}$ ", context.current_user, context.current_directory);
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        match tools.get(s.trim_end_matches("\n")) {
            Some(x) =>
                match x(&mut context){
                    Some(text) => print!("{}\n", text),
                    _ => ()
                },
            None => print!("What?\n")
        }
    }
}
