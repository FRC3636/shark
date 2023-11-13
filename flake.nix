{
  inputs = { flake-utils.url = "github:numtide/flake-utils"; };

  outputs = { nixpkgs, flake-utils, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        devShell = pkgs.mkShell rec {
          name = "devShell";
          buildInputs = with pkgs; [
            openssl
            pkg-config
            clang
            llvmPackages_16.libllvm
            llvmPackages_16.stdenv

            fontconfig
            vulkan-loader
            wayland
            libxkbcommon
            libGL
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11

            alsa-lib
            udev
          ];

          LIBCLANG_PATH = with pkgs; pkgs.lib.makeLibraryPath [ libclang ];
          LD_LIBRARY_PATH = with pkgs;
            pkgs.lib.makeLibraryPath [
              libGL
              wayland
              libxkbcommon
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
            ];
        };
      }));
}
