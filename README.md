# logid

Rust logging framework using IDs to identify log events.

## Using `logid`

```Rust
use logid::err;
use logid_derive::ErrLogId;
use thiserror::Error;

#[derive(Debug, Default, Clone, Error, ErrLogId)]
enum CrateErrors {
    #[error("`SomeError` description.")]
    SomeError,

    #[error("`InternalError` description.")]
    #[default]
    InternalError,
}

fn my_func() -> Result<(), CrateErrors> {
    // some code ...

    // on error
    err!(CrateErrors::SomeError)
}
```

## Contributing

There is not yet a contributing guideline, but feel free to create issues and/or pull requests.

Note that any contribution submitted to `logid` is going to be MIT licensed.

## License

MIT Licensed
