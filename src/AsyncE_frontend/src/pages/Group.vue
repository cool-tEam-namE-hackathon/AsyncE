<template>
    <div class="container m-auto px-4">
        <div class="bg-white shadow-md rounded-lg p-6 w-full max-w-2xl mx-auto">
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900">
                    Create a New Group
                </h1>
                <p class="text-sm text-gray-500">
                    Set up a group for your asynchronous meetings
                </p>
            </div>
            <div class="space-y-4">
                <div class="space-y-2">
                    <Label>Group Name</Label>
                    <Input v-model="groupName" placeholder="Enter group name" />
                </div>
                <div class="space-y-2">
                    <Label>Group Image</Label>
                    <div class="flex items-center space-x-2">
                        <Input
                            type="file"
                            accept="image/*"
                            @on-file-change="onFileInput"
                        />
                    </div>
                </div>
            </div>
            <div class="mt-6">
                <Button
                    class="w-full"
                    @click="createGroup"
                    :disabled="isFormValid"
                >
                    Create Group
                </Button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";

import { fileToBlob } from "@/utils/helpers";

import { useGroupStore } from "@stores/group-store";

import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import Button from "@components/ui/button/Button.vue";
import { useRouter } from "vue-router";

const router = useRouter();
const groupStore = useGroupStore();

const groupName = ref<string>("");
const groupPicture = ref<Blob | null>();

const isFormValid = computed(() => {
    return !(groupName.value && groupPicture.value?.size);
});

function onFileInput(e: Event) {
    const file = (e.target as HTMLInputElement)?.files?.[0];

    if (file) {
        groupPicture.value = fileToBlob(file);
    }
}

async function createGroup() {
    if (!groupPicture.value) {
        return;
    }

    const payload = {
        name: "",
        picture: new Uint8Array(await groupPicture.value?.arrayBuffer()),
    };

    try {
        await groupStore.createGroup(payload);
        router.push("/group-list");
    } catch (e) {
        console.log(e);
    }
}
</script>
