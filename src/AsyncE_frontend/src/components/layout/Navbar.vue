<template>
    <header className="p-4 lg:p-6 h-14 flex items-center">
        <router-link to="/" className="flex items-center justify-center">
            <span>AsyncE</span>
        </router-link>
        <nav className="ml-auto flex gap-4 sm:gap-6">
            <button
                v-if="!isAuthenticated"
                class="text-sm font-medium hover:underline underline-offset-4"
                @click="login"
            >
                Login
            </button>
            <div v-else class="flex items-center gap-3">
                <router-link
                    to="/group"
                    class="text-sm font-medium hover:underline underline-offset-4"
                >
                    Create Group
                </router-link>
                <base-dropdown
                    label="Your account"
                    :options="USER_DROPDOWN_OPTIONS"
                    @on-logout-click="logout"
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
import { useRouter } from "vue-router";

import { USER_DROPDOWN_OPTIONS } from "@data/user-constants";

import BaseDropdown from "@shared/BaseDropdown.vue";

import { Avatar, AvatarFallback, AvatarImage } from "@components/ui/avatar";

const router = useRouter();

const userStore = useUserStore();
const { isAuthenticated, username, profilePicture } = storeToRefs(userStore);

async function login() {
    await userStore.login();
    window.location.reload();
}
function logout() {
    userStore.logout();
    router.push("/");
    window.location.reload();
}
</script>
