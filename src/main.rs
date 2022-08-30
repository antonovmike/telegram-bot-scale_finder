use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text, InlineKeyboardButton, KeyboardButton, self, InlineKeyboardButtonKind},
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;

// async fn buttons(api: Ref<Api>) -> Result<(), ExecuteError> {
//     let kind = InlineKeyboardButtonKind::CallbackData("IDK".to_string());
//     let button_a = InlineKeyboardButton::new("Text".to_string(), kind);
//     api.execute(button_a).await;
//     Ok(())
// }

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let slice = &message.data[..];
    let result = match slice {
        "C major" => "C  D  E F  G  A  H C",
        "C dorian" => "C  D Eb  F  G  A Hb  C",
        _ => "Wrong input"
    };
    let method = SendMessage::new(chat_id, result);
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    // let app = App::new(context, buttons);
    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}
