{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };

    package = import ./package.nix projectInputs;

    projectInputs = {
      inherit package;
      inherit pkgs;
      projectNamespace = import ./project.nix projectInputs;
    };

    formatter = pkgs.alejandra;
    shell = import ./shell.nix projectInputs;
  in {
    inherit shell;
    inherit (projectInputs.projectNamespace) scripts;

    packages."${system}".default = package;
    formatter."${system}" = formatter;
  };
}
