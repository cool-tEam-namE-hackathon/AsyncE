<template>
    <base-dialog
        :open="open"
        :is-closable="true"
        @on-close-dialog="emit('on-close-dialog')"
    >
        <template #title> Create Meeting </template>

        <template #content>
            <Input v-model="meetingName" class="w-full" />
        </template>

        <template #footer>
            <Button
                :disabled="!meetingName || isLoading"
                :is-loading="isLoading"
                class="w-full sm:w-auto"
                @click="createMeeting"
            >
                <template #default> Create Meeting </template>
                <template #loading>
                    <Icon
                        icon="prime:spinner"
                        width="16"
                        height="16"
                        class="mr-1 animate-spin text-white"
                    />
                    Creating...
                </template>
            </Button>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import BaseDialog from "@components/shared/BaseDialog.vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";

defineProps<{
    open: boolean;
}>();

const emit = defineEmits<{
    (e: "on-close-dialog"): void;
    (e: "on-create-meeting"): void;
}>();

const route = useRoute();
const groupStore = useGroupStore();

const meetingName = ref<string>("");
const isLoading = ref<boolean>(false);

async function createMeeting() {
    isLoading.value = true;
    try {
        await groupStore.createMeeting(
            route.params.id as string,
            meetingName.value,
        );

        emit("on-create-meeting");
        emit("on-close-dialog");
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isLoading.value = false;
    }
}
</script>
