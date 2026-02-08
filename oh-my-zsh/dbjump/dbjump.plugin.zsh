# dbjump oh-my-zsh plugin
# Provides completion and fzf integration for dbjump

# Load completions
fpath=(${0:A:h} $fpath)
autoload -Uz compinit
compinit -i

# Wrapper function for dbjump with fzf integration
function dbjump() {
    # If arguments provided, call dbjump directly
    if [[ $# -gt 0 ]]; then
        command dbjump "$@"
        return $?
    fi

    # No arguments - try fzf integration
    if command -v fzf >/dev/null 2>&1; then
        local config_file="${DBJUMP_CONFIG:-$HOME/.config/dbjump/config.toml}"

        # Check if config file exists
        if [[ ! -f "$config_file" ]]; then
            echo "Error: Config file not found. Run 'dbjump init' first." >&2
            return 1
        fi

        # Extract aliases from config file
        local aliases=($(grep -E '^\s*alias\s*=' "$config_file" | sed 's/.*"\(.*\)".*/\1/'))

        if [[ ${#aliases[@]} -eq 0 ]]; then
            echo "Error: No databases configured in $config_file" >&2
            echo "Edit the file to add database connections." >&2
            return 1
        fi

        # Show fzf selection
        local selected=$(printf '%s\n' "${aliases[@]}" | fzf \
            --height 40% \
            --reverse \
            --border \
            --prompt="Database > " \
            --preview="command dbjump info {}" \
            --preview-window=right:50%:wrap \
            --bind='ctrl-/:toggle-preview')

        if [[ -n "$selected" ]]; then
            command dbjump "$selected"
        fi
    else
        # fzf not installed, show help
        command dbjump --help
    fi
}

# Completion function for dynamic alias completion
_dbjump_aliases() {
    local config_file="${DBJUMP_CONFIG:-$HOME/.config/dbjump/config.toml}"

    if [[ -f "$config_file" ]]; then
        local aliases=($(grep -E '^\s*alias\s*=' "$config_file" | sed 's/.*"\(.*\)".*/\1/'))
        _describe 'database aliases' aliases
    fi
}

# Register the completion function
compdef _dbjump_aliases dbjump
