<template>
    <header class="container flex items-center py-4">
        <!-- NAVBAR -->
        <router-link to="/" class="flex items-center justify-center">
            <Icon icon="mynaui:globe" width="24" height="24" />
        </router-link>
        <nav class="ml-auto flex gap-4 sm:gap-6">
            <!-- NOT AUTHENTICATED -->
            <button
                v-if="!isAuthenticated || !userCredentials?.username"
                class="text-sm font-medium underline-offset-4 hover:underline"
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
                            <span>{{ userCredentials.username }}</span>
                            <Avatar>
                                <AvatarImage
                                    :src="profilePicture"
                                    :alt="userCredentials.username"
                                />
                                <AvatarFallback>
                                    {{ userCredentials.username }}
                                </AvatarFallback>
                            </Avatar>
                        </div>
                    </template>
                </base-dropdown>
            </div>
        </nav>
    </header>
</template>
<script setup lang="ts">
import { ref } from "vue";

import { storeToRefs } from "pinia";

import { useRouter } from "vue-router";

import { USER_DROPDOWN_OPTIONS } from "@data/user-constants";
import { Icon } from "@iconify/vue";
import BaseDropdown from "@shared/BaseDropdown.vue";

import { useUserStore } from "@stores/user-store";

import NavbarNotification from "@components/navbar/NavbarNotification.vue";
import { Avatar, AvatarFallback, AvatarImage } from "@components/ui/avatar";

const router = useRouter();
const userStore = useUserStore();

const { isAuthenticated, userCredentials, profilePicture } =
    storeToRefs(userStore);

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

    if (option === "Profile") {
        router.push({
            name: "ProfilePage",
        });
    }
}
</script>
