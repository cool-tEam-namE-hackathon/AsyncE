<template>
    <div class="container mx-auto mb-6 mt-10 h-full rounded-lg px-4">
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
                    <div class="flex flex-col items-center gap-3 sm:flex-row">
                        <Label>Username</Label>
                        <div class="relative w-full flex-1 sm:w-auto">
                            <Input
                                v-model="invitedUsername"
                                class="w-full"
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
                                width="24"
                                height="24"
                                class="absolute right-2 top-1/2 -translate-y-1/2 transform"
                                :class="{
                                    'text-red-700':
                                        isFieldError && invitedUsername,
                                    'text-green-700':
                                        !isFieldError && invitedUsername,
                                    hidden: !invitedUsername,
                                }"
                            />
                        </div>
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
                    class="w-full sm:w-auto"
                    @click="handleInvite"
                >
                    <template #default> Invite </template>
                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-white"
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
                <Input v-model="meetingName" class="w-full" />
            </template>

            <template #footer>
                <Button
                    :disabled="!meetingName || isLoading['meeting']"
                    :is-loading="isLoading['meeting']"
                    class="w-full sm:w-auto"
                    @click="createMeeting"
                >
                    <template #default> Create Meeting </template>
                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-white"
                        />
                        Creating...
                    </template>
                </Button>
            </template>
        </base-dialog>

        <!-- MAIN CONTENT -->
        <div class="flex min-h-[calc(100vh-8rem)] flex-col gap-6 lg:flex-row">
            <!-- MEETING LIST SECTION -->
            <div
                class="flex w-full flex-1 flex-col rounded-md border p-4 shadow-md sm:p-6"
            >
                <div
                    class="mb-6 flex flex-col items-center justify-between gap-4 sm:flex-row"
                >
                    <h2 class="text-xl font-bold">Active Meetings</h2>
                    <Button
                        class="w-full rounded-full sm:w-auto"
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
                <div class="space-y-2">
                    <div
                        v-for="(meeting, index) in meetingList"
                        :key="index"
                        class="flex items-center justify-between rounded-lg border border-gray-100 bg-gray-100 p-4 shadow-sm"
                    >
                        <div class="flex flex-col gap-2">
                            <h2 class="text-xl font-semibold">
                                {{ meeting.title }}
                            </h2>
                            <div class="flex items-center gap-1">
                                <Icon
                                    icon="line-md:person"
                                    width="24"
                                    height="24"
                                    class="text-black"
                                />
                                <span class="text-gray-500"
                                    >Created by: {{ meeting.created_by }}</span
                                >
                            </div>
                            <span class="text-sm text-gray-500"
                                >Created at:
                                {{
                                    convertDateToReadableFormat(
                                        meeting.created_time_unix,
                                    )
                                }}</span
                            >
                        </div>
                        <Button @click="goToMeetingPage(meeting.id.toString())">
                            Join
                        </Button>
                    </div>
                </div>

                <!-- NO MEETINGS -->
                <div
                    v-if="!meetingList.length"
                    class="flex h-full flex-col items-center justify-center text-center text-gray-500"
                >
                    <Icon
                        icon="line-md:file-document-cancel-twotone"
                        width="128"
                        height="128"
                        class="mb-4 text-black"
                    />
                    <h2 class="text-xl font-semibold text-gray-700">
                        No Meetings Found
                    </h2>
                    <p class="mt-3 text-gray-500">
                        You don't have any meetings scheduled yet. Start by
                        creating a new meeting!
                    </p>
                </div>
            </div>

            <!-- USER LIST AND CHAT SECTION -->
            <div
                class="flex h-[calc(100vh-8rem)] w-full flex-col gap-6 lg:w-1/4"
            >
                <!-- USER LIST -->
                <div
                    class="flex min-h-[250px] flex-col rounded-md border p-4 shadow-md sm:p-6 lg:h-2/5"
                >
                    <div class="mb-4 flex items-center justify-between">
                        <h2 class="text-xl font-bold">Members</h2>
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
                        <div class="space-y-2">
                            <div
                                v-for="(user, index) in currentGroup?.members"
                                :key="index"
                                class="flex items-center justify-between rounded-lg border border-gray-100 bg-gray-50 p-3 text-sm"
                            >
                                <div class="flex items-center gap-2">
                                    <Icon
                                        v-if="
                                            'Admin' in user.role ||
                                            currentGroup?.owner ===
                                                user.username
                                        "
                                        icon="mdi:crown"
                                        :class="{
                                            'text-yellow-500':
                                                currentGroup?.owner ===
                                                user.username,
                                            'text-blue-500':
                                                'Admin' in user.role,
                                        }"
                                        width="16"
                                        height="16"
                                    />
                                    <span class="font-medium text-gray-900">{{
                                        user.username
                                    }}</span>
                                </div>

                                <base-dropdown
                                    v-if="showDropdownFor(user)"
                                    label="Your account"
                                    :options="editUserOptions"
                                    @on-option-click="handleOptionClick"
                                >
                                    <template #trigger>
                                        <button @click="setSelectedUser(user)">
                                            <Icon
                                                icon="ph:dots-three-bold"
                                                width="16"
                                                height="16"
                                                class="ml-auto mt-1 text-black"
                                            />
                                        </button>
                                    </template>
                                </base-dropdown>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- CHAT WINDOW -->
                <div class="h-3/5 rounded-lg shadow-md">
                    <meeting-chat-window />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { storeToRefs } from "pinia";
import { useRoute, useRouter } from "vue-router";
import { useDebounceFn } from "@vueuse/core";
import { GroupMember } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { GroupMemberRole } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import MeetingChatWindow from "@components/Meeting/MeetingChatWindow.vue";
import BaseDialog from "@components/shared/BaseDialog.vue";
import BaseDropdown from "@components/shared/BaseDropdown.vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";
import { RoleKeys } from "@/types/api/model";
import { convertDateToReadableFormat } from "@/utils/helpers";

const route = useRoute();
const router = useRouter();

const userStore = useUserStore();
const groupStore = useGroupStore();

const { userCredentials } = storeToRefs(userStore);
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
const selectedUser = ref<GroupMember | null>(null);

const currentUser = computed(() => {
    return currentGroup.value?.members.find(
        (member) => member.username === userCredentials.value?.username,
    );
});

const ADMIN_OPTIONS = [
    {
        name: "Make admin",
        condition: (selectedUser: GroupMember) =>
            "Admin" in currentUser.value!.role &&
            !("Admin" in selectedUser.role),
    },
    {
        name: "Make member",
        condition: (selectedUser: GroupMember) =>
            "Admin" in currentUser.value!.role &&
            "Admin" in selectedUser.role &&
            currentGroup.value?.owner !== selectedUser.username,
    },
    {
        name: "Remove user",
        class: "text-red-500",
        condition: (selectedUser: GroupMember) =>
            "Admin" in currentUser.value!.role &&
            currentUser.value!.username !== selectedUser.username &&
            currentGroup.value?.owner !== selectedUser.username,
    },
];

const isFieldError = computed(() => {
    return !isError.value && inputtedUsername.value;
});

const editUserOptions = computed(() => {
    if (!selectedUser.value) return [];

    console.log(selectedUser.value);

    return ADMIN_OPTIONS.filter((option) =>
        option.condition(selectedUser.value!),
    );
});

function showDropdownFor(user: GroupMember): boolean {
    const currentUser = currentGroup.value?.members.find(
        (member) => member.username === userCredentials.value?.username,
    );

    const isCurrentUserAdmin = "Admin" in currentUser!.role;

    return (
        isCurrentUserAdmin &&
        user.username !== userCredentials.value?.username &&
        currentGroup.value?.owner !== user.username
    );
}

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

async function editRole(newRole: RoleKeys) {
    const role: GroupMemberRole = { [newRole]: null } as GroupMemberRole;
    try {
        await groupStore.editRole(
            route.params.id as string,
            selectedUser.value!.username,
            role,
        );
        getGroup();
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function kickMember() {
    try {
        await groupStore.kickMember(
            route.params.id as string,
            selectedUser.value!.username,
        );
        getGroup();
    } catch (e) {
        console.log((e as Error).message);
    }
}

function setSelectedUser(user: GroupMember) {
    selectedUser.value = user;
}

function handleOptionClick(option: string) {
    if (option === "Make admin") editRole("Admin");

    if (option === "Make member") editRole("Member");

    if (option === "Remove user") kickMember();
}

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
        console.log(meetingList.value);
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

async function init() {
    await getAllMeetings();
    getGroup();
}

await init();
</script>
