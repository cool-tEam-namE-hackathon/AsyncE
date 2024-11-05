<template>
    <div class="container h-full flex flex-col">
        <!-- INPUT GROUP TITLE -->
        <base-dialog
            :open="isConfirmationModalOpen"
            :is-closable="false"
            @on-close-dialog="toggleConfirmationModal"
            class="rounded-lg shadow-xl max-w-md w-full"
        >
            <template #title>
                <h2 class="text-xl font-medium mb-2">Upload Video</h2>
            </template>

            <template #content>
                <div class="space-y-6">
                    <div>
                        <Label
                            for="videoTitle"
                            class="block text-sm font-medium mb-1"
                        >
                            Video Title
                        </Label>
                        <Input
                            id="videoTitle"
                            v-model="videoTitle"
                            class="w-full px-3 py-2 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter video title"
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <Label class="text-sm font-medium">
                            Generate Subtitles
                        </Label>
                        <Switch v-model:checked="generateSubtitle" />
                    </div>
                    <div
                        v-show="generateSubtitle"
                        class="text-sm text-gray-500"
                    >
                        Generating subtitles will take some time depending on
                        the video size.
                    </div>
                </div>
            </template>

            <template #footer>
                <div class="flex justify-end space-x-3">
                    <Button
                        variant="secondary"
                        @click="toggleConfirmationModal"
                    >
                        Cancel
                    </Button>
                    <Button
                        :disabled="isUploading || !videoTitle"
                        :is-loading="isUploading"
                        @click="saveRecording"
                    >
                        <template #default> Upload Video </template>
                        <template #loading>
                            <Icon
                                icon="prime:spinner"
                                width="16"
                                height="16"
                                class="text-white animate-spin mr-1"
                            />
                            Uploading... {{ uploadVideoProgress.toFixed(0) }}%
                        </template>
                    </Button>
                </div>
            </template>
        </base-dialog>

        <!-- GROUP NAME -->
        <span> {{ meetingDetail?.title }}</span>

        <canvas ref="canvasRef" class="hidden" width="1280" height="720" />

        <!-- MEDIA -->
        <div class="flex flex-col py-8 gap-6">
            <!-- VIDEO -->
            <div class="flex flex-col bg-white rounded-lg shadow-md p-4">
                <div class="flex items-center justify-between">
                    <h2 class="text-xl font-semibold">Record New Video</h2>
                    <video-controls
                        v-model:selectedCamera="selectedCamera"
                        :camera-list="cameraList"
                        :enabled-camera="enabledCamera"
                        :enabled-screen="enabledScreen"
                        :is-recording-disabled="isRecordingDisabled"
                        :recording-phase-text="recordingPhaseText"
                        @on-toggle-camera="enabledCamera = !enabledCamera"
                        @on-toggle-screen="enabledScreen = !enabledScreen"
                        @on-record="handleRecord"
                    />
                </div>
            </div>

            <!-- NO CAMERA OR SCREEN YET -->
            <div
                v-if="!displayCamera && !displayStream"
                class="flex flex-col items-center justify-center min-h-[512px] bg-gray-200 rounded-lg"
            >
                <Icon
                    icon="fluent:video-off-32-regular"
                    width="64"
                    height="64"
                    style="color: black"
                />
                <p class="text-xl font-semibold">Video stream unavailable</p>
                <p class="text-sm mt-2">
                    Please start your camera or screen share
                </p>
            </div>

            <div
                v-else
                ref="containerEl"
                class="bg-gray-200 rounded-lg mb-4 w-full relative overflow-hidden"
            >
                <!-- SCREEN -->
                <div class="min-w-[512px]">
                    <video
                        ref="screenRef"
                        class="rounded-lg w-full h-full object-cover"
                        autoplay
                        muted
                    />
                </div>

                <!-- CAMERA -->
                <video
                    ref="cameraRef"
                    :class="[
                        displayStream
                            ? `select-none top-0 left-0`
                            : 'inset-0 w-full h-full object-cover',
                        'absolute',
                    ]"
                    :width="displayStream ? cameraDimensions.width : 'auto'"
                    :height="displayStream ? cameraDimensions.height : 'auto'"
                    muted
                    autoplay
                />
                <!-- <div class="flex-1">
                    <video-list ref="videoList" />
                </div> -->
            </div>
        </div>
    </div>

    <video v-if="url" autoplay muted controls>
        <source :src="url" type="video/mp4" />
        Your browser does not support the video tag.
    </video>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, onMounted } from "vue";

import { FFmpeg } from "@ffmpeg/ffmpeg";

import { useRoute } from "vue-router";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";

import {
    useUserMedia,
    useDevicesList,
    useDisplayMedia,
    useElementSize,
} from "@vueuse/core";

import { Icon } from "@iconify/vue";

import VideoList from "@components/video/VideoList.vue";
import VideoControls from "@components/video/VideoControls.vue";

import { Button } from "@components/ui/button";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import Switch from "@components/ui/switch/Switch.vue";

import BaseDialog from "@components/shared/BaseDialog.vue";

import { fetchFile, toBlobURL } from "@ffmpeg/util";

const route = useRoute();
const groupStore = useGroupStore();

const ffmpeg = ref<FFmpeg>();

const videoList = ref<InstanceType<typeof VideoList> | null>(null);

const selectedCamera = ref<string>();
const videoTitle = ref<string>("");
const url = ref<string>("");

const isRecording = ref<boolean>(false);
const isUploading = ref<boolean>(false);
const isConfirmationModalOpen = ref<boolean>(false);
const generateSubtitle = ref<boolean>(false);

const recordedChunks = ref<Blob[]>([]);

const animationFrameId = ref<number | null>(null);

const ctx = ref<CanvasRenderingContext2D | null>(null);

const canvasRef = ref<HTMLCanvasElement | null>(null);

const containerEl = ref<HTMLElement | null>(null);

const screenRef = ref<HTMLVideoElement | null>(null);
const cameraRef = ref<HTMLVideoElement | null>(null);

const mediaRecorder = ref<MediaRecorder | null>(null);

const { uploadVideoProgress, meetingDetail } = storeToRefs(groupStore);

const { stream: displayStream, enabled: enabledScreen } = useDisplayMedia();

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: true,
});

const { width: screenWidth, height: screenHeight } =
    useElementSize(containerEl);

const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

const { stream: displayCamera, enabled: enabledCamera } = useUserMedia({
    constraints: {
        video: { deviceId: selectedCamera },
        audio: { deviceId: currentMicrophone },
    },
});

const isRecordingDisabled = computed(() => {
    return !enabledCamera.value;
});

const cameraList = computed(() => {
    return cameras.value.map(({ label, deviceId }) => ({
        deviceId,
        name: label,
    }));
});

const recordingPhaseText = computed(() => {
    if (isRecording.value) {
        return "Stop";
    }
    return recordedChunks.value.length > 0 ? "Save" : "Record";
});

const cameraDimensions = computed(() => ({
    width: displayStream.value
        ? screenWidth.value * 0.25
        : canvasRef.value?.width ?? 0,
    height: displayStream.value
        ? screenHeight.value * 0.25
        : canvasRef.value?.height ?? 0,
}));

function toggleConfirmationModal() {
    isConfirmationModalOpen.value = !isConfirmationModalOpen.value;
}

function startRecording() {
    if (!canvasRef.value) return;

    startDrawing();

    const canvasStream = canvasRef.value.captureStream(60);

    const combinedStream = new MediaStream([
        ...canvasStream.getVideoTracks(),
        ...(displayCamera.value ? displayCamera.value.getAudioTracks() : []),
        ...(displayStream.value ? displayStream.value.getAudioTracks() : []),
    ]);

    mediaRecorder.value = new MediaRecorder(combinedStream, {
        mimeType: "video/webm; codecs=vp9",
    });

    mediaRecorder.value.ondataavailable = (e) => {
        if (e.data.size > 0) {
            recordedChunks.value.push(e.data);
        }
    };

    mediaRecorder.value.start();
    mediaRecorder.value.requestData();
    isRecording.value = true;
}

function stopRecording() {
    if (!mediaRecorder.value) return;

    mediaRecorder.value.requestData();
    mediaRecorder.value.stop();
    isRecording.value = false;
}

async function saveRecording() {
    isUploading.value = true;

    const blob = new Blob(recordedChunks.value, {
        type: "video/webm; codecs=vp9",
    });

    const data = new Uint8Array(await blob.arrayBuffer());

    url.value = URL.createObjectURL(blob);

    // try {
    //     await groupStore.uploadVideo(
    //         data,
    //         route.params.groupId as string,
    //         route.params.meetingId as string,
    //         videoTitle.value,
    //         generateSubtitle.value,
    //     );
    // } catch (e) {
    //     console.log((e as Error).message);
    // } finally {
    //     isUploading.value = false;
    //     toggleConfirmationModal();
    // }

    recordedChunks.value = [];
}

async function loadFFmpeg(ffmpeg: FFmpeg) {
    await ffmpeg.load({
        coreURL: await toBlobURL(
            "https://unpkg.com/@ffmpeg/core@0.12.3/dist/esm/ffmpeg-core.js",
            "text/javascript",
        ),
        wasmURL: await toBlobURL(
            "https://unpkg.com/@ffmpeg/core@0.12.3/dist/esm/ffmpeg-core.wasm",
            "application/wasm",
        ),
        workerURL: await toBlobURL(
            "https://unpkg.com/@ffmpeg/ffmpeg@0.12.3/dist/esm/worker.js",
            "text/javascript",
        ),
    });
}

async function getFFmpegInstance() {
    if (!ffmpeg.value) {
        ffmpeg.value = new FFmpeg();
        await loadFFmpeg(ffmpeg.value);
    }
    return ffmpeg.value;
}

async function convertToWebm(blob: Blob) {
    const ffmpegInstance = await getFFmpegInstance();

    await ffmpegInstance.writeFile("input.webm", await fetchFile(blob));
    await ffmpegInstance.exec([
        "-i",
        "input.webm",
        "-c",
        "copy",
        "output.webm",
    ]);

    const data = await ffmpegInstance.readFile("output.webm");

    const mp4Blob = new Blob([(data as Uint8Array).buffer], {
        type: "video/webm; codecs=vp9",
    });

    return mp4Blob;
}

function handleRecord() {
    if (recordingPhaseText.value === "Record") {
        startRecording();
    } else if (recordingPhaseText.value === "Stop") {
        stopRecording();
    } else {
        toggleConfirmationModal();
    }
}
async function fetchMeetingDetail() {
    const { groupId, meetingId } = route.params;

    try {
        await groupStore.getMeetingDetail(
            groupId as string,
            meetingId as string,
        );
    } catch (e) {
        console.log((e as Error).message);
    }
}

const requestVideoFrame = function (callback) {
    return window.setTimeout(function () {
        callback(Date.now());
    }, 1000 / 60); // 60 fps - just like requestAnimationFrame
};

function startDrawing() {
    if (!ctx.value || !canvasRef.value || isRecording.value) return;

    const scaledWidth = screenWidth.value * window.devicePixelRatio;
    const scaledHeight = screenHeight.value * window.devicePixelRatio;

    canvasRef.value.width = scaledWidth;
    canvasRef.value.height = scaledHeight;

    const originalWidth = screenWidth.value;
    const originalHeight = screenHeight.value;

    ctx.value.scale(window.devicePixelRatio, window.devicePixelRatio);

    const draw = () => {
        if (!ctx.value || !canvasRef.value) return;

        ctx.value.save();

        ctx.value.clearRect(0, 0, screenWidth.value, screenHeight.value);

        if (enabledScreen.value && displayStream.value) {
            const screenVideo = screenRef.value;
            if (screenVideo) {
                ctx.value.drawImage(
                    screenVideo,
                    0,
                    0,
                    originalWidth,
                    originalHeight,
                );
            }
        }

        if (enabledCamera.value && displayCamera.value) {
            const cameraVideo = cameraRef.value;
            if (cameraVideo) {
                const cameraWidth = Math.floor(screenWidth.value / 4);
                const cameraHeight = Math.floor(screenHeight.value / 4);
                ctx.value.drawImage(
                    cameraVideo,
                    0,
                    0,
                    enabledScreen.value ? cameraWidth : originalWidth,
                    enabledScreen.value ? cameraHeight : originalHeight,
                );
            }
        }
        ctx.value.restore();

        animationFrameId.value = requestVideoFrame(draw);
    };

    draw();
}

watchEffect(() => {
    if (cameraRef.value) {
        cameraRef.value.srcObject = displayCamera.value!;
    }

    if (screenRef.value) {
        screenRef.value.srcObject = displayStream.value!;
    }
});

onMounted(() => {
    if (canvasRef.value) {
        ctx.value = canvasRef.value.getContext("2d");
    }
});

async function init() {
    await fetchMeetingDetail();
}

await init();
</script>
