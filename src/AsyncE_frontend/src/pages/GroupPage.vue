<template>
    <div class="container h-full">
        <!-- UPLOAD PROGRESS MODAL -->
        <base-dialog :open="isOpen" :hide-close-button="true">
            <template #title>
                <span class="mb-4 text-lg font-semibold">
                    Uploading Recordings
                </span>
            </template>
            <template #content>
                <div class="flex justify-between">
                    <span>Camera Recording</span>
                    <span>{{ uploadVideoProgress.toFixed(1) }}%</span>
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
                    :disabled="isFieldError || isLoading || !invitedUsername"
                    :is-loading="isLoading"
                    @click="handleInvite"
                >
                    <template #default> Invite </template>

                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-black"
                        />
                        Inviting...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- GROUP NAME -->
        <span> {{ currentGroup?.name }}</span>
        <span>{{ uploadVideoProgress }}</span>

        <!-- MEDIA -->
        <div class="mx-auto h-full py-8">
            <div class="flex h-full flex-col gap-8">
                <div class="flex gap-6">
                    <!-- VIDEO -->
                    <div
                        class="flex h-full flex-1 flex-col rounded-lg bg-white p-4 shadow-lg"
                    >
                        <div class="mb-3 flex items-center justify-between">
                            <h2 class="text-xl font-semibold">
                                Record New Video
                            </h2>
                            <VideoControls
                                v-model:selectedCamera="selectedCamera"
                                :camera-list="cameraList"
                                :enabled-camera="enabledCamera"
                                :is-recording-disabled="isRecordingDisabled"
                                :recording-phase-text="recordingPhaseText"
                                @on-toggle-camera="
                                    enabledCamera = !enabledCamera
                                "
                                @on-record="handleRecord"
                            />
                        </div>

                        <!-- NO CAMERA OR SCREEN YET -->
                        <div
                            v-if="!displayCamera"
                            class="flex h-full flex-col items-center justify-center rounded-lg bg-gray-200 py-4"
                        >
                            <Icon
                                icon="fluent:video-off-32-regular"
                                width="64"
                                height="64"
                                style="color: black"
                            />
                            <p class="text-xl font-semibold">
                                Video stream unavailable
                            </p>
                            <p class="mt-2 text-sm">
                                Please start your camera or screen share
                            </p>
                        </div>

                        <video
                            v-else
                            ref="cameraRef"
                            class="h-full w-full rounded-lg object-cover"
                            muted
                            autoplay
                        />
                    </div>

                    <!-- USER MANAGEMENT -->
                    <div
                        class="w-64 rounded-lg border-[1px] border-solid border-slate-200 p-4"
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
                                class="mr-2 text-white"
                            />
                            Invite new users
                        </Button>
                    </div>
                </div>
                <h1>{{ message }}</h1>
                <div class="h-1/4 flex-1">
                    <p>Here</p>
                </div>
            </div>
        </div>
    </div>

    <!-- CHAT -->
    <chat-window />

    <video v-if="url" autoplay muted controls>
        <source :src="url" type="video/mp4" />
        Your browser does not support the video tag.
    </video>
</template>

<script setup lang="ts">
import ChatWindow from "@components/chat/ChatWindow.vue";
import BaseDialog from "@components/shared/BaseDialog.vue";
import BaseProgress from "@components/shared/BaseProgress.vue";
import { Button } from "@components/ui/button";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import VideoControls from "@components/video/VideoControls.vue";
import { FFmpeg } from "@ffmpeg/ffmpeg";
import type { LogEvent } from "@ffmpeg/ffmpeg/dist/esm/types";
import { fetchFile, toBlobURL } from "@ffmpeg/util";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import { useDebounceFn, useDevicesList, useUserMedia } from "@vueuse/core";
import { storeToRefs } from "pinia";
import { computed, ref, watchEffect } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const groupStore = useGroupStore();
const userStore = useUserStore();

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

const url = ref<string>("");
const inviteError = ref<string>("");

const isRecording = ref<boolean>(false);
const isOpen = ref<boolean>(false);
const isError = ref<boolean>(false);
const isLoading = ref<boolean>(false);

const recordedChunks = ref<Blob[]>([]);

const cameraRef = ref<HTMLVideoElement | null>(null);

const mediaRecorder = ref<MediaRecorder | null>(null);
const recordedVideo = ref<Uint8Array | null>(null);

const { currentGroup, uploadVideoProgress } = storeToRefs(groupStore);

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: true,
});

const selectedCamera = computed(() => cameras.value[0]?.deviceId);
const currentMicrophone = computed(() => microphones.value[0]?.deviceId);

const { stream: displayCamera, enabled: enabledCamera } = useUserMedia({
    constraints: {
        video: { deviceId: selectedCamera.value },
        audio: { deviceId: currentMicrophone.value },
    },
});

const isRecordingDisabled = computed(() => {
    return !enabledCamera.value;
});

const cameraList = computed(() => {
    return cameras.value
        .filter(({ deviceId }) => deviceId !== "")
        .map(({ label, deviceId }) => ({
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

const cameraProgress = computed(() => {
    return (
        (uploadedChunk.value.camera / totalUploadSize.value.camera) * 100 || 0
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
    if (!displayCamera.value) return;

    const combinedStream = new MediaStream(displayCamera.value.getTracks());
    const options = {
        mimeType: "video/webm",
    };

    mediaRecorder.value = new MediaRecorder(combinedStream, options);
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

/* async function sendPerChunk(chunk: Uint8Array, index: number, fileId: string) {
    return new Promise<void>((resolve) => {
        setTimeout(() => {
            // Simulate upload action
            console.log(`Uploaded chunk ${index + 1}`);
            resolve();
        }, 500);
    });
} */

/* async function prepareChunks(
    videoData: Uint8Array,
    type: keyof RecordedChunks,
) {
    const chunks = createChunks(videoData, MB);

    const fileId = generateUUID();

    uploadedChunk.value[type] = 0;

    for (let i = 0; i < chunks.length; i++) {
        const chunk = chunks[i];

        await sendPerChunk(chunk, i, fileId);

        uploadedChunk.value[type] += chunk.byteLength;
    }
} */

async function saveRecording() {
    const blob = new Blob(recordedChunks.value, { type: "video/webm" });

    const mp4Video = await convertToMp4(blob);

    const data = new Uint8Array(await mp4Video.arrayBuffer());

    url.value = URL.createObjectURL(mp4Video);

    console.log(url.value);

    await groupStore.addVideo(data, route.params.id as string, "Test title");

    recordedVideo.value = data;
    recordedChunks.value = [];
}

const message = ref<string>("");

async function convertToMp4(blob: Blob) {
    const ffmpeg = new FFmpeg();
    console.log("not loaded");

    ffmpeg.on("log", ({ message: msg }: LogEvent) => {
        message.value = msg;
    });

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

    console.log("loaded");

    await ffmpeg.writeFile("input.webm", await fetchFile(blob));

    console.log("here 1");

    try {
        await ffmpeg.exec(["-i", "input.webm", "output.mp4"]);
    } catch (e) {
        console.log(e);
    }

    const data = await ffmpeg.readFile("output.mp4");

    console.log("here2 ");

    const mp4Blob = new Blob([(data as Uint8Array).buffer], {
        type: "video/mp4",
    });

    console.log("here3 ");

    console.log(mp4Blob);

    return mp4Blob;
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

watchEffect(() => {
    if (cameraRef.value) {
        cameraRef.value.srcObject = displayCamera.value!;
    }
});

async function init() {
    await fetchGroupDetails();
}

await init();
</script>
