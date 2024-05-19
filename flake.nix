{
  description = "repo nix flake";
  outputs = {self, ...} @ inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        pkgs,
        lib,
        self',
        inputs',
        ...
      }: {
        devShells.default = with pkgs;
          mkShell {
            nativeBuildInputs = [alejandra treefmt pre-commit rustc clippy rustfmt cargo rust-analyzer pkg-config];
          };
        packages.example = with pkgs;
          rustPlatform.buildRustPackage {
            pname = "procmon";
            inherit ((lib.importTOML ./Cargo.toml).package) version;

            src = lib.sourceByRegex self [
              "(src)(/.*)?"
              "Cargo\.(toml|lock)"
            ];

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with lib; {
              description = "monitor for any changes in a filesystem and automatically restart if changes are found  - perfect for development";
              license = licenses.mit;
              platforms = platforms.linux;
            };

            nativeBuildInputs = [pkg-config];
            buildInputs = [];
          };
        packages.default = self'.packages.example;
      };
    };
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
}
