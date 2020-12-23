# ImageMagick

https://www.imagemagick.org

Tons of amazing examples:
http://www.fmwconcepts.com/imagemagick/index.php

## Identify

```bash
# Get image details
identify image.png
image.png PNG 1574x1169 1574x1169+0+0 8-bit sRGB 700792B 0.000u 0:00.000
```

## Crop

https://www.imagemagick.org/Usage/crop/

```bash
# crop dimensions are ${width}x${height}+${x}+${y}
convert image.png -crop 100x100+20+20 cropped-image.png
```

## Draw on an image

https://www.imagemagick.org/Usage/draw/#color
https://www.imagemagick.org/script/color.php

```bash
# draw on an image
convert image.png -fill "rgb(51%,82%,96%)" -draw "rectangle 0,0 10,5" image-with-rectangle.png
```

## Convert pdf to jpg

```bash
# see: https://stackoverflow.com/a/13784772
# note: this requires ghostscript installed as well as imagemagick for the pdf portion
#       brew install ghostscript
convert -density 300 my.pdf -quality 100 my.jpg
```

## Stitch images together

```bash
# see: https://superuser.com/a/290679

convert -append images-*.jpg vertically-appended.jpg

convert in-1.jpg in-5.jpg in-N.jpg +append horizontally-appended.jpg

# use montage for finer control of the layout, in this case 1 column by any number of rows
montage -mode concatenate -tile 1x in-*.jpg out.jpg
```
