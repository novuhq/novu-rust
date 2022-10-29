extern crate novu;
use novu::{
    events::{TriggerPayload, TriggerRecipient, TriggerRecipientsType},
    Novu,
};
use std::collections::HashMap;
#[async_std::main]
async fn main() {
    let novu = Novu::new("d96d718b4df9785d62a4f6348e8a5e30", None::<String>).unwrap();
    let result = novu
        .trigger(TriggerPayload {
            name: "ritta-test".to_string(),
            payload: HashMap::new(),
            to: TriggerRecipientsType::Single(
                TriggerRecipient::new("1").first_name("Midka").build(),
            ),
        })
        .await;

    match result {
        Ok(_) => {}
        Err(api_error) => println!("{}", api_error),
    }
}
