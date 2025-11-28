{
  description = "Basic Rust Development Environment using simpleFlake";

  # Flake inputs
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # Flake outputs
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:

    flake-utils.lib.simpleFlake {
      inherit self nixpkgs;
      name = "simple-flake";
      overlay = import rust-overlay;
      shell = ./shell.nix;
    };
}
