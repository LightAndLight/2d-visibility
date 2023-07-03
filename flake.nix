{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };

        rustVersion = "1.70.0";
      
      in {
        devShell =
          pkgs.mkShell {
            # See https://github.com/bevyengine/bevy/blob/1c5c94715cb17cda5ae209eef12a938501de90b5/docs/linux_dependencies.md#nix
            # for `bevy` dependencies.
            buildInputs = with pkgs; [
              (rust-bin.stable.${rustVersion}.default.override {
                extensions = [
                  "cargo"
                  "clippy"
                  "rustc"
                  "rust-src"
                  "rustfmt"
                ];
              })
              
              pkg-config
              alsa-lib
              udev
            ];
            
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              vulkan-loader
            ];
          };
      }
    );
}
