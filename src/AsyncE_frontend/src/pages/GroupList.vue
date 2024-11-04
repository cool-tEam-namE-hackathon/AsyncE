<template>
    <div class="flex-1 py-8">
        <div class="container mx-auto px-4">
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
                    class="flex flex-col overflow-hidden rounded-lg bg-white shadow-md"
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
                            {{ group.users.length }} members
                        </p>
                        <div class="flex items-center justify-between">
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
