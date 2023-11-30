from PIL import Image
import numpy as np


def analyze_image(file_path):
    # Load the image
    with Image.open(file_path) as img:
        # Convert the image to a numpy array
        data = np.array(img)

        # Find the min and max values for each channel
        min_vals = data.min(axis=(0, 1))
        max_vals = data.max(axis=(0, 1))

        print("Min values (R, G, B):", min_vals)
        print("Max values (R, G, B):", max_vals)


# Replace with the path to your normal map image
analyze_image("/home/vega/Coding/Graphics/raytrace-rs/assets/latlon-normal-map.png")
