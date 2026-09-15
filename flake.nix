{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        naersk' = pkgs.callPackage naersk { };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust-devshell";

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config

            socat
          ];
        };

        packages.zdir = naersk'.buildPackage {
          src = ./.;
          version = "0.1.1";
          postInstall = ''
            mkdir -p $out/share/zdir
            cp shell/zdir.nu $out/share/zdir/zdir.nu
          '';
        };

        apps.zdir = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.zdir}/bin/zdir";
        };
      }
    )
    // {
      homeModules.default = import ./home.nix { inherit self; };
    };
}
