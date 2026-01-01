{ pkgs, ... }:
pkgs.writeShellScriptBin "lint" ''
  if [[ -x ./nix/scripts/lint-runner ]]; then
    exec ./nix/scripts/lint-runner "$@"
  else
    echo "Error: lint-runner not found. Run from project root." >&2
    exit 1
  fi
''
