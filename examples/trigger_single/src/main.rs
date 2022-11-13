extern crate novu;
use novu::{
    events::{
        AllowedPayloadValues, TriggerPayload, TriggerRecipientBuilder, TriggerRecipientsType,
    },
    Novu,
};
use std::{collections::HashMap, env};
#[async_std::main]
async fn main() {
    let novu = Novu::new(env::var("API_TOKEN").unwrap(), None).unwrap();

    let mut payload: HashMap<String, AllowedPayloadValues> = HashMap::new();
    payload.insert(
        "name".to_string(),
        AllowedPayloadValues::STRING("Test".to_string()),
    );

    let result = novu
        .trigger(TriggerPayload {
            name: "testing".to_string(),
            payload,
            to: TriggerRecipientsType::Single(
                TriggerRecipientBuilder::new("testing")
                    .first_name("Test")
                    .email("me+test@midka.dev")
                    .build(),
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
