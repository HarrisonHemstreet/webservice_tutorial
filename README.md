## Purpose of this project:
The purpose of this project is to provide a tutorial for how one can build a webservice using the Rust programming language.

## Included technologies:
* The Rust Programming Language
* tokio (Rust runtime)
* SQLx (for connecting to PostgreSQL)
* dotenv (Rust crate for .env files)
* serde (a crate for working with JSON in Rust)
* actix-web (a framework for creating Rust web services)
* chrono (a helper library built on top of the time crate for a hopefully better API. Interfaces well with actix-web)

## How to generate documentation for this project:
1. run `cargo doc --open`. Running this command will cause Cargo to generate all the relevant documentation for this project. You can also google the documentation via going to docs.rs online and searching for the relevant crate you are interested in.

## How to run this project:
1. run this command: `cp env.example .env`. Make sure you have a `.env` file with the following values set: `DEV_FRONTEND_URL`, `FRONTEND_URL`, `JWT_SECRET`, `JWT_HOURS_ACTIVE`, `SKIP_AUTH`, `MODE`, `DATABASE_URL`. All these values will correlate to how you set up PostgreSQL. Follow the instructions found within the env.sample flie.
2. make sure you have `docker compose` installed (the current version as of writing this is the one lacking the hyphen, so not the `docker-compose` version. easiest way to install is with Docker Desktop)
3. run `docker compose up -d`
4. you should be able to hit the routes `127.0.0.1:8080/blogs` and receive a JSON response containing every blog post we have in the database.
5. if you would like to look at the GUI for PostgreSQL (PgAdmin4), then you can go here: `http://localhost:16543`. The username is `test@test.com` and the password is `test`
  i. to add the PostgreSQL server in PgAdmin4, after logging in click on "Add New Server". On the 'General' tab, name the server anything you would like. Next, select the 'Connection' tab. In place of the 'Host name/address', run this command: `ifconfig | grep inet`, and input one of the output ip addresses. For the 'port', input `5440`. For 'Maintenance database', input `root`. For 'Username', input `root`. For 'Password', input `root`. Finally, if everything has been input correctly, you should be able to hit the save button on the modal and you should be connected. If the `build.rs` and `get_db_url.sh` are working, after running `cargo run`, the correct db uri string should have been created. You can get the values for these variables from this string.
6. in the `init.sql`, you should see various SQL statements. Feel free to poke around at the table definitions to learn more about the foundations of this web service.
7. make sure you take note of the environment variable `SKIP_AUTH`. This env var determines if you will be using JWT authentication / JWT authorization. If you set it to true, the webservice will not take into account JWT authentication / JWT authorization. This setting is very convienent during the rapid development phase of the webservice. Prob a good idea to remove this functionality before using this template in production.

## Crates.io
[crates.io/crates/webservice_tutorial](https://crates.io/crates/webservice_tutorial)

## PG Admin guide
Please follow this guide to get PG Admin up and running [here](https://onexlab-io.medium.com/docker-compose-postgres-initdb-ba0021deef76)
