# Shuttle Template – Hyper

A lightweight, actor-based web server built with [Hyper](https://hyper.rs) and deployable to [Shuttle](https://shuttle.rs). This project is intended as a minimal, extensible template for building robust APIs or services with clean architecture and a test-driven foundation.

## ✨ Features

- Built directly on **Hyper 1.0**
- Uses the **actor model** to manage internal state and isolate logic
- Designed for deployment on **Shuttle’s zero-config hosting**
- Fully covered by **integration + unit tests**
- Unified, structured **JSON response format**
- leverages Shuttle's internal tracing machinery

## 📦 Routes

| Route       | Description                                                                 |
|-------------|-----------------------------------------------------------------------------|
| `/_health`  | Health check. Returns `200 OK` with an empty body.                          |
| `/ping`     | Triggers a `PingActor`, increments a counter, returns `{ "msg": "Pong" }`. |
| `/count`    | Returns the current ping count from the `PingActor`.                        |
| `/metrics`  | Returns aggregated metrics from a dedicated `AnalyticsActor`.               |
| `/echo`     | Echos query parameters back as a JSON object.                               |
| Any other   | Returns `404 Not Found`.                                                    |

## 🧠 Architecture

This template separates responsibilities via lightweight **actors**, each communicating over `tokio::mpsc` or `oneshot` channels. Each actor manages its own internal state and lifecycle. The router dispatches to handler functions that compose state, respond with JSON, and propagate error handling via a unified `ApiError` type.

All responses follow a standard JSON structure:

```json
{
  "msg": "success",
  "content": ...
}
```

Errors are structured similarly:

```json
{
  "msg": "error",
  "error": "Detailed error message"
}
```

## 🧪 Testing

- Integration tests cover every endpoint (success + failure cases)
- The internal query parsing utility is also unit tested
- the analytics actor is unit tested
- Test setup supports custom actor injection and state mocking

## 🚀 Deployment

To deploy on Shuttle:

```bash
shuttle deploy
```

To run locally:

```bash
shuttle run
```

## 🛠️ Extending the Template

This project is designed as a *launchpad*. To start a new app:

1. Use this as a template (manually or with `cargo generate`)
2. Add your own actors, routes, or middleware
3. If you find reusable improvements — PR them back into this template

## 📂 Project Structure

```
src/
├── actors/            # PingActor, AnalyticsActor, etc.
├── routes/            # Route handlers, one per file
├── state.rs           # Shared AppState passed to handlers
├── service.rs         # Hyper + Shuttle integration
├── types.rs           # Common aliases, JSON envelope types
├── utilities.rs       # Small helpers (e.g., query parsing)
tests/
├── api/               # Integration tests per endpoint
```

