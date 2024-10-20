<template>
    <base-dialog :open="dialogValue" @on-close-dialog="closePreviewDialog">
        <template #title>
            <span>{{ video.title }}</span>
        </template>

        <template #content>
            <video :src="url" class="rounded-md" autoplay />
        </template>
    </base-dialog>
</template>
x

<script setup lang="ts">
import { computed } from "vue";

import { VideoHeader } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import BaseDialog from "@shared/BaseDialog.vue";

const props = defineProps<{
    isOpen: boolean;
    video: VideoHeader;
    url: string;
}>();
const emits = defineEmits<{
    (e: "on-close-dialog", payload: boolean): void;
}>();

const dialogValue = computed({
    get: () => props.isOpen,
    set: (newVal) => emits("on-close-dialog", newVal),
});

function closePreviewDialog(value) {
    console.log(value);
}
</script>
