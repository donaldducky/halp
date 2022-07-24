# git

```bash
# rebase from the beginning of time!
git rebase -i --root

# replace author on entire repo
# Note: this is a destructive action, it will rewrite SHAs and change timestamps
git rebase --root --exec 'git commit --amend --no-edit --reset-author'

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
```
