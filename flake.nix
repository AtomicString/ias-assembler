{
  description = "Python";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=release-24.11";
  };

  outputs = { self, nixpkgs } : 
  let 
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {
    
    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = with pkgs; [
          python3
          geckodriver
          firefox
          python3Packages.selenium
          busybox
      ];
    };

  };
}
