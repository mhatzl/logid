# logid

Rust logging framework using IDs to identify log events.

## Using `logid`

```Rust
use logid::{log_id::{LogId, LogLevel}, err, ErrLogId};
use thiserror::Error;

#[derive(Debug, Clone, ErrLogId, Error)]
enum CrateError {
    #[error("`SomeError` description.")]
    SomeError,

    #[error("`InternalError` description.")]
    InternalError,
}

fn my_func() -> Result<(), CrateError> {
    // some code ...
    
    // on error
    err!(CrateError::SomeError)
}
```

## Contributing

There is not yet a contributing guideline, but feel free to create issues and/or pull requests.

Note that any contribution submitted to `logid` is going to be MIT licensed.

## License

MIT Licensed
