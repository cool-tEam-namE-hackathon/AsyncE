<template>
    <div class="container mx-auto px-4 py-8">
        <div class="mb-12 flex items-center justify-between">
            <h1 class="text-3xl font-bold text-gray-900">Your Groups</h1>
            <router-link
                to="/create-group"
                class="inline-flex items-center rounded-full bg-black px-6 py-2.5 text-sm font-medium text-white transition-colors hover:bg-gray-800"
            >
                <Icon
                    icon="mdi-light:plus-circle"
                    class="mr-2"
                    width="1.2rem"
                    height="1.2rem"
                />
                Create New Group
            </router-link>
        </div>

        <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            <div
                v-for="(group, index) in groupList"
                :key="index"
                class="group cursor-pointer overflow-hidden rounded-2xl border-[1px] border-solid bg-white transition-all hover:shadow-lg"
                @click="router.push(`/group/${group.id}`)"
            >
                <div class="aspect-video overflow-hidden">
                    <img
                        :src="blobToURL(group.profile_picture_blob)"
                        class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                    />
                </div>
                <div class="p-6">
                    <h2 class="mb-1 text-xl font-semibold text-gray-900">
                        {{ group.name }}
                    </h2>
                    <p class="mb-4 text-sm text-gray-600">
                        {{ group.members.length }} member(s)
                    </p>
                    <div
                        class="inline-flex items-center text-sm font-medium text-gray-900"
                    >
                        View Group
                        <Icon
                            icon="mdi-light:arrow-right"
                            class="ml-2 transition-transform group-hover:translate-x-1"
                            width="1rem"
                            height="1rem"
                        />
                    </div>
                </div>
            </div>
        </div>

        <div
            v-if="!groupList.length"
            class="mt-12 flex flex-col items-center rounded-2xl bg-white p-12 text-center shadow-md"
        >
            <Icon
                icon="ph:empty-bold"
                width="64"
                height="64"
                class="text-black"
            />
            <h2 class="my-3 text-2xl font-semibold text-gray-900">
                No Groups Found
            </h2>
            <p class="mb-8 max-w-md text-gray-600">
                Looks like you haven't created or joined any groups yet. Start
                your journey by creating a new group and connecting with others!
            </p>
            <router-link
                to="/create-group"
                class="inline-flex items-center rounded-full bg-black px-6 py-2.5 text-sm font-medium text-white transition-colors hover:bg-gray-800"
            >
                <Icon
                    icon="mdi-light:plus-circle"
                    class="mr-2"
                    width="1.2rem"
                    height="1.2rem"
                />
                Create New Group
            </router-link>
        </div>
    </div>
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
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
