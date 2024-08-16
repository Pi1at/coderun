{
  description = "rust devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs;
          [
            just
            cargo-generate
            cargo-llvm-cov
            cargo-watch
            cargo-semver-checks
            rustToolchain
            pkg-config
            openssl
          ];
        buildInputs = with pkgs; [ ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          # example of shellHook
          # shellHook = ''
          #   export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          #   cargo install cargo-snippet --features="binaries"
          # '';
        };
      }
    );
}
