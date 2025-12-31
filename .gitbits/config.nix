{
  injections = [
    {
      name = "lintfra";
      remote = "https://github.com/Alb-O/lintfra.git";
      use = [
        "lint/ast-rules"
        "lint/custom-rules"
        "nix/scripts"
        "sgconfig.yml"
      ];
    }
  ];
}
