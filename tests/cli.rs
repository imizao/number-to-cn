use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn run_with_input(input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_test-demo"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn binary");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .expect("failed to write input");
    }

    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        if child
            .try_wait()
            .expect("failed to wait for child")
            .is_some()
        {
            return child
                .wait_with_output()
                .expect("failed to collect child output");
        }

        if Instant::now() >= deadline {
            child.kill().expect("failed to kill hung child");
            panic!("binary did not exit after stdin closed");
        }

        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn exits_when_stdin_is_closed() {
    let output = run_with_input("");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).is_empty());
}

#[test]
fn prints_converted_number_without_debug_quotes() {
    let output = run_with_input("20\n");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Result is: 二十"));
    assert!(!stdout.contains("\"二十\""));
}

#[test]
fn continues_after_invalid_input_until_eof() {
    let output = run_with_input("abc\n1001\n");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Invalid input, please input a valid number"));
    assert!(stdout.contains("Result is: 一千零一"));
}
