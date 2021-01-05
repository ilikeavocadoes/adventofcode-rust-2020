with import ./nixpkgs.nix;

mkShell {
  buildInputs = [ rustc cargo rustfmt entr ];
}
