use std::process::Command;

fn ai_vision_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ai-vision"))
}

#[test]
fn test_hello_world() {
    let output = ai_vision_cmd()
        .arg("--image")
        .arg("tests/testdata/hello_world.png")
        .arg("Reply with only the text in this image, exactly as written.")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed. stderr: {}",
        stderr
    );
    assert!(stdout.contains("hello world"), "stdout was: {}", stdout);
}

#[test]
fn test_hello_ai_agents() {
    let output = ai_vision_cmd()
        .arg("--image")
        .arg("tests/testdata/hello_ai_agents.png")
        .arg("Reply with only the text in this image, exactly as written.")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed. stderr: {}",
        stderr
    );
    assert!(stdout.contains("hello ai agents"), "stdout was: {}", stdout);
}

#[test]
fn test_concatenated_images() {
    let output = ai_vision_cmd()
        .arg("--image")
        .arg("tests/testdata/hello_world.png")
        .arg("--image")
        .arg("tests/testdata/hello_ai_agents.png")
        .arg("Reply with the concatenated text of the two images, separated by a space.")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed. stderr: {}",
        stderr
    );
    assert!(stdout.contains("hello world"), "stdout was: {}", stdout);
    assert!(stdout.contains("hello ai agents"), "stdout was: {}", stdout);
}

#[test]
fn test_hello_world_flash_model() {
    let output = ai_vision_cmd()
        .arg("--model")
        .arg("gemini-flash-latest")
        .arg("--image")
        .arg("tests/testdata/hello_world.png")
        .arg("Reply with only the text in this image, exactly as written.")
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed. stderr: {}",
        stderr
    );
    assert!(stdout.contains("hello world"), "stdout was: {}", stdout);
}
