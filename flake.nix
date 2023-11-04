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
          rustVersion = "1.71.0";
          packageFun = import ./Cargo.nix;
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
                  libxkbcommon
                  wayland
                  vulkan-loader
                  libGL
                  xorg.libX11
                ] ++ drv.buildInputs;
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
          ];

          LIBCLANG_PATH = with pkgs; pkgs.lib.makeLibraryPath [ libclang ];
        };
      }));
}
