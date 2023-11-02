# Notification Event GraphQL API

This crate provides a GraphQL API for managing notification events. It is built with Rust and uses the Juniper library for GraphQL server implementation.

## Project Structure

The project is structured as follows:

- `src/lib.rs`: Main library file.
- `src/schema.rs`: GraphQL schema definition for the `NotificationEvent` model.
- `src/models/notification_event.rs`: `NotificationEvent` model definition.
- `src/resolvers/mutation.rs`: Mutation resolvers for the GraphQL schema.
- `src/resolvers/query.rs`: Query resolvers for the GraphQL schema.
- `src/resolvers/notification_event.rs`: Resolvers for the `NotificationEvent` type.
- `tests/notification_event.rs`: Tests for the `NotificationEvent` model and its resolvers.

## Setup

To set up the project, follow these steps:

1. Install Rust: Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust on your machine.

2. Clone the repository: Clone this repository to your local machine using `git clone`.

3. Build the project: Navigate to the project directory and run `cargo build` to build the project.

4. Run the tests: Run `cargo test` to run the tests and ensure everything is set up correctly.

## Usage

To start the GraphQL server, run `cargo run` in the project directory. This will start the server on `localhost:8000`.

You can then send GraphQL queries and mutations to `localhost:8000/graphql`.

## Documentation

For more detailed documentation, see the comments in the source code. Each module, function, and type has a comment explaining what it does and how to use it.