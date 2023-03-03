let
  sources = import ./nix/sources.nix;
  mozilla-overlay = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs {
    overlays = [ mozilla-overlay ];
  };
in
pkgs.mkShell {
  name = "clang-env-with-rust";
  packages = with pkgs; [
    rustc
    cargo
    clang
    llvmPackages.libclang
    darwin.apple_sdk.frameworks.Accelerate

    rustfmt
    clippy
    rust-analyzer
  ];
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
  '';
}

