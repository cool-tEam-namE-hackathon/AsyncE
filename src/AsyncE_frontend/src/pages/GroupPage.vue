<template>
    <div></div>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { useGroupStore } from "@stores/group-store";

const route = useRoute();
const groupStore = useGroupStore();

async function fetchGroupDetails() {
    const { id } = route.params;
    const groupId = BigInt(id as string);

    const response = await groupStore.getGroup(groupId);

    console.log(response);
}

async function init() {
    await fetchGroupDetails();
}

await init();
</script>
