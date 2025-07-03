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

    rustStable = pkgs.rust-bin.nightly.latest.default.override {
      extensions = ["rust-src" "cargo" "rustc" "rustc-codegen-cranelift-preview"];
    };

    formatter = pkgs.alejandra;
    shell = import ./shell.nix {
      inherit pkgs;
      inherit rustStable;
    };
    package = import ./package.nix {
      inherit pkgs;
    };
  in {
    inherit shell;

    packages."${system}".default = package;
    formatter."${system}" = formatter;
  };
}
