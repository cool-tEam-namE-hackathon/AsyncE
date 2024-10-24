enable_transcription_using_whisper = True
subtitle_height_px = 100
timestamp_chunk_duration_seconds = 5
timestamp_chunk_duration_ms = timestamp_chunk_duration_seconds * 1000

video_io_format_ext = "webm"
image_output_format_ext = "jpeg"

retrieve_video_chunk_size_bytes = int(1.8 * 1024 * 1024)

concat_video_processor_workers = 2
subtitle_video_processor_workers = 1  # do not set higher than 1 if transcription using whisper is enabled (will consume too much memory)

verbose_debug_prints = False
custom_log_prints = True
