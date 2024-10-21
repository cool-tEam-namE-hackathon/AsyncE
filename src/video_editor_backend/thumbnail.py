from io import BytesIO

import config
from moviepy.editor import VideoFileClip
from PIL import Image


def convert_image_to_bytesio(image, format: str) -> BytesIO:
    bytes_io = BytesIO()
    image.save(bytes_io, format)
    bytes_io.seek(0)
    return bytes_io


def generate_thumbnail(video_path: str) -> BytesIO:
    with VideoFileClip(video_path) as clip:
        if clip.duration >= 1:
            time = 1
        else:
            time = 0
        frame = clip.get_frame(time)

    thumbnail = Image.fromarray(frame)
    return convert_image_to_bytesio(thumbnail, config.image_output_format_ext)
