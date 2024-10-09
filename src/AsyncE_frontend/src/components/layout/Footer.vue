<template>
    <footer
        className="flex flex-col mt-auto gap-2 sm:flex-row py-6 w-full shrink-0 items-center px-4 md:px-6 border-t"
    >
        <p className="text-xs text-gray-500 dark:text-gray-400">
            © 2024 AsyncE. All rights reserved.
        </p>
        <div className="sm:ml-auto flex gap-4 sm:gap-6">
            <router-link
                to="/"
                className="text-xs hover:underline underline-offset-4"
            >
                Terms of Service
            </router-link>
            <router-link
                to="/"
                className="text-xs hover:underline underline-offset-4"
            >
                Privacy
            </router-link>
        </div>
    </footer>
</template>

<script setup></script>
