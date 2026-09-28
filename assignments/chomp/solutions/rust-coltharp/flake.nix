{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.default = pkgs.mkShell {
      packages = [
        pkgs.nil
        pkgs.cargo
        pkgs.clippy
        pkgs.rust-analyzer
        pkgs.rustc
        pkgs.rustfmt
        pkgs.libiconv
      ];
      shellHook = ''
        export DYLD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$DYLD_LIBRARY_PATH
      '';
    };
  });
}
