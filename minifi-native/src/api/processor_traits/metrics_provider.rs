pub trait MetricsProvider {
    fn calculate_metrics(&self) -> Vec<(String, f64)> { vec![] }
}