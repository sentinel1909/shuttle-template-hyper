# Shuttle Template - Hyper

A raw Hyper server, using [Hyper](https://hyper.rs) and hosted on [Shuttle](https://shuttle.rs)

Routes are minimal:

- `/_health` returns a `200 OK` response with empty body
- `/ping` invokes an actor which counts the number of pings and returns `Pong` as the response body
- the PingCountActor keeps track of state internally, this state is not shared amongst the rest of the code
- `/count` returns the total number of times the `/ping` endpoint has been called
- returns `404 Not Found` for any other route
