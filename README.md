# Web Service Tutorial in Rust

## Overview

This project serves as a comprehensive tutorial for building a web service using the Rust programming language. It covers various aspects, from setting up a PostgreSQL database to handling JSON responses.

## Technologies Used

- **Rust**: The core programming language used for this project.
- **Tokio**: An asynchronous runtime for Rust.
- **SQLx**: A Rust library for connecting to PostgreSQL databases.
- **Dotenv**: A Rust crate for handling `.env` files.
- **Serde**: A Rust crate for serializing and deserializing data structures.
- **Actix-Web**: A Rust framework for building web services.
- **Chrono**: A time manipulation library for Rust.

## Documentation

To generate documentation for this project:

```bash
cargo doc --open
```

Alternatively, you can find documentation for each crate on [docs.rs](https://docs.rs/).

## Getting Started

### Prerequisites

- Ensure you have Docker Compose installed. If not, you can download it from [Docker Desktop](https://www.docker.com/products/docker-desktop).

### Setup

1. **Environment Variables**: Copy the sample environment file and configure the variables.

    ```bash
    cp env.example .env
    ```

2. **Docker Compose**: Start the Docker containers.

    ```bash
    docker compose up -d
    ```

3. **Test Routes**: Open your browser or use a tool like Postman to hit the following route:

    ```
    http://127.0.0.1:8080/blog
    ```

### Database GUI (PgAdmin4)

- Access the PgAdmin4 interface at `http://localhost:16543`.
- Username: `test@test.com`
- Password: `test`

For detailed instructions on adding a PostgreSQL server in PgAdmin4, refer to the [PG Admin guide](https://onexlab-io.medium.com/docker-compose-postgres-initdb-ba0021deef76).

### SQL Schema

The `init.sql` file contains the SQL statements that define the database schema. Feel free to explore it to understand the database structure.

### Authentication

The `SKIP_AUTH` environment variable controls JWT authentication. Set it to `true` to disable JWT during development.

## Additional Resources

- [Crates.io Package](https://crates.io/crates/webservice_tutorial)
- [PG Admin Guide](https://onexlab-io.medium.com/docker-compose-postgres-initdb-ba0021deef76)
