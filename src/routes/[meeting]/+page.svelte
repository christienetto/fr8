<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { page } from '$app/stores'; // To access route params (meeting name)
    import "../styles.css";
  
    let meetingFiles: string[] = [];
    let meetingName: string | undefined;
  
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
            <li class="file-item">{file}</li>
          {/each}
        </ul>
      </section>
    {:else}
      <p>No files available for this meeting.</p>
    {/if}
  </main>
  