<template>
    <div class="flex flex-col gap-4 text-center">
        <div>
            <button @click="enabled = !enabled">
                {{ enabled ? "Stop" : "Start" }} sharing my screen
            </button>
        </div>
        <div v-if="enabled">
            <button @click="startRecording" :disabled="isRecording">
                Start Recording
            </button>
            <button @click="stopRecording" :disabled="!isRecording">
                Stop Recording
            </button>
            <button
                @click="saveRecording"
                :disabled="isRecording || recordedChunks.length === 0"
            >
                Save Recording
            </button>
        </div>
        <div>
            <video ref="video" muted autoplay controls class="h-100 w-auto" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { useDisplayMedia } from "@vueuse/core";
import { ref, watchEffect } from "vue";

const video = ref<HTMLVideoElement>();
const { stream, enabled } = useDisplayMedia({
    audio: true,
});

const mediaRecorder = ref<MediaRecorder | null>(null);
const recordedChunks = ref<Blob[]>([]);
const isRecording = ref(false);

watchEffect(() => {
    if (video.value) video.value.srcObject = stream.value!;
});

const startRecording = () => {
    if (!stream.value) return;

    mediaRecorder.value = new MediaRecorder(stream.value);

    mediaRecorder.value.ondataavailable = (event) => {
        if (event.data.size > 0) {
            recordedChunks.value.push(event.data);
        }
    };

    mediaRecorder.value.start();
    isRecording.value = true;
};

const stopRecording = () => {
    if (!mediaRecorder.value) return;

    mediaRecorder.value.stop();
    isRecording.value = false;
};

const saveRecording = () => {
    const blob = new Blob(recordedChunks.value, { type: "video/webm" });

    // const url = URL.createObjectURL(blob);
    // const a = document.createElement("a");
    // document.body.appendChild(a);
    // a.style.display = "none";
    // a.href = url;
    // a.download = "screen-recording.webm";

    console.log(a);
};
</script>
