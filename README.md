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
- get strict `serde` `serialization` and `deserialization`
By default it is lazy so if `post` request comes with more fields it will just natch the ones
defined in the struct and **ignore** the others. It is `lazy` parsing.
You can make it strict by adding a decorator to the struct under the `serialization/deserialization` one
Which will output an error message and wont also be tricked by using duplicate values..
```rust
#[serde(deby_unknown_fields)]
```
