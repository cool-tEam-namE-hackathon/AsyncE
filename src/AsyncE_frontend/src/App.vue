<template>
    <div v-if="!isInitialized">
        <base-spinner />
    </div>
    <div v-else class="flex h-dvh flex-col">
        <Navbar />
        <router-view v-slot="{ Component }">
            <Suspense timeout="0">
                <component :is="Component" />
                <template #fallback>
                    <base-spinner />
                </template>
            </Suspense>
        </router-view>
        <Footer />

        <!-- REGISTER -->
        <base-dialog
            :open="isOpen"
            :hide-close-button="true"
            :is-closable="false"
        >
            <template #title> Input your username </template>

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
                        <Label>Profile picture</Label>
                        <Input
                            type="file"
                            accept="image/*"
                            @on-file-change="onFileInput"
                        />
                    </div>
                </div>
            </template>

            <template #footer>
                <Button
                    :disabled="isLoading"
                    :is-loading="isLoading"
                    @click="register"
                >
                    <template #default> Register </template>

                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-black"
                        />
                        Registering...
                    </template>
                </Button>
            </template>
        </base-dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";

import { storeToRefs } from "pinia";

import { Icon } from "@iconify/vue";

import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";

import Footer from "@components/layout/Footer.vue";
import Navbar from "@components/layout/Navbar.vue";
import BaseDialog from "@components/shared/BaseDialog.vue";
import BaseSpinner from "@components/shared/BaseSpinner.vue";
import Button from "@components/ui/button/Button.vue";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";

import { User } from "./types/api/model";
import { fileToBlob } from "./utils/helpers";

const userStore = useUserStore();
const websocketStore = useWebsocketStore();
const { isAuthenticated } = storeToRefs(userStore);

const isOpen = ref<boolean>(false);
const isInitialized = ref<boolean>(false);
const isLoading = ref<boolean>(false);
const error = ref<string>("");

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
        isOpen.value = false;
        window.location.reload();
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
        imageBlob.value = fileToBlob(file);
    }
}

async function init() {
    await userStore.init();

    try {
        await websocketStore.setWebsockets();
        const hasCredentials = await userStore.getUserCredentials();

        if (!hasCredentials && isAuthenticated) isOpen.value = true;
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isInitialized.value = true;
    }
}
init();
</script>
