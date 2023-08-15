# pico2812bmatrix
12x20 led matrix using a raspberry pico and ws2812b led strips on the hardware side and [rp-rs](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico) and [my self implemented render](https://github.com/johannAlbertS/matrixrender) on the software side. 

This repository contains the needed software.

## Software parts
### driver
The ```driver/``` directory contains the hardcoded code to shift an alpaca over the matrix (I wrote this in the Youth Village on the CCCamp2023). To compile it change into ```driver``` and create a ```build``` directory. Then clone the [pico-sdk repo] (https://github.com/raspberrypi/pico-sdk). Export ```PICO_SKD_PATH``` and do ```cmake ..```. Then you can do ```make``` and can copy the uf2 file to the pico.
### rust
Probably just do ```cargo run```. This stuff is rather intended to be universal, **so feel free to play around in the ```main.rs``` file, to build build cool apps**. (Anyone in the JugendVillage can give to me.) 