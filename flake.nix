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

            # fontconfig

            udev
            alsa-lib
            vulkan-loader

            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            libxkbcommon
            wayland

            glib
            atk
            gtk3
            cairo
          ];

          LIBCLANG_PATH = with pkgs; pkgs.lib.makeLibraryPath [ libclang ];
          LD_LIBRARY_PATH = with pkgs;
            pkgs.lib.makeLibraryPath [
              udev
              alsa-lib
              vulkan-loader

              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr

              libxkbcommon
              wayland

              glib
              atk
              gtk3
              cairo
            ];
        };
      }));
}
