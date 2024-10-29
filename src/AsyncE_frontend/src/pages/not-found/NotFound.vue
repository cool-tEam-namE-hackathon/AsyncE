<template>
    <div
        class="bg-gray-100 flex flex-col justify-center items-center px-4 h-full"
    >
        <div class="text-center">
            <h1 class="text-9xl font-bold text-gray-800">404</h1>
            <h2 class="text-4xl font-semibold text-gray-600 mt-4">
                Page Not Found
            </h2>
            <p class="text-lg text-gray-500 mt-4 mb-8">
                Oops! The page you're looking for doesn't exist.
            </p>
            <Button
                class="mx-auto"
                @click="
                    router.push({
                        name: 'Home',
                    })
                "
            >
                <div class="flex items-center gap-2 py-1">
                    <Icon
                        icon="lucide:home"
                        width="24"
                        height="24"
                        class="text-white"
                    />
                    <span>Go Home</span>
                </div>
            </Button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";

import { Icon } from "@iconify/vue";

import { Button } from "@components/ui/button";

const router = useRouter();
</script>
