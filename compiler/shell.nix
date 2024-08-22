let
  pkgs = import <nixpkgs> {};
in
pkgs.stdenv.mkDerivation {
  name = "ninescript compiler dependencies";
  buildInputs = with pkgs; [ clang_18 llvm_18 pkg-config libffi libxml2 ];
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
  '';
}
