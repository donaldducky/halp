# Video

## Conversion

```bash
# https://ffmpeg.org/ffmpeg.html#Stream-copy
# copy without re-encoding
ffmpeg -i input.webm -c copy output.mp4

# convert to mp4
ffmpeg -i "video.avi" -c:v copy -c:a copy "video.mp4"

# convert to mp4 for ipad/iphone/ipod
# https://superuser.com/a/960721
ffmpeg -i input.mp4 -vcodec libx264 -profile:v main -level 3.1 -preset medium -crf 23 -x264-params ref=4 -acodec copy -movflags +faststart output.mp4
```

## Metadata

```bash
# get info about video file
# https://superuser.com/a/595205
mediainfo filename.avi

# get specific fields
mediainfo --Output=Video;%Duration% filename.mp4

# other options include
# brew install mplayer
mplayer -vo null -ao null -frames 0 -identify videofile.mp4

# ffprobe (comes with ffmpeg?)
ffprobe -v error -show_format -show_streams a.mp4

# exiftool
exiftool a.mp4
```

## TODO

- [ ] [Trim Videos Instantly](https://bernd.dev/2020/04/trim-videos-instantly/)
  - https://news.ycombinator.com/item?id=22775502
