{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-pc-windows-msvc" ];
        };
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-xwin
            cargo-tauri
            bun
            nodejs_22
            llvmPackages.llvm
            llvmPackages.clang-unwrapped
            pkg-config
            openssl
          ];

          shellHook = ''
            export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
          '';
        };
      });
}
