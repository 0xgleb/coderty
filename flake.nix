{
  inputs = {
    devenv.url = "github:cachix/devenv";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    # foundry = {
    #   url = "github:shazow/foundry.nix/monthly";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
  };

  outputs = { self, nixpkgs, flake-utils, devenv }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # foundry-pkg = foundry.defaultPackage.${system};
      in {
        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;

          modules = [{
            packages = with pkgs;
              [ solc ipfs-upload-client ]
              ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
                libiconv
                frameworks.Security
                frameworks.CoreFoundation
              ]);

            dotenv.enable = true;
            difftastic.enable = true;

            # https://devenv.sh/languages/
            languages.nix.enable = true;

            # https://devenv.sh/pre-commit-hooks/
            pre-commit.hooks = {
              nixfmt = {
                enable = true;
                fail_fast = true;
              };
            };
          }];
        };
      });
}
