<script setup lang="ts">

</script>

<template>
<div class="box">
    <select name="input-format" id="input-format">
        <option value="mp3">MP3</option>
        <option value="m4a">M4A</option>
        <option value="flac">FLAC</option>
        <option value="weba">WEBA</option>
    </select>

    <!-- flèche SVG HTML -->
    <svg class="arrow" viewBox="0 0 20 20" fill="white" xmlns="http://www.w3.org/2000/svg">
        <path d="M5.25 7.5L10 12.25L14.75 7.5H5.25Z"/>
    </svg>
</div>
</template>

<style scoped>
.box {
  position: relative;
  display: inline-flex;
  align-items: center;
  background-color: #333;
  border: 1px solid #555;
  border-radius: 6px;
  color: white;
  overflow: hidden;
}

.box select {
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;

  background-color: #0f0f0f98;
  padding: 0.5em 2em 0.5em 0.8em;
}

.box .arrow {
  position: absolute;
  right: 0.2em;
  pointer-events: none; /* pour éviter de bloquer le clic */
  width: 1em;
  height: 1em;
}

</style>
