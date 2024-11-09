import math
import os
import uuid
from io import BytesIO
from typing import Tuple

import config
from file_repository import VideoProcessingType, get_video_path


def generate_uuid() -> str:
    return str(uuid.uuid4())


def get_chunk_count_and_file_size(path: str) -> Tuple[int, int]:
    size = os.path.getsize(path)
    return math.ceil(size / config.retrieve_video_chunk_size_bytes), size


def generate_new_video_path(id: str, video_type: VideoProcessingType) -> str:
    video_path, _ = get_video_path(id, video_type, check_file_exists=False)
    return video_path


def extract_video_chunk(video_path: str, chunk_number: int) -> BytesIO:
    with open(video_path, "rb") as video_file:
        video_file.seek((chunk_number - 1) * config.retrieve_video_chunk_size_bytes)
        return BytesIO(video_file.read(config.retrieve_video_chunk_size_bytes))
