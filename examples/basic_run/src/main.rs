extern crate novu;
use novu::{
    events::{TriggerPayload, TriggerRecipient, TriggerRecipientsType},
    Novu,
};
use std::collections::HashMap;
#[async_std::main]
async fn main() {
    let novu = Novu::new("", None::<String>).unwrap();
    let result = novu
        .trigger(TriggerPayload {
            name: "testing".to_string(),
            payload: HashMap::new(),
            to: TriggerRecipientsType::Single(
                TriggerRecipient::new("1").first_name("Midka").build(),
            ),
        })
        .await;

    match result {
        Ok(event) => {
            println!(
                "Notification sent!!! \n\nack: {}\nstatus: {}\ntransaction_id: {}",
                event.acknowledged, event.status, event.transaction_id
            )
        }
        Err(api_error) => println!("An error occurred: {}", api_error),
    }
}
