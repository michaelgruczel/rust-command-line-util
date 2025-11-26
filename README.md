# rust-command-line-util

Simple personal highly opinionated Command line util written with Rust.

## build and run it from code

```
$ cd mgutil
$ cargo run -- --command sp --alias testcase
$ cargo run -- --command l
$ cargo run -- --command d --alias testcase
$ cargo fmt
$ cargo build --bin mgutil
$ cd target/debug
$ ./mgutil --command sp --alias rust-cmd-path
$ ./mgutil --command l
```

the util support the following logic

```
# list all bookmarks
mgutil --command l

# safe a path as bookmark
mgutil --command sp --alias <ALIAS> --value <PATH>

# safe current path as bookmark
mgutil --command sp --alias <ALIAS>

# open new iterm shell with directory in bookmark
mgutil --command p --alias <ALIAS>

# delete bookmark
mgutil --command d --alias <ALIAS>
```

