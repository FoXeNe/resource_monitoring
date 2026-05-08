mod manager;
mod reader;

fn main() {
    let res = manager::cpu_percent::get();
    println!("{}", res);
}
