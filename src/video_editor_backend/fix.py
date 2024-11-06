import tempfile

import config
import ffmpeg
from file_repository import copy_file


def fix_video(input_video_path: str):
    with tempfile.NamedTemporaryFile(
        suffix=f".{config.video_io_format_extension}"
    ) as output_video_file:
        output_video_path = output_video_file.name
        ffmpeg.input(input_video_path).output(output_video_path).run(
            overwrite_output=True
        )
        copy_file(output_video_path, input_video_path)
