# Lintfra shell setup
# Install pre-commit hook
if [ -t 0 ] && [ -d .git ]; then
  if [ -x ./nix/scripts/pre-commit ]; then
    cp ./nix/scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
  fi
fi

echo "Lint commands:"
echo "  lint        - Run unified lint (ast-grep + custom rules)"
echo "  lint --json - Output lint results as JSON stream"
