# dota-hero-grid

Data structures and a recursive tree layout system for Dota 2 hero grid
configurations. Supports JSON serialization via serde.

## Modules

- **`models`** — core types: `GridMap`, `Grid`, `Category` with positional
  data and hero lookups through `rdotaconstants`.
- **`serials`** — JSON serialization for `GridMap` into the grid config format.
- **`geometry`** — recursive `Node` tree (behind the `geometry` feature).
  Build a tree of `Row`/`Column` splits, then call `layout()` to compute
  positions, and `into_flat()` to collect all `Category` entries with their
  final coordinates.

## Example

```rust
use dota_hero_grid::{GridMap, Grid, Category, serialize};

let mut map = GridMap::new();
let mut grid = Grid::new("example");

let cat = Category::new("str", (10.0, 20.0), (300.0, 150.0));
grid.add_category(cat);
map.add_grid(grid);

let json = serialize(&map).unwrap();
```

With the `geometry` feature:

```rust
use dota_hero_grid::geometry::{Node, Direction};

// column with a row nested inside
let mut root = Node::Column(vec![
    Node::Category(Category::new("top",  (0., 0.), (0., 0.))),
    Node::Row(vec![
        Node::Category(Category::new("left",  (0., 0.), (0., 0.))),
        Node::Category(Category::new("right", (0., 0.), (0., 0.))),
    ]),
]);

root.layout((0.0, 0.0), (1200.0, 800.0));

let categories = root.into_flat();
// categories[0] — top,   pos: (0, 0),     size: (1200, 400)
// categories[1] — left,  pos: (0, 400),   size: (600,  400)
// categories[2] — right, pos: (600, 400), size: (600,  400)
```

## Feature flags

| Flag | Description |
|---|---|
| *(none)* | Core types and serialization only |
| `geometry` | Recursive tree layout (`Node`, `Direction`) |

## License

MIT
