projectInputs: {
  rust = projectInputs.pkgs.rust-bin.selectLatestNightlyWith (toolchain:
    toolchain.minimal.override {
      targets = [
        "x86_64-apple-darwin"
        "x86_64-unknown-linux-gnu"
        "aarch64-apple-darwin"
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
  scripts = {
    fix = import ./scripts/bin/fix/main.nix projectInputs;

    format = {
      format-nix = import ./scripts/bin/format-nix/main.nix projectInputs;
      format-rust = import ./scripts/bin/format-rust/main.nix projectInputs;
      format-shell = import ./scripts/bin/format-shell/main.nix projectInputs;
      format-yaml = import ./scripts/bin/format-yaml/main.nix projectInputs;
    };

    lint = {
      lint-github-actions = import ./scripts/bin/lint-github-actions/main.nix projectInputs;
      lint-rust = import ./scripts/bin/lint-rust/main.nix projectInputs;
      lint-shell = import ./scripts/bin/lint-shell/main.nix projectInputs;
    };

    local-ci = import ./scripts/bin/local-ci/main.nix projectInputs;
  };
}
