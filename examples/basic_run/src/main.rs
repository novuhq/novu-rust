extern crate novu;
use novu::*;
use std::collections::HashMap;
#[async_std::main]
async fn main() {
    let novu = Novu::new("".to_string(), None).unwrap();
    let result = novu
        .trigger(
            "",
            ITriggerPayloadOptions {
                payload: HashMap::new(),
                to: TriggerRecipientsType::Single("".to_string()),
            },
        )
        .await;
    println!("Result: {:?}", result);

    assert!(result.is_err());
}
