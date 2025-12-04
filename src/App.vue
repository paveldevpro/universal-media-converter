<script setup lang="ts">
import { Ref, ref } from "vue";
import ComboBox from "./components/ComboBox.vue";
import DropFile from "./components/DropFile.vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { FileInfo } from "./types";

const inputFiles: Ref<FileInfo[]> = ref([]);
const dropping = ref(false);

/*function onNewFiles(files: FileList)
{
  inputFiles.value.push(...files);
  console.log(inputFiles.value);
}*/

function removeFile(i: number)
{
  inputFiles.value.splice(i, 1);
}


listen('tauri://drag-over', () => {
  dropping.value = true;
});

listen('tauri://drag-leave', () => {
  dropping.value = false;
});

listen('tauri://drag-drop', e => {
  dropping.value = false;

  const payload: any = e.payload;
  const paths: Array<String> = payload.paths;

  console.log(paths);

  invoke("add_file", { filepath: paths[0] });
});

listen("file-added", e => {
  console.log(e.payload);

  const payload: any = e.payload;
  const fileInfo: FileInfo = {
    name: payload
  };

  inputFiles.value.push(fileInfo);
});

</script>

<template>
  <main class="container">
    <h2>Universal Media Converter (UMC)</h2>
    <DropFile />

    <div class="files-container mt-5 mb-5">
      <div v-for="file in inputFiles" :key="file.name" class="file-item">
        <div class="file-info">
          <p class="font-semibold text-amber-300">{{ file.name }}</p>
        </div>

        <div class="file-params">
          <div class="flex items-center">
            <div class="mr-2">Output :</div>
            <ComboBox />
          </div>

          <button class="file-convert-btn">
            Convert
          </button>

          <button class="icon-btn close-btn ml-10" @click="removeFile(inputFiles.indexOf(file))">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24">
              <path fill="currentColor" d="M20 6.91L17.09 4L12 9.09L6.91 4L4 6.91L9.09 12L4 17.09L6.91 20L12 14.91L17.09 20L20 17.09L14.91 12z"/>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <h3>Global Settings</h3>
    <div>
      <label>
        Output Format :
        <ComboBox />
      </label>
    </div>

    <div class="drop-notification" v-if="dropping">
      <img
        src="./assets/mdi--file-document-add.svg"
        alt="Add Files Icon"
        class="w-15"
      />
      <div>Drop any files here</div>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>

<style>
:root {
  font-family: "Elms Sans", Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.icon-btn {
  padding: 0;
  background: none;
  border: none;
  box-shadow: none;
  color: #e8e8e8;
}

.icon-btn:active {
  background: none;
}

.close-btn {
  transition: color .2s;
}

.close-btn:hover {
  color: rgb(255, 80, 80);
}

.close-btn:active {
  color: rgb(201, 63, 63);
}

h2 {
  font-size: x-large;
  font-weight: bold;
  margin-top: 10px;
  margin-bottom: 30px;
}

h3 {
  font-size: large;
  font-weight: bold;
  margin-top: 5px;
  margin-bottom: 10px;
}

.drop-notification {
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background-color: rgb(0, 93, 180);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 20px;
  font-size: x-large;
}

.container {
  width: 100%;
  margin: 0;
  padding: 0;
  border: 0;
  display: flex;
  flex-direction: column;
  justify-content: start;
  align-items: center;
  text-align: center;
}

.files-container {
  display: flex;
  flex-direction: column;
  align-items: start;
  /*border: 1px solid rgb(63, 96, 139);*/
  width: 70%;
}

.file-item {
  display: flex;
  justify-content: start;
  padding: 5px 10px 5px 10px;
  align-items: center;
  width: 100%;
  border: 1px solid rgb(107, 107, 107);
}

.file-info {
  display: flex;
  justify-content: start;
  width: 40%;
}

.file-params {
  display: flex;
  justify-content: end;
  width: 60%;
}

.file-convert-btn {
  padding: 5px;
  margin-left: 10px;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 10px;
  font-size: 18px;
}

.drop {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 300px;
  gap: 10px;
  background-color: #4b4b4b;
  border: 2px dashed rgb(167, 167, 167);
  border-radius: 10px;
  cursor: pointer;
}

.drop:hover {
  background-color: #242424;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
select,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
select,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  select {
    /*appearance: none;*/
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>