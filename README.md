# Diesel

## Commands

```bash
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli --no-default-features --features sqlite

diesel migration run
diesel migration redo

cargo run --bin show_posts
cargo run --bin write_post
```

## Sqlite

```bash
sqlite3
.open test.sqlite3

.schema posts
SELECT * FROM posts;
INSERT INTO table (column1,column2,..) VALUES(value1,value2,...);
UPDATE posts SET published = true WHERE id = 1;
```

## Reference

- [Diesel Getting Started](http://diesel.rs/guides/getting-started/)