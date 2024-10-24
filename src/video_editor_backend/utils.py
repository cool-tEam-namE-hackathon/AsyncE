import math
import os
import uuid
from typing import Tuple

import config


def generate_uuid():
    return str(uuid.uuid4())


def get_chunk_count_and_file_size(path: str) -> Tuple[int, int]:
    size = os.path.getsize(path)
    return math.ceil(size / config.retrieve_video_chunk_size_bytes), size
