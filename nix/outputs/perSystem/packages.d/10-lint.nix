{
  pkgs,
  lib,
  self,
  ...
}:
let
  astGrep = import "${self}/nix/lib/ast-grep-rule.nix" { inherit lib; };
  customRule = import "${self}/nix/lib/custom-rule.nix" { inherit lib; };

  # ast-grep rules from lint/rules/
  rulesDir = "${self}/lint/rules";
  ruleFiles = builtins.filter (f: lib.hasSuffix ".nix" f) (
    builtins.attrNames (builtins.readDir rulesDir)
  );
  rules = map (f: {
    name = lib.removeSuffix ".nix" f;
    rule = import (rulesDir + "/${f}") { inherit (astGrep) mkRule; };
  }) ruleFiles;

  # Generate YAML files
  generatedRules = pkgs.runCommand "ast-grep-rules" { buildInputs = [ pkgs.yq-go ]; } ''
    mkdir -p $out
    ${lib.concatMapStringsSep "\n" (
      r: ''echo '${astGrep.toJson r.rule}' | yq -P > $out/${r.name}.yml''
    ) rules}
  '';

  # custom rules from lint/custom/
  customRulesDir = "${self}/lint/custom";
  customRuleFiles = builtins.filter (f: lib.hasSuffix ".nix" f) (
    builtins.attrNames (builtins.readDir customRulesDir)
  );
  customRules = map (f: import (customRulesDir + "/${f}") customRule) customRuleFiles;
  customRulesJson = builtins.toJSON customRules;

  # The lint runner script
  lintRunner = "${self}/nix/scripts/lint-runner.nu";
in
{
  # Standalone installable imp-lint package
  imp-lint = pkgs.writeShellScriptBin "imp-lint" ''
    export LINTFRA_RULES="${generatedRules}"
    export LINTFRA_CUSTOM_RULES='${customRulesJson}'
    exec ${pkgs.nushell}/bin/nu ${lintRunner} "$@"
  '';

  default = pkgs.writeShellScriptBin "imp-lint" ''
    export LINTFRA_RULES="${generatedRules}"
    export LINTFRA_CUSTOM_RULES='${customRulesJson}'
    exec ${pkgs.nushell}/bin/nu ${lintRunner} "$@"
  '';
}
