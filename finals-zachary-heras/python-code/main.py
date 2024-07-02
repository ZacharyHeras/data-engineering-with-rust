import argparse
import numpy as np
from pathlib import Path
import PIL
import time
import uuid

from matplotlib import pyplot as plt
from PIL import Image


def get_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description='Creates images for training breath phase counting CNN model.')

    parser.add_argument(
        '--fig-width-pixels', help='Width of figure in pixels.', required=True)

    parser.add_argument(
        '--fig-height-pixels', help='Height of figure in pixels.', required=True)

    args = parser.parse_args()

    return args


def fig_to_numpy(audio_clip: np.ndarray,
                 num_samples: int,
                 fig_dpi: int,
                 fig_width_pixels: int,
                 fig_height_pixels: int,
                 lower_ylim: int,
                 upper_ylim: int) -> np.ndarray:

    fig_width_inches = fig_width_pixels / fig_dpi
    fig_height_inches = fig_height_pixels / fig_dpi

    fig = plt.figure(num=1, clear=True)
    ax = fig.add_subplot()

    fig.set_dpi(fig_dpi)
    fig.set_size_inches(fig_width_inches, fig_height_inches)

    ax.set_position([0, 0, 1, 1])
    ax.set_axis_off()
    ax.set_xlim((0, num_samples))
    ax.set_ylim((lower_ylim, upper_ylim))

    ax.plot(audio_clip)

    fig.canvas.draw()
    plot_buffer = fig.canvas.buffer_rgba()

    plt.close(fig)

    plot_ndarray_1d = np.frombuffer(plot_buffer, dtype=np.uint8)
    plot_ndarray_2d = plot_ndarray_1d.reshape(fig_height_pixels,
                                              fig_width_pixels, 4)

    return plot_ndarray_2d


def get_reversed_pil_image(image: np.ndarray) -> PIL.Image:
    flipped_image = np.flip(image, 1)
    flipped_image = Image.fromarray(flipped_image)
    return flipped_image


def main(args: argparse.Namespace) -> None:
    start_time = time.time()
    fig_height_pixels = int(args.fig_height_pixels)
    fig_width_pixels = int(args.fig_width_pixels)

    fig_dpi = 50

    sample_rate = 5000
    clip_length = 1
    num_samples = sample_rate * clip_length
    freq = 5

    lower_ylim = -1.2
    upper_ylim = 1.2

    t = np.linspace(0.0, clip_length, num_samples)
    audio_clip = np.sin(2 * np.pi * freq * t)

    for _ in range(1001):
        image = fig_to_numpy(audio_clip,
                             num_samples,
                             fig_dpi,
                             fig_width_pixels,
                             fig_height_pixels,
                             lower_ylim,
                             upper_ylim)

        pil_image = Image.fromarray(image)
        pil_image_reversed = get_reversed_pil_image(image)

        clip_uuid = uuid.uuid4()

        png_directory = Path('python_figures')
        png_directory.mkdir(parents=True, exist_ok=True)

        pil_image.save(png_directory /
                       f'image_{clip_uuid}_o.png')

        pil_image_reversed.save(png_directory /
                                f'reversed_image_{clip_uuid}_o.png')

    elapsed_time = time.time() - start_time

    print(elapsed_time)


if __name__ == '__main__':
    args = get_args()
    main(args)
