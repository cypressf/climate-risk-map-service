# MIT Climate Risk Map Web Service

## Configure and setup

Install MySQL and set up a user with privileges to create and edit a database. Doesn't matter what you call the user or the database.

```
CREATE USER 'mit'@'localhost' IDENTIFIED BY '<password>';
GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, DROP, REFERENCES ON `climate_risk_map`.* TO 'mit'@'localhost';
```

Copy `.env.template` to `.env` and replace values with your own, or set them as environment variables. Environment variables overwrite anything in the `.env` file. The `.env` file is an optional convenience mostly for dev builds.


Install [rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), [sqlx-cli](https://github.com/launchbadge/sqlx/tree/HEAD/sqlx-cli), create the database, and run the database migrations.

```
curl https://sh.rustup.rs -sSf | sh
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

## Build and run

`cargo run` to run a dev build

`cargo run --release` to run a release build
