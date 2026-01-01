{
  injections = [
    {
      name = "lintfra";
      remote = "https://github.com/Alb-O/lintfra.git";
      use = [
        "lint/ast-rules"
        "lint/custom-rules"
        "nix/scripts"
        "nix/outputs/perSystem/packages.d/10-lint.nix"
        "nix/outputs/perSystem/shell-packages.d/10-lintfra.nix"
        "nix/outputs/perSystem/shellHook.d/10-lintfra.sh"
        "sgconfig.yml"
      ];
    }
  ];
}
