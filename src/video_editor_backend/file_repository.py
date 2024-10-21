import os
from io import BytesIO
from typing import Union

import config

processed_subtitle_video_ids = []


def get_video_path(video_id: str):
    return f"{video_id}.{config.video_output_format_ext}"


def append_video_file(video_path: str, video_bytes: bytes):
    with open(video_path, "ab") as video_file:
        video_file.write(video_bytes)


def retrieve_subtitle_video(video_id: str) -> Union[BytesIO, None]:
    if video_id not in processed_subtitle_video_ids:
        return

    video_bytesio = BytesIO()
    video_path = get_video_path(video_id)
    with open(video_path, "rb") as video:
        video_bytesio.write(video.read())
    video_bytesio.seek(0)

    os.remove(video_path)
    return video_bytesio


def save_subtitle_video(video, output_video_id: str):
    video.write_videofile(
        get_video_path(output_video_id),
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        bitrate="5000k",
        fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
    )
    processed_subtitle_video_ids.append(output_video_id)
