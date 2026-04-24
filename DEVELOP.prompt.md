Create a Rust CLI tool for passing images into LLMs.

`README.md` specifies the tool's name and feature set.

## Testing

### Test data generation

Write a helper Rust command line utility that renders some text into a `.png`
image.

* Make it possible to invoke the utility from `cargo`.
* Make `ai-vision` the default binary.
* Make it so that `cargo install <repo_url>` only installs the `ai-vision`
  binary.

Invoke the utility to generate images that say "Hello world" and
"Hello AI agents". Save the images in `tests/testdata/`, so they can be used
by the tests below.

### Integration tests

Write the following Rust integration tests. Use the command line examples above.

1. (one test per image) Ask the LLM to reply with the text in each image.
   Check that the output matches the text used to generate the image.

2. Ask the LLM to output the concatenated text in the two images. Check that
   the output matches the texts used to generate the two images.

3. Redo one of the tests in step 1, but use the model override to specify the
   `gemini-flash-latest` model.

## Coding standards

1. Use the 2024 Rust edition.
2. Integrate Rust Clippy in the build process, to catch common errors.
3. Disallow `unsafe` usage in all the code generated for this project.
4. Run `rustfmt` after implementing all the features and tests.
