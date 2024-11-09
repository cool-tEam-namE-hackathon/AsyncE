<template>
    <footer
        class="mt-auto flex w-full shrink-0 flex-col items-center gap-2 border-t px-4 py-6 sm:flex-row md:px-6"
    >
        <p class="text-xs text-gray-500 dark:text-gray-400">
            © 2024 AsyncE. All rights reserved.
        </p>
        <div class="flex gap-4 sm:ml-auto sm:gap-6">
            <router-link
                to="/"
                class="text-xs underline-offset-4 hover:underline"
            >
                Home
            </router-link>
            <router-link
                to="/#learn-more"
                class="text-xs underline-offset-4 hover:underline"
            >
                Learn More
            </router-link>
        </div>
    </footer>
</template>

<script setup></script>
