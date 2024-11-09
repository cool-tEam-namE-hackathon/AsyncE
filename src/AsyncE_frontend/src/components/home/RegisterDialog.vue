<template>
    <base-dialog :open="open" :hide-close-button="true" :is-closable="false">
        <template #title>
            Input your username<span class="text-red-500"> *</span>
        </template>

        <template #description> Username must be unique </template>

        <template #content>
            <div class="space-y-3">
                <div class="space-y-2">
                    <Label
                        :class="{
                            'text-red-500': error,
                        }"
                    >
                        Username
                        <span class="text-red-500"> *</span>
                    </Label>
                    <Input
                        v-model="username"
                        :class="{
                            'border-red-400': error,
                            'focus-visible:ring-0': true,
                        }"
                    />
                    <div v-if="error" class="mt-2 text-sm text-red-500">
                        {{ error }}
                    </div>
                </div>

                <div class="space-y-2">
                    <Label
                        >Profile picture<span class="text-red-500">
                            *</span
                        ></Label
                    >
                    <Input
                        type="file"
                        accept="image/*"
                        @on-file-change="onFileInput"
                    />
                </div>

                <template v-if="!imageUrl">
                    <img
                        src="/images/placeholder.webp"
                        class="max-h-80 w-full rounded-md object-cover"
                        alt="placeholder"
                    />
                </template>

                <template v-else>
                    <img
                        :src="imageUrl"
                        class="w-full rounded-md object-contain"
                        alt="placeholder"
                    />
                </template>
            </div>
        </template>

        <template #footer>
            <Button
                :disabled="isLoading || isFormValid"
                :is-loading="isLoading"
                @click="register"
            >
                <template #default> Register </template>

                <template #loading>
                    <Icon
                        icon="prime:spinner"
                        width="16"
                        height="16"
                        class="mr-1 animate-spin text-white"
                    />
                    Registering...
                </template>
            </Button>
        </template>
    </base-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { Icon } from "@iconify/vue";
import { useUserStore } from "@stores/user-store";
import BaseDialog from "@components/shared/BaseDialog.vue";
import Button from "@components/ui/button/Button.vue";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import { User } from "@/types/api/model";
import { fileToBlob } from "@/utils/helpers";

defineProps<{
    open: boolean;
}>();

const emit = defineEmits<{
    (e: "on-close-dialog"): void;
}>();

const userStore = useUserStore();

const isLoading = ref<boolean>(false);
const error = ref<string>("");
const imageUrl = ref<string>("");

const username = ref<string>("");
const imageBlob = ref<Blob | null>(null);

const isFormValid = computed(() => {
    return !(username.value && imageBlob.value?.size);
});

async function register() {
    if (!imageBlob.value) {
        return;
    }

    isLoading.value = true;

    const payload: User = {
        username: username.value,
        profile_picture_blob: new Uint8Array(
            await imageBlob.value.arrayBuffer(),
        ),
    };

    try {
        await userStore.register(payload);
        window.location.reload();
        emit("on-close-dialog");
    } catch (e) {
        const err = e as Error;
        error.value = err.message;
    } finally {
        isLoading.value = false;
    }
}

function onFileInput(e: Event) {
    const file = (e.target as HTMLInputElement)?.files?.[0];

    if (file) {
        imageUrl.value = URL.createObjectURL(file);
        imageBlob.value = fileToBlob(file);
    }
}
</script>
