#!/bin/bash

cd animation/

ffmpeg -framerate 24 -i %d.png -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p ../output.mp4
