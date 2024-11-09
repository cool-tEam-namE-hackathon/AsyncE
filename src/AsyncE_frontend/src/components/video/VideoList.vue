<template>
    <div class="container">
        <!-- SINGLE VIDEO -->
        <view-video-dialog
            :open="isViewVideoOpen"
            :current-video="currentVideo!"
            @on-close-dialog="toggleViewVideoModal"
        />

        <!-- COMBINED VIDEO -->
        <combined-video-dialog
            :open="isCombinedVideoOpen"
            @on-close-dialog="toggleCombinedVideoModal"
        />

        <div
            class="mb-4 flex items-center justify-between rounded-lg bg-white p-4 shadow-md"
        >
            <h2 class="text-lg font-semibold">Video List</h2>
            <Button
                :disabled="!localVideoThumbnail.length"
                @click="toggleCombinedVideoModal"
            >
                View Combined Video
            </Button>
        </div>

        <ScrollArea class="mb-8 w-full rounded-md border shadow-md">
            <div class="flex w-full space-x-4 p-4">
                <div
                    v-for="(thumbnail, index) in localVideoThumbnail"
                    ref="thumbnailsRef"
                    :key="thumbnail"
                    class="flex gap-3"
                >
                    <div
                        class="cursor-pointer"
                        @click="toggleViewVideoModal(index)"
                    >
                        <div class="h-36 w-64">
                            <img
                                :src="thumbnail"
                                alt="thumbnail"
                                class="h-full w-full rounded-md object-cover"
                            />
                        </div>
                    </div>

                    <div
                        v-if="index !== localVideoThumbnail.length - 1"
                        class="h-full w-px bg-gray-300"
                    ></div>
                </div>

                <!-- SKELETON PLACEHOLDER -->
                <div v-if="isFetchingThumbnails" class="flex gap-3">
                    <div
                        v-for="index in 5"
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
                    v-if="!localVideoThumbnail.length && !isFetchingThumbnails"
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
import { ref, nextTick } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import { Icon } from "@iconify/vue";
import { Button } from "@ui/button";
import { ScrollArea, ScrollBar } from "@ui/scroll-area";
import { useGroupStore } from "@stores/group-store";
import { useWebsocketStore } from "@stores/websocket-store";
import CombinedVideoDialog from "@components/meeting/CombinedVideoDialog.vue";
import ViewVideoDialog from "@components/meeting/ViewVideoDialog.vue";
import { Thumbnail, VideoFrameHeader } from "@/types/api/model";

const route = useRoute();
const groupStore = useGroupStore();
const websocketStore = useWebsocketStore();

const { selectedVideo } = storeToRefs(groupStore);

const thumbnailsRef = ref<HTMLDivElement[]>([]);

const currentVideo = ref<VideoFrameHeader>();

const isViewVideoOpen = ref<boolean>(false);
const isCombinedVideoOpen = ref<boolean>(false);
const isFetchingThumbnails = ref<boolean>(false);

const localVideoThumbnail = ref<string[]>([]);

async function toggleViewVideoModal(index: number = -1) {
    isViewVideoOpen.value = !isViewVideoOpen.value;

    if (isViewVideoOpen.value) {
        await getVideo(index);
    }
}

function toggleCombinedVideoModal() {
    getMeetingVideo();
    isCombinedVideoOpen.value = !isCombinedVideoOpen.value;
}

async function scrollToView() {
    if (!thumbnailsRef.value) return;

    await nextTick();

    thumbnailsRef.value[localVideoThumbnail.value.length - 1]?.scrollIntoView({
        behavior: "smooth",
    });
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
    isFetchingThumbnails.value = true;

    try {
        await groupStore.getAllThumbnails(
            route.params.groupId as string,
            (thumbnail) => {
                localVideoThumbnail.value.push(thumbnail);
            },
        );
        scrollToView();
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isFetchingThumbnails.value = false;
    }
}

async function getMeetingVideo() {
    try {
        await groupStore.getMeetingVideo(
            route.params.groupId as string,
            route.params.meetingId as string,
        );
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function handleThumbnailAvailable(thumbnail: Thumbnail) {
    try {
        await groupStore.getSpecificThumbnail(
            route.params.groupId as string,
            route.params.meetingId as string,
            thumbnail.frame_index,
            (t) => {
                localVideoThumbnail.value.push(t);
            },
        );
    } catch (e) {
        console.log((e as Error).message);
    }
}

websocketStore.setOnThumbnailAvailable(handleThumbnailAvailable);

function init() {
    getAllThumbnails();
    getMeetingVideo();
}
init();
</script>
