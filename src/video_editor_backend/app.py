import concurrent.futures
import os

import config
from concat import concat_videos
from file_repository import (VideoProcessingType, append_bytes_to_file,
                             get_processed_video_path, get_video_path,
                             videos_to_concat)
from fix import fix_video
from flask import Flask, Response, jsonify, make_response, request, send_file
from subtitles import generate_subtitle_video
from thumbnail import generate_thumbnail
from utils import (extract_video_chunk, generate_new_video_path, generate_uuid,
                   get_chunk_count_and_file_size)


def get_processed_video_info_response(
    id: str, video_type: VideoProcessingType
) -> Response:
    video_path = get_processed_video_path(id, video_type)
    if video_path == None:
        return make_response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            404,
        )

    chunk_count, file_size = get_chunk_count_and_file_size(video_path)
    return make_response(
        jsonify({"chunk_count": chunk_count, "file_size": file_size}), 200
    )


def get_processed_video_chunk_response(
    id: str, number: int, video_type: VideoProcessingType
) -> Response:
    if number < 1:
        return make_response("Invalid given number is not possible", 400)

    video_path = get_processed_video_path(id, video_type)
    if video_path == None:
        return make_response(
            f"Video with id '{id}' either hasn't yet finished processing or doesn't exist",
            404,
        )

    chunk_count, _ = get_chunk_count_and_file_size(video_path)
    if number > chunk_count:
        return make_response("Given chunk number exceeds chunk count", 400)

    video_bytesio = extract_video_chunk(video_path, number)
    if number == chunk_count:
        os.remove(video_path)
    return make_response(
        send_file(
            video_bytesio,
            mimetype="application/octet-stream",
        ),
        200,
    )


def video_does_not_exist_response(id) -> Response:
    return make_response(f"Video with id '{id}' doesn't exist", 404)


concat_video_worker_pool_executor = concurrent.futures.ThreadPoolExecutor(
    max_workers=config.concat_video_processor_workers
)
subtitle_video_worker_pool = concurrent.futures.ThreadPoolExecutor(
    max_workers=config.subtitle_video_processor_workers
)

app = Flask(__name__)


@app.route("/concat/<id>")
def get_processed_concat_video_info(id: str) -> Response:
    return get_processed_video_info_response(id, VideoProcessingType.CONCAT)


@app.route("/concat/<id>/<int:number>")
def get_processed_concat_video_chunk(id: str, number: int) -> Response:
    return get_processed_video_chunk_response(id, number, VideoProcessingType.CONCAT)


@app.route("/concat/start", methods=["POST"])
def start_chunk_for_concat_video() -> Response:
    video_bytes = request.data

    id = generate_uuid()
    video_path = generate_new_video_path(id, VideoProcessingType.CONCAT)
    append_bytes_to_file(video_path, video_bytes)
    videos_to_concat[id].append(video_path)

    return make_response(id, 200)


@app.route("/concat/<id>/add", methods=["POST"])
def append_chunk_for_concat_video(id: str) -> Response:
    video_bytes = request.data

    if id not in videos_to_concat:
        return video_does_not_exist_response(id)

    append_bytes_to_file(videos_to_concat[id][-1], video_bytes)

    return make_response(id, 200)


@app.route("/concat/<id>/new", methods=["POST"])
def append_chunk_for_concat_video_and_mark_next_video(id: str) -> Response:
    video_bytes = request.data

    if id not in videos_to_concat:
        return video_does_not_exist_response(id)

    video_path = generate_new_video_path(generate_uuid(), VideoProcessingType.CONCAT)
    append_bytes_to_file(video_path, video_bytes)
    videos_to_concat[id].append(video_path)

    return make_response(id, 200)


@app.route("/concat/<id>/end", methods=["POST"])
def process_concat_video(id: str) -> Response:
    video_bytes = request.data

    if id not in videos_to_concat:
        return video_does_not_exist_response(id)

    append_bytes_to_file(videos_to_concat[id][-1], video_bytes)
    future_processed_concat_video = generate_uuid()
    concat_video_worker_pool_executor.submit(
        concat_videos, id, future_processed_concat_video
    )

    return make_response(future_processed_concat_video, 200)


@app.route("/subtitles/<id>")
def get_processed_subtitle_video_info(id: str) -> Response:
    return get_processed_video_info_response(id, VideoProcessingType.SUBTITLE)


@app.route("/subtitles/<id>/<int:number>")
def get_processed_subtitle_video_chunk(id: str, number: int) -> Response:
    return get_processed_video_chunk_response(id, number, VideoProcessingType.SUBTITLE)


@app.route("/subtitles/start", methods={"POST"})
def start_chunk_for_subtitle_video() -> Response:
    video_bytes = request.data

    id = generate_uuid()
    video_path = generate_new_video_path(id, VideoProcessingType.SUBTITLE)
    append_bytes_to_file(video_path, video_bytes)

    return make_response(id, 200)


@app.route("/subtitles/<id>/add", methods=["POST"])
def append_chunk_for_subtitle_video(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id, VideoProcessingType.SUBTITLE)
    if not exists:
        return video_does_not_exist_response(id)
    append_bytes_to_file(video_path, video_bytes)

    return make_response(id, 200)


@app.route("/subtitles/<id>/end", methods=["POST"])
def process_subtitle_video(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id, VideoProcessingType.SUBTITLE)
    if not exists:
        return video_does_not_exist_response(id)
    append_bytes_to_file(video_path, video_bytes)

    future_processed_subtitle_video_id = generate_uuid()
    subtitle_video_worker_pool.submit(
        generate_subtitle_video, video_path, future_processed_subtitle_video_id
    )

    return make_response(future_processed_subtitle_video_id, 200)


@app.route("/thumbnail/start", methods=["POST"])
def start_chunk_for_video_thumbnail() -> Response:
    video_bytes = request.data

    id = generate_uuid()
    video_path = generate_new_video_path(id, VideoProcessingType.THUMBNAIL)
    append_bytes_to_file(video_path, video_bytes)

    return make_response(id, 200)


@app.route("/thumbnail/<id>/add", methods=["POST"])
def append_chunk_for_video_thumbnail(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id, VideoProcessingType.THUMBNAIL)
    if not exists:
        return video_does_not_exist_response(id)
    append_bytes_to_file(video_path, video_bytes)

    return make_response(id, 200)


@app.route("/thumbnail/<id>/end", methods=["POST"])
def create_thumbnail_from_video(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id, VideoProcessingType.THUMBNAIL)
    if not exists:
        return video_does_not_exist_response(id)
    append_bytes_to_file(video_path, video_bytes)
    fix_video(video_path)

    thumbnail_bytesio = generate_thumbnail(video_path)
    os.remove(video_path)

    return make_response(send_file(thumbnail_bytesio, mimetype="image/jpeg"), 200)


if __name__ == "__main__":
    app.run(port=17191, debug=True)
