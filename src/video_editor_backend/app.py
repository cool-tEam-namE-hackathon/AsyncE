import concurrent.futures
import os
import tempfile
from io import BytesIO

import config
from file_repository import (append_video_file,
                             get_processed_subtitle_video_path, get_video_path)
from flask import Flask, Response, jsonify, make_response, request, send_file
from subtitles import generate_subtitle_video
from thumbnail import generate_thumbnail
from utils import generate_uuid, get_chunk_count_and_file_size


def extract_video_chunk(video_path: str, chunk_number: int) -> BytesIO:
    with open(video_path, "rb") as video_file:
        video_file.seek((chunk_number - 1) * config.retrieve_video_chunk_size_bytes)
        return BytesIO(video_file.read(config.retrieve_video_chunk_size_bytes))


worker_pool_executor = concurrent.futures.ThreadPoolExecutor(
    max_workers=config.video_processor_workers
)

app = Flask(__name__)


@app.route("/subtitles/<id>")
def get_processed_subtitle_video_info(id: str) -> Response:
    video_path = get_processed_subtitle_video_path(id)
    if video_path == None:
        return make_response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            400,
        )

    chunk_count, file_size = get_chunk_count_and_file_size(video_path)
    return make_response(
        jsonify({"chunk_count": chunk_count, "file_size": file_size}), 200
    )


@app.route("/subtitles/<id>/<int:number>")
def get_processed_subtitle_video(id: str, number: int) -> Response:
    if number < 1:
        return make_response("Invalid given number is not possible", 400)
    video_path = get_processed_subtitle_video_path(id)
    if video_path == None:
        return make_response(
            f"Video id '{id}' either hasnn't yet finished processing or doesn't exist",
            400,
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
            mimetype=f"application/octet-stream",
        ),
        200,
    )


@app.route("/subtitles/start", methods={"POST"})
def start_chunk_for_subtitle_video() -> Response:
    video_bytes = request.data

    while True:
        id = generate_uuid()
        video_path, exists = get_video_path(id)
        if not exists:
            break

    append_video_file(video_path, video_bytes)
    return make_response(id, 200)


@app.route("/subtitles/<id>/add", methods=["PUT"])
def append_chunk_for_subtitle_video(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id)
    if not exists:
        return make_response(f"Video id '{id}' doesn't exist", 400)
    append_video_file(video_path, video_bytes)
    return make_response(id, 200)


@app.route("/subtitles/<id>/end", methods=["POST"])
def create_subtitle_video(id: str) -> Response:
    video_bytes = request.data

    video_path, exists = get_video_path(id)
    if not exists:
        return make_response(f"Video id '{id}' doesn't exist", 400)
    append_video_file(video_path, video_bytes)

    output_video_id = generate_uuid()
    worker_pool_executor.submit(generate_subtitle_video, video_path, output_video_id)
    return make_response(output_video_id, 200)


@app.route("/thumbnail", methods=["POST"])
def create_thumbnail_from_video() -> Response:
    video_bytes = request.data

    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()

        thumbnail_bytesio = generate_thumbnail(video_file.name)

    return make_response(send_file(thumbnail_bytesio, mimetype="image/jpeg"), 200)


if __name__ == "__main__":
    app.run(debug=True)
