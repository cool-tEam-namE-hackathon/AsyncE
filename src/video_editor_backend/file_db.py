import os
from io import BytesIO
from typing import Union

import config

video_ids = []


def retrieve_video(video_id: str) -> Union[BytesIO, None]:
    if video_id not in video_ids:
        return

    video_bytesio = BytesIO()
    video_path = f"{video_id}.{config.video_output_format_ext}"
    with open(video_path, "rb") as video:
        video_bytesio.write(video.read())
    video_bytesio.seek(0)

    os.remove(video_path)
    return video_bytesio


def save_video(video, video_id: str):
    video.write_videofile(
        f"{video_id}.{config.video_output_format_ext}",
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        bitrate="5000k",
        fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
    )
    video_ids.append(video_id)
