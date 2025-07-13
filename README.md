# hyprlang-fmt

A (the?) formatter for hyprlang.

- Opinionated
- Fast
- Just Works [tm]

## Installation

Install with nix:

```bash
nix profile install github:jkhmtx/hyprlang-fmt
```

Arch/other users will have to build from source, for now, sorry! It's very simple just to install `nix` (the package manager) alongside `pacman`, and run the instructions above.

An AUR package is in the works. In the meantime, please create an issue if you run into "issues", ha: [CONTRIBUTING.md](./CONTRIBUTING.md)

---

## Usage

This formatter was built basically with IDE usage in mind. Therefore, it doesn't read paths (right now), and only operates with `stdin` and `stdout`.

See the [Integration](#Integration) section for more information.

### Command Line

Help:

```bash
hyprlang-fmt --help
```

Format a file:

```bash
cat /path/to/hypr-conf-file | hyprlang-fmt >tmp
mv tmp /path/to/hypr-conf-file
```

Dry run:

```bash
cat /path/to/hypr-conf-file | hyprlang-fmt
```

---

## Integration

### Neovim + `conform.nvim`

1. Make sure that `hyprlang-fmt` is installed and available in your `PATH`
2. Check that filetype detection is on and that `hyrplang` is a recognized filetype by navigating to a hyprlang file (e.g. `~/.config/hypr/hyprland.conf`), and running in command mode:

```
:set filetype?
```

Older versions of neovim may not have `hyprlang` filetype detection out of the box. See `:help vim.filetype.add` and/or [how the `tree-sitter-hyprlang` plugin used to do it](tree-sitter-example).

3. Create a [custom formatter](conform-custom-formatter) for `conform.nvim` and specify the `formatters_by_ft` with `hyprlang`, like so:

```lua
local conform = require("conform")

conform.setup({
    formatters = {
        -- The name 'hyprlang-fmt' is arbitrary, it can be whatever you like
        ["hyprlang-fmt"] = {
            -- This is the name of the binary, so it must match 'hyprlang-fmt'
            command = "hyprlang-fmt",
            inherit = false,
        },
    },
    formatters_by_ft = {
        -- 'hyprlang-fmt' here refers to the key '["hyprlang-fmt"]' in 'formatters' above
        hyprlang = { "hyprlang-fmt" }
    }
})
```

[conform-custom-formatter]: https://github.com/stevearc/conform.nvim?tab=readme-ov-file#customizing-formatters
[tree-sitter-example]: https://github.com/tree-sitter-grammars/tree-sitter-hyprlang/blob/90b3ddf8a85b5ea3d9dc4920fddb16182a192e14/plugin/init.lua#L2

---

## Bug Reports

Please be nice! This is a hobby project.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more information.
