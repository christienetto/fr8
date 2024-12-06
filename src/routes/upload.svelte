<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import "../styles.css";
  
    let file: File | null = null;
  
    function onFileChange(event: Event) {
      const target = event.target as HTMLInputElement;
      file = target?.files ? target.files[0] : null;
    }
  
    async function uploadFile() {
      if (file) {
        const fileData = await file.arrayBuffer();
        try {
          await invoke("upload_file", {
            fileName: file.name,
            fileData: Array.from(new Uint8Array(fileData)),
          });
          alert("File uploaded successfully!");
        } catch (error) {
          alert("Error uploading file: " + error);
        }
      } else {
        alert("Please select a file first.");
      }
    }
  </script>
  
  <main class="container">
    <h1 class="title">Upload a File</h1>
  
    <section class="upload-section">
      <input type="file" class="file-input" on:change={onFileChange} />
      <button class="upload-button" on:click={uploadFile}>Upload File</button>
    </section>
  </main>
  
  <style>
    /* Add your styles here */
  </style>
  