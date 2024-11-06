import os
from collections import defaultdict
from enum import Enum
from typing import Tuple, Union

import config

videos_to_concat = defaultdict(list)

processed_subtitle_video_ids = []
processed_concat_video_ids = []


def append_bytes_to_file(file_path: str, file_bytes: bytes) -> None:
    with open(file_path, "ab") as file:
        file.write(file_bytes)


def copy_file(src_file_path: str, dst_file_path: str):
    with open(src_file_path, "rb") as src_file, open(dst_file_path, "wb") as dst_file:
        dst_file.write(src_file.read())


class VideoProcessingType(Enum):
    CONCAT = "cat"
    SUBTITLE = "sub"
    THUMBNAIL = "thu"


def get_video_path(
    video_id: str, video_type: VideoProcessingType, check_file_exists: bool = True
) -> Tuple[str, bool]:
    video_path = f"{video_type.value}_{video_id}.{config.video_io_format_extension}"
    if check_file_exists and os.path.isfile(video_path):
        return video_path, True
    return video_path, False


def get_processed_video_path(
    video_id: str, video_type: VideoProcessingType
) -> Union[str, None]:
    if (
        video_type == VideoProcessingType.SUBTITLE
        and video_id not in processed_subtitle_video_ids
    ):
        return
    elif (
        video_type == VideoProcessingType.CONCAT
        and video_id not in processed_concat_video_ids
    ):
        return
    video_path, exists = get_video_path(video_id, video_type)
    if not exists:
        return
    return video_path


def write_video(video, output_video_path: str):
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


def save_concat_video(video, output_video_id: str) -> None:
    output_video_path, _ = get_video_path(
        output_video_id, VideoProcessingType.CONCAT, check_file_exists=False
    )
    write_video(video, output_video_path)
    processed_concat_video_ids.append(output_video_id)


def save_subtitle_video(video, output_video_id: str) -> None:
    output_video_path, _ = get_video_path(
        output_video_id, VideoProcessingType.SUBTITLE, check_file_exists=False
    )
    write_video(video, output_video_path)
    processed_subtitle_video_ids.append(output_video_id)
