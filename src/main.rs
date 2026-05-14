mod manager;
mod bot;
mod traits;
use crate::manager::cpu_percent::CpuMetrics;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    bot::bot::start(CpuMetrics).await;
}
