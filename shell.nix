let
  rust-overlay =
    import (builtins.fetchGit {
      url = "https://github.com/oxalica/rust-overlay.git";
      rev = "84c58400556c1c5fa796cbc3215ba5bbd3bd848f";
    });
  nixpkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in
with nixpkgs; pkgs.mkShell {
  nativeBuildInputs = [
    rust-bin.stable.latest.minimal
    pkg-config
  ];

  buildInputs = [ openssl ];
}
