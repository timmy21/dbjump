#!/usr/bin/env bash
set -e

echo "Installing dbjump..."
echo ""

# Check if we're in the project directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "Error: Please run this script from the dbjump project directory" >&2
    exit 1
fi

# Compile release version
echo "Building release binary..."
cargo build --release

# Install binary
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"
cp target/release/dbjump "$INSTALL_DIR/"
echo "✓ Binary installed to $INSTALL_DIR/dbjump"

# Check if binary is in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "⚠  Warning: $INSTALL_DIR is not in your PATH"
    echo "   Add this line to your ~/.zshrc or ~/.bashrc:"
    echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
fi

# Generate completion script
echo "Generating completions..."
"$INSTALL_DIR/dbjump" completions zsh > oh-my-zsh/dbjump/_dbjump
echo "✓ Completions generated"

# Install oh-my-zsh plugin
if [[ -d "$HOME/.oh-my-zsh/custom/plugins" ]]; then
    echo "Installing oh-my-zsh plugin..."
    PLUGIN_DIR="$HOME/.oh-my-zsh/custom/plugins/dbjump"
    mkdir -p "$PLUGIN_DIR"
    cp -r oh-my-zsh/dbjump/* "$PLUGIN_DIR/"
    echo "✓ Oh-My-Zsh plugin installed to $PLUGIN_DIR"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "To enable the plugin, add 'dbjump' to plugins in ~/.zshrc:"
    echo ""
    echo "  plugins=(git docker ... dbjump)"
    echo ""
    echo "Then reload your shell:"
    echo "  source ~/.zshrc"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
else
    echo "Oh-My-Zsh not detected. Installing plugin to ~/.config/dbjump/"
    PLUGIN_DIR="$HOME/.config/dbjump/plugin"
    mkdir -p "$PLUGIN_DIR"
    cp -r oh-my-zsh/dbjump/* "$PLUGIN_DIR/"
    echo "✓ Plugin files copied to $PLUGIN_DIR"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Add this line to your ~/.zshrc:"
    echo ""
    echo "  source $PLUGIN_DIR/dbjump.plugin.zsh"
    echo ""
    echo "Then reload your shell:"
    echo "  source ~/.zshrc"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
fi

echo ""
echo "Next steps:"
echo "  1. Enable the plugin (see instructions above)"
echo "  2. Run 'dbjump init' to create configuration file"
echo "  3. Edit ~/.config/dbjump/config.toml to add databases"
echo "  4. Run 'dbjump validate' to check your configuration"
echo "  5. Use 'dbjump <alias>' to connect, or just 'dbjump' for fzf selection"
echo ""
echo "Installation complete! 🚀"
