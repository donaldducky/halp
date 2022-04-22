# Lua

## Stylua

Lua Formatter

```sh
# install using asdf
# Alternatively, add it to [$HOME/.default-cargo-crates](https://github.com/code-lever/asdf-rust#default-cargo-crates)
# to have it installed whenever a new rust version gets installed.
cargo install stylua
asdf reshim rust

# running
stylua <files_or_folders>...
```

[Configuration](https://crates.io/crates/stylua#user-content-configuration)

```toml
# stylua.toml
indent_type = "Spaces"
indent_width = 2
quote_style = "AutoPreferSingle"
```
