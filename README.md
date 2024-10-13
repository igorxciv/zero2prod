`cargo llvm-cov`

`cargo-watch -x check -x test -x run`

`cargo clippy`

`cargo fmt -- --check`

`cargo audit`

`cargo +nightly rustc --profile=check -- -Zunpretty=expanded`

https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part0/.github/workflows


```
export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
sqlx migrate add create_subscriptions_table
```