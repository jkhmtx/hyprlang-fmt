# Contributing

Below is the guide for working in this repository.

## Setting up your machine

Use the `./env.sh` script to install all required dependencies.

1. Read the script before executing it - it's not that long :)
2. Make sure you follow the instructions that accompany the output.

### `direnv` and `.envrc`

The script above will install `direnv`. You can read more about it [here](https://direnv.net/).

When you change branches or pull latest changes, make sure to run `direnv allow`.

The `.envrc` is the script that is executed when running `direnv allow`. To modify the developer environment, you can change the `shell` output in `flake.nix`.

## Filing Issues

Please be nice! This is a hobby project.

Pull requests are welcome, but only considered for merge after an issue is raised.

Feature additions require a discussion before a pull request will be considered. The lead time for acknowledgement may be 1-2 weeks.

Bug reports are always welcome. If filing a bug report, please see if your bug is already discussed in another issue, first.
