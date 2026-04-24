#!/bin/sh

cat DEVELOP.prompt.md | \
    gemini --skip-trust --approval-mode yolo --model gemini-3.1-pro-preview
