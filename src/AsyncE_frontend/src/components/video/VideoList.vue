<template>
    <div class="container">
        <!-- SINGLE VIDEO -->
        <base-dialog
            :open="isPreviewOpen"
            :is-closable="true"
            :class="
                isSelectedVideoNotEmpty ? 'sm:min-h-fit' : 'sm:min-h-[50vh]'
            "
            class="sm:min-w-[90vw] md:min-w-[80vw]"
            @on-close-dialog="togglePreviewModal"
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

        <!-- COMBINED VIDEO -->
        <base-dialog
            :open="isCombinedVideoOpen"
            :is-closable="true"
            :class="isVideoNotEmpty ? 'sm:min-h-fit' : 'sm:min-h-[50vh]'"
            class="sm:min-w-[90vw] md:min-w-[80vw]"
            @on-close-dialog="toggleCombinedVideoModal"
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
                        Videos are being concatenated. You can close this modal
                        and check back later.
                    </p>
                </div>
            </template>
        </base-dialog>

        <div
            class="mb-4 flex items-center justify-between rounded-lg bg-white p-4 shadow-md"
        >
            <h2 class="text-lg font-semibold">Video List</h2>
            <Button
                :disabled="!videoThumbnail.length"
                @click="toggleCombinedVideoModal"
            >
                View Combined Video
            </Button>
        </div>

        <ScrollArea class="mb-8 w-full rounded-md border shadow-md">
            <div class="flex w-full space-x-4 p-4">
                <div
                    v-for="(thumbnail, index) in videoThumbnail"
                    :key="thumbnail"
                    class="flex gap-3"
                >
                    <div
                        class="cursor-pointer"
                        @click="togglePreviewModal(index)"
                    >
                        <div class="h-36 w-64">
                            <img
                                :src="thumbnail"
                                alt="thumbnail"
                                class="h-full w-full object-cover"
                            />
                        </div>
                    </div>

                    <div
                        v-if="index !== videoThumbnail.length - 1"
                        class="h-full w-px bg-gray-300"
                    ></div>
                </div>

                <!-- SKELETON PLACEHOLDER -->
                <div v-if="isFetchingThumbnails" class="flex gap-3">
                    <div
                        v-for="index in 10"
                        :key="index"
                        class="flex items-center gap-3"
                    >
                        <div
                            class="h-36 w-64 animate-pulse rounded-md bg-gray-200"
                        ></div>

                        <div
                            v-if="index !== 5"
                            class="h-full w-px bg-gray-300"
                        ></div>
                    </div>
                </div>

                <div
                    v-if="!videoThumbnail.length && !isFetchingThumbnails"
                    class="flex w-full flex-col items-center justify-center rounded-lg border bg-gray-100 p-6"
                >
                    <Icon
                        icon="heroicons-outline:exclamation-circle"
                        class="mb-4 text-gray-400"
                        width="48"
                        height="48"
                    />
                    <p class="text-lg font-medium text-gray-700">
                        No video thumbnails found
                    </p>
                    <p class="text-sm text-gray-500">
                        Please upload a video or check back later.
                    </p>
                </div>
            </div>
            <ScrollBar orientation="horizontal" />
        </ScrollArea>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import { Icon } from "@iconify/vue";
import BaseDialog from "@shared/BaseDialog.vue";
import { Button } from "@ui/button";
import { ScrollArea, ScrollBar } from "@ui/scroll-area";
import { useGroupStore } from "@stores/group-store";
import { VideoFrameHeader } from "@/types/api/model";
import { convertDateToReadableFormat } from "@/utils/helpers";

const route = useRoute();
const groupStore = useGroupStore();

const { videoThumbnail, selectedVideo, meetingVideo } = storeToRefs(groupStore);

const currentVideo = ref<VideoFrameHeader>();

const isPreviewOpen = ref<boolean>(false);
const isCombinedVideoOpen = ref<boolean>(false);

const isFetchingThumbnails = ref<boolean>(false);

const isVideoNotEmpty = computed(() => meetingVideo.value.length > 0);
const isSelectedVideoNotEmpty = computed(() => {
    if (!currentVideo.value) return false;
    return currentVideo.value.url.length > 0;
});

function togglePreviewModal(index: number = -1) {
    isPreviewOpen.value = !isPreviewOpen.value;

    if (isPreviewOpen.value) {
        getVideo(index);
    }
}

function toggleCombinedVideoModal() {
    isCombinedVideoOpen.value = !isCombinedVideoOpen.value;
}

async function getVideo(index: number) {
    currentVideo.value = undefined;
    try {
        await groupStore.getVideo(
            route.params.groupId as string,
            route.params.meetingId as string,
            index,
        );

        currentVideo.value = selectedVideo.value.get(index);
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function getAllThumbnails() {
    console.log("fetch thumbnail");
    isFetchingThumbnails.value = true;
    try {
        await groupStore.getAllThumbnails(route.params.groupId as string);
        console.log(videoThumbnail.value);
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isFetchingThumbnails.value = false;
    }
}

async function getMeetingVideo() {
    // isFetchingVideos.value = true;
    try {
        await groupStore.getMeetingVideo(
            route.params.groupId as string,
            route.params.meetingId as string,
        );
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        // isFetchingVideos.value = false;
    }
}

function init() {
    getAllThumbnails();
    getMeetingVideo();
}
init();

defineExpose({
    getAllThumbnails,
});
</script>
