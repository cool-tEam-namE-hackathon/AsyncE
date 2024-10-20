<template>
    <div class="container h-full flex flex-col">
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
                            class="text-black animate-spin mr-1"
                        />
                        Inviting...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- INPUT GROUP TITLE -->
        <base-dialog
            :open="isConfirmationModalOpen"
            @on-close-dialog="toggleConfirmationModal"
        >
            <template #title>
                <div>Confirm</div>
            </template>

            <template #content>
                <div class="space-y-3">
                    <Label>Title</Label>
                    <Input v-model="videoTitle" />
                </div>
            </template>

            <template #footer>
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
                            class="text-black animate-spin mr-1"
                        />
                        Uploading...
                        {{ uploadVideoProgress.toFixed(0) }}%
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- GROUP NAME -->
        <span> {{ currentGroup?.name }}</span>

        <!-- MEDIA -->
        <div class="flex-grow flex flex-col py-8">
            <div class="flex flex-col gap-8 h-full">
                <div class="flex h-3/4 gap-6">
                    <!-- VIDEO -->
                    <div
                        class="flex flex-col flex-1 bg-white rounded-lg h-full shadow-lg p-4 overflow-hidden"
                    >
                        <div class="flex items-center justify-between mb-3">
                            <h2 class="text-xl font-semibold">
                                Record New Video
                            </h2>
                            <video-controls
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

                        <div class="relative flex-grow overflow-hidden">
                            <!-- NO CAMERA OR SCREEN YET -->
                            <div
                                v-if="!displayCamera"
                                class="absolute inset-0 flex flex-col items-center justify-center bg-gray-200 rounded-lg"
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
                                <p class="text-sm mt-2">
                                    Please start your camera or screen share
                                </p>
                            </div>

                            <video
                                v-else
                                ref="cameraRef"
                                class="absolute inset-0 w-full h-full object-cover rounded-lg"
                                muted
                                autoplay
                            />
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
                        <span class="text-sm text-gray-400">
                            Manage and invite users to your video sessions
                        </span>
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

                <div class="flex-1">
                    <video-list ref="videoList" />
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
import { ref, watchEffect, computed } from "vue";

import { FFmpeg } from "@ffmpeg/ffmpeg";

import { useRoute } from "vue-router";
import { useUserStore } from "@stores/user-store";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";

import { useUserMedia, useDevicesList, useDebounceFn } from "@vueuse/core";

import { Icon } from "@iconify/vue";

import VideoList from "@components/video/VideoList.vue";
import VideoControls from "@components/video/VideoControls.vue";
import ChatWindow from "@components/chat/ChatWindow.vue";

import { Button } from "@components/ui/button";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";

import BaseDialog from "@components/shared/BaseDialog.vue";

import { fetchFile, toBlobURL } from "@ffmpeg/util";

const route = useRoute();
const groupStore = useGroupStore();
const userStore = useUserStore();

const ffmpeg = ref<FFmpeg>();

const videoList = ref<InstanceType<typeof VideoList> | null>(null);

const selectedCamera = ref<string>();
const invitedUsername = ref<string>("");
const inputtedUsername = ref<string>("");
const url = ref<string>("");
const inviteError = ref<string>("");
const videoTitle = ref<string>("");

const isInviteUserDialogOpen = ref<boolean>(false);
const isRecording = ref<boolean>(false);
const isUploading = ref<boolean>(false);
const isError = ref<boolean>(false);
const isLoading = ref<boolean>(false);
const isConfirmationModalOpen = ref<boolean>(false);

const recordedChunks = ref<Blob[]>([]);

const cameraRef = ref<HTMLVideoElement | null>(null);

const mediaRecorder = ref<MediaRecorder | null>(null);

const { currentGroup, uploadVideoProgress } = storeToRefs(groupStore);

const { videoInputs: cameras, audioInputs: microphones } = useDevicesList({
    requestPermissions: false,
});

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

const isFieldError = computed(() => {
    return !isError.value && inputtedUsername.value;
});

function toggleInviteModal() {
    isInviteUserDialogOpen.value = !isInviteUserDialogOpen.value;
}

function toggleConfirmationModal() {
    isConfirmationModalOpen.value = !isConfirmationModalOpen.value;
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

    const combinedStream = new MediaStream(displayCamera.value);
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

async function saveRecording() {
    isUploading.value = true;

    const blob = new Blob(recordedChunks.value, { type: "video/webm" });

    const mp4Video = await convertToMp4(blob);

    const data = new Uint8Array(await mp4Video.arrayBuffer());

    url.value = URL.createObjectURL(mp4Video);

    try {
        await groupStore.addVideo(
            data,
            route.params.id as string,
            videoTitle.value,
        );
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        videoList.value?.getAllVideos();
        isUploading.value = false;
        toggleConfirmationModal();
    }

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

async function convertToMp4(blob: Blob) {
    const ffmpegInstance = await getFFmpegInstance();

    await ffmpegInstance.writeFile("input.webm", await fetchFile(blob));
    await ffmpegInstance.exec(["-i", "input.webm", "-c", "copy", "output.mp4"]);

    const data = await ffmpegInstance.readFile("output.mp4");

    const mp4Blob = new Blob([(data as Uint8Array).buffer], {
        type: "video/mp4",
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
