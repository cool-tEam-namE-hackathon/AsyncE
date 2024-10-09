<template>
    <header className="p-4 lg:p-6 h-14 flex items-center">
        <router-link to="/" className="flex items-center justify-center">
            <span>AsyncE</span>
        </router-link>
        <nav className="ml-auto flex gap-4 sm:gap-6">
            <router-link
                to="/group"
                class="text-sm font-medium hover:underline underline-offset-4"
            >
                Group
            </router-link>
        </nav>
    </header>
</template>
<script setup></script>
