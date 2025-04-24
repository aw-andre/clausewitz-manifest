{
  description = "Rust/HTMX Dev Shell";

  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

  outputs = { nixpkgs, ... }:
    let pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
      devShells."x86_64-linux".default = pkgs.mkShell {
        packages = with pkgs; [ gcc rustc clippy cargo pnpm ];
        shellHook = ''
          export RUST_BACKTRACE=1
        '';
      };
    };
}
