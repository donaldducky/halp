# youtube-dl

```bash
# download playlist as mp3s with thumbnail as cover
# also keep the downloaded video
youtube-dl -ciw --extract-audio --embed-thumbnail --keep-video --restrict-filenames --audio-format mp3 -o "%(title)s.%(ext)s" "https://www.youtube.com/playlist?list=<playlist-id>"
# resume starting at playlist item 27
youtube-dl --playlist-start 27 -ciw --extract-audio --embed-thumbnail --keep-video --restrict-filenames --audio-format mp3 -o "%(title)s.%(ext)s" "https://www.youtube.com/playlist?list=<playlist-id>"

# download video
youtube-dl "https://www.youtube.com/watch?v=<video-id>"

# download playlist
youtube-dl "https://www.youtube.com/playlist?list=<playlist-id>"

# download video as m4a with thumbnail as cover
# requires AtomicParsley (brew install atomicparsley)
youtube-dl --extract-audio --embed-thumbnail --audio-format m4a -o "%(title)s.%(ext)s" "https://www.youtube.com/watch?v=<video-id>"

# download thumbnail
youtube-dl --write-thumbnail --skip-download "https://www.youtube.com/watch?v=<video-id>"

# download from a list of urls
youtube-dl --batch-file input.txt

# resuming
# Note: just re-run the command and it will continue where it left off
# -i, --ignore-errors
# -w, --no-overwrites
# -c, --continue
youtube-dl -ciw --playlist-items 1-100 --extract-audio --audio-format mp3 --restrict-filenames "https://www.youtube.com/playlist?list=<playlist-id>"
```

## Output template

List of variables that can be used in the output template using python [string formatting operations](https://docs.python.org/2/library/stdtypes.html#string-formatting) (ie. `-o "%(title)s.%(ext)s"`).
[ytdl-org/youtube-dl/README.md#output-template](https://github.com/ytdl-org/youtube-dl/blob/211cbfd5d46025a8e4d8f9f3d424aaada4698974/README.md#output-template)
