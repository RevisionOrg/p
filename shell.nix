{pkgs ? import <nixpkgs> {}}: let
  rustupToolchain = "stable";

  rustBuildTargetTriple = "x86_64-unknown-linux-gnu";
  rustBuildHostTriple = "x86_64-unknown-linux-gnu";
in
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustup
      webkitgtk
      openssl
    ];
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
    # Avoid polluting home dir with local project stuff.
    RUSTUP_HOME = toString ./.rustup;
    CARGO_HOME = toString ./.cargo;

    RUSTUP_TOOLCHAIN = rustupToolchain;

    # Set linux as the default cargo target so that we don't
    # have use the `--target` argument on every `cargo` invocation.
    CARGO_BUILD_TARGET = rustBuildTargetTriple;

    shellHook = ''
      export PATH=$PATH:${CARGO_HOME}/bin
      export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTriple}/bin/

      # Ensures our linux target is added via rustup.
      rustup target add "${rustBuildTargetTriple}"
    '';
  }
