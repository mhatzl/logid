# logid

Rust logging framework using IDs to identify log entries.

## Motivation
### Split User and Program Information

In Rust, many crates have adopted the concept to return `Result<T, dyn std::error::Error>` if execution of a function might fail.
This allows a somewhat flexible error handling, but why is it even necessary to forward the full underlying error?

The approach of `logid` is to minimize the information, that is returned in case of failed execution, so that
the caller is able to react accordingly. In the case of `logid`, it was decided to only return a `LogId` number.
This number is used to uniquely identify errors, warnings and more. Using this approach, the return type
might look like `Result<T, LogId>`.




## Using `logid`


## Usage Guidelines


## Contributing



## License


