<template>
    <div>group list</div>
    <img :src="groupPicture" alt="" />
</template>
<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useGroupStore } from "@stores/group-store";

const groupStore = useGroupStore();

const { groupPicture } = storeToRefs(groupStore);

async function getGroup() {
    const response = await groupStore.getAllGroups();
    console.log(response);
}

async function init() {
    await getGroup();
}

init();
</script>
