{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  outputs = { self, nixpkgs, flake-utils, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.73.0";
          packageFun = import ./Cargo.nix;
        };
      in
      {
        packages.default = rustPkgs.workspace.shellac { };
        devShells.default = rustPkgs.workspaceShell {
          packages = [ cargo2nix.packages.${system}.default ];
        };
      }
    );
}
