<template>
    <label for="">Group name</label>
    <input v-model="groupName" type="text" />
    <label for="">Group profile picture</label>
    <input type="file" accept="image/*" @change="onFileInput" />
    <button @click="createGroup">Create Group</button>
</template>

<script setup lang="ts">
import { ref } from "vue";

import { fileToBlob } from "@/utils/helpers";
import { useUserStore } from "@/stores/user-store";

const userStore = useUserStore();

const groupName = ref<string>("");
const groupPicture = ref<Blob | null>();

function onFileInput(e) {
    const file = e.target.files[0];
    groupPicture.value = fileToBlob(file);
}

async function createGroup() {
    if (!groupPicture.value) {
        return;
    }

    const payload = {
        name: groupName.value,
        picture: new Uint8Array(await groupPicture.value?.arrayBuffer()),
    };

    await userStore.createGroup(payload);
}
</script>
