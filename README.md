# AXUM TODO APP

## Tech Stack
- *Language:* Rust
- *Framework:* Axum
- *Database:* PostgreSQL
- *DB interaction:* sqlx

Simple TODO App implemented in Rust.

## Project Setup
1. Clone the repo onto your device
2. Create a .env file and add the following code.
This will add the database URL for your Postgres server
```
DATABASE_URL=postgres://<username>:<password>@localhost/<database_name>
```
3. Open your terminal and set the migrations using the following command
```
sqlx migrate run 
```
4. You can revert the migrations using the following command
```
sqlx migrate revert
```
5. Use cargo run to build and run the program
```
cargo run
```

## APIs

| Method   | URL                                      | Description                              |
| -------- | ---------------------------------------- | ---------------------------------------- |
| `GET`    | `/todos`                                 | Retrieve all posts.                      |
| `POST`   | `/todos`                                 | Create a new todo.                       |
| `GET`    | `/todo/<title>`                          | Retrieve todo with title: <title>        |
| `PUT`    | `/todo/<title>`                          | Update data of todo with title: <title>  |
| `DELETE`   | `/todo/<title>`                        | Delete todo with title: <title>          |

## Future Work

- Integration with frontend using a framework like Yew
- Adding middlewares
- Adding JWT Authentication

