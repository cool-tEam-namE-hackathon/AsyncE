import os
import tempfile
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


def convert_video_to_bytesio(video) -> BytesIO:
    with tempfile.NamedTemporaryFile(suffix=".mp4") as output_video:
        output_video_path = output_video.name
        video.write_videofile(
            output_video_path,
            codec="libx264",
            audio_codec="aac",
            remove_temp=True,
            preset="ultrafast",
        )

        bytes_io = BytesIO()
        bytes_io.write(output_video.read())
        bytes_io.seek(0)
        return bytes_io


def edit_subtitles_into_video(
    input_video_path: str, transcription: List[Tuple[int, str]]
) -> BytesIO:
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
    return convert_video_to_bytesio(output_video)


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


@app.route("/subtitles", methods=["POST"])
def create_video_with_subtitles() -> Response:
    video_bytes = request.data

    with tempfile.NamedTemporaryFile() as video_file:
        video_file.write(video_bytes)
        video_file.flush()
        video_path = video_file.name

        video_with_subtitles = edit_subtitles_into_video(
            video_path, transcribe_video(video_path)
        )

    return send_file(video_with_subtitles, mimetype="video/mp4")


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
