{
  description = "A Obsidian like app right in your terminal.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      naerskLib = pkgs.callPackage naersk { };
    in {

      packages.x86_64-linux.default = naerskLib.buildPackage {
        src = ./.;
        buildInputs = with pkgs; [ openssl ];
        nativeBuildInputs = with pkgs; [ pkg-config ];
      };
    };
}
