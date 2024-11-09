<template>
    <base-dialog
        :open="open"
        :is-closable="true"
        @on-close-dialog="
            invitedUsername = '';
            emit('on-close-dialog');
        "
    >
        <template #title> Invite user </template>

        <template #description>
            Enter the username of the user you'd like to invite.
        </template>

        <template #content>
            <div class="space-y-2">
                <div class="flex flex-col items-center gap-3 sm:flex-row">
                    <Label>Username</Label>
                    <div class="relative w-full flex-1 sm:w-auto">
                        <Input
                            v-model="invitedUsername"
                            class="w-full"
                            :class="{
                                'border-red-400': showRed,
                                'border-green-700':
                                    !isFieldError && invitedUsername,
                                'focus-visible:ring-0': true,
                            }"
                            @update:model-value="validateUsername"
                        />
                        <Icon
                            :icon="
                                showRed
                                    ? 'gridicons:cross-circle'
                                    : 'ep:success-filled'
                            "
                            width="24"
                            height="24"
                            class="absolute right-2 top-1/2 -translate-y-1/2 transform"
                            :class="{
                                'text-red-700': showRed,
                                'text-green-700':
                                    !isFieldError && invitedUsername,
                                hidden: !invitedUsername,
                            }"
                        />
                    </div>
                </div>
                <div v-if="inviteError" class="text-sm text-red-500">
                    {{ inviteError }}
                </div>
            </div>
        </template>

        <template #footer>
            <Button
                :disabled="isFieldError || isLoading || !invitedUsername"
                :is-loading="isLoading"
                class="w-full sm:w-auto"
                @click="handleInvite"
            >
                <template #default> Invite </template>
                <template #loading>
                    <Icon
                        icon="prime:spinner"
                        width="16"
                        height="16"
                        class="mr-1 animate-spin text-white"
                    />
                    Inviting...
                </template>
            </Button>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRoute } from "vue-router";
import { useDebounceFn } from "@vueuse/core";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
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
const userStore = useUserStore();
const groupStore = useGroupStore();

const isError = ref<boolean>(false);
const isLoading = ref<boolean>(false);

const inviteError = ref<string>("");
const invitedUsername = ref<string>("");
const inputtedUsername = ref<string>("");

const isFieldError = computed(() => {
    return !isError.value && inputtedUsername.value;
});

const showRed = computed(
    () =>
        isFieldError.value ||
        (invitedUsername.value && invitedUsername.value.length < 3),
);

const validateUsername = useDebounceFn(async (payload) => {
    try {
        isError.value = await userStore.validateUsername(payload);
        inputtedUsername.value = payload;
    } catch (e) {
        console.log((e as Error).message);
    }
}, 500);

async function handleInvite() {
    isLoading.value = true;
    try {
        await groupStore.inviteUser(
            BigInt(route.params.id as string),
            inputtedUsername.value,
        );
        emit("on-close-dialog");
    } catch (e) {
        inviteError.value = (e as Error).message;
    } finally {
        isLoading.value = false;
    }
}
</script>
