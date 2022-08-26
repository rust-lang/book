#!/bin/bash

cd $HOME/rust-book/telemetry/notebooks
source .env/bin/activate
python3 build_summaries.py
