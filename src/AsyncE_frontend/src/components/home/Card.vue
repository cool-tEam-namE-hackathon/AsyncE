<template>
    <div
        class="max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow-md"
    >
        <div class="mb-4 flex items-center text-gray-700">
            <span class="text-xl font-bold">{{ props.title }}</span>
        </div>
        <span class="text-gray-600">
            {{ props.description }}
        </span>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{ title: string; description: string }>();
</script>
