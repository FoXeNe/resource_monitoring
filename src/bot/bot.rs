use teloxide::prelude::*;
use std::sync::Arc;
use crate::traits::MetricsProvider;

pub async fn start(metrics: impl MetricsProvider + Send + Sync + 'static) {
    let bot = Bot::from_env();
    let metrics = Arc::new(metrics);

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let metrics = Arc::clone(&metrics);
        async move {
            if let Some(_) = msg.text() {
                let usage = metrics.get();
                bot.send_message(msg.chat.id, format!("{:.1}%", usage))
                    .send()
                    .await?;
            }
            Ok::<(), teloxide::RequestError>(())
        }
    })
    .await;
}
