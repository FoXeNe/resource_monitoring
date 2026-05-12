use crate::manager::reader::cpu_reader::read;
use crate::traits::MetricsProvider;

pub struct CpuMetrics;

impl MetricsProvider for CpuMetrics {
    fn get(&self) -> f64 {
        let (delta_total, delta_idle) = find_delta(); 
        (1.0 - (delta_idle / delta_total)) * 100.0
    }
}

fn find_delta() -> (f64, f64) {
    let (total_start, idle_start) = read();
    std::thread::sleep(std::time::Duration::from_millis(500));
    let (total_end, idle_end) = read();

    let delta_total = total_end - total_start;
    let delta_idle = idle_end - idle_start;

    (delta_total as f64, delta_idle as f64)
}
