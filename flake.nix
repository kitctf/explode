{
  description = "Setup your **explo**it **d**evelopment **e**nvironment";
  inputs = {
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, gitignore, nixpkgs, naersk, fenix }:
    let forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          naersk' = pkgs.callPackage naersk { };
          inherit (gitignore.lib) gitignoreSource gitignoreFilterWith;
        in {
          default = naersk'.buildPackage {
            version =
              (pkgs.lib.importTOML ./Cargo.toml).package.version;
            src = pkgs.lib.cleanSourceWith {
              filter = gitignoreFilterWith {
                basePath = ./.;
                extraRules = ''
                  /README.md
                '';
              };
              src = ./.;
            };
          };
        });
    };
}
