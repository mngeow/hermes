#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"

# All global skill directories OpenCode searches
TARGETS=(
  "$HOME/.config/opencode/skills"   # Primary OpenCode path
  "$HOME/.claude/skills"            # Claude-compatible path
  "$HOME/.agents/skills"            # Agent-compatible path
)

for target in "${TARGETS[@]}"; do
  parent="$(dirname "$target")"

  # Create parent directory if it doesn't exist
  mkdir -p "$parent"

  if [ -L "$target" ]; then
    echo "Updating symlink: $target -> $REPO_DIR"
    rm "$target"
  elif [ -d "$target" ]; then
    echo "WARNING: $target already exists as a directory."
    echo "  Back it up and remove it, then re-run this script."
    continue
  fi

  ln -s "$REPO_DIR" "$target"
  echo "Linked: $target -> $REPO_DIR"
done

echo "Done. Skills are now available globally."