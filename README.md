# Novu's Rust SDK

> _NOTE: THIS PROJECT IS NOT CURRENTLY IN CRATES.IO_

This is the **Blazingly Fast** (not really) SDK for [novu](https://github.com/novuhq/novu). This SDK should be used in your server-side code. 🚀

## TODO

- [x] Event Triggering
- [ ] Event broadcasting
- [ ] Canceling Triggered events
- [ ] Subscribers (Work In Progress)
- [ ] Activity
- [ ] Integrations
- [ ] Notification Templates
- [ ] Notification Groups
- [ ] Template Changes
- [ ] Environments
- [ ] Feeds
- [ ] Documentation (docs.rs)

## Quick Start

### Requirements

1. Rust
1. An API key for novu. You can get one [here](https://web.novu.co) by opening settings and clicking "Api Keys" tab. Or by using your selfhosted instance 🙂

### Triggering a notification

~~You need to add the `novu` dependency to your `Cargo.toml`. This can be done with `cargo add novu`.~~ Currently not possible.

We currently only support async/await.

```rust
// Here you create the instance
// If you're selfhosting, You can replace `None` with `Some("your selfhosted novu instance api url")`
let novu = Novu::new("<your api key>", None).unwrap();

let result = novu
    .trigger(TriggerPayload {
        name: "<your template name>".to_string(),
        payload: HashMap::new(),
        to: TriggerRecipientsType::Single(
            TriggerRecipientBuilder::new("<your subscriber id>")
                .first_name("<first name>") // Optional
                .email("<email>") // Optional
                .build(),
        ),
    })
    .await;

// Here you can handle the outcome.
match result {
    Ok(event) => {
        println!(
            "Notification sent!!! \n\nack: {}\nstatus: {}\ntransaction_id: {}",
            event.acknowledged, event.status, event.transaction_id
        )
    }
    Err(api_error) => println!("An error occurred: {}", api_error),
}
```

#### Things you need to done

- Replace `<your api key>` with your api key
- Replace `<your template name>` with the template you want to use
- Replace `<your subscriber id>` with your subscriber id

#### Optional

- Add `first_name, email, avatar, phone_number, last_name, etc` to the subscriber

## Contributing

Glad that you want to contribute! 🎉

> Currently we don't have `CONTRIBUTION.md` or `CODE_OF_CONDUCT.md`, but these will be created soon™

API Docs: [docs.novu.co/api](https://docs.novu.co/api)

### Steps to follow

1. Fork the repository
1. Make your changes
1. Test that those work 😄
1. Push the changes to your fork and make a PR (Pull Request)
1. Now someone will review it and hopefully merge it to the repository 🎉🎉

## Support

For support, join [Novu's Discord server](https://discord.gg/novu) or email [me@midka.dev](mailto:me@midka.dev).
