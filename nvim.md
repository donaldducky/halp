# Neovim

## Lua

Converting from vimscript to lua.

```vim
let g:lightline = {
  \ 'colorscheme': 'deus',
  \ }
```

`:lua print(vim.inspect(vim.api.nvim_eval("{'colorscheme': 'deus'}")))`

```lua
vim.g.lightline = {
  colorscheme: 'deus'
}
```

[nvim_eval](https://github.com/nanotee/nvim-lua-guide#using-vimscript-from-lua)

## LSP

Built in LSP.

- [Bash2Basics: Neovim Builtin LSP Setup Guide](https://www.youtube.com/watch?v=puWgHa7k3SY)

`:LspInfo` to see what's attached
`:h vim.lsp.buf.<tab>`


learn about
- [ ] taglist
  - ctrl+t not working as described in the bash2basics video. why? are there settings or plugins affecting it?
- [ ] jumplist
- [ ] using undo stack (mundo plugin that I have installed)

## Execute commands from cli

```bash
# dump key mappings
nvim --headless '+redir! mappings.txt' '+silent verbose map' '+redir END' '+q'
```
