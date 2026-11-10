{ inputs, ... }:

{
  perSystem =
    { pkgs, lib, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;

      csvFilter = path: _type: builtins.match ".*csv$" path != null;
      customFilter = path: type: (csvFilter path type) || (craneLib.filterCargoSources path type);
      src = lib.cleanSourceWith {
        src = ../.;
        filter = customFilter;
        name = "source";
      };

      commonArgs = {
        inherit src;
        strictDeps = true;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in
    {
      packages.default = craneLib.buildPackage (
        commonArgs
        // {
          doCheck = false;
          inherit cargoArtifacts;
        }
      );
      checks = {
        clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );
        nextest = craneLib.cargoNextest (
          commonArgs
          // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestPartitionsExtraArgs = "--no-tests=pass";
          }
        );
      };
    };
}
