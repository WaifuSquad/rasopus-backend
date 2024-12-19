{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {

  buildInputs = [
    pkgs.rustup
  ];

  TEMPDIR = "/tmp/rust/shell";
  TMPDIR = "/tmp/rust/shell";
  TEMP = "/tmp/rust/shell";
  TMP = "/tmp/rust/shell";

  shellHook = ''
    DATA_DIR="/tmp/rust"
    export RUSTUP_HOME="$DATA_DIR/rustup"
    export CARGO_HOME="$DATA_DIR/cargo"
    export PATH=$CARGO_niHOME/bin:$PATH
    mkdir -p "$CARGO_HOME"
    mkdir -p "$RUSTUP_HOME"
    mkdir -p "$DATA_DIR/shell"

    rustup default stable
    rustup update
    cargo fetch

    echo
    echo
    echo

    echo "Rustup installed at $RUSTUP_HOME"
    echo "Cargo installed at $CARGO_HOME"
    echo "$(rustup --version)"
    echo "$(cargo --version)"
  '';

}
