projectInputs: {
  rust = projectInputs.pkgs.rust-bin.selectLatestNightlyWith (toolchain:
    toolchain.minimal.override {
      targets = [
        "x86_64-unknown-linux-gnu"
        "aarch64-unknown-linux-gnu"
      ];
      extensions = [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustc-codegen-cranelift-preview"
        "rustfmt"
      ];
    });

  root = let
    inherit (projectInputs) pkgs;
    inherit (pkgs.lib) isAttrs;
    inherit (pkgs.lib.attrsets) filterAttrs mapAttrs;

    importAttrs = mapAttrs (_: value:
      if isAttrs value
      then importAttrs value
      else (import value projectInputs));
    generated = let
      generated = import ./generated.nix {};
    in (generated.root // filterAttrs (key: _: key != "root") generated);
  in
    importAttrs generated;
}
