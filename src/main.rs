use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::env;
use regex::Regex;
use std::path::{Path, PathBuf};

struct Context {
    current_directory: PathBuf,
    current_user: String,
    args: String,
    default_directory: PathBuf,
}

fn cd(context: &mut Context) -> Result<Option<String>, String> {
    context.current_directory = match context.args.len() {
        0 => Path::new(&context.args).to_path_buf(),
        _ => Path::new(&context.default_directory).to_path_buf(),
    };
    Ok(None)
}

fn up(context: &mut Context) -> Result<Option<String>, String> {
    match context.current_directory.parent() {
        Some(parent) => context.current_directory = parent.to_path_buf(),
        None => return Err(format!("'{}' doesn't exist", context.args)),
    }
    Ok(None)
}

fn ls(context: &mut Context) -> Result<Option<String>, String> {
    match std::fs::read_dir(&context.current_directory) {
        Ok(read_dir) => {
            let output = read_dir
                .filter_map(|x| x.ok())
                .map(|x| x.path().display().to_string())
                .collect::<Vec<String>>()
                .join("\n");
            Ok(Some(output))
        }
        Err(_) => Err(format!("Couldn't read directory '{}'", &context.current_directory.display())),
    }
}

fn main() {
    let mut context = Context {
        default_directory: env::current_dir().unwrap(),
        current_directory: env::current_dir().unwrap(),
        current_user: "Stefano".to_string(),
        args: String::new(),
    };
    let mut tools: HashMap<&str, fn(&mut Context) -> Result<Option<String>, String>> = HashMap::new();
    tools.insert("pwd", |c: &mut Context| Ok(Some(format!("{}", c.current_directory.display()))));
    tools.insert("cd", cd);
    tools.insert("ls", ls);
    tools.insert("up", up);

    let line_parser = Regex::new(r"^([^ ]+) ?(.+)?$").unwrap();
    loop {
        print!("{}:{}$ ", context.current_user, context.current_directory.display());
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        s = s.trim().to_string();
        let captures = line_parser.captures(&s).unwrap();
        let tool = &captures[1];
        context.args = match captures.get(2) {
            Some(capture) => capture.as_str().to_string(),
            _ => String::new(),
        };
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
