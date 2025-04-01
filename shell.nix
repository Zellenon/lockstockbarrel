{ pkgs ? import <nixpkgs> {} }:
with pkgs; pkgs.mkShell {
# nativeBuildInputs is usually what you want -- tools you need to run
	nativeBuildInputs = [ 
    buildPackages.pkg-config 
    buildPackages.clang 
    buildPackages.lld 
    buildPackages.mold ];
	buildInputs =  [
		cargo
			rustc
			rustfmt
			pre-commit
			rustPackages.clippy
			alsa-lib
			udev
#NOTE Add more deps
			vulkan-loader
			xorg.libX11
			x11basic
			xorg.libXrandr
			xorg.libXcursor
			xorg.libXi
            systemd
	];
	shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
                pkgs.lib.makeLibraryPath [
                  udev
                  alsa-lib
                  vulkan-loader
                ]
              }"'';
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
}
