import os
import tempfile
from typing import List, Tuple

import config
import speech_recognition as sr
from file_repository import save_subtitle_video
from moviepy.editor import (ColorClip, CompositeVideoClip, TextClip,
                            VideoFileClip)
from pydub import AudioSegment

recognizer = sr.Recognizer()


def sphinx_transcriber(audio_data: sr.AudioData) -> str:
    if config.custom_log_prints:
        print("transcribing using sphinx...")
    return recognizer.recognize_sphinx(audio_data)


def vosk_transcriber(audio_data: sr.AudioData) -> str:
    if config.custom_log_prints:
        print("transcribing using vosk...")
    return recognizer.recognize_vosk(audio_data)


def whisper_transcriber(audio_data: sr.AudioData) -> str:
    if config.custom_log_prints:
        print("transcribing using whisper...")
    return recognizer.recognize_whisper(audio_data)


transcribers = []
if config.enable_transcription_using_whisper:
    transcribers.append(whisper_transcriber)
if os.path.isdir("model"):
    transcribers.append(vosk_transcriber)
transcribers.append(sphinx_transcriber)


def transcribe_audio(audio) -> List[Tuple[int, str]]:
    audio_duration_ms = len(audio)
    chunks = []
    for i in range(0, audio_duration_ms, config.timestamp_chunk_duration_ms):
        if i + config.timestamp_chunk_duration_ms > audio_duration_ms:
            chunk_end_ms = audio_duration_ms
        else:
            chunk_end_ms = i + config.timestamp_chunk_duration_ms
        chunks.append(audio[i:chunk_end_ms])

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
    return transcribe_audio(
        AudioSegment.from_file(video_path, format=config.video_io_format_ext)
    )


def generate_subtitle_video(input_video_path: str, output_video_id: str):
    transcription = transcribe_video(input_video_path)
    video = VideoFileClip(input_video_path)
    video_length = video.duration
    subtitle_clips = []

    for start_seconds, text in transcription:
        seconds_remaining = video_length - start_seconds
        if seconds_remaining < config.timestamp_chunk_duration_seconds:
            subtitle_duration = seconds_remaining
        else:
            subtitle_duration = config.timestamp_chunk_duration_seconds

        bg_subtitle = ColorClip(
            size=(video.w, config.subtitle_height_px), color=(0, 0, 0, 128)
        )
        bg_subtitle = (
            bg_subtitle.set_duration(subtitle_duration)
            .set_start(start_seconds)
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
            text_subtitle.set_duration(subtitle_duration)
            .set_start(start_seconds)
            .set_position(("center", "bottom"))
        )

        subtitle_clips.append(bg_subtitle)
        subtitle_clips.append(text_subtitle)

    output_video = CompositeVideoClip([video] + subtitle_clips)
    save_subtitle_video(output_video, output_video_id)
    os.remove(input_video_path)
