{
  description = "Rust development shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { 
    self, 
    nixpkgs,
    fenix,
  }: 
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [fenix.overlays.default];
    };
    rust = with fenix.packages.${system}; combine [
      stable.cargo
      stable.clippy
      stable.rust-src
      stable.rustc
      stable.rustfmt
      stable.rust-analyzer
      targets.wasm32-unknown-unknown.stable.rust-std
    ];
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
          rust
          pkgs.bacon
          pkgs.wasm-pack
      ];
    };
  };
}
