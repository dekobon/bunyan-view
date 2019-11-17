[![Build Status](https://travis-ci.org/dekobon/bunyan-view.svg?branch=master)](https://travis-ci.org/dekobon/bunyan-view)

# Bunyan Viewer

This project is a rewrite of the [node-bunyan](https://github.com/trentm/node-bunyan/) bunyan format 
[log viewer CLI tool](https://github.com/trentm/node-bunyan/blob/master/bin/bunyan) done in Rust. 
This is my first Rust project, so expectations should be set accordingly.

## Divergences

We aim to provide output that is as close as possible to the [node-bunyan](https://github.com/trentm/node-bunyan/)
viewer with a few intentional [divergences](DIVERGENCES.md).

## Testing

When running the automated testing suite, be sure to enable the `dumb_terminal` feature so that colorization is 
disabled. You can do this by invoking `cargo test` as follows:
```
  $ cargo test --features dumb_terminal
```

## License
This project is licensed under the MPLv2. Please see the [LICENSE.txt](/LICENSE.txt)
file for more details.