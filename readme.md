# Rust TCP and HTTP Servers — Workspace

A Cargo workspace containing four member crates that progressively build from a raw TCP echo server up to a functioning HTTP server with static pages and a JSON API.

Demonstrates how to build an HTTP server from scratch in Rust — starting from raw TCP sockets and working up layer by layer, without any async runtime or web framework. Covers traits and their implementations (`From`, `Default`, `Read`, `Write`), lifetimes, enums with pattern matching, `Result`-based error handling, a shared library crate consumed across the workspace, trait-based routing, static file serving with content-type detection, and a JSON REST endpoint via `serde`.

```
rssa_web_server/
├── tcpserver/      # Raw TCP echo server
├── tcpclient/      # Raw TCP client
├── http/           # HTTP parsing library (request + response)
└── httpserver/     # Full HTTP server built on top of the http library
```

---

## 1. `tcpserver` — TCP Echo Server

Binds to `127.0.0.1:3000`, accepts connections in a loop, reads up to 1 KB from each client and echoes it back verbatim.

```sh
cargo run -p tcpserver
```

## 2. `tcpclient` — TCP Client

Connects to `127.0.0.1:3000`, sends `"Hello world"`, reads back the first 5 bytes of the response and prints them.

```sh
cargo run -p tcpclient
```

> Start `tcpserver` first, then run `tcpclient` in a separate terminal.

---

## 3. `http` — HTTP Parsing Library

A dependency-free library that converts raw bytes into typed Rust structs and back. Used by `httpserver`.

### Types

**`HttpRequest`** — parses an incoming HTTP request string into:
- `method` — `Method::Get`, `Method::Post`, or `Method::Uninitialized`
- `version` — `Version::V1_1`, `Version::V2_0`, or `Version::Uninitialized`
- `resource` — `Resource::Path(String)`
- `headers` — `HashMap<String, String>`
- `msg_body` — `String`

Implemented via `From<String> for HttpRequest`.

**`HttpResponse<'a>`** — builds an HTTP response from:
- `status_code` — e.g. `"200"`, `"404"`, `"500"`
- `headers` — optional `HashMap<&str, &str>` (defaults to `Content-Type: text/html`)
- `body` — optional `String`

Status text is derived automatically from the status code. Serialized to a wire-format string via `From<HttpResponse> for String`. Written to any `impl Write` stream via `send_response()`.

### Run tests

```sh
cargo test -p http
```

---

## 4. `httpserver` — HTTP Server

A single-threaded HTTP/1.1 server that listens on `localhost:3001`, parses incoming requests, routes them to the appropriate handler, and writes back an `HttpResponse`.

```sh
cargo run -p httpserver
```

### Architecture

```
TcpListener
    └── Server::run()
            └── Router::route()
                    ├── StaticPageHandler   → GET /
                    ├── StaticPageHandler   → GET /health
                    ├── StaticPageHandler   → GET /*.css, /*.js, /*.html
                    ├── WebServiceHandler   → GET /api/shipping/orders
                    └── PageNotFoundHandler → everything else
```

### Routes

| Method | Path                    | Response                          |
|--------|-------------------------|-----------------------------------|
| GET    | `/`                     | `public/index.html`               |
| GET    | `/health`               | `public/health.html`              |
| GET    | `/styles.css`           | `public/styles.css`               |
| GET    | `/api/shipping/orders`  | JSON array from `data/orders.json`|
| any    | unknown path            | `public/404.html`                 |

### Dependencies

- `http` — local workspace library
- `serde` + `serde_json` — JSON serialization for `OrderStatus`

### Static files

Served from `httpserver/public/`. The `PUBLIC_PATH` environment variable overrides the default path.

### JSON data

`httpserver/data/orders.json` — sample order list served at `/api/shipping/orders`. The `DATA_PATH` environment variable overrides the default path.
