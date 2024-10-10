<template>
    <div v-if="isReady" class="h-dvh flex flex-col">
        <Navbar />
        <RouterView />
        <Footer />

        <base-dialog :open="isOpen" :hide-close-button="true">
            <template #title> Input your username </template>

            <template #description> Username must be unique </template>

            <template #content>
                <div class="space-y-3">
                    <div class="space-y-2">
                        <Label>Username</Label>
                        <Input v-model="username" />
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
                <Button @click="register" :disabled="isFormValid"
                    >Submit</Button
                >
            </template>
        </base-dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, watchEffect, watch } from "vue";
import { storeToRefs } from "pinia";

import Navbar from "@components/layout/Navbar.vue";
import Footer from "@components/layout/Footer.vue";

import Button from "@components/ui/button/Button.vue";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";

import BaseDialog from "@components/shared/BaseDialog.vue";

import { useUserStore } from "@stores/user-store";
import { fileToBlob } from "./utils/helpers";
import { User } from "./types/api/model";

const userStore = useUserStore();
const { isAuthenticated, isReady } = storeToRefs(userStore);

const isOpen = ref<boolean>(false);

const username = ref<string>("");
const imageBlob = ref<Blob | null>(null);

const isFormValid = computed(() => {
    return !(username.value && imageBlob.value?.size);
});

async function register() {
    if (!imageBlob.value) {
        return;
    }

    const payload: User = {
        username: [username.value],
        profile_picture_blob: new Uint8Array(
            await imageBlob.value.arrayBuffer(),
        ),
    };
    await userStore.register(payload);

    isOpen.value = false;

    window.location.reload();
}

function onFileInput(e: Event) {
    const file = (e.target as HTMLInputElement)?.files?.[0];

    if (file) {
        imageBlob.value = fileToBlob(file);
    }
}

async function init() {
    await userStore.init();

    console.log("is auth", isAuthenticated.value);
}

watch(
    isReady,
    async () => {
        console.log(isReady.value);
        if (isReady.value) {
            const response = await userStore.getUserCredentials();
            console.log(response);
            if (!response?.length && isAuthenticated.value) {
                isOpen.value = true;
            }
        }
    },
    { immediate: true },
);

init();
</script>
