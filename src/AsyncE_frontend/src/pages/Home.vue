<template>
    <!-- ONE -->
    <div class="w-full py-12 md:py-24 lg:py-32 xl:py-48">
        <div class="px-4 md:px-6">
            <div class="flex flex-col items-center space-y-4 text-center">
                <div class="space-y-2">
                    <h1
                        class="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none"
                    >
                        Welcome to AsyncE
                    </h1>
                    <p
                        class="mx-auto max-w-[700px] text-gray-500 md:text-xl dark:text-gray-400"
                    >
                        Connect, collaborate, and celebrate at your own pace.
                        Join our asynchronous gatherings and be part of a global
                        community.
                    </p>
                </div>
                <div class="space-x-4">
                    <button>Get Started</button>
                    <button>Learn More</button>
                </div>
            </div>
        </div>
    </div>

    <!-- TWO -->
    <section
        class="flex flex-col items-center w-full py-12 md:py-24 lg:py-32 bg-gray-100 dark:bg-gray-200"
    >
        <div class="px-4 md:px-6">
            <h2
                class="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-12"
            >
                Key Features
            </h2>
            <div class="inline-grid grid-cols-1 md:grid-cols-2 gap-4 mx-auto">
                <!-- CARD 1 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Flexible Timing</span>
                    </div>
                    <span class="text-gray-600">
                        Participate whenever it suits you, across all time
                        zones.
                    </span>
                </div>

                <!-- CARD 2 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Rich Discussions</span>
                    </div>
                    <span class="text-gray-600">
                        Engage in thoughtful conversations without the pressure
                        of real-time responses.
                    </span>
                </div>

                <!-- CARD 3 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Global Networking</span>
                    </div>
                    <span class="text-gray-600">
                        Connect with professionals and enthusiasts from around
                        the world.
                    </span>
                </div>

                <!-- CARD 4 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">
                            Knowledge Repository
                        </span>
                    </div>
                    <span class="text-gray-600">
                        Access and contribute to a growing library of
                        discussions, resources, and insights from past
                        gatherings.
                    </span>
                </div>
            </div>
        </div>
    </section>
</template>
<script setup lang="ts"></script>
