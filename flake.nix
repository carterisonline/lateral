{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-manifest = {
      url = "https://static.rust-lang.org/dist/2021-10-21/channel-rust-nightly.toml";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      rust-manifest,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            fenix.packages.x86_64-linux.complete.toolchain
            cargo-bootimage
            rust-analyzer
            qemu
          ];
        };
      }
    );
}
