{
  pkgs,
}:
pkgs.mkShell {
  packages = [
    (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ]
  ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv ]);
}
