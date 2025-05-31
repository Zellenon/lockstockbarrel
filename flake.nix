{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              alsa-lib
              buildPackages.pkg-config
              buildPackages.clang
              buildPackages.lld
              buildPackages.mold
            ];
            buildInputs = [
              openssl
              pkg-config
              eza
              fd
              pre-commit
              rust-bin.nightly.latest.default
              rustfmt
              systemd
              udev
              vulkan-loader
              xorg.libX11
              x11basic
              xorg.libXrandr
              xorg.libXcursor
              xorg.libXi
              wayland
            ];

            shellHook = ''
              alias ls=eza
              alias find=fd
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
                pkgs.lib.makeLibraryPath [
                  udev
                  alsa-lib
                  vulkan-loader
                  libxkbcommon
                  wayland
                ]
              }"'';
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
