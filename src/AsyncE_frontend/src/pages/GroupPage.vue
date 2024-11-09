<template>
    <div class="container mx-auto mb-6 mt-6 h-full rounded-lg px-4">
        <!-- INVITE USER DIALOG -->
        <invite-user-dialog
            :open="isInviteUserDialogOpen"
            @on-close-dialog="toggleInviteDialog"
        />

        <!-- CREATE MEETING DIALOG -->
        <create-meeting-dialog
            :open="isCreateMeetingDialogOpen"
            @on-create-meeting="getAllMeetings"
            @on-close-dialog="toggleCreateMeetingDialog"
        />

        <!-- MAIN CONTENT -->
        <div class="flex h-full flex-col gap-6 lg:flex-row">
            <!-- MEETING LIST SECTION -->
            <div
                class="flex h-[300px] flex-1 flex-col rounded-md border p-4 shadow-md sm:h-[350px] sm:p-6 lg:h-full"
            >
                <div
                    class="mb-6 flex flex-col items-center justify-between gap-4 sm:flex-row"
                >
                    <h2 class="text-xl font-bold">Active Meetings</h2>
                    <Button
                        class="flex w-full gap-2 rounded-full sm:w-auto"
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
                    v-if="!meetingList.length"
                    class="flex flex-1 flex-col items-center justify-center text-center text-gray-500"
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

                <div v-else class="flex-1 space-y-2 overflow-y-auto">
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
                            <span class="text-sm text-gray-500">
                                Created at:
                                {{
                                    convertDateToReadableFormat(
                                        meeting.created_time_unix,
                                    )
                                }}
                            </span>
                        </div>
                        <Button @click="goToMeetingPage(meeting.id.toString())">
                            Open
                        </Button>
                    </div>
                </div>
            </div>

            <!-- USER LIST AND CHAT SECTION -->
            <div class="flex h-full flex-col gap-6 overflow-hidden lg:w-1/4">
                <!-- USER LIST -->
                <div
                    class="flex h-1/2 flex-col overflow-auto rounded-md border p-4 shadow-md sm:p-6 lg:h-2/5"
                >
                    <div class="mb-4 flex items-center justify-between">
                        <h2 class="text-xl font-bold">Members</h2>
                        <Button
                            class="rounded-full"
                            @click="toggleInviteDialog"
                        >
                            <Icon
                                icon="mdi:invite"
                                width="24"
                                height="24"
                                class="text-white"
                            />
                        </Button>
                    </div>
                    <div class="flex-1 space-y-3 overflow-auto">
                        <div
                            v-for="(user, index) in currentGroup?.members"
                            :key="index"
                            class="flex items-center justify-between rounded-lg border border-gray-100 bg-gray-50 p-3 text-sm"
                        >
                            <div class="flex items-center gap-2">
                                <Icon
                                    v-if="
                                        'Admin' in user.role ||
                                        currentGroup?.owner === user.username
                                    "
                                    icon="mdi:crown"
                                    :class="{
                                        'text-yellow-500':
                                            currentGroup?.owner ===
                                            user.username,
                                        'text-blue-500': 'Admin' in user.role,
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

                <div class="flex h-1/2 flex-col overflow-auto lg:h-3/5">
                    <group-chat-window />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { storeToRefs } from "pinia";
import { useRoute, useRouter } from "vue-router";
import { GroupMember } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { GroupMemberRole } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import CreateMeetingDialog from "@components/group/CreateMeetingDialog.vue";
import GroupChatWindow from "@components/group/GroupChatWindow.vue";
import InviteUserDialog from "@components/group/InviteUserDialog.vue";
import BaseDropdown from "@components/shared/BaseDropdown.vue";
import { Button } from "@components/ui/button";
import { RoleKeys } from "@/types/api/model";
import { convertDateToReadableFormat } from "@/utils/helpers";

const route = useRoute();
const router = useRouter();

const userStore = useUserStore();
const groupStore = useGroupStore();

const { userCredentials } = storeToRefs(userStore);
const { meetingList, currentGroup } = storeToRefs(groupStore);

const isCreateMeetingDialogOpen = ref<boolean>(false);

const isInviteUserDialogOpen = ref<boolean>(false);

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

function toggleInviteDialog() {
    isInviteUserDialogOpen.value = !isInviteUserDialogOpen.value;
}

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

function toggleCreateMeetingDialog() {
    isCreateMeetingDialogOpen.value = !isCreateMeetingDialogOpen.value;
}

function goToMeetingPage(id: string) {
    router.push(`/group/${route.params.id}/meeting/${id}`);
}

async function getAllMeetings() {
    console.log("call");
    try {
        await groupStore.getAllMeetings(route.params.id as string);
        console.log("done");
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
