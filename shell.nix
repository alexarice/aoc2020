{ pkgs ? import <nixpkgs> { }}:

with pkgs;
with lib;

mkShell {
  buildInputs = [
    cargo
    rustc
  ];
}
