<template>
    <base-dialog
        :open="open"
        :isClosable="false"
        :hideCloseButton="true"
        class="w-full max-w-md rounded-lg shadow-xl"
        @on-close-dialog="emit('on-close-dialog')"
    >
        <template #title>
            <h2 class="mb-2 text-xl font-medium">Upload Your Video</h2>
        </template>

        <template #content>
            <video v-if="url" autoplay muted controls class="w-full">
                <source :src="url" />
                Your browser does not support the video tag.
            </video>
            <div
                v-else
                class="flex w-full flex-col items-center justify-center"
            >
                <Icon
                    icon="prime:spinner"
                    width="64"
                    height="64"
                    class="mb-4 animate-spin text-black"
                />
                <p class="text-sm text-gray-600">
                    Please wait while the video is loading...
                </p>
            </div>

            <div class="space-y-6">
                <div>
                    <Label
                        for="videoTitle"
                        class="mb-1 block text-sm font-medium"
                    >
                        Video Title<span class="text-red-500"> *</span>
                    </Label>
                    <Input
                        id="videoTitle"
                        v-model="videoTitle"
                        class="w-full rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="Enter video title"
                    />
                </div>
                <div class="flex items-center justify-between">
                    <Label
                        class="text-sm font-medium"
                        :class="{
                            'text-gray-700':
                                !userCredentials?.subscription?.length,
                        }"
                    >
                        Generate Subtitles
                    </Label>

                    <base-tooltip
                        :text="
                            userCredentials?.subscription?.length
                                ? 'You need to subscribe to use this feature'
                                : ''
                        "
                        :delay="200"
                    >
                        <template #trigger>
                            <Switch
                                v-model:checked="generateSubtitle"
                                :disabled="
                                    !userCredentials?.subscription.length
                                "
                            />
                        </template>
                    </base-tooltip>
                </div>
                <div v-show="generateSubtitle" class="text-sm text-gray-500">
                    Generating subtitles will take some time depending on the
                    video size.
                </div>
            </div>
        </template>

        <template #footer>
            <div class="flex justify-end space-x-3">
                <Button variant="secondary" @click="emit('on-close-dialog')">
                    Cancel
                </Button>
                <Button
                    :disabled="isUploading || !videoTitle"
                    :is-loading="isUploading"
                    @click="saveRecording"
                >
                    <template #default> Upload </template>
                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-white"
                        />
                        Uploading... {{ uploadVideoProgress.toFixed(0) }}%
                    </template>
                </Button>
            </div>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import { Icon } from "@iconify/vue";
import { useToast } from "@ui/toast";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import BaseDialog from "@components/shared/BaseDialog.vue";
import BaseTooltip from "@components/shared/BaseTooltip.vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";
import Label from "@components/ui/label/Label.vue";
import Switch from "@components/ui/switch/Switch.vue";

const props = defineProps<{
    open: boolean;
    url: string;
    recordedChunks: Blob[];
}>();

const emit = defineEmits<{
    (e: "on-close-dialog"): void;
}>();

const { userCredentials } = storeToRefs(useUserStore());
const { uploadVideoProgress } = storeToRefs(useGroupStore());

const route = useRoute();
const groupStore = useGroupStore();
const { toast } = useToast();

const isUploading = ref<boolean>(false);
const generateSubtitle = ref<boolean>(false);
const videoTitle = ref<string>("");

const videoMimeType = MediaRecorder.isTypeSupported(
    'video/webm; codecs="vp8, opus"',
)
    ? 'video/webm; codecs="vp8, opus"'
    : "video/webm";

async function saveRecording() {
    isUploading.value = true;

    const blob = new Blob(props.recordedChunks, {
        type: videoMimeType,
    });

    const data = new Uint8Array(await blob.arrayBuffer());

    try {
        await groupStore.uploadVideo(
            data,
            route.params.groupId as string,
            route.params.meetingId as string,
            videoTitle.value,
            generateSubtitle.value,
        );
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isUploading.value = false;
        videoTitle.value = "";
        emit("on-close-dialog");
    }

    toast({
        title: "You video is being processed",
        description: "Once finished, your video will appear on the video list",
    });
}
</script>
