<template>
    <div class="container mb-2 flex flex-col">
        <!-- INPUT GROUP TITLE -->
        <base-dialog
            :open="isConfirmationModalOpen"
            :is-closable="false"
            @on-close-dialog="toggleConfirmationModal"
            class="w-full max-w-md rounded-lg shadow-xl"
        >
            <template #title>
                <h2 class="mb-2 text-xl font-medium">Upload Video</h2>
            </template>

            <template #content>
                <div class="space-y-6">
                    <div>
                        <Label
                            for="videoTitle"
                            class="mb-1 block text-sm font-medium"
                        >
                            Video Title
                        </Label>
                        <Input
                            id="videoTitle"
                            v-model="videoTitle"
                            class="w-full rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter video title"
                        />
                    </div>
                    <template v-if="userCredentials?.subscription.length">
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
                            Generating subtitles will take some time depending
                            on the video size.
                        </div>
                    </template>
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
                                class="mr-1 animate-spin text-white"
                            />
                            Uploading... {{ uploadVideoProgress.toFixed(0) }}%
                        </template>
                    </Button>
                </div>
            </template>
        </base-dialog>

        <!-- GROUP NAME -->
        <span> {{ meetingDetail?.title }}</span>

        <canvas ref="canvasRef" class="hidden" />

        <!-- MEDIA -->
        <div class="flex flex-col gap-6 py-8">
            <!-- VIDEO -->
            <div class="flex flex-col rounded-lg bg-white p-4 shadow-md">
                <div class="flex items-center justify-between">
                    <h2 class="text-xl font-semibold">Record New Video</h2>
                    <video-controls
                        v-model="selectedCamera"
                        :camera-list="cameraList"
                        :enabled-camera="enabledCamera"
                        :enabled-screen="enabledScreen"
                        :is-control-disabled="isControlDisabled"
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
                class="flex min-h-[512px] flex-col items-center justify-center rounded-lg bg-gray-200"
            >
                <Icon
                    icon="fluent:video-off-32-regular"
                    width="64"
                    height="64"
                    style="color: black"
                />
                <p class="text-xl font-semibold">Video stream unavailable</p>
                <p class="mt-2 text-sm">
                    Please start your camera or screen share
                </p>
            </div>

            <div
                v-else
                ref="containerEl"
                class="relative mb-4 w-full overflow-hidden rounded-lg bg-gray-200"
            >
                <!-- SCREEN -->
                <div class="min-w-[512px]">
                    <video
                        ref="screenRef"
                        class="h-full w-full rounded-lg object-cover"
                        autoplay
                        muted
                    />
                </div>

                <!-- CAMERA -->
                <video
                    ref="cameraRef"
                    :class="[
                        displayStream
                            ? `left-0 top-0 select-none`
                            : 'inset-0 h-full w-full object-cover',
                        'absolute',
                    ]"
                    :width="displayStream ? cameraDimensions.width : 'auto'"
                    :height="displayStream ? cameraDimensions.height : 'auto'"
                    muted
                    autoplay
                />
            </div>
        </div>
    </div>

    <video-list ref="videoList" />

    <video v-if="url" autoplay muted controls>
        <source :src="url" type="video/webm" />
        Your browser does not support the video tag.
    </video>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, onMounted } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import {
    useUserMedia,
    useDevicesList,
    useDisplayMedia,
    useElementSize,
} from "@vueuse/core";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import BaseDialog from "@components/shared/BaseDialog.vue";
import { Button } from "@components/ui/button";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import Switch from "@components/ui/switch/Switch.vue";
import VideoControls from "@components/video/VideoControls.vue";
import VideoList from "@components/video/VideoList.vue";

const route = useRoute();
const groupStore = useGroupStore();

const videoList = ref<InstanceType<typeof VideoList> | null>(null);

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

const { userCredentials } = storeToRefs(useUserStore());
const { uploadVideoProgress, meetingDetail } = storeToRefs(groupStore);

const { stream: displayStream, enabled: enabledScreen } = useDisplayMedia();

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: true,
});

const { width: screenWidth, height: screenHeight } =
    useElementSize(containerEl);

const selectedCamera = computed(() => cameras.value[0].deviceId);
const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

const { stream: displayCamera, enabled: enabledCamera } = useUserMedia({
    constraints: {
        // @ts-expect-error passing .value will cause problems when selecting a different camera
        video: { deviceId: selectedCamera },
        // @ts-expect-error passing .value will cause problems when selecting a different camera
        audio: { deviceId: currentMicrophone },
    },
});

const isControlDisabled = computed(() => recordingPhaseText.value === "Stop");

const isRecordingDisabled = computed(
    () => !displayCamera.value && !displayStream.value,
);

const cameraList = computed(() =>
    cameras.value
        .filter(({ deviceId }) => deviceId !== "")
        .map(({ label, deviceId }) => ({
            deviceId,
            name: label,
        })),
);

const recordingPhaseText = computed(() => {
    if (isRecording.value) {
        return "Stop";
    }
    return recordedChunks.value.length > 0 ? "Save" : "Record";
});

const cameraDimensions = computed(() => ({
    width: displayStream.value
        ? screenWidth.value * 0.25
        : (canvasRef.value?.width ?? 0),
    height: displayStream.value
        ? screenHeight.value * 0.25
        : (canvasRef.value?.height ?? 0),
}));

function toggleConfirmationModal() {
    isConfirmationModalOpen.value = !isConfirmationModalOpen.value;
}

const videoMimeType = MediaRecorder.isTypeSupported(
    "video/webm; codecs=vp8,opus",
)
    ? "video/webm; codecs=vp8,opus"
    : "video/webm";

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
        mimeType: videoMimeType,
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
        type: videoMimeType,
    });

    const data = new Uint8Array(await blob.arrayBuffer());

    url.value = URL.createObjectURL(blob);

    try {
        await groupStore.uploadVideo(
            data,
            route.params.groupId as string,
            route.params.meetingId as string,
            videoTitle.value,
            generateSubtitle.value,
        );
        videoList.value?.getAllThumbnails();
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isUploading.value = false;
        toggleConfirmationModal();
    }

    recordedChunks.value = [];
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
    const originalWidth = screenWidth.value;
    const originalHeight = screenHeight.value;

    canvasRef.value.width = scaledWidth;
    canvasRef.value.height = scaledHeight;
    ctx.value.scale(window.devicePixelRatio, window.devicePixelRatio);

    const draw = () => {
        if (!ctx.value || !canvasRef.value) return;

        ctx.value.clearRect(0, 0, originalWidth, originalHeight);

        if (enabledScreen.value && displayStream.value && screenRef.value) {
            ctx.value.drawImage(
                screenRef.value,
                0,
                0,
                originalWidth,
                originalHeight,
            );
        }

        // Draw camera if enabled
        if (enabledCamera.value && displayCamera.value && cameraRef.value) {
            const camera = cameraRef.value;
            const videoAspectRatio = camera.videoWidth / camera.videoHeight;

            if (enabledScreen.value) {
                const maxCornerWidth = Math.floor(originalWidth / 4);
                const maxCornerHeight = Math.floor(originalHeight / 4);
                const cornerAspectRatio = maxCornerWidth / maxCornerHeight;

                let cornerWidth, cornerHeight;

                if (videoAspectRatio > cornerAspectRatio) {
                    cornerWidth = maxCornerWidth;
                    cornerHeight = cornerWidth / videoAspectRatio;
                } else {
                    cornerHeight = maxCornerHeight;
                    cornerWidth = cornerHeight * videoAspectRatio;
                }

                ctx.value.drawImage(camera, 0, 0, cornerWidth, cornerHeight);
            } else {
                // Camera full screen if screen is off
                const scale = Math.max(
                    originalWidth / camera.videoWidth,
                    originalHeight / camera.videoHeight,
                );
                const newWidth = camera.videoWidth * scale;
                const newHeight = camera.videoHeight * scale;
                const x = (originalWidth - newWidth) / 2;
                const y = (originalHeight - newHeight) / 2;

                ctx.value.drawImage(camera, x, y, newWidth, newHeight);
            }
        }

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
