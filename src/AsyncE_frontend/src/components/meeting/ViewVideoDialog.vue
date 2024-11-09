<template>
    <base-dialog
        :open="open"
        :is-closable="true"
        :class="isSelectedVideoNotEmpty ? 'sm:min-h-fit' : 'sm:min-h-[50vh]'"
        class="sm:min-w-[90vw] md:min-w-[80vw]"
        @on-close-dialog="emit('on-close-dialog')"
    >
        <template v-if="isSelectedVideoNotEmpty" #title>
            <h2 class="text-xl font-bold text-gray-800">
                {{ currentVideo?.title }}
            </h2>
        </template>

        <template #content>
            <div v-if="isSelectedVideoNotEmpty">
                <!-- Metadata Section -->
                <div class="mb-1 space-y-2">
                    <div
                        class="flex items-center justify-between text-sm text-gray-600"
                    >
                        <p>
                            Created by:
                            <span class="font-medium text-gray-700">{{
                                currentVideo?.created_by
                            }}</span>
                        </p>

                        <p>
                            Created on:
                            <span class="font-medium text-gray-700">
                                {{
                                    convertDateToReadableFormat(
                                        currentVideo?.created_time_unix,
                                    )
                                }}
                            </span>
                        </p>
                    </div>
                </div>

                <!-- Video Section -->
                <video
                    ref="previewVideoRef"
                    :src="currentVideo?.url"
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
                    Please wait while the video is loading...
                </p>
            </div>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Icon } from "@iconify/vue";
import BaseDialog from "@components/shared/BaseDialog.vue";
import { VideoFrameHeader } from "@/types/api/model";
import { convertDateToReadableFormat } from "@/utils/helpers";

const props = defineProps<{
    open: boolean;
    currentVideo: VideoFrameHeader;
}>();

const emit = defineEmits<{
    (e: "on-close-dialog"): void;
}>();

const isSelectedVideoNotEmpty = computed(() => {
    if (!props.currentVideo) return false;
    return props.currentVideo.url.length > 0;
});
</script>
