import os
from collections import defaultdict
from typing import Tuple, Union

import config

videos_to_concat = defaultdict(int)

processed_subtitle_video_ids = []
processed_concat_video_ids = []


def get_video_path(video_id: str, check_file_exists: bool = True) -> Tuple[str, bool]:
    video_path = f"{video_id}.{config.video_output_format_ext}"
    if check_file_exists and os.path.isfile(video_path):
        return video_path, True
    return video_path, False


def append_video_file(video_path: str, video_bytes: bytes):
    with open(video_path, "ab") as video_file:
        video_file.write(video_bytes)


def get_processed_subtitle_video_path(video_id: str) -> Union[str, None]:
    if video_id not in processed_subtitle_video_ids:
        return
    video_path, exists = get_video_path(video_id)
    if not exists:
        return None
    return video_path


def save_subtitle_video(video, output_video_id: str):
    output_video_path, _ = get_video_path(output_video_id, check_file_exists=False)
    video.write_videofile(
        output_video_path,
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        # bitrate="1000k",
        # fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
    )
    processed_subtitle_video_ids.append(output_video_id)
