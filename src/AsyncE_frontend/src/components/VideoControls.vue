<template>
    <div class="flex gap-3">
        <base-select
            class="w-[100px]"
            v-model="selectedCamera"
            placeholder="Select camera"
            :options="cameraList"
        />
        <Button
            :variant="enabledCamera ? 'default' : 'outline'"
            :disabled="!selectedCamera"
            @click="emits('on-toggle-camera')"
        >
            <Icon class="mr-2" icon="lucide:camera" width="24" height="24" />
            Camera
        </Button>
        <Button
            :variant="enabledScreen ? 'default' : 'outline'"
            @click="emits('on-toggle-screen')"
        >
            <Icon class="mr-2" icon="gg:screen" width="24" height="24" />
            Screen
        </Button>
        <Button :disabled="isRecordingDisabled" @click="emits('on-record')">
            <Icon
                :class="[
                    recordingPhaseText === 'Stop'
                        ? 'text-red-500 animate-pulse'
                        : '',
                    'mr-2',
                ]"
                icon="fluent:record-20-regular"
                width="24"
                height="24"
            />
            {{ recordingPhaseText }}
        </Button>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { VideoControlProps } from "@/types/api/model";

import { Icon } from "@iconify/vue";

import { Button } from "@ui/button";

import BaseSelect from "@shared/BaseSelect.vue";

const props = defineProps<VideoControlProps>();
const emits = defineEmits<{
    (e: "update:selectedCamera", payload: string): void;
    (e: "on-toggle-camera"): void;
    (e: "on-toggle-screen"): void;
    (e: "on-record"): void;
}>();

const selectedCamera = computed({
    get: () => props.selectedCamera,
    set: (newVal) => emits("update:selectedCamera", newVal!),
});
</script>
