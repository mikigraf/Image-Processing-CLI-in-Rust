# Image-Processing-CLI-in-Rust

Simple CLI for processing images.

This project uses following libraries:

https://github.com/PistonDevelopers/image


Processed images are being stored in the same folder as the source image using the name of the source image with an appended suffix.

Source: hotelroom.jpg

Copy: hotelroomCopy.jpg

Blur: hotelroomBlur.jpg


## Implemented functions
* ***(NAIVE) Histogram for colors (RGB)*** -o histogram 
* ***(NAIVE) Histogram for grayscale images*** -o histogramGrayscale 
* ***Average color*** -o average 
* ***Copy:*** -o copy 
* ***Thumbnail:*** -o thumbnail -v 96
* ***blur:*** -o blur -v 4.0 
* ***brighten:*** -o brighten -v 10
* ***huerotate:*** -o huerotate -v 10
* ***contrast:*** -o contrast -v 20.0
* ***grayscale***: -o grayscale 
* ***invert*** -o invert

## Examples 

### Copy image
```
ipcli -o copy --image "D:\Image\hotelroom.jpg"
```

### Create a thumbnail 
```
ipcli -o thumbnail -v 96 --image "D:\Image\hotelroom.jpg"
```
