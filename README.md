# Image-Processing-CLI-in-Rust

Simple CLI for processing images.

This project uses following libraries:
https://github.com/PistonDevelopers/image

## Implemented functions
* ***Copy:*** -o copy 
* ***Thumbnail:*** -o thumbnail -v 96
* ***blur:*** -o blur -v 4.0 
* ***brighten:*** -o brighten -v 10
* ***huerotate:*** -o huerotate -v 10
* ***contrast:*** -o contrast -v 20.0


## Examples 

### Copy image
```
ipcli -o copy --image "D:\Image\hotelroom.jpg"
```

### Create a thumbnail 
```
ipcli -o thumbnail -v 96 --image "D:\Image\hotelroom.jpg"
```
