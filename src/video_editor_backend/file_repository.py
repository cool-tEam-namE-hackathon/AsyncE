import os
from typing import Union

import config

processed_subtitle_video_ids = []


def get_video_path(video_id: str):
    return f"{video_id}.{config.video_output_format_ext}"


def get_processed_subtitle_video_path(video_id: str) -> Union[str, None]:
    if video_id not in processed_subtitle_video_ids:
        return
    video_path = get_video_path(video_id)
    if os.path.isfile(video_path):
        return video_path


def append_video_file(video_path: str, video_bytes: bytes):
    with open(video_path, "ab") as video_file:
        video_file.write(video_bytes)


def save_subtitle_video(video, output_video_id: str):
    video.write_videofile(
        get_video_path(output_video_id),
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        # bitrate="1000k",
        # fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
    )
    processed_subtitle_video_ids.append(output_video_id)
