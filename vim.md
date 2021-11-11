# VIM

## Execute commands without triggering autocommands

```vim
" save
:noautocmd w
:noa w
```

## Undo Tree

VIM stores changes in a tree structure.

This allows us to make changes, back up and go down another branch of changes
without worry of losing any of the changes.

Undo changes can be persisted across sessions.

```viml
" Tips for more secure persistant storage and use
" https://vi.stackexchange.com/a/53
if !isdirectory($HOME."/.vim")
    call mkdir($HOME."/.vim", "", 0770)
endif
if !isdirectory($HOME."/.vim/undo")
    call mkdir($HOME."/.vim/undo", "", 0700)
endif
set undodir=~/.vim/undo
set undofile
set undolevels=10000
```

Navigating the undo tree is difficult to visualize but a plugin exists to make
it much simpler: [vim-mundo](https://simnalamburt.github.io/vim-mundo/)

```viml
:MundoToggle
```

Learn more:
- `:help undo-redo`
- `:help undo-persistence`
- `:help undo-tree`
- https://thevaluable.dev/vim-intermediate/#undo-tree
