pub trait MetricsProvider: Send + Sync {
    fn get(&self) -> f64;
}
