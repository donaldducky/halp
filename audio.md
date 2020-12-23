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
```

## Extract audio from video

```bash
# extract aac audio from video to put on an iOS device
ffmpeg -i input.mp4 -vn -c:a aac output.m4a
```

## Text to speech

```bash
# text to speech
say -v Karen -o hello.aiff "hello there"

# list all voices
say -v?
```
