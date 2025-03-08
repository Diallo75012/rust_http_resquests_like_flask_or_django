## Well commented code to see:
- how to create a route
- how to create a get and post request to same route-path
- how to create more than one route
- how to create structured output using `serde`, `struct` and `Json`

Use this boilerplate for future project to integrate llm call for example
`still need Deserialize` as we use only `Serialize`

# dependencied
- install `axum`, `serde`, `tokio` (Japan!)
```rust
cargo add axum
cargo add serde --feature derive
cargo add toki --feature full
# optionally if needed
cargo add serde_json
```
- this one to refresh when changing code without needing to manually recompile
```rust
cargo watch -x "run"`
```
