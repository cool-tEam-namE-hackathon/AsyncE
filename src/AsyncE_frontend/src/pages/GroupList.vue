<template>
    <div class="flex-1 py-8">
        <div class="container mx-auto px-4">
            <div class="flex justify-between items-center mb-10">
                <h1 class="text-2xl font-bold text-gray-900">Your Groups</h1>
                <router-link to="/create-group">
                    <Button>
                        <Icon
                            class="mr-2"
                            icon="mdi-light:plus-circle"
                            width="1.2rem"
                            height="1.2rem"
                        />
                        Create New Group
                    </Button>
                </router-link>
            </div>

            <div class="grid grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                <div
                    v-for="(group, index) in groupList"
                    :key="index"
                    class="flex flex-col bg-white shadow-md rounded-lg overflow-hidden"
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
                            {{ group.users.length }} members
                        </p>
                        <div class="flex justify-between items-center">
                            <Button> View Group </Button>
                            <Button variant="ghost" size="sm">
                                <Settings class="h-4 w-4" />
                            </Button>
                        </div>
                    </div>
                </div>
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
    await groupStore.getAllGroups();
    console.log(groupList.value);
}

async function init() {
    await getAllGroups();
}

await init();
</script>
