<template>
    <div class="container mb-6 mt-10 h-full rounded-lg">
        <!-- INVITE USER DIALOG -->
        <base-dialog
            :open="isInviteUserDialogOpen"
            :is-closable="true"
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
                            class="mr-1 animate-spin text-black"
                        />
                        Inviting...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- CREATE MEETING DIALOG -->
        <base-dialog
            :open="isCreateMeetingDialogOpen"
            :is-closable="true"
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
                            class="mr-1 animate-spin text-black"
                        />
                        Creating...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- Main Content -->
        <div class="flex h-full flex-col gap-6 md:flex-row">
            <!-- MEETING LIST SECTION -->
            <div
                class="flex w-full flex-1 flex-col rounded-md border p-6 shadow-sm"
            >
                <div class="mb-6 flex items-center justify-between">
                    <h2 class="text-xl font-bold">List of meetings</h2>
                    <Button
                        class="rounded-full"
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
                    class="mb-4 cursor-pointer rounded-lg bg-gray-50 p-4 shadow-sm transition duration-200 ease-in-out hover:bg-gray-100"
                    @click="goToMeetingPage(meeting.id.toString())"
                >
                    <span class="font-semibold">{{ meeting.title }}</span>
                </div>
            </div>

            <!-- USER LIST AND CHAT SECTION -->
            <div class="flex h-full w-full flex-col gap-6 md:w-1/4">
                <!-- USER LIST -->
                <div
                    class="flex h-2/5 flex-col rounded-md border p-6 shadow-sm"
                >
                    <div class="mb-4 flex items-center justify-between">
                        <h2 class="text-xl font-bold">List of users</h2>
                        <Button class="rounded-full" @click="toggleInviteModal">
                            <Icon
                                icon="mdi:invite"
                                width="24"
                                height="24"
                                class="text-white"
                            />
                        </Button>
                    </div>
                    <div class="h-full space-y-3 overflow-auto">
                        <div
                            v-for="(user, index) in currentGroup?.members"
                            :key="index"
                        >
                            <div
                                class="rounded-md bg-gray-50 p-2 text-sm shadow-sm transition duration-200 ease-in-out hover:bg-gray-100"
                            >
                                {{ user.username }} ({{ user.role }})
                            </div>
                        </div>
                    </div>
                </div>

                <!-- CHAT WINDOW -->
                <div class="h-3/5 shadow-sm">
                    <meeting-chat-window />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import MeetingChatWindow from "@components/Meeting/MeetingChatWindow.vue";
import BaseDialog from "@components/shared/BaseDialog.vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import { useDebounceFn } from "@vueuse/core";
import { storeToRefs } from "pinia";
import { ref, computed } from "vue";
import { useRoute, useRouter } from "vue-router";

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
