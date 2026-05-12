mod manager;
mod traits;
use crate::traits::MetricsProvider;
use crate::manager::cpu_percent::CpuMetrics;

fn main() {
    let metrics = CpuMetrics;
    println!("{:.1}%", metrics.get());
}
