import concurrent.futures
import tempfile
import uuid

import config
from file_db import retrieve_video
from flask import Flask, Response, request, send_file
from subtitles import generate_video_with_subtitles
from thumbnail import generate_thumbnail

worker_pool_executor = concurrent.futures.ThreadPoolExecutor(max_workers=3)


def edit_video_with_subtitles_task(video_bytes: bytes, video_id: str) -> None:
    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()
        video_path = video_file.name

        generate_video_with_subtitles(video_path, video_id)


app = Flask(__name__)


@app.route("/subtitles/<id>")
def get_video_with_subtitles(id: str) -> Response:
    processed_video_bytesio = retrieve_video(id)
    if processed_video_bytesio == None:
        return Response(
            f"Video id '{id}' processing hasn't finished yet or doesn't exist",
            status=400,
        )
    return send_file(
        processed_video_bytesio,
        mimetype=f"video/{config.video_output_format_ext}",
    )


@app.route("/subtitles", methods=["POST"])
def create_video_with_subtitles() -> Response:
    video_bytes = request.data

    video_id = str(uuid.uuid4())
    worker_pool_executor.submit(edit_video_with_subtitles_task, video_bytes, video_id)
    return Response(video_id, status=200)


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
