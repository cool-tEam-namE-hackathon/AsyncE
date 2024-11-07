<template>
    <header class="container flex items-center py-4">
        <!-- NAVBAR -->
        <router-link to="/" class="flex items-center justify-center">
            <div class="flex items-center gap-2">
                <Icon
                    icon="fluent-emoji-high-contrast:japanese-symbol-for-beginner"
                    width="24"
                    height="24"
                />
                <p class="text-lg">AsyncE</p>
            </div>
        </router-link>
        <nav class="ml-auto flex gap-4 sm:gap-6">
            <!-- NOT AUTHENTICATED -->
            <Button
                v-if="!isAuthenticated || !userCredentials?.username"
                @click="login"
                variant="ghost"
            >
                Login
            </Button>

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
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import BaseDropdown from "@shared/BaseDropdown.vue";
import { useUserStore } from "@stores/user-store";
import NavbarNotification from "@components/navbar/NavbarNotification.vue";
import { Avatar, AvatarFallback, AvatarImage } from "@components/ui/avatar";
import Button from "@/components/ui/button/Button.vue";
import { USER_DROPDOWN_OPTIONS } from "@/data/data-constants";

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
