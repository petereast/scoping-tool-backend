# scoping-tool
A little web-app to help scoping sessions, written in Rust, using actix-web and simple event-based state logic.

## Running the service
### On your local machine
Assuming you've got cargo installed, you should just be able to use `cargo run`, or if you've just downloaded the binary, you should be able to do './scoping-tool-backend'.

### Through docker
1. First, build the container using `docker build -t scoping-tool-backend .`
2. Then run the container using `docker run scoping-tool-backend`

### Using frontent
TODO

## API
This tool has the following endpoints:

### POST /start-new-session
With the payload
```json
{
  "title": "A natural language title for the session",
  "description": "A natural language description for the session"
}
```
Will return:
```json
{
  "ok": true
}
```

### POST /end-session
With the payload
```json
{
  "id": "{some uuid}"
}
```
Will return:
```json
{
  "ok": true
}
```
This endpoint will always return the same thing, regardless of if the session exists or not so that it can not be used to infer existing sessions.
