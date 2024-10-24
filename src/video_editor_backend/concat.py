import os

from file_repository import save_concat_video, videos_to_concat
from moviepy.editor import VideoFileClip, concatenate_videoclips


def concat_videos(video_paths, output_video_id):
    save_concat_video(
        concatenate_videoclips(
            [VideoFileClip(video_path) for video_path in video_paths]
        ),
        output_video_id,
    )
    for video_path in video_paths:
        os.remove(video_path)
    del videos_to_concat[id]
