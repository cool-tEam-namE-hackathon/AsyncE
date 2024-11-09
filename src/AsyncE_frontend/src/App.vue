<template>
    <register-dialog
        :open="isRegisterDialogOpen"
        @on-close-dialog="toggleRegisterDialog"
    />

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
    </div>
    <Toaster />
</template>

<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";
import RegisterDialog from "@components/home/RegisterDialog.vue";
import Footer from "@components/layout/Footer.vue";
import Navbar from "@components/layout/Navbar.vue";
import BaseSpinner from "@components/shared/BaseSpinner.vue";
import Toaster from "@components/ui/toast/Toaster.vue";

const userStore = useUserStore();
const websocketStore = useWebsocketStore();
const { isAuthenticated } = storeToRefs(userStore);

const isRegisterDialogOpen = ref<boolean>(false);
const isInitialized = ref<boolean>(false);

function toggleRegisterDialog() {
    isRegisterDialogOpen.value = !isRegisterDialogOpen.value;
}

async function init() {
    await userStore.init();

    try {
        await websocketStore.setWebsockets();
        const hasCredentials = await userStore.getUserCredentials();

        if (!hasCredentials && isAuthenticated) toggleRegisterDialog();
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isInitialized.value = true;
    }
}
init();
</script>
