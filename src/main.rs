use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let [query, filename] = &args[1..3] {
        println!("{:?}", (query, filename));
    };

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
