# hyprlang-fmt

A formatter for hyprlang.

## Installation

Install with nix:

```bash
nix profile install github:jkhmtx/hyprlang-fmt
```

---

## Usage

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

## Bug Reports

Please be nice. This is a hobby project.

See [./CONTRIBUTING.md](./CONTRIBUTING.md) for more information.
