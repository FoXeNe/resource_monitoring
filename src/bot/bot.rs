use teloxide::prelude::*;
use dotenvy::dotenv;
use tokio::runtime::Runtime;
use crate::manager::cpu_percent;

pub fn start() {
    dotenv().ok();

    let rt = Runtime::new().expect("не удалось создать рантайм");

    rt.block_on(async {
        let bot = Bot::from_env();

        teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            if let Some(text) = msg.text() {
                let usage = cpu_percent::get();
                bot.send_message(msg.chat.id, format!("{:.1}%", usage))
                    .send()
                    .await?;
            }
            Ok::<(), teloxide::RequestError>(())
        })
        .await;
    });
}
