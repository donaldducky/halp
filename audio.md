# Audio

## Conversion

```bash
# aif to caf
afconvert -f caff -d aacl@22050 -c 1 audio-file.aif audio-file.caf

# aif to mp3
ffmpeg -i sound.aiff -f mp3 -acodec libmp3lame -ab 192000 -ar 44100 sound.mp3

# ogg to mp3
# see https://askubuntu.com/a/983554
ffmpeg -i "audio.ogg" "audio.mp3"

# set bitrate to 192
ffmpeg -b:a 192k -i "audio.ogg" "audio.mp3"

# do a whole directory
for i in *.ogg; do ffmpeg -i "$i" "${i%.*}.mp3"; done
```

## Metadata

```bash
# Add cover to mp3
brew install eye-d3
find . -name '*.mp3' -exec eyeD3 --add-image="cover.jpg":FRONT_COVER "{}" \;

# Extract cover from mp3 (requires a folder to exist)
mkdir images-folder
eyeD3 song.mp3 --write-images images-folder

# Add cover to m4a (AtomicParsley)
brew install atomicparsley
atomicparsley song.m4a --artwork cover.jpg

# Add cover to m4a (mp4art)
# note: the art didn't show up in iina when using mp4art
# using atomicparsley was fine
brew install mp4v2
mp4art --add cover.jpg song.m4a
```

## Extract audio from video

```bash
# extract aac audio from video to put on an iOS device
ffmpeg -i input.mp4 -vn -c:a aac output.m4a
```

## Trim audio

```bash
# Extract a portion of audio from an mp3
# Format is hh:mm:ss[.xxx]
# Note: this does not copy image metadata (ie. FRONT_COVER)
ffmpeg -i input.mp3 -vn -acodec copy -ss 00:00:11 -to 00:01:46 output.mp3
```

## Text to speech

```bash
# text to speech
say -v Karen -o hello.aiff "hello there"

# list all voices
say -v?
```
