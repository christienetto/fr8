<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation"; // To navigate to a different page
  import "./styles.css";

  let meetings: string[] = [];

  // Automatically load the meetings when the page loads
  onMount(async () => {
    try {
      meetings = await invoke("list_meetings");
    } catch (error) {
      alert("Error fetching meetings: " + error);
    }
  });

  // Function to handle meeting click and navigate to the detail page
  function navigateToMeeting(meeting: string) {
    goto(`/${meeting}`); // Navigate to the detail page of the clicked meeting
  }

  // Navigate to the upload page
  function navigateToUpload() {
    goto('/upload'); // Navigate to the upload page
  }
</script>

<main class="container">
  <h1 class="title">TLDR</h1>

  <!-- Upload File Button -->
  <section class="upload-section">
    <button class="upload-button" on:click={navigateToUpload}>Upload File</button>
  </section>

  <!-- List Meetings Section -->
  <section class="meeting-list-section">
    <h2 class="section-title">List of Meetings</h2>
    

    <video width="320" height="240" controls>
      <source src="movie.mp4" type="video/mp4">
      <source src="movie.ogg" type="video/ogg">
    Your browser does not support the video tag.
    </video>



    <ul class="meeting-list">
      {#each meetings as meeting}
        <li class="meeting-item" on:click={() => navigateToMeeting(meeting)}>{meeting}</li>
      {/each}
    </ul>
  </section>
</main>
