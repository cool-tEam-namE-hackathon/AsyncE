<template>
    <div class="flex-1 py-8">
        <div class="container mx-auto">
            <div class="flex justify-between items-center mb-10">
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

            <div class="grid grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                <div
                    v-for="(group, index) in groupList"
                    :key="index"
                    class="flex flex-col bg-white shadow-md rounded-lg overflow-hidden cursor-pointer"
                    @click="router.push(`/group/${group.id}`)"
                >
                    <img
                        class="flex-1 object-cover w-full h-auto"
                        :src="blobToURL(group.profile_picture_blob)"
                    />
                    <div class="p-4">
                        <h2 class="text-xl font-semibold text-gray-800 mb-2">
                            {{ group.name }}
                        </h2>
                        <p class="text-gray-600 mb-4">
                            {{ group.members.length }} members
                        </p>
                        <div class="flex justify-between items-center">
                            <Button> View Group </Button>
                        </div>
                    </div>
                </div>
            </div>

            <div
                v-if="!groupList.length"
                class="flex flex-col items-center justify-center bg-gray-100 shadow-md rounded-xl p-10 mt-10 border border-gray-300 text-center"
            >
                <Icon
                    icon="mdi-light:account-group"
                    width="5rem"
                    height="5rem"
                    class="text-gray-500 mb-6"
                />
                <h2 class="text-3xl font-semibold text-gray-900 mb-3">
                    No Groups Found
                </h2>
                <p class="text-gray-700 mb-8 max-w-lg">
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
import { storeToRefs } from "pinia";

import { useRouter } from "vue-router";
import { useGroupStore } from "@stores/group-store";

import { Icon } from "@iconify/vue";

import { Button } from "@components/ui/button";

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
