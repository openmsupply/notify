# Notify Backend

This is the backend for `Notify` written in Rust.
It provides a GraphQL API used by Users to manage notifications setup.
As well as the logic to send notifications via email and telegram

## Exploring GraphQL Api

When served locally, visit: http://localhost:8001/graphql

You can also tools such as GraphiQL (use the same URL as above): https://graphiql-online.com/

## Tests

- To run all standard tests:

```bash
cargo test
```

- To run email tests:
  Start mailhog (or similar email capture service).
  For install instructions visit: https://github.com/mailhog/MailHog

```
cargo test --features=email-tests --package service --lib -- email::email_test --nocapture
```

- To run telegram tests:
  Create a telegram bot and set the TELEGRAM_TOKEN and TELEGRAM_CHAT_ID environment variables.

```
  cargo test --features=telegram-tests --package telegram --lib -- --nocapture
```

## Email

By default Notify will send emails via a local SMTP server on port 1025.
To change this to your own server, update the `configuration/local.yaml` file with your own settings.

```
mail:
  host: "YOUR_SMTP_SERVER"
  port: "YOUR_SMTP_PORT"
  starttls: true
  username: "YOUR_SMTP_USERNAME"
  password: "YOUR_SMTP_PASSWORD"
  from: "YOUR EMAIL ADDRESS"
```

## Telegram

To configure telegram, you need to create a bot and get a token.

To create a bot open a telegram chat to @BotFather
Create a new bot with the command `/newbot`
BotFather will ask you for a username for the bot, call it something like `notify_YOURNAME_bot`
(The username must end in `bot` and be unique)
All going well you should get a message containing your bot's token.
Add this token to `configuration/local.yaml`.

To use your new bot within telegram groups, you need to disable privacy mode using the command `/setprivacy` in a chat with @BotFather.
See: https://core.telegram.org/bots#6-botfather and https://core.telegram.org/bots/features#privacy-mode

## SSL/https

Notify is designed to run behind a SSL proxy such as [Caddy](https://caddyserver.com) or Nginx, so doesn't provide SSL support itself.

> WARNING: DO NOT RUN THIS IN PRODUCTION WITHOUT SSL!

## Deployment
