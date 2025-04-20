# Contributing

First off, thanks for taking the time to contribute — whether it’s fixing a bug, adding a feature, or just sharing feedback, it’s appreciated.

While this project started as a personal learning experiment, it's grown into a minimal but solid foundation for building actor-based services with Hyper and Shuttle. Contributions are welcome, even if it's just to make the template better for the next person.

## ✍️ Ways to Contribute

- Fix typos or clarify documentation
- Suggest improvements to project structure or architecture
- Report bugs in the routing or actor behavior
- Add tests to cover edge cases
- Improve error handling or response formatting
- Generalize helper utilities for broader use

## 🛠 Project Philosophy

- Simplicity over abstraction — this template aims to be approachable
- Test-driven — every endpoint and failure case should be verifiable
- Actor-based — system state is encapsulated and communicated over channels
- Explicit error handling — no `unwrap()` in production logic

## 🧪 Before You Submit

1. Clone and build the project:
   ```bash
   cargo build
   ```

2. Run the tests:
   ```bash
   cargo test
   ```

3. If possible, add a test for any new feature or fix.

## 🧹 Code Style

- Follow idiomatic Rust conventions
- Prefer small, focused commits
- Add comments to any non-obvious logic

## 💬 Communication

This is a solo project for now, so PRs may be reviewed on a best-effort basis. If you're unsure about something, feel free to open an issue or draft PR and start a discussion.

Thanks again!
