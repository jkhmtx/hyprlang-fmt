{pkgs, ...}:
pkgs.writeShellApplication {
  name = "testing-123";
  text = ''
    echo "tessting-123"
  '';
}
