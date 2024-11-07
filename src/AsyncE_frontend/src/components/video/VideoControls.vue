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
            @click="emits('on-toggle-camera', enabledCamera)"
        >
            <template #default>
                <div class="flex items-center">
                    <Icon
                        class="mr-2"
                        icon="lucide:camera"
                        width="24"
                        height="24"
                    />
                    Camera
                </div>
            </template>
        </Button>

        <Button
            :variant="enabledScreen ? 'default' : 'outline'"
            @click="emits('on-toggle-screen', enabledScreen)"
        >
            <template #default>
                <div class="flex items-center">
                    <Icon
                        class="mr-2"
                        icon="lucide:camera"
                        width="24"
                        height="24"
                    />
                    Screen
                </div>
            </template>
        </Button>
        <Button :disabled="isRecordingDisabled" @click="emits('on-record')">
            <template #default>
                <Icon
                    :class="[
                        recordingPhaseText === 'Stop'
                            ? 'animate-pulse text-red-500'
                            : '',
                        'mr-2',
                    ]"
                    icon="fluent:record-20-regular"
                    width="24"
                    height="24"
                />
                {{ recordingPhaseText }}
            </template>
        </Button>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Icon } from "@iconify/vue";
import BaseSelect from "@shared/BaseSelect.vue";
import { Button } from "@ui/button";
import { VideoControlProps } from "@/types/api/model";

const props = defineProps<VideoControlProps>();
const emits = defineEmits<{
    (e: "update:modelValue", payload: string): void;
    (e: "on-toggle-camera", payload: boolean): void;
    (e: "on-toggle-screen", payload: boolean): void;
    (e: "on-record"): void;
}>();

const selectedCamera = ref<string>(props.modelValue);
</script>
