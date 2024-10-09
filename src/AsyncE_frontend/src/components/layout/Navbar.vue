<template>
    <header className="p-4 lg:p-6 h-14 flex items-center">
        <router-link to="/" className="flex items-center justify-center">
            <span>AsyncE</span>
        </router-link>
        <nav className="ml-auto flex gap-4 sm:gap-6">
            <button
                v-if="!isAuthenticated"
                to="/login"
                class="text-sm font-medium hover:underline underline-offset-4"
                @click="login"
            >
                Login
            </button>
            <div v-else class="flex items-center gap-3">
                <!-- <router-link
                    to="/group"
                    class="text-sm font-medium hover:underline underline-offset-4"
                >
                    Create Group
                </router-link> -->
                <span class="text-sm font-medium">{{ username }}</span>
                <button
                    @click="logout"
                    class="text-sm font-medium hover:underline underline-offset-4"
                >
                    Logout
                </button>
            </div>
        </nav>
    </header>
</template>
<script setup>
import { storeToRefs } from "pinia";
import { useUserStore } from "@/stores/user-store";

const userStore = useUserStore();
const { isAuthenticated, username } = storeToRefs(userStore);

async function login() {
    await userStore.login();
    window.location.reload();
}
function logout() {
    userStore.logout();
    window.location.reload();
}
</script>
