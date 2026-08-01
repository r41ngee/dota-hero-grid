# dotagrid-cli

A small command-line tool that converts a Dota 2 hero grid configuration
written in TOML into the JSON format used by the game (`hero_grid_config.json`).

Built on top of [dota-hero-grid](https://crates.io/crates/dota-hero-grid).

## Usage

```sh
dotagrid-cli <INPUT> [-o OUTPUT]
```

| Argument | Description |
|---|---|
| `<INPUT>` | Path to a TOML config file (required) |
| `-o, --output` | Output file path. Defaults to `hero_grid_config.json` |

## TOML format

The input file contains one or more grids. Each grid has a name and a list of
categories, where each category defines its position, size, and the heroes it
contains:

```toml
[[grids]]
name = "test"

[[grids.categories]]
name = "123213"
x = 0
y = 500
w = 500
h = 300
heroes = [
    "antimage",
    "pudge",
]
```

## Example

```sh
dotagrid-cli examples/simple.toml -o hero_grid_config.json
```

## License

MIT
