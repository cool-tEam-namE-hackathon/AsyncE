<template>
    <div class="container h-full">
        <!-- UPLOAD PROGRESS MODAL -->
        <base-dialog :open="isOpen" :hide-close-button="true">
            <template #title>
                <span class="text-lg font-semibold mb-4">
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
                            class="text-black animate-spin mr-1"
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
        <div class="mx-auto py-8 h-full">
            <div class="flex flex-col gap-8 h-full">
                <div class="flex gap-6">
                    <!-- VIDEO -->
                    <div
                        class="flex flex-col flex-1 bg-white rounded-lg shadow-lg h-full p-4"
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

                        <!-- NO CAMERA OR SCREEN YET -->
                        <div
                            v-if="!displayCamera"
                            class="flex flex-col items-center justify-center bg-gray-200 rounded-lg h-full py-4"
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
                            class="object-cover w-full h-full rounded-lg"
                            muted
                            autoplay
                        />
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
                <div class="flex-1 h-1/4">
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
import { ref, watchEffect, computed } from "vue";

import { useRoute } from "vue-router";
import { useUserStore } from "@stores/user-store";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";

import { useUserMedia, useDevicesList, useDebounceFn } from "@vueuse/core";

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
import { MB } from "@/data/user-constants";

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
        mimeType: 'video/mp4; codecs="avc1.42E01E, mp4a.40.2"'
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
    const chunks = createChunks(videoData, MB);

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
    await groupStore.addVideo(data, route.params.id as string, "Test title");

    url.value = URL.createObjectURL(blob);
    recordedVideo.value = data;
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
