# pgrx script adapted from https://github.com/pgcentralfoundation/pgrx/blob/develop/nix/extension.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
        lib = pkgs.lib;
        pwd = builtins.toString ./.;

        rustToolchain = pkgs.rust-bin.stable."1.81.0".default;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
        rustPlatform = pkgs.makeRustPlatform { cargo = rustToolchain; rustc = rustToolchain; };
      in rec {
        devShells = {
          default = craneLib.devShell {
            buildInputs = [
              pkgs.protobuf
              pkgs.kind
            ];
          };
        };
      });
}
