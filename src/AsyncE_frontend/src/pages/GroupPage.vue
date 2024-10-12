<template>
    <div class="container relative h-full">
        <span> {{ currentGroup?.name }}</span>
        <div class="mx-auto py-8">
            <div class="flex flex-col lg:flex-row gap-8">
                <!-- VIDEO -->
                <div class="w-full">
                    <div class="bg-white rounded-lg shadow-md p-6">
                        <div class="flex items-center justify-between mb-3">
                            <h2 class="text-xl font-semibold">
                                Record New Video
                            </h2>
                            <div class="flex gap-3">
                                <base-select
                                    v-model="selectedCamera"
                                    placeholder="Select camera"
                                    :options="cameraList"
                                />

                                <Button
                                    :variant="
                                        enabledScreen ? 'default' : 'outline'
                                    "
                                    :disabled="isSharingDisabled"
                                    @click="enabledScreen = !enabledScreen"
                                >
                                    <Icon
                                        class="mr-2"
                                        icon="gg:screen"
                                        width="24"
                                        height="24"
                                    />
                                    Screen
                                </Button>
                                <Button
                                    :variant="
                                        enabledCamera ? 'default' : 'outline'
                                    "
                                    :disabled="isSharingDisabled"
                                    @click="enabledCamera = !enabledCamera"
                                >
                                    <Icon
                                        class="mr-2"
                                        icon="flowbite:video-camera-outline"
                                        width="24"
                                        height="24"
                                    />
                                    Camera
                                </Button>
                                <Button
                                    :disabled="
                                        !(enabledCamera && enabledScreen)
                                    "
                                    @click="handleRecord"
                                >
                                    <Icon
                                        :class="[
                                            recordingPhaseText === 'Stop'
                                                ? 'text-red-500 animate-pulse'
                                                : '',
                                            'mr-2',
                                        ]"
                                        icon="fluent:record-20-regular"
                                        width="24"
                                        height="24"
                                    />
                                    {{ recordingPhaseText }}
                                </Button>
                            </div>
                        </div>
                        <div
                            ref="containerEl"
                            class="aspect-w-16 aspect-h-9 bg-gray-200 rounded-lg mb-4"
                        >
                            <video
                                ref="videoRef"
                                class="rounded-lg object-cover w-full max-w-full max-h-[1000px]"
                                controls
                                autoplay
                                muted
                            />
                            <div
                                ref="videoEl"
                                :style="{
                                    userSelect: 'none',
                                    position: 'fixed',
                                    top: `${restrictedY}px`,
                                    left: `${restrictedX}px`,
                                }"
                            >
                                <video
                                    ref="cameraRef"
                                    muted
                                    autoplay
                                    :width="cameraDimensions.width"
                                    :height="cameraDimensions.height"
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed } from "vue";

import { useRoute } from "vue-router";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";

import { Icon } from "@iconify/vue";

import {
    useDisplayMedia,
    useUserMedia,
    useDevicesList,
    useDraggable,
    useElementBounding,
    useElementSize,
    clamp,
} from "@vueuse/core";

import BaseSelect from "@components/shared/BaseSelect.vue";

import { Button } from "@components/ui/button";

import { MediaRecorders, RecordedChunks, Video } from "@/types/api/model";

const route = useRoute();
const groupStore = useGroupStore();

const selectedCamera = ref<string>();

const isScreenRecording = ref<boolean>(false);
const isCameraRecording = ref<boolean>(false);

const videoRef = ref<HTMLVideoElement>();
const cameraRef = ref<HTMLVideoElement>();

const containerEl = ref<HTMLElement | null>(null);
const videoEl = ref<HTMLElement | null>(null);

const recordedChunks = ref<RecordedChunks>({
    screen: [],
    camera: [],
});
const mediaRecorders = ref<MediaRecorders>({
    screen: null,
    camera: null,
});

const recordedVideo = ref<Video>({
    screen: null,
    camera: null,
});

const { currentGroup } = storeToRefs(groupStore);

const { enabled: enabledScreen, stream: displayStream } = useDisplayMedia();

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: true,
});

const { stream: displayCamera, enabled: enabledCamera } = useUserMedia({
    constraints: {
        video: { deviceId: selectedCamera },
    },
});

const { left, right, top, bottom } = useElementBounding(containerEl);
const { width, height } = useElementBounding(videoEl);
const { width: screenWidth, height: screenHeight } =
    useElementSize(containerEl);
const { x, y } = useDraggable(videoEl);

const restrictedX = computed(() =>
    clamp(left.value, x.value, right.value - width.value),
);
const restrictedY = computed(() =>
    clamp(top.value, y.value, bottom.value - height.value),
);

const isSharingDisabled = computed(() => {
    return isCameraRecording.value || isScreenRecording.value;
});

const cameraList = computed(() => {
    return cameras.value.map(({ label, deviceId }) => ({
        deviceId,
        name: label,
    }));
});

const recordingPhaseText = computed(() => {
    if (isScreenRecording.value || isCameraRecording.value) {
        return "Stop";
    }
    return recordedChunks.value.camera.length > 0 ||
        recordedChunks.value.screen.length > 0
        ? "Save"
        : "Record";
});

const cameraDimensions = computed(() => {
    return {
        width: screenWidth.value * 0.25,
        height: screenHeight.value * 0.25,
    };
});

function startRecording(type: keyof MediaRecorders) {
    const mediaStream =
        type === "screen" ? displayStream.value : displayCamera.value;

    if (!mediaStream) return;

    mediaRecorders.value[type] = new MediaRecorder(mediaStream);

    mediaRecorders.value[type].ondataavailable = (e) => {
        if (e.data.size > 0) {
            recordedChunks.value[type].push(e.data);
        }
    };

    mediaRecorders.value[type].start();

    if (type === "screen") {
        isScreenRecording.value = true;
    } else {
        isCameraRecording.value = true;
    }
}

function stopRecording(type: keyof MediaRecorders) {
    if (!mediaRecorders.value[type]) return;

    mediaRecorders.value[type].stop();

    if (type === "screen") {
        isScreenRecording.value = false;
    } else {
        isCameraRecording.value = false;
    }
}

async function saveRecording(type: keyof RecordedChunks) {
    const blob = new Blob(recordedChunks.value[type], { type: "video/webm" });
    recordedVideo.value[type] = new Uint8Array(await blob.arrayBuffer());

    recordedChunks.value[type] = [];
}

async function handleRecord() {
    if (recordingPhaseText.value === "Record") {
        startRecording("screen");
        startRecording("camera");
    } else if (recordingPhaseText.value === "Stop") {
        stopRecording("screen");
        stopRecording("camera");
    } else {
        await Promise.all([saveRecording("screen"), saveRecording("camera")]);
        uploadRecording();
    }
}
async function fetchGroupDetails() {
    const { id } = route.params;
    const groupId = BigInt(id as string);

    await groupStore.getGroup(groupId);
}

async function uploadRecording() {
    const { id } = route.params;
    const groupId = BigInt(id as string);

    if (!recordedVideo.value.screen || !recordedVideo.value.camera) {
        return;
    }

    const payload = {
        id: groupId,
        screen: recordedVideo.value.screen,
        camera: recordedVideo.value.camera,
    };

    const response = await groupStore.addVideo(payload);

    console.log(response);
}

watchEffect(() => {
    if (videoRef.value) {
        videoRef.value.srcObject = displayStream.value!;
    }

    if (cameraRef.value) {
        cameraRef.value.srcObject = displayCamera.value!;
    }
});

async function init() {
    await fetchGroupDetails();
}

await init();
</script>
