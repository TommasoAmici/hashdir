# hashdir

A CLI to calculate a single hash for the contents of a directory.

It uses the `xxh3` algorithm to calculate the hash of each file and it
respsects `.gitignore` files, thanks to the [`ignore` crate](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore).

```sh
hashdir dir
```

Is roughly equivalent to this bash one-liner:

```sh
find dir -type f -print0 | sort -z | xargs -0 sha1sum | sha1sum
```
