{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.mdbook

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
