# zdir integration for nushell.
# derived from zoxide's nushell integration.
# !! DO NOT EDIT !! unless an EDITME: is present

# MARK: instructions
# 1. save this to somewhere like '~/.config/nushell/zdir.nu'
# 2. add 'source ~/.config/nushell/zdir.nu' to your config.nu file
# 3. if you want to, add 'alias cd = zd' to your config.nu
# ---> lets you use the 'cd' command to interact with zdir

# MARK: zd function
def --env --wrapped __zdir_zd [...args: string] {
  let path = match $args {
    [] => {'~'},
    ['-'] => {'-'},
    _ => {
      let tmp = (mktemp --tmpdir zdir_XXXXXX)
      (^zdir $tmp ...$args)
      let selection = (open $tmp | str trim -r -c "\n")
      rm $tmp

      $selection
    }
  }

  cd $path
}

# MARK: interactive function
def --env --wrapped __zdir_zi [...args: string] {
  let tmp = (mktemp --tmpdir zdir_XXXXXX)
  (^zdir $tmp ...$args)
  let path = (open $tmp | str trim -r -c "\n")
  rm $tmp

  cd $path
}

# MARK: EDITME: aliases
alias zd = __zdir_zd
alias zi = __zdir_zi
