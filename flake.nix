{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
  };

  outputs = { self, nixpkgs }: let pkgs = import nixpkgs {system = "x86_64-linux";}; in
  with pkgs; {
    devShells.x86_64-linux.default = mkShell {
      buildInputs = [
        rustc
        rustfmt
        clippy
        cargo
        rust-analyzer
        bacon
      ];
    };
  };
}
