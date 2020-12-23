# Image

## Conversion

```bash
# webp to jpg (or png, etc)
ffmpeg -i input.webp output.jpg
```

## Metadata

```bash
# remove exif data from images
exiftool -all= my-image.jpg
```

## Manipulation

see: [ImageMagick](imagemagick.md)

```bash
# Rotate 90 degrees
# https://unix.stackexchange.com/a/365595
convert "in.jpg" -rotate 90 "out.jpg"
# in-place edit
mogrify -rotate 90 *.jpg

# Resize image
convert "in.jpg" -resize 32x32 "out.jpg"
```
