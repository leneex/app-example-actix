# Actix

Example [Actix](https://actix.rs/) app. Actix Web is a fast, practical web framework for Rust.

## Deploy settings

| Field | Value |
| --- | --- |
| Build | Railpack |
| Project directory | `/` |
| Install command | leave empty |
| Build command | leave empty |
| Start command | leave empty |
| Application port | `7001` |
| Health check path | `/` |

## <a name="dev"></a>Run locally

```bash
# install dependencies
cargo fetch

# run in dev mode
cargo run

# build for production
cargo run --release
```
