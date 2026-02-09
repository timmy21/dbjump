# dbjump oh-my-zsh plugin
#
# This plugin is a thin wrapper that calls `dbjump shell zsh`.
# For manual installation (without oh-my-zsh), add this to your .zshrc:
#   eval "$(dbjump shell zsh)"
#
# To customize the quick connect command name:
#   eval "$(dbjump shell --cmd j zsh)"

if (( $+commands[dbjump] )); then
  eval "$(dbjump shell --cmd ${DBJUMP_CMD_OVERRIDE:-j} zsh)"
else
  echo '[oh-my-zsh] dbjump not found, please install it from https://github.com/timmy21/dbjump'
fi
