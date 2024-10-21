import concurrent.futures
import math
import os
import tempfile
import uuid
from io import BytesIO

import config
from file_repository import (append_video_file,
                             get_processed_subtitle_video_path, get_video_path)
from flask import Flask, Response, request, send_file
from subtitles import generate_subtitle_video
from thumbnail import generate_thumbnail


def generate_uuid():
    return str(uuid.uuid4())


def get_chunk_count(video_path: str) -> int:
    return math.ceil(
        os.path.getsize(video_path) / config.retrieve_video_chunk_size_bytes
    )


worker_pool_executor = concurrent.futures.ThreadPoolExecutor(max_workers=3)

app = Flask(__name__)


@app.route("/subtitles/<id>")
def get_subtitle_video_chunk_count(id) -> Response:
    video_path = get_processed_subtitle_video_path(id)
    if video_path == None:
        return Response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            status=400,
        )

    chunk_count = get_chunk_count(video_path)
    return Response(str(chunk_count), status=200)


@app.route("/subtitles/<id>/<int:number>")
def get_subtitle_video(id: str, number: int) -> Response:
    if number < 1:
        return Response("Invalid given number is not possible", status=400)

    video_path = get_processed_subtitle_video_path(id)
    if video_path == None:
        return Response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            status=400,
        )

    chunk_count = get_chunk_count(video_path)
    if number > chunk_count:
        return Response("Given chunk number exceeds chunk count", status=400)

    with open(video_path, "rb") as video_file:
        video_file.seek((number - 1) * config.retrieve_video_chunk_size_bytes)
        video_bytesio = BytesIO(video_file.read(config.retrieve_video_chunk_size_bytes))

    if number == chunk_count:
        os.remove(video_path)

    return send_file(
        video_bytesio,
        mimetype=f"application/octet-stream",
    )


@app.route("/subtitles", methods={"POST"})
def start_chunk_for_subtitle_video() -> Response:
    video_bytes = request.data

    id = generate_uuid()
    append_video_file(get_video_path(id), video_bytes)
    return Response(id, status=200)


@app.route("/subtitles/<id>", methods=["PUT"])
def append_chunk_for_subtitle_video(id) -> Response:
    video_bytes = request.data

    append_video_file(get_video_path(id), video_bytes)
    return Response(id, status=200)


@app.route("/subtitles/<id>", methods=["POST"])
def create_subtitle_video(id) -> Response:
    video_bytes = request.data

    video_path = get_video_path(id)
    append_video_file(video_path, video_bytes)
    output_video_id = generate_uuid()
    worker_pool_executor.submit(generate_subtitle_video, video_path, output_video_id)
    return Response(output_video_id, status=200)


@app.route("/thumbnail", methods=["POST"])
def create_thumbnail_from_video() -> Response:
    video_bytes = request.data

    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()

        thumbnail_bytesio = generate_thumbnail(video_file.name)

    return send_file(thumbnail_bytesio, mimetype="image/jpeg")


if __name__ == "__main__":
    app.run(debug=True)
