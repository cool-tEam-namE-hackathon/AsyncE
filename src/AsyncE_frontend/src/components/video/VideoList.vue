<template>
    <div>video</div>
    <!-- <base-dialog
        :open="isPreviewOpen"
        class="md:min-w-[800px] md:min-h-[450px] sm:min-w-[90vw] sm:min-h-[50vh]"
        @on-close-dialog="closePreviewDialog"
    >
        <template #title>
            <span>{{ selectedVideo?.title }}</span>
        </template>

        <template #content>
            <div class="relative group" @click="toggleVideo">
                <div
                    class="absolute rounded-md w-full h-full inset-0 flex justify-center items-center bg-black bg-opacity-50 opacity-0 group-hover:opacity-100 transition-opacity duration-300 cursor-pointer"
                >
                    <Icon
                        v-if="!isVideoPlaying"
                        icon="line-md:play-filled"
                        width="48"
                        height="48"
                        class="text-white cursor-pointer"
                    />
                    <Icon
                        v-else
                        icon="ic:round-pause"
                        width="48"
                        height="48"
                        class="text-white cursor-pointer"
                    />
                </div>
                <video
                    ref="previewVideoRef"
                    :src="videoUrl"
                    class="rounded-md w-full h-full"
                    muted
                />
            </div>
        </template>
    </base-dialog>

    <ScrollArea class="border rounded-md w-full">
        <div class="flex p-4 space-x-4 w-max">
            <div
                v-for="video in videosList"
                :key="video.url"
                class="flex flex-col gap-3"
            >
                <span>{{ video.video.title }}</span>
                <div
                    class="relative group cursor-pointer"
                    @click="previewVideo(video.video, video.url)"
                >
                    <video
                        :src="video.url"
                        class="w-64 h-36 object-cover rounded-md"
                    >
                        Your browser does not support the video tag.
                    </video>
                    <div
                        class="absolute z-50 inset-0 flex justify-center items-center bg-black bg-opacity-50 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                    >
                        <Icon
                            icon="line-md:play-filled"
                            width="48"
                            height="48"
                            class="text-white"
                        />
                    </div>
                </div>
            </div>

            <div v-if="isFetchingVideos" class="w-full">
                <div class="flex flex-wrap gap-4">
                    <div v-for="i in 5" :key="i" class="flex flex-col gap-3">
                        <div
                            class="w-32 h-5 bg-gray-200 rounded animate-pulse"
                        ></div>
                        <div
                            class="w-64 h-36 bg-gray-200 rounded animate-pulse"
                        ></div>
                    </div>
                </div>
            </div>

            <div
                v-if="!videosList.length && !isFetchingVideos"
                class="flex justify-center items-center w-full h-36"
            >
                <p class="text-gray-400 text-center flex-grow">
                    It looks like there are no videos in your group yet. Start
                    making some to see them appear here!
                </p>
            </div>
        </div>
        <ScrollBar orientation="horizontal" />
    </ScrollArea> -->

    <!-- <video v-if="meetingVideo" autoplay muted controls>
        <source :src="meetingVideo" type="video/mp4" />
        Your browser does not support the video tag.
    </video> -->
    <div v-for="thumbnail in videoThumbnail" :key="thumbnail">
        <img :src="thumbnail" alt="asdads">

    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

import { storeToRefs } from "pinia";

import { useRoute } from "vue-router";

import { useGroupStore } from "@stores/group-store";

// import { Icon } from "@iconify/vue";

// import BaseDialog from "@shared/BaseDialog.vue";

// import { ScrollArea, ScrollBar } from "@ui/scroll-area";

const route = useRoute();
const groupStore = useGroupStore();

const { videosList, videoThumbnail, meetingVideo } = storeToRefs(groupStore);

// const previewVideoRef = ref<HTMLVideoElement | null>(null);
// const videoUrl = ref<string>("");
// const isPreviewOpen = ref<boolean>(false);
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

// function previewVideo(video: VideoHeader, url: string) {
//     selectedVideo.value = video;
//     videoUrl.value = url;

//     isPreviewOpen.value = true;
// }

// function closePreviewDialog() {
//     isVideoPlaying.value = false;
//     isPreviewOpen.value = !isPreviewOpen.value;
// }

async function getAllVideos() {
    // isFetchingVideos.value = true;
    try {
        await groupStore.getAllThumbnails(route.params.groupId as string);
        console.log("here");
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
    console.log("here 2");

    getAllVideos();
    getMeetingVideo();
}
init();
</script>
