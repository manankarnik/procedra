{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        with pkgs;
        {
          devShells.default = mkShell rec {
            nativeBuildInputs = [
              pkg-config
            ];
            buildInputs = [
              prisma-engines
              openssl
              udev
              alsa-lib
              vulkan-loader
              cairo
              gtk3
              wasm-bindgen-cli
            ];
            shellHook = ''
              export PRISMA_SCHEMA_ENGINE_BINARY="${pkgs.prisma-engines}/bin/schema-engine"
              export PRISMA_QUERY_ENGINE_BINARY="${pkgs.prisma-engines}/bin/query-engine"
              export PRISMA_QUERY_ENGINE_LIBRARY="${pkgs.prisma-engines}/lib/libquery_engine.node"
              export PRISMA_FMT_BINARY="${pkgs.prisma-engines}/bin/prisma-fmt"
              export PATH="$PWD/node_modules/.bin/:$PATH"
            '';
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
        }
      );
}
