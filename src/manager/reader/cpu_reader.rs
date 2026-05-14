use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read() -> (u64, u64) {
    let file = File::open("/proc/stat")
        .expect("не получается прочитать /proc/stat. проверьте права доступа");
    let reader = BufReader::new(file);

    let mas: Vec<u64> = reader.lines()
        .next()
        .expect("файл пустой")
        .expect("ошибка чтения")
        .split_whitespace()
        .skip(1)
        .map(|i| i.parse::<u64>().expect("ошибка парсинга"))
        .collect();
    let total: u64 = mas.iter().sum();
    let idle: u64 = mas[3];
    (total, idle)
}
