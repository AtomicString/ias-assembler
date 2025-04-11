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

    packages.x86_64-linux.docker = pkgs.dockerTools.buildImage {
        name = "ias-assembler";
        tag = "latest";

        created = "now";
        
        copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ 
              (pkgs.runCommand "web-dir" {} ''
                mkdir -p $out/app/web
                cp -r ${./web} $out/app/web/
              '')
              pkgs.busybox
            ];
            pathsToLink = [ "/" ];
          };

        config = {
          Cmd = [ "/bin/busybox" "httpd" "-f" "-v" "-p" "8000" "-c" "httpd.conf" ];
          WorkingDir = "/app/web";
          ExposedPorts = {
            "8000/tcp" = {};
          };
        };
    };
  };
}
