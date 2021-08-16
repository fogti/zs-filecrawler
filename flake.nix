{
  description = "filecrawler with resumability";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    yz-flake-utils.url = "github:YZITE/flake-utils/main";
  };
  outputs = { nixpkgs, yz-flake-utils, ... }:
    yz-flake-utils.lib.mkFlakeFromProg {
      prevpkgs = nixpkgs;
      progname = "zs-filecrawler";
      drvBuilder = final: prev:
        (import ./Cargo.nix { pkgs = final; }).rootCrate.build;
    };
}
