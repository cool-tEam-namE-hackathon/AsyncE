<template>
    <div
        class="fixed inset-0 flex flex-col items-center justify-center bg-gray-100 bg-opacity-75 z-50"
    >
        <div class="relative">
            <Icon icon="mynaui:globe" width="24" height="24" />
            <div
                class="absolute inset-0 border-2 border-t-gray-400 border-b-gray-400 border-l-transparent border-r-transparent rounded-full animate-spin"
            ></div>
        </div>
        <p class="mt-4 text-sm font-medium text-gray-600">Loading...</p>
    </div>
</template>

<script setup lang="ts">
import { Icon } from "@iconify/vue";
</script>
