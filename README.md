# Shuttle Template - Hyper

A raw Hyper server, using [Hyper](https://hyper.rs) and hosted on [Shuttle](https://shuttle.rs)

Routes are minimal:

- `/health_check` returns a `200 OK` response with empty body
- returns `404 Not Found` for any other route
