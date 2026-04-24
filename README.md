# ai-vision-cli

## Installation

The repository name is `ai-vision-cli`. The binary name is `ai-vision`.

```posix-terminal
cargo install --git https://github.com/pwnall/ai-vision-cli
ai-vision --help
```

## Features

### Basic usage

The `--image` argument specifies the path to an image file. Supported
extensions: `.png`, `.jpeg`, `.jpg`, `.webp`.

The text prompt is an optional positional argument.

The model's text response is written to the standard output.

```posix-terminal
ai-vision --image input.png "Reply with the text in this image."
```

### File input for the text prompt

The text prompt can be read from a file passed via the `--text` argument.

```posix-terminal
echo "Reply with the text in this image." > input.txt
ai-vision --image input.png --text input.txt
```

### Multiple images

The `--image` argument can be passed multiple times to upload multiple images.

```posix-terminal
ai-vision --image first.png --image second.png \
    "Reply with a bulleted list of differences between the two images."
```

### LLM API server

The tool uses the
[OpenAI Chat Completions API][openai-completions-api-reference].

The default API server is
[the Gemini OpenAI-compatible endpoint][gemini-openai-endpoint-docs],
`https://generativelanguage.googleapis.com/v1beta/openai`. The
default model is `gemini-pro-latest`, the [latest release][gemini-latest-naming]
of Gemini Pro.

#### Model selection

The `--model` argument overrides the model name passed to the API server.

```posix-terminal
ai-vision --model gemini-flash-latest --image input.png \
    "Reply with the text in this image."
```

#### Endpoint selection

The `--api-base-url` argument overrides the base URL for the LLM API server.

```posix-terminal
ai-vision --api-base-url http://localhost:8080/v1 --model gemini-flash-latest \
    --image input.png "Reply with the text in this image."
```

#### Authentication

The API key is read from one of the following environment variables (in order):

* `GEMINI_API_KEY`
* `OPENAI_API_KEY`

### Error messages

1. Missing input file - "File not found: {image_file}"
2. Unsupported file extension - "Unsupported image format: {image_file}"
3. Missing API key - "API key not found. Set GEMINI_API_KEY or OPENAI_API_KEY."

### Documentation

The `--help` argument works as expected.

[gemini-last-naming]: https://ai.google.dev/gemini-api/docs/models#latest
[gemini-openai-endpoint-docs]: https://ai.google.dev/gemini-api/docs/openai
[openai-completions-api-reference]: https://developers.openai.com/api/reference/resources/chat/subresources/completions
