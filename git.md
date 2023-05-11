# git

```bash
# rebase from the beginning of time!
git rebase -i --root

# replace author on entire repo
# Note: this is a destructive action, it will rewrite SHAs and change timestamps
git rebase --root --exec 'git commit --amend --no-edit --reset-author'

# prefix multiple commit messages
git rebase --exec 'git commit --amend -m "PREFIX: $(git show --no-patch --format=%B)"' origin/main

# find history of a deleted file
git log --all --full-history -- <path-to-file>

# find deleted files
git log --diff-filter=D --summary
git log --diff-filter=D --summary -- <path-to-file>

# checkout previous branch
git checkout -

# or...
git checkout @{-1}

# or n branches ago
git checkout @{-3}

# find the branch
git rev-parse --symbolic-full-name @{-3}

# find symlinks in a repo
git ls-tree HEAD -r | rg 120000
```

## Convert repo to bare repo

```bash
mv repo/.git repo.git
rm -fr repo
cd repo.git
git config --bool core.bare true
```

- https://stackoverflow.com/a/2200662
