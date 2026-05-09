mod manager;
mod reader;
mod bot;

fn main() {
    let res = manager::cpu_percent::get();
    println!("{}", res);
    bot::bot::start();
}
