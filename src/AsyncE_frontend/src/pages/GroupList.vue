<template>
    <div class="flex-1 py-8">
        <div class="container mx-auto">
            <div class="mb-10 flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900">Your Groups</h1>
                <router-link to="/create-group">
                    <Button>
                        <template #default>
                            <Icon
                                class="mr-2"
                                icon="mdi-light:plus-circle"
                                width="1.2rem"
                                height="1.2rem"
                            />
                            Create New Group
                        </template>
                    </Button>
                </router-link>
            </div>

            <div class="grid grid-cols-3 gap-6 md:grid-cols-4 lg:grid-cols-5">
                <div
                    v-for="(group, index) in groupList"
                    :key="index"
                    class="flex cursor-pointer flex-col overflow-hidden rounded-lg bg-white shadow-md"
                    @click="router.push(`/group/${group.id}`)"
                >
                    <img
                        class="h-auto w-full flex-1 object-cover"
                        :src="blobToURL(group.profile_picture_blob)"
                    />
                    <div class="p-4">
                        <h2 class="mb-2 text-xl font-semibold text-gray-800">
                            {{ group.name }}
                        </h2>
                        <p class="mb-4 text-gray-600">
                            {{ group.members.length }} members
                        </p>
                        <div class="flex items-center justify-between">
                            <Button> View Group </Button>
                        </div>
                    </div>
                </div>
            </div>

            <div
                v-if="!groupList.length"
                class="mt-10 flex flex-col items-center justify-center rounded-xl border border-gray-300 bg-gray-100 p-10 text-center shadow-md"
            >
                <Icon
                    icon="mdi-light:account-group"
                    width="5rem"
                    height="5rem"
                    class="mb-6 text-gray-500"
                />
                <h2 class="mb-3 text-3xl font-semibold text-gray-900">
                    No Groups Found
                </h2>
                <p class="mb-8 max-w-lg text-gray-700">
                    Looks like you haven’t created or joined any groups yet.
                    Start your journey by creating a new group and connecting
                    with others!
                </p>
                <router-link to="/create-group">
                    <Button>
                        <template #default>
                            <Icon
                                class="mr-2"
                                icon="mdi-light:plus-circle"
                                width="1.2rem"
                                height="1.2rem"
                            />
                            Create New Group
                        </template>
                    </Button>
                </router-link>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { Button } from "@components/ui/button";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";
import { blobToURL } from "@/utils/helpers";

const router = useRouter();
const groupStore = useGroupStore();

const { groupList } = storeToRefs(groupStore);

async function getAllGroups() {
    try {
        await groupStore.getAllGroups();
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function init() {
    await getAllGroups();
}

await init();
</script>
