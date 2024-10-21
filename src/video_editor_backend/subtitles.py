import os
import tempfile
from typing import List, Tuple

import config
import ffmpeg
import speech_recognition as sr
from file_db import save_video
from moviepy.editor import (ColorClip, CompositeVideoClip, TextClip,
                            VideoFileClip)
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


def transcribe_audio(audio_path: str) -> List[Tuple[int, str]]:
    audio = AudioSegment.from_wav(audio_path)
    audio_duration_ms = len(audio)

    chunks = [
        audio[i : i + config.timestamp_chunk_duration_ms]
        for i in range(0, audio_duration_ms, config.timestamp_chunk_duration_ms)
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
                        timestamp_seconds = i * config.timestamp_chunk_duration_seconds
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


def generate_video_with_subtitles(video_path: str, video_id: str) -> None:
    transcription = transcribe_video(video_path)
    video = VideoFileClip(video_path)
    subtitle_clips = []

    for timestamp, text in transcription:
        bg_subtitle = ColorClip(
            size=(video.w, config.subtitle_height_px), color=(0, 0, 0, 128)
        )
        bg_subtitle = (
            bg_subtitle.set_duration(config.timestamp_chunk_duration_seconds)
            .set_start(timestamp)
            .set_position(("center", "bottom"))
        )

        text_subtitle = TextClip(
            text,
            fontsize=32,
            color="white",
            size=(video.w, config.subtitle_height_px),
            method="caption",
        )
        text_subtitle = (
            text_subtitle.set_duration(config.timestamp_chunk_duration_seconds)
            .set_start(timestamp)
            .set_position(("center", "bottom"))
        )

        subtitle_clips.append(bg_subtitle)
        subtitle_clips.append(text_subtitle)

    output_video = CompositeVideoClip([video] + subtitle_clips)
    save_video(output_video, video_id)
