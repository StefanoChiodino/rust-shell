use std::io::{stdout, stdin, Write, Read};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    let mut tools: HashMap<&str, fn(String) -> String> = HashMap::new();
    tools.insert("pwd", |x: String| format!("{}", x));
    let user = "Stefano";
    let current_path = "~";
    let mut s= String::new();
    loop{
        print!("{}:{}$ " , user, current_path);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        print!("{}", s);
        match tools.get(s.trim_end_matches("\n")) {
            Some(x) => print!("{}", x("dsa".to_string())),
            None=> print!("What?\n")
        }
    }
}
