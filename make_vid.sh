#!/bin/bash

cd animation/

# delete the old output.mp4
rm ../output.mp4
ffmpeg -framerate 60 -i %d.png -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p ../output.mp4
