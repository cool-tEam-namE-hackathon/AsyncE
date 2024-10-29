<template>
    <header class="container py-4 flex items-center">
        <!-- NAVBAR -->
        <router-link to="/" class="flex items-center justify-center">
            <Icon icon="mynaui:globe" width="24" height="24" />
        </router-link>
        <nav class="ml-auto flex gap-4 sm:gap-6">
            <!-- NOT AUTHENTICATED -->
            <button
                v-if="!isAuthenticated || !username"
                class="text-sm font-medium hover:underline underline-offset-4"
                @click="login"
            >
                Login
            </button>

            <!-- AUTHENTICATED -->
            <div v-else class="flex items-center gap-3">
                <!-- DASHBOARD LINK -->
                <router-link to="/group-list"> Dashboard </router-link>

                <!-- NOTIFICATION -->
                <navbar-notification />

                <!-- USER DROPDOWN -->
                <base-dropdown
                    label="Your account"
                    :options="USER_DROPDOWN_OPTIONS"
                    @on-option-click="handleOptionClick"
                >
                    <template #trigger>
                        <div class="flex items-center gap-3">
                            <span>{{ username }}</span>
                            <Avatar>
                                <AvatarImage
                                    :src="profilePicture"
                                    :alt="username"
                                />
                                <AvatarFallback>{{ username }}</AvatarFallback>
                            </Avatar>
                        </div>
                    </template>
                </base-dropdown>
            </div>
        </nav>
    </header>
</template>
<script setup lang="ts">
import { storeToRefs } from "pinia";

import { useUserStore } from "@stores/user-store";

import { USER_DROPDOWN_OPTIONS } from "@data/user-constants";

import NavbarNotification from "@components/navbar/NavbarNotification.vue";

import BaseDropdown from "@shared/BaseDropdown.vue";

import { Icon } from "@iconify/vue";

import { Avatar, AvatarFallback, AvatarImage } from "@components/ui/avatar";

const userStore = useUserStore();

const { isAuthenticated, username, profilePicture } = storeToRefs(userStore);

async function login() {
    await userStore.login();
    window.location.reload();
}
async function logout() {
    userStore.logout();
    window.location.href = "/";
}

function handleOptionClick(option: string) {
    if (option === "Logout") logout();
}
</script>
