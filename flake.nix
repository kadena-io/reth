{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in with pkgs; rec {
        # jww (2024-09-17): This `mkShell'` override is needed to avoid a
        # linking problem that only occurs on Intel macOS:
        # ```
        # Undefined symbols for architecture x86_64: "_SecTrustEvaluateWithError"
        # ```
        mkShell' = mkShell.override {
          stdenv = if stdenv.isDarwin then overrideSDK stdenv "11.0" else stdenv;
        };

        devShells.default = mkShell' {
          buildInputs = [
            openssl
            pkg-config
            (rust-bin.beta.latest.default.override {
               extensions = [ "rust-src" ];
             })
            rust-bin.beta.latest.rust-analyzer
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreServices
          ];
        };
      }
    );
}
