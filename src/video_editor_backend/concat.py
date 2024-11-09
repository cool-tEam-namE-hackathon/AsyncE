import os

from file_repository import save_concat_video, videos_to_concat
from fix import fix_video
from moviepy.editor import VideoFileClip, concatenate_videoclips


def concat_videos(concat_video_list_id, output_video_id) -> None:
    video_paths = videos_to_concat[concat_video_list_id]
    clips = []
    for video_path in video_paths:
        fix_video(video_path)
        clips.append(VideoFileClip(video_path))

    save_concat_video(
        concatenate_videoclips(clips),
        output_video_id,
    )
    for video_path in video_paths:
        os.remove(video_path)
    videos_to_concat.pop(concat_video_list_id)
