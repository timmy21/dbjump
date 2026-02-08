# dbjump oh-my-zsh plugin
#
# Features:
# - Smart completion with alias deduplication
# - Auto fzf trigger: Run `dbjump connect` or `dbjump info` without args to select from fzf
# - Tab completion: Use `dbjump connect <TAB>` for traditional completion
# - fzf completion: Use `dbjump connect **<TAB>` for interactive fzf selection
#
# Requirements for **<TAB> trigger:
# - fzf must be installed
# - fzf shell integration must be enabled (usually automatic with fzf installation)

# Wrapper function with smart fzf integration
function dbjump() {
    # Special handling: connect or info subcommand without alias argument
    if [[ $# -eq 1 && ($1 == "connect" || $1 == "info") ]]; then
        _dbjump_fzf_select "$1"
        return $?
    fi

    # Otherwise call the original command
    command dbjump "$@"
}

# fzf selector function
function _dbjump_fzf_select() {
    local subcommand=$1
    local config_file="${DBJUMP_CONFIG:-$HOME/.config/dbjump/config.toml}"

    # Check if fzf is installed
    if ! command -v fzf >/dev/null 2>&1; then
        echo "Error: fzf is not installed. Please install fzf or provide alias directly." >&2
        echo "Usage: dbjump $subcommand <alias>" >&2
        return 1
    fi

    # Check if config file exists
    if [[ ! -f "$config_file" ]]; then
        echo "Error: Config file not found. Run 'dbjump init' first." >&2
        return 1
    fi

    # Extract aliases from config file
    local aliases=($(grep -E '^\s*alias\s*=' "$config_file" | sed 's/.*"\(.*\)".*/\1/'))

    if [[ ${#aliases[@]} -eq 0 ]]; then
        echo "Error: No databases configured in $config_file" >&2
        return 1
    fi

    # Set fzf prompt based on subcommand
    local prompt_text="Select database"
    if [[ $subcommand == "connect" ]]; then
        prompt_text="Connect to"
    elif [[ $subcommand == "info" ]]; then
        prompt_text="Show info for"
    fi

    # Run fzf selection
    local selected=$(printf '%s\n' "${aliases[@]}" | fzf \
        --height 40% \
        --reverse \
        --border \
        --prompt="$prompt_text > " \
        --preview="command dbjump info {} 2>/dev/null || echo 'Loading...'" \
        --preview-window=right:50%:wrap \
        --bind='ctrl-/:toggle-preview')

    if [[ -n "$selected" ]]; then
        # Insert command into command line for user to edit/execute
        print -z "dbjump $subcommand $selected"
    fi
}

# Completion function
_dbjump() {
    local context state state_descr line
    typeset -A opt_args
    local subcmd="${words[2]}"

    _arguments -C \
        '1: :->subcommand' \
        '*::arg:->args'

    case $state in
        subcommand)
            local -a subcommands
            subcommands=(
                'connect:Connect to a database'
                'init:Initialize configuration file'
                'list:List all configured databases'
                'info:Show connection information for a database'
                'validate:Validate configuration file'
                'completions:Generate shell completions'
            )
            _describe 'command' subcommands
            ;;
        args)
            # Check the word immediately before the cursor
            # If it is 'connect' or 'info', we are looking for the alias argument
            local prev="${words[CURRENT-1]}"
            case $prev in
                connect|info)
                    local config_file="${DBJUMP_CONFIG:-$HOME/.config/dbjump/config.toml}"
                        if [[ -f "$config_file" ]]; then
                            local -a aliases
                            aliases=($(grep -E '^\s*alias\s*=' "$config_file" | sed 's/.*"\(.*\)".*/\1/'))
                            _describe 'database alias' aliases
                        fi
                    ;;
                list)
                    _arguments '--format[Output format]:format:(text json)'
                    ;;
                init)
                    _arguments '--force[Overwrite existing configuration]'
                    ;;
                completions)
                    _arguments ':shell:(bash zsh fish)'
                    ;;
            esac
            ;;
    esac
}

# Register completion
compdef _dbjump dbjump

# Custom fzf completion for dbjump
# Triggered by: dbjump connect **<TAB> or dbjump info **<TAB>
_fzf_complete_dbjump() {
    local tokens=(${(z)LBUFFER})
    local cmd="${tokens[1]}"
    local subcommand="${tokens[2]}"

    # Only trigger for connect and info subcommands
    if [[ $subcommand != "connect" && $subcommand != "info" ]]; then
        return 1
    fi

    _fzf_complete "--height 40% --reverse --border --prompt='Select alias > ' --preview='dbjump info {} 2>/dev/null || echo Loading...' --preview-window=right:50%:wrap" "$@" < <(
        _dbjump_get_aliases
    )
}

# Post-processing function to handle selected alias
_fzf_complete_dbjump_post() {
    # Read from stdin and output the selected alias
    local alias_name
    read -r alias_name
    echo -n "$alias_name"
}

# Helper function to get aliases from config
_dbjump_get_aliases() {
    local config_file="${DBJUMP_CONFIG:-$HOME/.config/dbjump/config.toml}"

    if [[ -f "$config_file" ]]; then
        grep -E '^\s*alias\s*=' "$config_file" | sed 's/.*"\(.*\)".*/\1/'
    fi
}
