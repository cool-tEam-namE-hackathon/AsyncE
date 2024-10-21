import concurrent.futures
import tempfile
import uuid

import config
from file_repository import (append_video_file, get_video_path,
                             retrieve_subtitle_video)
from flask import Flask, Response, request, send_file
from subtitles import generate_subtitle_video
from thumbnail import generate_thumbnail


def generate_uuid():
    return str(uuid.uuid4())


worker_pool_executor = concurrent.futures.ThreadPoolExecutor(max_workers=3)

app = Flask(__name__)


@app.route("/subtitles/<id>")
def get_subtitle_video(id: str) -> Response:
    processed_video_bytesio = retrieve_subtitle_video(id)
    if processed_video_bytesio == None:
        return Response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            status=400,
        )
    return send_file(
        processed_video_bytesio,
        mimetype=f"video/{config.video_output_format_ext}",
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
