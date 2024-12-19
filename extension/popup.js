let mediaRecorder;
let recordedChunks = [];
let recordedBlob;

async function startRecording() {
  try {
    // Request screen sharing
    const stream = await navigator.mediaDevices.getDisplayMedia({
      video: { mediaSource: "screen" },
      audio: true, // Optional: Capture audio as well
    });

    // Initialize MediaRecorder
    mediaRecorder = new MediaRecorder(stream);

    // Collect data chunks
    mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        recordedChunks.push(event.data);
      }
    };

    // Handle stop event
    mediaRecorder.onstop = () => {
      recordedBlob = new Blob(recordedChunks, { type: "video/webm" });
      console.log("Recording complete. Ready for upload.");
      document.getElementById("uploadBtn").disabled = false; // Enable upload button
      recordedChunks = [];
    };

    // Start recording
    mediaRecorder.start();
    console.log("Recording started!");

    // Stop recording after 10 seconds (example)
    setTimeout(() => {
      mediaRecorder.stop();
      console.log("Recording stopped!");
    }, 10000);

  } catch (err) {
    console.error("Error accessing screen media:", err);
    alert("Screen recording failed: " + err.message);
  }
}

async function uploadRecording() {
  if (!recordedBlob) {
    alert("No recording available to upload.");
    return;
  }

  const formData = new FormData();
  formData.append("file", recordedBlob, "screen-recording.webm");

  try {
    const response = await fetch("http://localhost:9424/upload", {
      method: "POST",
      body: formData,
    });

    if (response.ok) {
      console.log("Upload successful!");
      alert("Recording uploaded successfully.");
    } else {
      console.error("Upload failed:", response.statusText);
      alert("Failed to upload recording.");
    }
  } catch (err) {
    console.error("Error uploading recording:", err);
    alert("An error occurred during upload.");
  }
}

// Attach event listeners
document.getElementById("startRecordingBtn").addEventListener("click", startRecording);
document.getElementById("uploadBtn").addEventListener("click", uploadRecording);
