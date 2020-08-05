# Night Library
Small side-project library management system written in Rust to practice Rust.

![Preview](https://raw.githubusercontent.com/SaltyAom/night-library/master/client/assets/preview.png)

## What include
* Basic add, remove, list-all book.
* JWT and refresh token.
* User-book releation (borrow-return)
* Frontend

### Tech Stack:
* Actix Web
* PostgreSQL
* Diesel & r2d2
* GraphQL (using Juniper)
* Preact
* TypeScript

# Set up
The following will help you to setup Night Library.

## Requirement
* Rust 1.39+
* [Diesel Cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli#installation)
* PostgreSQL
* Node.js (Required for changed in frontend)

1. Create database 'night' in Postgres
```bash
$ psql

$ CREATE DATABASE night;
```

2. Done

## Frontend
Frontend is created with Preact using Preact CLI.

On any client-side build, the build will automatically moved to `root directory` aka. `night-library/client`