{
  __inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  __functor =
    _:
    {
      pkgs,
      rust-overlay,
      rootSrc,
      ...
    }:
    let
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile (rootSrc + "/rust-toolchain.toml");
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
    in
    {
      default = rustPlatform.buildRustPackage {
        pname = "tome";
        version = "0.1.0";
        src = rootSrc;
        cargoLock.lockFile = rootSrc + "/Cargo.lock";
      };

      kak-ffi = rustPlatform.buildRustPackage {
        pname = "kak-ffi";
        version = "0.1.0";
        src = rootSrc;
        cargoLock.lockFile = rootSrc + "/Cargo.lock";
        buildAndTestSubdir = "crates/kak-ffi";
        postInstall = ''
          mkdir -p $out/include
          cp crates/kak-ffi/include/kak_ffi.h $out/include/
        '';
      };

      kakoune = pkgs.kakoune-unwrapped;
    };
}
