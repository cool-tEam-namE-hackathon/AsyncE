import os
from collections import defaultdict
from enum import Enum
from typing import Tuple, Union

import config

videos_to_concat = defaultdict(list)

processed_subtitle_video_ids = []
processed_concat_video_ids = []


class VideoType(Enum):
    CONCAT = "cat"
    SUBTITLE = "sub"


def get_video_path(
    video_id: str, video_type: VideoType, check_file_exists: bool = True
) -> Tuple[str, bool]:
    video_path = f"{video_type.value}_{video_id}.{config.video_io_format_ext}"
    if check_file_exists and os.path.isfile(video_path):
        return video_path, True
    return video_path, False


def append_video_file(video_path: str, video_bytes: bytes):
    with open(video_path, "ab") as video_file:
        video_file.write(video_bytes)


def get_processed_video_path(video_id: str, video_type: VideoType) -> Union[str, None]:
    if (
        video_type == VideoType.SUBTITLE
        and video_id not in processed_subtitle_video_ids
    ):
        return
    elif video_type == VideoType.CONCAT and video_id not in processed_concat_video_ids:
        return
    video_path, exists = get_video_path(video_id, video_type)
    if not exists:
        return
    return video_path


def save_concat_video(video, output_video_id: str):
    output_video_path, _ = get_video_path(
        output_video_id, VideoType.CONCAT, check_file_exists=False
    )
    if config.custom_log_prints:
        print(f"saving video to {output_video_path}")
    video.write_videofile(
        output_video_path,
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        # bitrate="1000k",
        # fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
        verbose=config.verbose_debug_prints,
        logger=None,
    )
    processed_concat_video_ids.append(output_video_id)


def save_subtitle_video(video, output_video_id: str):
    output_video_path, _ = get_video_path(
        output_video_id, VideoType.SUBTITLE, check_file_exists=False
    )
    if config.custom_log_prints:
        print(f"saving video to {output_video_path}")
    video.write_videofile(
        output_video_path,
        codec="libvpx",
        audio_codec="libvorbis",
        remove_temp=True,
        # bitrate="1000k",
        # fps=30,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
        verbose=config.verbose_debug_prints,
        logger=None,
    )
    processed_subtitle_video_ids.append(output_video_id)
