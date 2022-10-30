extern crate novu;
use novu::{
    events::{TriggerPayload, TriggerRecipient, TriggerRecipientsType},
    Novu,
};
use std::collections::HashMap;
#[async_std::main]
async fn main() {
    // Again, this has been reset. So hard to not leak keys lol
    let novu = Novu::new("2aea87f0ec8c090c64416f75373b1eb3", None::<String>).unwrap();
    let result = novu
        .trigger(TriggerPayload {
            name: "testing".to_string(),
            payload: HashMap::new(),
            to: TriggerRecipientsType::Multiple(
                [
                    TriggerRecipient::new("1").first_name("Midka").build(),
                    TriggerRecipient::new("midka@ritta.fi")
                        .first_name("Midka")
                        .last_name("Ritta")
                        .email("midka@ritta.fi")
                        .build(),
                ]
                .to_vec(),
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
