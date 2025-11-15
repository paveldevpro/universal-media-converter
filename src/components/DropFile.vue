<script setup lang="ts">
    import { Ref, ref } from 'vue';

    const emits = defineEmits(['change']);

    const isDragging = ref(false);
    const fileInput: Ref<any | null> = ref(null);
    //const files: Ref<File[]> = ref([]);

    function onChange()
    {
        //files.value.push(...fileInput.value?.files);
        emits('change', fileInput.value?.files);
        const dt = new DataTransfer();
        fileInput.value.files = dt.files;
    }

    function drop(e: any)
    {
        if (e.dataTransfer.files.length > 0)
        {
            fileInput.value.files = e.dataTransfer.files;
            onChange();
        }

        isDragging.value = false;
    }

    /*function toFileList(filesArray: File[])
    {
        const dt = new DataTransfer();
        filesArray.forEach(f => dt.items.add(f));
        return dt.files;
    }

    function remove(i: number)
    {
        //files.value.splice(i, 1);
        //fileInput.value.files = toFileList(files.value);
    }*/
</script>

<template>
    <div class="main">
        <div
            class="dropzone-container"
            :style="isDragging && 'border-color: lime;'"
            @dragover.prevent="isDragging = true"
            @dragleave="isDragging = false"
            @drop.prevent="drop"
            @click="fileInput.click()"
        >
            <input
                type="file"
                multiple
                name="file"
                id="fileInput"
                class="hidden-input"
                @change="onChange"
                ref="fileInput"
                accept=".weba,.flac,.m4a,.mp3"
            />
            
            <div class="dropzone-content">
                <div v-if="isDragging"><b>Release</b> to drop files here.</div>
                <div v-else><b>Drop</b> files here or <b>click</b> to upload.</div>
            </div>

            <!--
            <div class="preview-container" v-if="files.length">
                <div v-for="file in files" :key="file.name" class="preview-card">
                    <div>
                        <p>
                            {{ file.name }}
                        </p>
                    </div>
                    <div>
                        <button
                            class="preview-close-button"
                            type="button"
                            @click.stop="remove(files.indexOf(file))"
                            title="Remove file"
                        >
                            <b>x</b>
                        </button>
                    </div>
                </div>
            </div>
            -->
        </div>
    </div>
</template>

<style scoped>
    .main {
        display: flex;
        flex-grow: 0;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    .dropzone-container {
        padding: 2.5rem;
        background: #333435;
        border: 2px dashed #9e9e9e;
        border-radius: 10px;
        position: relative;
        cursor: pointer;
        width: 55vh;
    }

    .dropzone-container:hover {
        background: #242525;
    }

    .hidden-input {
        opacity: 0;
        overflow: hidden;
        position: absolute;
        width: 1px;
        height: 1px;
        pointer-events: none;
    }

    .dropzone-content {
        font-size: 16px;
        display: block;
        pointer-events: none;
    }

    .preview-container {
        display: flex;
        margin-top: 2rem;
    }

    .preview-card {
        display: flex;
        border: 1px solid #a2a2a2;
        padding: 5px;
        margin-left: 5px;
    }

    .preview-close-button {
        margin-left: 2px;
        display: block;
        pointer-events: all;
    }
</style>
