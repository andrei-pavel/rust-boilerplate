mod config;

fn main() {
    let c = config::configure();
    for i in c {
        println!("{} {}", i.0.as_str().unwrap(), i.1.as_str().unwrap());
    }
}
