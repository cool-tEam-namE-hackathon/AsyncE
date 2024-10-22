<template>
    <div class="container mt-10 rounded-lg h-full mb-6">
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
                    :disabled="
                        isFieldError || isLoading['invite'] || !invitedUsername
                    "
                    :is-loading="isLoading['invite']"
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

        <base-dialog
            :open="isCreateMeetingDialogOpen"
            @on-close-dialog="toggleCreateMeetingDialog"
        >
            <template #title> Create New Meeting </template>

            <template #content>
                <Input v-model="meetingName" />
            </template>

            <template #footer>
                <Button
                    :disabled="!meetingName || isLoading['meeting']"
                    :is-loading="isLoading['meeting']"
                    @click="createMeeting"
                >
                    <template #default> Create Meeting </template>

                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="text-black animate-spin mr-1"
                        />
                        Creating...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <div class="flex gap-3 h-full">
            <div class="flex-1 flex flex-col border rounded-md p-4">
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-xl font-bold">List of meetings</h2>
                    <Button
                        class="text-white rounded-full flex items-center gap-3"
                        @click="toggleCreateMeetingDialog"
                    >
                        <Icon
                            icon="tabler:plus"
                            width="22"
                            height="22"
                            class="text-white"
                        />
                        Create Meeting
                    </Button>
                </div>
                <div
                    v-for="(meeting, index) in meetingList"
                    :key="index"
                    class="bg-gray-50 p-3 rounded-lg hover:bg-gray-100 transition duration-200 ease-in-out mb-3"
                    @click="goToMeetingPage(meeting.id.toString())"
                >
                    <span class="font-semibold">{{ meeting.title }}</span>
                </div>
            </div>

            <div class="flex flex-col gap-3 w-1/4 h-full">
                <div class="flex flex-col border rounded-md p-4 h-1/4">
                    <div class="rounded-lg mb-4">
                        <div class="flex justify-between items-center mb-4">
                            <div class="flex items-center">
                                <h2 class="text-xl font-bold">List of users</h2>
                            </div>
                            <Button
                                class="text-white p-2 rounded-full"
                                @click="toggleInviteModal"
                            >
                                <Icon
                                    icon="mdi:invite"
                                    width="24"
                                    height="24"
                                    class="text-white"
                                />
                            </Button>
                        </div>
                        <div
                            v-for="(user, index) in currentGroup?.users"
                            :key="index"
                            class="mb-2"
                        >
                            <div
                                class="p-2 bg-gray-50 hover:bg-gray-100 transition duration-200 ease-in-out rounded-md text-sm"
                            >
                                {{ user }}
                            </div>
                        </div>
                        <!-- <div
                            class="max-h-60 overflow-y-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800"
                        >
                            No users
                        </div> -->
                    </div>
                </div>

                <div class="h-3/4">
                    <meeting-chat-window />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";

import { storeToRefs } from "pinia";

import { useDebounceFn } from "@vueuse/core";

import { useRoute, useRouter } from "vue-router";
import { useUserStore } from "@stores/user-store";
import { useGroupStore } from "@stores/group-store";

import { Icon } from "@iconify/vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";

import MeetingChatWindow from "@components/Meeting/MeetingChatWindow.vue";
import BaseDialog from "@components/shared/BaseDialog.vue";

const route = useRoute();
const router = useRouter();

const userStore = useUserStore();
const groupStore = useGroupStore();

const { meetingList, currentGroup } = storeToRefs(groupStore);

const isCreateMeetingDialogOpen = ref<boolean>(false);
const meetingName = ref<string>("");
const isLoading = ref<{
    meeting: boolean;
    invite: boolean;
}>({
    meeting: false,
    invite: false,
});

const isInviteUserDialogOpen = ref<boolean>(false);
const invitedUsername = ref<string>("");
const inputtedUsername = ref<string>("");
const inviteError = ref<string>("");
const isError = ref<boolean>(false);

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
    isLoading.value["invite"] = true;
    try {
        await groupStore.inviteUser(
            BigInt(route.params.id as string),
            inputtedUsername.value,
        );
        isInviteUserDialogOpen.value = false;
    } catch (e) {
        inviteError.value = (e as Error).message;
    } finally {
        isLoading.value["invite"] = false;
    }
}

function toggleCreateMeetingDialog() {
    isCreateMeetingDialogOpen.value = !isCreateMeetingDialogOpen.value;
}

function goToMeetingPage(id: string) {
    router.push(`/group/${route.params.id}/meeting/${id}`);
}

async function createMeeting() {
    isLoading.value["meeting"] = true;
    try {
        await groupStore.createMeeting(
            route.params.id as string,
            meetingName.value,
        );

        isLoading.value["meeting"] = false;

        getAllMeetings();
        toggleCreateMeetingDialog();
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function getAllMeetings() {
    try {
        await groupStore.getAllMeetings(route.params.id as string);
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function getGroup() {
    try {
        await groupStore.getGroup(route.params.id as string);
    } catch (e) {
        console.log((e as Error).message);
    }
}

function init() {
    getAllMeetings();
    getGroup();
}

init();
</script>
