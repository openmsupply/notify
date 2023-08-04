# Notify Backend

This is the backend for `Notify` written in Rust.
It provides a GraphQL API used by Users to manage notifications setup.
As well as the logic to send notifications via email and telegram

## Exploring GraphQL Api

When served locally, visit: http://localhost:8001/graphql

You can also tools such as GraphiQL (use the same URL as above): https://graphiql-online.com/

## Tests

- To run all tests:

```bash
cargo test
```

- To run email tests:
  Start mailhog (or similar email capture service).
  For install instructions visit: https://github.com/mailhog/MailHog

```
cargo test --features=email-tests --package service --lib -- email::email_test --nocapture
```

## Email

TODO

##

## SSL/https

Notify is designed to run behind a SSL proxy such as [Caddy](https://caddyserver.com) or Nginx, so doesn't provide SSL support itself.

> WARNING: DO NOT RUN THIS IN PRODUCTION WITHOUT SSL!

## Deployment
