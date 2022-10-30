extern crate novu;
use novu::{
    events::{AllowedPayloadValues, TriggerPayload, TriggerRecipient, TriggerRecipientsType},
    Novu,
};
use std::{collections::HashMap, env};
#[async_std::main]
async fn main() {
    let novu = Novu::new(env::var("API_TOKEN").unwrap(), None::<String>).unwrap();

    let mut payload: HashMap<String, AllowedPayloadValues> = HashMap::new();
    payload.insert(
        "name".to_string(),
        AllowedPayloadValues::STRING("Midka".to_string()),
    );

    let serialized = serde_json::to_string(&payload).unwrap();

    println!("{}", serialized);

    let result = novu
        .trigger(TriggerPayload {
            name: "testing".to_string(),
            payload,
            to: TriggerRecipientsType::Single(
                TriggerRecipient::new("10294729")
                    .first_name("Midka")
                    .email("midka@koira.testausserveri.fi")
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
