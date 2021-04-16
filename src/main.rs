use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        let usr = message.update.from().unwrap();
        let ans = format!(
            "id:{}\nname:{}\ntext:{}",
            usr.id,
            usr.username.as_ref().unwrap_or(&String::new()),
            &message.update.text().unwrap_or(&String::new())
        );
        let k = KeyboardMarkup::new(vec![
            vec![KeyboardButton::new("A")],
            vec![KeyboardButton::new("B"), KeyboardButton::new("C")],
        ])
        .one_time_keyboard(true);
        let m = ReplyMarkup::Keyboard(k);
        message.answer(&ans).reply_markup(m).await?;
        respond(())
    })
    .await;
}
