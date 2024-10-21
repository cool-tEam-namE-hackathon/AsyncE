import concurrent.futures
import os
import tempfile
import uuid
from io import BytesIO
from typing import List, Tuple

import ffmpeg
import speech_recognition as sr
from flask import Flask, Response, request, send_file
from moviepy.editor import (ColorClip, CompositeVideoClip, TextClip,
                            VideoFileClip)
from PIL import Image
from pydub import AudioSegment

recognizer = sr.Recognizer()


def sphinx_transcriber(audio_data: sr.AudioData) -> str:
    print("transcribing using sphinx...")
    return recognizer.recognize_sphinx(audio_data)


def vosk_transcriber(audio_data: sr.AudioData) -> str:
    print("transcribing using vosk...")
    return recognizer.recognize_vosk(audio_data)


def whisper_transcriber(audio_data: sr.AudioData) -> str:
    print("transcribing using whisper...")
    return recognizer.recognize_whisper(audio_data)


transcribers = []
transcribers.append(whisper_transcriber)
if os.path.isdir("model"):
    transcribers.append(vosk_transcriber)
transcribers.append(sphinx_transcriber)

timestamp_chunk_duration_seconds = 5
timestamp_chunk_duration_ms = timestamp_chunk_duration_seconds * 1000
subtitle_height = 100

video_ids = []


def retrieve_video_as_bytesio(video_path: str) -> BytesIO:
    bytes_io = BytesIO()
    with open(video_path, "rb") as video:
        bytes_io.write(video.read())
    bytes_io.seek(0)
    os.remove(video_path)
    return bytes_io


def save_video(video, id: str):
    video.write_videofile(
        f"{id}.mp4",
        codec="libx264",
        audio_codec="aac",
        remove_temp=True,
        # preset="ultrafast", # will make the processing time much faster, but bigger output size
    )
    video_ids.append(id)


def generate_video_with_subtitles(
    input_video_path: str, transcription: List[Tuple[int, str]], id: str
) -> None:
    video = VideoFileClip(input_video_path)
    subtitle_clips = []

    for timestamp, text in transcription:
        bg_subtitle = ColorClip(size=(video.w, subtitle_height), color=(0, 0, 0, 128))
        bg_subtitle = (
            bg_subtitle.set_duration(timestamp_chunk_duration_seconds)
            .set_start(timestamp)
            .set_position(("center", "bottom"))
        )

        text_subtitle = TextClip(
            text,
            fontsize=32,
            color="white",
            size=(video.w, subtitle_height),
            method="caption",
        )
        text_subtitle = (
            text_subtitle.set_duration(timestamp_chunk_duration_seconds)
            .set_start(timestamp)
            .set_position(("center", "bottom"))
        )

        subtitle_clips.append(bg_subtitle)
        subtitle_clips.append(text_subtitle)

    output_video = CompositeVideoClip([video] + subtitle_clips)
    save_video(output_video, id)


def transcribe_audio(audio_path: str) -> List[Tuple[int, str]]:
    audio = AudioSegment.from_wav(audio_path)
    audio_duration_ms = len(audio)

    chunks = [
        audio[i : i + timestamp_chunk_duration_ms]
        for i in range(0, audio_duration_ms, timestamp_chunk_duration_ms)
    ]
    transcription = []

    for i, chunk in enumerate(chunks):
        with tempfile.NamedTemporaryFile() as chunk_audio_file:
            chunk_audio_path = chunk_audio_file.name
            chunk.export(chunk_audio_path, format="wav")

            with sr.AudioFile(chunk_audio_path) as source:
                audio_data = recognizer.record(source)

                for transcriber in transcribers:
                    try:
                        text = transcriber(audio_data)
                        timestamp_seconds = i * timestamp_chunk_duration_seconds
                        transcription.append((timestamp_seconds, text))
                        break
                    except sr.UnknownValueError as error:
                        print(f"the audio could not be transcribed: {error}")
                    except sr.RequestError as error:
                        print(f"could not request transcription: {error}")
                    except Exception as error:
                        print(
                            f"something went wrong when transcribing the audio: {error}"
                        )

    return transcription


def transcribe_video(video_path: str) -> List[Tuple[int, str]]:
    with tempfile.NamedTemporaryFile(suffix=".wav") as audio_file:
        audio_path = audio_file.name
        ffmpeg.input(video_path).output(audio_path).overwrite_output().run()

        return transcribe_audio(audio_path)


def convert_image_to_bytesio(image, format: str) -> BytesIO:
    bytes_io = BytesIO()
    image.save(bytes_io, format)
    bytes_io.seek(0)
    return bytes_io


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

worker_pool_executor = concurrent.futures.ThreadPoolExecutor(max_workers=5)


def edit_video_worker_task(video_bytes: bytes, id: str) -> None:
    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()
        video_path = video_file.name

        generate_video_with_subtitles(video_path, transcribe_video(video_path), id)


@app.route("/subtitles/<id>")
def get_video_with_subtitles(id: str) -> Response:
    for video_id in video_ids:
        if video_id == id:
            return send_file(
                retrieve_video_as_bytesio(f"{video_id}.mp4"), mimetype="video/mp4"
            )
    return Response(
        f"Video id '{id}' hasn't finished processing or doesn't exist",
        status=400,
    )


@app.route("/subtitles", methods=["POST"])
def create_video_with_subtitles() -> Response:
    video_bytes = request.data

    future_video_id = str(uuid.uuid4())
    worker_pool_executor.submit(edit_video_worker_task, video_bytes, future_video_id)
    return Response(future_video_id, status=200)


@app.route("/thumbnail", methods=["POST"])
def create_thumbnail_from_video() -> Response:
    video_bytes = request.data

    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()

        thumbnail = extract_thumbnail(video_file.name)

    return send_file(thumbnail, mimetype="image/jpeg")


if __name__ == "__main__":
    app.run(debug=True)
