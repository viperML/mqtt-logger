{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    ...
  }: let
    src = inputs.nix-filter.lib {
      root = ./.;
      include = [
        (inputs.nix-filter.lib.inDirectory "src")
        "Cargo.toml"
        "Cargo.lock"
        "build.rs"
      ];
    };

    cargo-toml = builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"));
  in
    flake-parts.lib.mkFlake {inherit self;} {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = {
        system,
        pkgs,
        config,
        ...
      }: {
        packages = {
          _toolchain_dev = with inputs.fenix.packages.${system}; (stable.withComponents [
            "rustc"
            "cargo"
            "rust-src"
            "clippy"
            "rustfmt"
            "rust-analyzer"
          ]);
        };

        devShells.default = with pkgs;
          mkShell { # Shell with CC
            name = "nh-dev";
            RUST_SRC_PATH = "${config.packages._toolchain_dev}/lib/rustlib/src/rust/library";
            packages = [
              config.packages._toolchain_dev
            ];
          };
      };
    };
}
