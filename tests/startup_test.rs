use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;

#[test]
#[ignore] // Ignored by default since it requires a display/graphics environment
fn test_app_starts_without_crashing() {
    // Spawn the application in the background
    let mut child = Command::new("cargo")
        .args(&["run", "--release"])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn application");

    // Let it run for a few seconds (enough time to get past startup and a few frames)
    thread::sleep(Duration::from_secs(3));

    // Try to kill it gracefully
    let _ = child.kill();
    let output = child.wait_with_output().expect("Failed to wait for child");

    // Check if there were any panics or crashes in stderr
    let stderr = String::from_utf8_lossy(&output.stderr);

    // If the app panicked or had a critical error, it will show up in stderr
    assert!(
        !stderr.contains("panicked") && !stderr.contains("SIGABRT") && !stderr.contains("SIGSEGV"),
        "Application crashed or panicked during startup: {}",
        stderr
    );

    println!("Application started successfully and ran for 3 seconds without crashing");
}
