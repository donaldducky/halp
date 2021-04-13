# git

```bash
# rebase from the beginning of time!
git rebase -i --root

# find history of a deleted file
git log --all --full-history -- <path-to-file>

# find deleted files
git log --diff-filter=D --summary
git log --diff-filter=D --summary -- <path-to-file>
```
