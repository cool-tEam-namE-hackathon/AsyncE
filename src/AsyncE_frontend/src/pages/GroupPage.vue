<template>
    <div class="container relative h-full">
        <!-- UPLOAD PROGRESS MODAL -->
        <base-dialog :open="isOpen" :hide-close-button="true">
            <template #title>
                <span class="text-lg font-semibold mb-4">
                    Uploading Recordings
                </span>
            </template>
            <template #content>
                <div class="flex justify-between">
                    <span>Screen Recording</span>
                    <span>{{ screenProgress.toFixed(1) }}%</span>
                </div>
                <base-progress v-model="screenProgress" />

                <div class="flex justify-between">
                    <span>Camera Recording</span>
                    <span>{{ cameraProgress.toFixed(1) }}%</span>
                </div>
                <base-progress v-model="cameraProgress" />
            </template>
        </base-dialog>

        <!-- INVITE USER DIALOG -->
        <base-dialog
            :open="isInviteUserDialogOpen"
            @on-close-dialog="toggleInviteModal"
        >
            <template #title> Invite user </template>

            <template #description>
                Enter the username of the user you'd like to invite.
            </template>

            <template #content>
                <div class="space-y-2">
                    <div class="flex items-center gap-3">
                        <Label>Username</Label>
                        <Input
                            v-model="invitedUsername"
                            :class="{
                                'border-red-400':
                                    isFieldError && invitedUsername,
                                'border-green-700':
                                    !isFieldError && invitedUsername,
                                'focus-visible:ring-0': true,
                            }"
                            @update:model-value="validateUsername"
                        />
                        <Icon
                            icon="ep:success-filled"
                            width="32"
                            height="32"
                            :class="{
                                'text-red-700': isFieldError && invitedUsername,
                                'text-green-700':
                                    !isFieldError && invitedUsername,
                                hidden: !invitedUsername,
                            }"
                        />
                    </div>
                    <div v-if="inviteError" class="text-sm text-red-500">
                        {{ inviteError }}
                    </div>
                </div>
            </template>

            <template #footer>
                <Button
                    :disabled="isFieldError || isLoading"
                    :is-loading="isLoading"
                    @click="handleInvite"
                >
                    <template #default> Invite </template>

                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="text-black animate-spin mr-1"
                        />
                        Inviting...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- GROUP NAME -->
        <span> {{ currentGroup?.name }}</span>

        <!-- MEDIA -->
        <div class="mx-auto py-8">
            <div class="flex flex-col lg:flex-row gap-8">
                <div class="w-full flex gap-8">
                    <!-- VIDEO -->
                    <div class="flex-1 bg-white rounded-lg shadow-sm p-6">
                        <div class="flex items-center justify-between mb-3">
                            <h2 class="text-xl font-semibold">
                                Record New Video
                            </h2>
                            <video-controls
                                v-model:selectedCamera="selectedCamera"
                                :camera-list="cameraList"
                                :enabled-camera="enabledCamera"
                                :enabled-screen="enabledScreen"
                                :is-recording-disabled="isRecordingDisabled"
                                :recording-phase-text="recordingPhaseText"
                                @on-toggle-camera="onToggleCamera"
                                @on-toggle-screen="onToggleScreen"
                                @on-record="handleRecord"
                            />
                        </div>

                        <!-- NO CAMERA OR SCREEN YET -->
                        <div
                            v-if="!displayCamera && !displayStream"
                            class="flex flex-col items-center justify-center bg-gray-200 rounded-lg h-96"
                        >
                            <Icon
                                icon="fluent:video-off-32-regular"
                                width="64"
                                height="64"
                                style="color: black"
                            />
                            <p className="text-xl font-semibold">
                                Video stream unavailable
                            </p>
                            <p className="text-sm mt-2">
                                Please start your camera or screen share
                            </p>
                        </div>

                        <div
                            v-else
                            ref="containerEl"
                            class="bg-gray-200 rounded-lg mb-4 w-full relative overflow-hidden"
                        >
                            <!-- SCREEN -->
                            <video
                                ref="videoRef"
                                class="rounded-lg w-full h-full object-cover"
                                autoplay
                                muted
                            />

                            <!-- CAMERA -->
                            <div ref="videoEl">
                                <video
                                    ref="cameraRef"
                                    :class="[
                                        displayStream
                                            ? `select-none top-0 left-0`
                                            : 'inset-0 w-full h-full object-cover',
                                        'absolute',
                                    ]"
                                    :width="
                                        displayStream
                                            ? cameraDimensions.width
                                            : 'auto'
                                    "
                                    :height="
                                        displayStream
                                            ? cameraDimensions.height
                                            : 'auto'
                                    "
                                    muted
                                    autoplay
                                />
                            </div>
                        </div>
                    </div>

                    <!-- USER MANAGEMENT -->
                    <div
                        class="p-4 rounded-lg border-solid border-[1px] border-slate-200 w-64"
                    >
                        <div class="flex items-center gap-2">
                            <Icon
                                icon="lucide:users"
                                width="24"
                                height="24"
                                style="color: black"
                            />
                            <h1>User Management</h1>
                        </div>
                        <span class="text-sm text-gray-400"
                            >Manage and invite users to your video
                            sessions</span
                        >
                        <Button class="mt-2" @click="toggleInviteModal">
                            <Icon
                                icon="lucide:user-plus"
                                width="24"
                                height="24"
                                class="text-white mr-2"
                            />
                            Invite new users
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <!-- CHAT -->
    <chat-window />

    <!-- <video v-if="url" autoplay muted controls>
        <source :src="url" type="video/mp4" />
        Your browser does not support the video tag.
    </video> -->
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, onMounted } from "vue";

import { useRoute } from "vue-router";
import { useUserStore } from "@stores/user-store";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";

import {
    useDisplayMedia,
    useUserMedia,
    useDevicesList,
    useElementSize,
    useDebounceFn,
} from "@vueuse/core";

import { Icon } from "@iconify/vue";

import VideoControls from "@components/video/VideoControls.vue";
import ChatWindow from "@components/chat/ChatWindow.vue";

import { Button } from "@components/ui/button";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";

import BaseDialog from "@components/shared/BaseDialog.vue";
import BaseProgress from "@components/shared/BaseProgress.vue";

import { RecordedChunks } from "@/types/api/model";
import { generateUUID, createChunks } from "@/utils/helpers";

const route = useRoute();
const groupStore = useGroupStore();
const userStore = useUserStore();

const selectedCamera = ref<string>();
const uploadedChunk = ref<{
    screen: number;
    camera: number;
}>({
    screen: 0,
    camera: 0,
});
const totalUploadSize = ref<{
    screen: number;
    camera: number;
}>({
    screen: 0,
    camera: 0,
});

const animationFrameId = ref<number | null>(null);

const url = ref<string>("");
const inviteError = ref<string>("");

const isRecording = ref<boolean>(false);
const isOpen = ref<boolean>(false);
const isError = ref<boolean>(false);
const isLoading = ref<boolean>(false);

const recordedChunks = ref<Blob[]>([]);

const ctx = ref<CanvasRenderingContext2D | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const containerEl = ref<HTMLElement | null>(null);
const videoEl = ref<HTMLElement | null>(null);
const videoRef = ref<HTMLVideoElement | null>(null);
const cameraRef = ref<HTMLVideoElement | null>(null);

const mediaRecorder = ref<MediaRecorder | null>(null);
const recordedVideo = ref<Uint8Array | null>(null);

// const audioContext = ref<AudioContext>(new AudioContext());
// const mediaStreamAudioDestinationNode = ref<MediaStreamAudioDestinationNode | null>(null);

const { currentGroup } = storeToRefs(groupStore);

const { enabled: enabledScreen, stream: displayStream } = useDisplayMedia({
    audio: true,
});

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: true,
});

const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

const { stream: displayCamera, enabled: enabledCamera } = useUserMedia({
    constraints: {
        video: { deviceId: selectedCamera },
        audio: { deviceId: currentMicrophone },
    },
});

const { width: screenWidth, height: screenHeight } =
    useElementSize(containerEl);

const isRecordingDisabled = computed(() => {
    return !enabledCamera.value && !enabledScreen.value;
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

const cameraProgress = computed(() => {
    return (
        (uploadedChunk.value.camera / totalUploadSize.value.camera) * 100 || 0
    );
});

const screenProgress = computed(() => {
    return (
        (uploadedChunk.value.screen / totalUploadSize.value.screen) * 100 || 0
    );
});

const isInviteUserDialogOpen = ref<boolean>(false);

const invitedUsername = ref<string>("");
const inputtedUsername = ref<string>("");

const isFieldError = computed(() => {
    return !isError.value && inputtedUsername.value;
});

function toggleInviteModal() {
    isInviteUserDialogOpen.value = !isInviteUserDialogOpen.value;
}

const validateUsername = useDebounceFn(async (payload) => {
    try {
        isError.value = await userStore.validateUsername(payload);
        inputtedUsername.value = payload;
    } catch (e) {
        console.log((e as Error).message);
    }
}, 500);

async function handleInvite() {
    isLoading.value = true;
    try {
        await groupStore.inviteUser(
            BigInt(route.params.id as string),
            inputtedUsername.value,
        );
        isInviteUserDialogOpen.value = false;
    } catch (e) {
        inviteError.value = (e as Error).message;
    } finally {
        isLoading.value = false;
    }
}

function startRecording() {
    if (!canvasRef.value) return;

    const canvasStream = canvasRef.value.captureStream(30);
    // const audioTracks = [
    //     displayCamera.value?.getAudioTracks()[0],
    //     displayStream.value?.getAudioTracks()[0],
    // ];

    // const validAudioTracks = audioTracks.filter(Boolean);

    console.log("display camera", displayCamera.value?.getAudioTracks());
    console.log("display stream", displayStream.value?.getAudioTracks());

    // mediaStreamAudioDestinationNode.value = new MediaStreamAudioDestinationNode(audioContext.value);

    const combinedStream = new MediaStream([
        ...canvasStream.getVideoTracks(),
        // mediaStreamAudioDestinationNode.value.stream.getAudioTracks()[0]
    ]);

    mediaRecorder.value = new MediaRecorder(combinedStream, {
        mimeType: "video/mp4",
    });

    mediaRecorder.value.ondataavailable = (e) => {
        if (e.data.size > 0) {
            recordedChunks.value.push(e.data);
        }
    };

    mediaRecorder.value.start();
    isRecording.value = true;
}

function stopRecording() {
    if (!mediaRecorder.value) return;

    mediaRecorder.value.stop();
    isRecording.value = false;
}

async function sendPerChunk(chunk: Uint8Array, index: number, fileId: string) {
    return new Promise<void>((resolve) => {
        setTimeout(() => {
            // Simulate upload action
            console.log(`Uploaded chunk ${index + 1}`);
            resolve();
        }, 500);
    });
}

async function prepareChunks(
    videoData: Uint8Array,
    type: keyof RecordedChunks,
) {
    const chunkSize = 1024 * 1024; // 1 mb
    const chunks = createChunks(videoData, chunkSize);

    const fileId = generateUUID();

    uploadedChunk.value[type] = 0;

    for (let i = 0; i < chunks.length; i++) {
        const chunk = chunks[i];

        await sendPerChunk(chunk, i, fileId);

        uploadedChunk.value[type] += chunk.byteLength;
    }
}

async function saveRecording() {
    const blob = new Blob(recordedChunks.value, { type: "video/mp4" });
    const data = new Uint8Array(await blob.arrayBuffer());

    // await groupStore.addVideo(data);

    recordedVideo.value = data;
    url.value = URL.createObjectURL(blob);
    // totalUploadSize.value[type] = recordedVideo.value[type].byteLength;
    // await prepareChunks(recordedVideo.value[type], type);

    recordedChunks.value = [];
}

async function handleRecord() {
    if (recordingPhaseText.value === "Record") {
        startRecording();
    } else if (recordingPhaseText.value === "Stop") {
        stopRecording();
    } else {
        // isOpen.value = true;
        await saveRecording();
        // isOpen.value = false;
        // uploadRecording();
    }
}
async function fetchGroupDetails() {
    const { id } = route.params;
    const groupId = BigInt(id as string);

    try {
        const response = await groupStore.getGroup(groupId);
        console.log(response);
    } catch (e) {
        console.log((e as Error).message);
    }
}

// async function uploadRecording() {
//     const { id } = route.params;
//     const groupId = BigInt(id as string);

//     if (!recordedVideo.value.screen || !recordedVideo.value.camera) {
//         return;
//     }

//     const payload = {
//         id: groupId,
//         screen: recordedVideo.value.screen,
//         camera: recordedVideo.value.camera,
//     };

//     const response = await groupStore.addVideo(payload);

//     console.log(response);
// }

function startDrawing() {
    if (!ctx.value) return;

    const draw = () => {
        if (!ctx.value || !canvasRef.value) return;
        ctx.value.clearRect(
            0,
            0,
            canvasRef.value.width,
            canvasRef.value.height,
        );

        // Draw screen capture
        if (enabledScreen.value && displayStream.value) {
            const screenVideo = videoRef.value;
            if (screenVideo) {
                ctx.value.drawImage(
                    screenVideo,
                    0,
                    0,
                    canvasRef.value.width,
                    canvasRef.value.height,
                );
            }
        }

        // Draw camera feed
        if (enabledCamera.value && displayCamera.value) {
            const cameraVideo = cameraRef.value;
            if (cameraVideo) {
                ctx.value.drawImage(
                    cameraVideo,
                    0,
                    0,
                    cameraDimensions.value.width,
                    cameraDimensions.value.height,
                );
            }
        }

        animationFrameId.value = requestAnimationFrame(draw);
    };

    draw();
}

async function onToggleCamera() {
    enabledCamera.value = !enabledCamera.value;

    const audioTrack = displayCamera.value?.getAudioTracks()[0]!;
    // const anotherMediaStreamAudioSourceNode = new MediaStreamAudioSourceNode(
    //     audioContext.value,
    //     { mediaStream: displayCamera.value! }
    // );

    if (enabledCamera.value) {
        mediaRecorder.value?.stream.addTrack(audioTrack);
        console.log("added toggle camera track");
    } else {
        mediaRecorder.value?.stream.removeTrack(audioTrack);
        console.log("removed toggle camera track");
    }
}

async function onToggleScreen() {
    enabledScreen.value = !enabledScreen.value;

    const audioTrack = displayStream.value?.getAudioTracks()[0]!;
    if (enabledScreen.value) {
        mediaRecorder.value?.stream.addTrack(audioTrack);
        console.log("added toggle screen track");
    } else {
        mediaRecorder.value?.stream.removeTrack(audioTrack);
        console.log("removed toggle screen track");
    }
}

watchEffect(() => {
    if (videoRef.value) {
        videoRef.value.srcObject = displayStream.value!;
    }
    if (cameraRef.value) {
        cameraRef.value.srcObject = displayCamera.value!;
    }
});

onMounted(async () => {
    if (canvasRef.value) {
        canvasRef.value.width = 1920;
        canvasRef.value.height = 1080;
        ctx.value = canvasRef.value.getContext("2d");
        startDrawing();
    }
});

async function init() {
    // inviteUser();
    await fetchGroupDetails();

    // ws.value!.onmessage = async (event) => {
    //     console.log(event);
    // };
}

await init();
</script>

<style scoped></style>
