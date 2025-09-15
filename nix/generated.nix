{...}: {
  release.push-tags = ../release/scripts/bin/push-tags/main.nix;
  release.secrets.edit = ../release/secrets/scripts/bin/edit/main.nix;
  release.secrets.get-json = ../release/secrets/scripts/bin/get-json/main.nix;
  root.build-and-test = ../scripts/bin/build-and-test/main.nix;
  root.check = ../scripts/bin/check/main.nix;
  root.debug = ../scripts/bin/debug/main.nix;
  root.docs-help = ../scripts/bin/docs-help/main.nix;
  root.docs-rust = ../scripts/bin/docs-rust/main.nix;
  root.docs = ../scripts/bin/docs/main.nix;
  root.fix = ../scripts/bin/fix/main.nix;
  root.format-nix = ../scripts/bin/format-nix/main.nix;
  root.format-rust = ../scripts/bin/format-rust/main.nix;
  root.format = ../scripts/bin/format/main.nix;
  root.format-shell = ../scripts/bin/format-shell/main.nix;
  root.format-yaml = ../scripts/bin/format-yaml/main.nix;
  root.lint-github-actions = ../scripts/bin/lint-github-actions/main.nix;
  root.lint-rust = ../scripts/bin/lint-rust/main.nix;
  root.lint = ../scripts/bin/lint/main.nix;
  root.lint-shell = ../scripts/bin/lint-shell/main.nix;
  root.lint-version = ../scripts/bin/lint-version/main.nix;
  root.local-ci = ../scripts/bin/local-ci/main.nix;
  root.test-rust = ../scripts/bin/test-rust/main.nix;
  root.test = ../scripts/bin/test/main.nix;
  testbed.test-e2e = ../testbed/scripts/bin/test-e2e/main.nix;
}
