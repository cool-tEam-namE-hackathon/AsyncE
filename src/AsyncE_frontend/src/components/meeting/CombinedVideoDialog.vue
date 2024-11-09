<template>
    <base-dialog
        :open="open"
        :is-closable="true"
        :class="isVideoNotEmpty ? 'sm:min-h-fit' : 'sm:min-h-[50vh]'"
        class="sm:min-w-[90vw] md:min-w-[80vw]"
        @on-close-dialog="emit('on-close-dialog')"
    >
        <template #content>
            <div v-if="isVideoNotEmpty">
                <video
                    v-if="isVideoNotEmpty"
                    :src="meetingVideo"
                    class="mt-6 rounded-md"
                    autoplay
                    controls
                />
            </div>
            <div v-else class="flex flex-col items-center justify-center">
                <Icon
                    icon="prime:spinner"
                    width="64"
                    height="64"
                    class="mb-4 animate-spin text-black"
                />
                <p class="text-lg text-gray-600">
                    Videos are being concatenated. You can close this modal and
                    check back later.
                </p>
            </div>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { storeToRefs } from "pinia";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import BaseDialog from "@components/shared/BaseDialog.vue";

defineProps<{
    open: boolean;
}>();

const emit = defineEmits<{
    (e: "on-close-dialog"): void;
}>();

const { meetingVideo } = storeToRefs(useGroupStore());

const isVideoNotEmpty = computed(() => meetingVideo.value.length > 0);
</script>
