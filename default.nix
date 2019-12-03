with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "env";
  buildInputs = [
    bashInteractive
    cargo
    gnuplot
  ] ++ stdenv.lib.optional stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];
}
