{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  };

  outputs = { nixpkgs, flake-utils, cargo2nix, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;

          rustChannel = "nightly";
          extraRustComponents = [ "rustc-dev" ];
        };
      in rec {
        packages = rec {
          shark-visualizer =
            (rustPkgs.workspace.shark-visualizer { }).overrideAttrs (drv: {
              postFixup = ''
                patchelf --add-rpath ${pkgs.vulkan-loader}/lib $out/bin/shark-visualizer
                patchelf --add-rpath ${pkgs.libGL}/lib $out/bin/shark-visualizer
              '';

              nativeBuildInputs = with pkgs;
                [ pkg-config ] ++ drv.nativeBuildInputs;

              buildInputs = with pkgs;
                [
                  pkg-config
                  fontconfig
                  vulkan-loader
                  wayland
                  libxkbcommon
                  libGL
                  xorg.libXcursor
                  xorg.libXrandr
                  xorg.libXi
                  xorg.libX11
                ] ++ drv.buildInputs;

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
            });
          default = shark-visualizer;
        };

        apps = rec {
          shark-visualizer =
            flake-utils.lib.mkApp { drv = packages.shark-visualizer; };
          default = shark-visualizer;
        };

        devShell = rustPkgs.workspaceShell {
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
            cargo2nix.packages.${system}.cargo2nix
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
