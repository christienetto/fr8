use std::process::{Command, exit};
use std::fs;
use std::path::{Path, PathBuf};

// Define the greet command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn upload_file(file_name: String, file_data: Vec<u8>) {
    // Define the upload directory path
    let upload_dir = "/home/chris/src/randy-bobandy/src-tauri/src/uploads";

    // Ensure the directory exists, if not, create it
    if !std::path::Path::new(upload_dir).exists() {
        std::fs::create_dir_all(upload_dir).expect("Failed to create uploads directory");
    }

    // Define the full file path
    let file_path = format!("{}/{}", upload_dir, file_name);

    // Save the file to the specified location
    std::fs::write(&file_path, file_data).expect("Unable to save file");

    println!("File uploaded successfully to: {}", file_path);

    // Optionally, you can perform further processing like converting with ffmpeg
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file_path)  // Using the uploaded file's path
        .arg("-ac")
        .arg("1")
        .arg("-ar")
        .arg("16000")
        .arg("-c:a")
        .arg("pcm_s16le")
        .arg(format!("{}/output.wav", upload_dir))  // Saving the output in the same directory
        .output()
        .expect("Failed to execute command");

    // Check if the command executed successfully
    if output.status.success() {
        println!("Command executed successfully.");
        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}


// Define the list_meetings command
#[tauri::command]
fn list_meetings() -> Result<Vec<String>, String> {
    let upload_dir = Path::new("/home/chris/src/randy-bobandy/src-tauri/src/uploads");
    if !upload_dir.exists() {
        return Err("Uploads directory not found".to_string());
    }
    let entries = fs::read_dir(upload_dir).map_err(|e| e.to_string())?;
    let meetings: Vec<String> = entries
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().into_string().ok())
                .filter(|name| Path::new(&upload_dir.join(name)).is_dir())
        })
        .collect();
    Ok(meetings)
}

// Define the get_meeting_files command
#[tauri::command]
fn get_meeting_files(meeting: String) -> Result<(Option<String>, Option<String>), String> {
    let meeting_dir = Path::new("/home/chris/src/randy-bobandy/src-tauri/src/uploads").join(meeting);
    if !meeting_dir.exists() || !meeting_dir.is_dir() {
        return Err("Meeting directory not found".to_string());
    }

    let mut video_file = None;
    let mut transcript_file = None;

    // Iterate through the files in the meeting directory
    for entry in fs::read_dir(meeting_dir).map_err(|e| e.to_string())? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                // Check for video file (.mp4)
                if ext == "mp4" {
                    video_file = path.file_name().and_then(|name| name.to_str().map(String::from));
                }
                // Check for transcript file (.txt or .wav)
                else if ext == "txt" || ext == "wav" {
                    transcript_file = path.file_name().and_then(|name| name.to_str().map(String::from));
                }
            }
        }
    }

    match (video_file, transcript_file) {
        (Some(video), Some(transcript)) => Ok((Some(video), Some(transcript))),
        _ => Err("Required files (video and transcript) not found".to_string()),
    }
}
#[tauri::command]
fn get_files_in_meeting(meeting: String) -> Result<Vec<String>, String> {
    let meeting_dir = Path::new("/home/chris/src/randy-bobandy/src-tauri/src/uploads").join(meeting);
    if !meeting_dir.exists() || !meeting_dir.is_dir() {
        return Err("Meeting directory not found".to_string());
    }

    let entries = fs::read_dir(meeting_dir).map_err(|e| e.to_string())?;
    let files: Vec<String> = entries
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.file_name().into_string().ok())
        })
        .collect();

    Ok(files)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // Register all commands in a single invoke_handler call
        .invoke_handler(tauri::generate_handler![greet, upload_file, list_meetings, get_meeting_files, get_files_in_meeting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
