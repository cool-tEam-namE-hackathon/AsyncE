import tempfile
from io import BytesIO

from flask import Flask, Response, request, send_file
from moviepy.editor import VideoFileClip
from PIL import Image


def convert_image_to_bytesio(image, format: str) -> BytesIO:
    img_io = BytesIO()
    image.save(img_io, format)
    img_io.seek(0)
    return img_io


def extract_thumbnail(video_path: str, format: str = "jpeg") -> BytesIO:
    with VideoFileClip(video_path) as clip:
        if clip.duration >= 1:
            time = 1
        else:
            time = 0
        frame = clip.get_frame(time)

    thumbnail = Image.fromarray(frame)
    return convert_image_to_bytesio(thumbnail, format)


app = Flask(__name__)


@app.route("/thumbnail", methods=["POST"])
def create_thumbnail_from_video() -> Response:
    video_bytes = request.data

    with tempfile.NamedTemporaryFile(delete=True) as tmp_video_file:
        tmp_video_file.write(video_bytes)
        tmp_video_file.flush()
        thumbnail = extract_thumbnail(tmp_video_file.name)

    return send_file(thumbnail, mimetype="image/jpeg")


if __name__ == "__main__":
    app.run(debug=True)
