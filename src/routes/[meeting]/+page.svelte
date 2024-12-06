<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { page } from '$app/stores'; // To access route params (meeting name)
    import "../styles.css";
    
    let meetingFiles: string[] = [];
    let meetingName: string | undefined;
    let selectedVideo: string | null = null;
  
    // Automatically load files for the selected meeting when the page loads
    onMount(async () => {
      // Get the meeting name from the route parameters
      const { meeting } = $page.params;
      meetingName = meeting;
    
      try {
        meetingFiles = await invoke("get_files_in_meeting", { meeting: meetingName });
      } catch (error) {
        alert("Error fetching files for the meeting: " + error);
      }
    });
    
    // Filter for mp4 and wav files
    function filterMediaFiles(files: string[]) {
      return files.filter(file => file.endsWith('.mp4') || file.endsWith('.wav'));
    }
    
    // Handle video selection
    function selectVideo(file: string) {
      selectedVideo = file; // Set selected video file
    }
    
    // Go back to the previous page
    function goBack() {
      window.history.back();
    }
  </script>
  
  <main class="container">
    <h1 class="title">Files in {meetingName}</h1>
  
    <!-- Go Back Button -->
    <button class="go-back-button" on:click={goBack}>Go Back</button>
  
    <!-- Meeting Files Section -->
    {#if meetingFiles.length > 0}
      <section class="meeting-files-section">
        <h2 class="section-title">Files</h2>
        <ul class="meeting-files-list">
          {#each filterMediaFiles(meetingFiles) as file}
            <li class="file-item" on:click={() => selectVideo(file)}>
              {file}
            </li>
          {/each}
        </ul>
      </section>
    
      <!-- Video Section (if a video is selected) -->
      {#if selectedVideo && selectedVideo.endsWith('.mp4')}
      <li>{selectedVideo}</li>
        <section class="video-section">
          <h2>Playing: {selectedVideo}</h2>
          <video width="100%" controls>
            <source src={`https://strona.agency/strapi/uploads/leo_Feature_Video_a16da38438.mp4`} type="video/mp4">
            <track kind="captions" src="" srclang="en" label="English">
            Your browser does not support the video tag.
          </video>
          
          
        </section>
      {/if}
    {:else}
      <p>No files available for this meeting.</p>
    {/if}
  </main>
  
  <style>
    .file-item {
      cursor: pointer;
      padding: 10px;
      border: 1px solid #ccc;
      margin: 5px 0;
    }
  
    .file-item:hover {
      background-color: #f0f0f0;
    }
  
    .go-back-button {
      background-color: #007BFF;
      color: white;
      padding: 10px 20px;
      border: none;
      border-radius: 5px;
      cursor: pointer;
    }
  
    .go-back-button:hover {
      background-color: #0056b3;
    }
  
    .video-section {
      margin-top: 20px;
    }
  
    video {
      max-width: 100%;
      margin-top: 10px;
    }
  </style>
  