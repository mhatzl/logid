# logid

Rust logging framework using IDs to identify log entries.

## Motivation
### Split User and Program Information

In Rust, many crates have adopted the concept to return `Result<T, dyn std::error::Error>` if execution of a function might fail.
This allows a somewhat flexible error handling, but why is it even necessary to forward the full underlying error?

The approach of `logid` is to minimize the information, that is returned in case of failed execution, so that
the caller function is able to react accordingly. In the case of `logid`, it was decided to only return a `LogId` number.
This number is used to uniquely identify errors, warnings and more. Using this approach, the return type
might look like `Result<T, LogId>`. Therefore, the `LogId` may be used to handle the program flow without the need to
send the full error information with it.

The error information that is added to describe it is mostly for users of the program, but not for the program itself.
Therefore, it was decided to set this user centered information via [`tracing`](https://github.com/tokio-rs/tracing),
and link them together using the `LogId`. 

Besides errors, `LogId`s may also be used to set warnings, information, or debug information.
The principle is always the same. A `LogId` identifies the severity and links set tracing events together.

### Capture `LogId` Information

Tracing events represent single points in time during program execution.
Since additional information for `LogId`s are each set as an event, an optional
map may be used to capture all set `LogId`s with their additional information.

It is possible to use the built-in map of the `logid` crate, or provide a custom one
for more control.

The map may at some point be drained. All captured entries of set `LogId`s so far that were finalized are
returned, and removed from the map. `MappedLogId`s are finalized either manually using `finalize()`,
or automatically after a `MappedLogId` goes out of scope.

Finalizing means that no more information will be added to a `MappedLogId`, making
an entry safe to inspect. An entry that was finalized is marked as `drainable`.

## Using `logid`

At first, a `LogId` must be created. The function `get_log_id` may be used for this.
The function uses bit-shifting to arrange `LogId`s according to severity and source position.
Since `LogId` is a wrapper around `isize`, it is possible to assign a value to an enum,
and later convert it into a `LogId`.

With the `LogId` created, the next step is to set an initial event, and optionally
map the `LogId`. There are three variants available.

1. `set_event` ... Uses the built-in map to capture the `LogId`
2. `set_event_with` ... Uses a given map to capture the `LogId`
3. `set_silent_event` ... Sets a trace without capturing the `LogId`

After setting the event, it is possible to chain additional information to set `LogId`.
Those functions all start with `add_`.

The following example shows the usage as return value:

~~~Rust
const SOME_ERROR: LogId = get_log_id(0, 0, EventLevel::Error, 0);

fn my_func() -> Result<usize, LogId> {
  // some code ...

  // on error
  Err(SOME_ERROR.set_event("Some error message", file!(), line!())
      .add_cause("Cause of error -> unknown").finalize()  
  )
}
~~~

## Usage Guidelines

The following guidelines should help to ease the integration of `logid`,
and help standardize the use across crates.

- Write *user focused* message in `set_event` calls

  The message given to the `set_event` functions should be written
  in a way that users of the crate understand the message.

  Further *developer focused* information should be added using
  the `add_*` functions.

  This differentiation is useful to create more meaningful messages to
  both user and developer.

- Create tracing `span`s on public functions

  `span`s in tracing are useful to trace the program execution flow.
  Creating a `span` on public function calls helps to keep track
  of crate borders.

- Use enums for `LogId`s

  Creating enums for `LogId`s and grouping them per severity
  is useful especially for errors to have better matching capabilities.

## Contributing

Feel free to create issues and pull requests.
However, feedback about the general concept is of greater value at this stage of development.

Note that any contribution intentionally submitted to `logid`
is going to be MIT licensed.

## License

MIT Licensed
