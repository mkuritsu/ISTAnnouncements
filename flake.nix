{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      ...
    }:
    let
      eachSystem =
        fn:
        nixpkgs.lib.genAttrs (import systems) (
          system:
          let
            pkgs = import nixpkgs { inherit system; };
          in
          fn pkgs
        );
    in
    {
      nixosModules.default = import ./nixosModule.nix self;

      packages = eachSystem (pkgs: rec {
        default = istannoucements;
        istannoucements = pkgs.callPackage ./package.nix { };
      });

      devShells = eachSystem (pkgs: {
        default = pkgs.callPackage ./shell.nix { };
      });
    };
}
