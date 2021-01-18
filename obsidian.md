# Obsidian

[https://obsidian.md/](https://obsidian.md/)

## Searching

https://publish.obsidian.md/help/Plugins/Search

```
# search by regex
/myregex/

# search for empty files
content:-/.*/

# search by tag
tag:#my-tag

# search by tag, excluding a sub tag
tag:#my-tag -tag:#my-tag/nested

# search by title
file:🔥
```

## Embedding

https://publish.obsidian.md/help/How+to/Embed+files
https://publish.obsidian.md/help/Advanced+topics/Accepted+file+formats

```
![[Note]]

![[image.png|100x100]]

![AltText|100x100](https://url/to/image.png)
```

### Embed search results using query blocks

Embed all checkboxes:
```query
"- [ ]"
```

Embed tags:
```query
tag:my-tag tag:another-tag
```

## Linking

https://publish.obsidian.md/help/How+to/Link+to+blocks

```
# link to a block
# typing after the note name will start a search for auto-generated block ids
[[note#^block_id]]
```
