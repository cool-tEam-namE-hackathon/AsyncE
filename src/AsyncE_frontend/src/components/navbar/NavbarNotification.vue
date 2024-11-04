<template>
    <!-- NOTIFICATION DROPDOWN -->
    <base-popover class="w-fit p-0">
        <template #trigger>
            <Button
                variant="outline"
                class="relative rounded-full p-0 border-none px-2 shadow-none"
            >
                <Icon
                    icon="nimbus:notification"
                    width="24"
                    height="24"
                    class="text-black"
                />

                <span
                    v-show="invites.length"
                    class="flex items-center justify-center absolute top-0 right-0 bg-red-500 h-5 w-5 text-white text-xs font-bold rounded-full"
                >
                    {{ invites.length }}
                </span>
            </Button>
        </template>

        <template #content>
            <div class="overflow-auto max-h-72 h-fit">
                <div class="px-4 mt-2 text-sm font-medium">Group Invites</div>
                <hr v-show="invites.length" class="mt-2" />
                <div
                    v-for="invite in invites"
                    :key="Number(invite.group_id)"
                    class="py-2 text-sm"
                >
                    <div class="px-4 mb-2">{{ invite.group_name }}</div>
                    <div class="flex items-center gap-3 px-4">
                        <Button
                            size="sm"
                            variant="outline"
                            :is-loading="isLoading['accept']"
                            :disabled="isLoading['accept']"
                            @click="
                                handleInvitation(
                                    invite.group_id,
                                    true,
                                    'accept',
                                )
                            "
                        >
                            <template #default>
                                <Icon
                                    icon="charm:tick"
                                    width="16"
                                    height="16"
                                    class="text-black mr-1"
                                />
                                Approve
                            </template>

                            <template #loading>
                                <Icon
                                    icon="prime:spinner"
                                    width="16"
                                    height="16"
                                    class="text-black animate-spin mr-1"
                                />
                                Approving...
                            </template>
                        </Button>
                        <Button
                            size="sm"
                            variant="outline"
                            :is-loading="isLoading['reject']"
                            :disabled="isLoading['reject']"
                            @click="
                                handleInvitation(
                                    invite.group_id,
                                    false,
                                    'reject',
                                )
                            "
                        >
                            <template #default>
                                <Icon
                                    icon="oui:cross"
                                    width="16"
                                    height="16"
                                    class="text-black mr-1"
                                />
                                Reject
                            </template>

                            <template #loading>
                                <Icon
                                    icon="prime:spinner"
                                    width="16"
                                    height="16"
                                    class="text-black animate-spin mr-1"
                                />
                                Rejecting...
                            </template>
                        </Button>
                    </div>
                </div>

                <div
                    v-if="!invites.length"
                    class="mt-2 text-sm text-muted font-medium"
                >
                    <hr />
                    <div class="p-4 text-gray-400">You have no invitation</div>
                </div>
            </div>
        </template>
    </base-popover>
</template>

<script setup lang="ts">
import Button from "@components/ui/button/Button.vue";
import { GroupInviteResponse } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { Icon } from "@iconify/vue";
import BasePopover from "@shared/BasePopover.vue";
import { useGroupStore } from "@stores/group-store";
import { useWebsocketStore } from "@stores/websocket-store";
import { ref } from "vue";
import { GroupInvite } from "@/types/api/model";

const groupStore = useGroupStore();
const websocketStore = useWebsocketStore();

const isLoading = ref<{ accept: boolean; reject: boolean }>({
    accept: false,
    reject: false,
});
const invites = ref<GroupInvite[] | GroupInviteResponse[]>([]);

websocketStore.setOnGroupInvited(handleGroupInvite);

async function getAllInvites() {
    try {
        const response = await groupStore.getInvites();
        invites.value = [...invites.value, ...response];
    } catch (e) {
        console.log((e as Error).message);
    }
}

function handleGroupInvite(group: GroupInvite | GroupInviteResponse) {
    invites.value = [...invites.value, group];
}

async function handleInvitation(
    groupId: bigint,
    invitation: boolean,
    type: "accept" | "reject",
) {
    isLoading.value[type] = true;

    try {
        await groupStore.handleInvitation(groupId, invitation);
        invites.value = invites.value.filter(
            (invite) => invite.group_id !== groupId,
        );
    } catch (e) {
        console.log(e);
    } finally {
        isLoading.value[type] = false;
    }
}

async function init() {
    getAllInvites();
}

// init();
</script>
