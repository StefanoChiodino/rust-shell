use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::env;
use regex::Regex;
use std::path::Path;

struct Context {
    current_directory: String,
    current_user: String,
    args: String,
}

fn cd(context: &mut Context) -> Result<Option<String>, String> {
    let regex = Regex::new(r"[\\/][^\\/]+$").unwrap();
    if context.args.len() > 0 {
        if Path::new(&context.args).exists() {
            context.current_directory = context.args.to_string();
        } else {
            return Err(format!("'{}' doesn't exist", context.args));
        }
    } else {
        context.current_directory = regex.replace(&context.current_directory, "").to_string();
    }
    Ok(None)
}

fn main() {
    let mut context = Context {
        current_directory: env::current_dir().unwrap().display().to_string(),
        current_user: "Stefano".to_string(),
        args: String::new(),
    };
    let mut tools: HashMap<&str, fn(&mut Context) -> Result<Option<String>, String>> = HashMap::new();
    tools.insert("pwd", |c: &mut Context| Ok(Some(format!("{}", c.current_directory))));
    tools.insert("cd", cd);

    let line_parser = Regex::new(r"^([^ ]+) ?(.+)?$").unwrap();
    loop {
        print!("{}:{}$ ", context.current_user, context.current_directory);
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        s = s.trim().to_string();
        // println!("capturing: '{}'", s);
        let captures = line_parser.captures(&s).unwrap();
        let tool = &captures[1];
        context.args = match captures.get(2) {
            Some(capture) => capture.as_str().to_string(),
            _ => String::new(),
        };
        // println!("tool : '{}'", tool);
        // println!("args : '{}'", context.args);
        match tools.get(tool) {
            Some(x) =>
                match x(&mut context) {
                    Ok(Some(text)) => println!("{}", text),
                    Err(error) => println!("Error: '{}'", error),
                    _ => ()
                },
            None => println!("What?")
        }
    }
}
