#!/bin/bash

python3 -m env .venv
source .venv/bin/activate
pip install -r src/video_editor_backend/requirements.txt
