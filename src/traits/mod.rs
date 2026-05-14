pub trait MetricsProvider {
    fn get(&self) -> f64;
}
