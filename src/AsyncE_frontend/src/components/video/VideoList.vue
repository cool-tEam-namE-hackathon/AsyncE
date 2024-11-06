<template>
    <div class="container">
        <base-dialog
            :open="isPreviewOpen"
            :is-closable="true"
            class="sm:min-h-[50vh] sm:min-w-[90vw] md:min-h-[80vh] md:min-w-[80vw]"
            @on-close-dialog="togglePreviewModal"
        >
            <template #content>
                <div v-if="selectedVideo">
                    <video
                        ref="previewVideoRef"
                        :src="selectedVideo"
                        class="h-full w-full rounded-md"
                        autoplay
                        controls
                        muted
                    />
                </div>
                <Icon
                    v-else
                    icon="prime:spinner"
                    width="16"
                    height="16"
                    class="mr-1 animate-spin text-black"
                />
            </template>
        </base-dialog>

        <ScrollArea class="w-full rounded-md border">
            <div class="flex w-max space-x-4 p-4">
                <div
                    v-for="(thumbnail, index) in videoThumbnail"
                    :key="thumbnail"
                    class="flex flex-col gap-3"
                >
                    <div
                        class="cursor-pointer"
                        @click="togglePreviewModal(index)"
                    >
                        <img
                            :src="thumbnail"
                            alt="thumbnail"
                            class="h-36 w-64"
                        />
                    </div>
                </div>
                <!--
                <div v-if="isFetchingVideos" class="w-full">
                    <div class="flex flex-wrap gap-4">
                        <div v-for="i in 5" :key="i" class="flex flex-col gap-3">
                            <div
                                class="h-5 w-32 animate-pulse rounded bg-gray-200"
                            ></div>
                            <div
                                class="h-36 w-64 animate-pulse rounded bg-gray-200"
                            ></div>
                        </div>
                    </div>
                </div> -->
                <!-- <div
                    v-if="!videosList.length && !isFetchingVideos"
                    class="flex h-36 w-full items-center justify-center"
                >
                    <p class="flex-grow text-center text-gray-400">
                        It looks like there are no videos in your group yet. Start
                        making some to see them appear here!
                    </p>
                </div> -->
            </div>
            <ScrollBar orientation="horizontal" />
        </ScrollArea>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

import { storeToRefs } from "pinia";

import { useRoute } from "vue-router";

import BaseDialog from "@shared/BaseDialog.vue";
import { ScrollArea, ScrollBar } from "@ui/scroll-area";

import { useGroupStore } from "@stores/group-store";

// import { Icon } from "@iconify/vue";

const route = useRoute();
const groupStore = useGroupStore();

const { videoThumbnail, selectedVideo } = storeToRefs(groupStore);

// const previewVideoRef = ref<HTMLVideoElement | null>(null);
// const videoUrl = ref<string>("");
const isPreviewOpen = ref<boolean>(false);
// const isVideoPlaying = ref<boolean>(false);
const isFetchingVideos = ref<boolean>(false);

// defineExpose({
//     getAllVideos,
// });

// function toggleVideo() {
//     if (!previewVideoRef.value) return;

//     if (!isVideoPlaying.value) {
//         previewVideoRef.value.play();
//         isVideoPlaying.value = !isVideoPlaying.value;
//     } else {
//         previewVideoRef.value.pause();
//         isVideoPlaying.value = !isVideoPlaying.value;
//     }
// }

function togglePreviewModal(index: number = -1) {
    isPreviewOpen.value = !isPreviewOpen.value;

    if (isPreviewOpen.value) getVideo(index);
}

async function getVideo(index: number) {
    try {
        await groupStore.getVideo(
            route.params.groupId as string,
            route.params.meetingId as string,
            index,
        );
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function getAllThumbnails() {
    // isFetchingVideos.value = true;
    try {
        await groupStore.getAllThumbnails(route.params.groupId as string);
        console.log(videoThumbnail.value);
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        // isFetchingVideos.value = false;
    }
}

async function getMeetingVideo() {
    isFetchingVideos.value = true;
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
