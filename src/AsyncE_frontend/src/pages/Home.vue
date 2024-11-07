<template>
    <div class="paper-grid w-full bg-[#FAFAFA]" ref="paperGrid">
        <div
            class="flex flex-col items-center justify-center gap-8 px-8 py-16 sm:px-16 md:gap-20 md:px-24 md:py-24 lg:flex-row lg:px-32 lg:py-40"
        >
            <div class="space-y-6">
                <div class="space-y-6">
                    <h1
                        class="text-3xl font-light tracking-tighter sm:text-4xl md:text-5xl lg:text-7xl/none"
                    >
                        Empower<br />your communications<br />using AsyncE
                    </h1>
                    <p
                        class="max-w-[700px] text-gray-500 dark:text-gray-400 md:text-xl"
                    >
                        <b class="font-semibold">Connect</b>,
                        <b class="font-semibold">collaborate</b>, and
                        <b class="font-semibold">celebrate</b> at your own pace
                        through
                        <b class="font-semibold">asynchronous meetings</b>, with
                        both <b class="font-semibold">instant</b> and
                        <b class="font-semibold">real-time chat</b>
                    </p>
                </div>
                <div class="space-x-4">
                    <button>Get Started</button>
                    <button>Learn More</button>
                </div>
            </div>

            <div
                class="flex justify-center gap-2 overflow-hidden lg:items-center lg:gap-0"
            >
                <div
                    class="relative h-[40vh] w-44 overflow-hidden sm:w-72 md:w-80 lg:h-[60vh] lg:w-96"
                    v-motion-slide-visible-top
                >
                    <transition-group
                        name="vertical-carousel-left"
                        tag="div"
                        class="tilted-left relative h-full"
                    >
                        <div
                            v-for="({ left }, index) in home_images"
                            :key="index"
                            class="tilted-left absolute inset-0 h-full w-full transition-all duration-500"
                            v-show="currentIndex === index"
                        >
                            <img
                                :src="left.image_url"
                                class="h-full object-cover"
                                alt="Vertical carousel tilted left"
                            />
                        </div>
                    </transition-group>
                </div>
                <div
                    class="relative h-[40vh] w-44 overflow-hidden sm:w-72 md:w-80 lg:h-[60vh] lg:w-96"
                    v-motion-slide-visible-bottom
                >
                    <transition-group
                        name="vertical-carousel-right"
                        tag="div"
                        class="tilted-right relative h-full"
                    >
                        <div
                            v-for="({ right }, index) in home_images"
                            :key="index"
                            class="tilted-right absolute inset-0 h-full w-full transition-all duration-500"
                            v-show="currentIndex === index"
                        >
                            <img
                                :src="right.image_url"
                                class="h-full object-cover"
                                alt="Vertical carousel tilted right"
                            />
                        </div>
                    </transition-group>
                </div>
            </div>
        </div>
    </div>

    <!-- TWO -->
    <section
        class="flex w-full flex-col items-center bg-gray-100 dark:bg-gray-200 md:py-24 lg:py-72"
    >
        <div class="px-4 md:px-6">
            <h2
                class="mb-12 text-center text-3xl font-bold tracking-tighter sm:text-5xl"
            >
                Key Features
            </h2>
            <div class="mx-auto inline-grid grid-cols-1 gap-4 md:grid-cols-2">
                <!-- CARD 1 -->
                <div
                    class="max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow-md"
                >
                    <div class="mb-4 flex items-center text-gray-700">
                        <span class="text-xl font-bold">Flexible Timing</span>
                    </div>
                    <span class="text-gray-600">
                        Participate whenever it suits you, across all time
                        zones.
                    </span>
                </div>

                <!-- CARD 2 -->
                <div
                    class="max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow-md"
                >
                    <div class="mb-4 flex items-center text-gray-700">
                        <span class="text-xl font-bold">Rich Discussions</span>
                    </div>
                    <span class="text-gray-600">
                        Engage in thoughtful conversations without the pressure
                        of real-time responses.
                    </span>
                </div>

                <!-- CARD 3 -->
                <div
                    class="max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow-md"
                >
                    <div class="mb-4 flex items-center text-gray-700">
                        <span class="text-xl font-bold">Global Networking</span>
                    </div>
                    <span class="text-gray-600">
                        Connect with professionals and enthusiasts from around
                        the world.
                    </span>
                </div>

                <!-- CARD 4 -->
                <div
                    class="max-w-sm rounded-lg border border-gray-200 bg-white p-6 shadow-md"
                >
                    <div class="mb-4 flex items-center text-gray-700">
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

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import home_images from "@/data/home_images";

const currentIndex = ref(0);
const paperGrid = ref<HTMLElement | null>(null);

function parallaxWhenScrolling() {
    if (paperGrid.value) {
        const r = Math.max(0, 155 - Math.floor(window.scrollY * 0.2));
        const g = Math.max(0, 155 - Math.floor(window.scrollY * 0.3));
        const b = Math.max(0, 155 - Math.floor(window.scrollY * 0.4));

        paperGrid.value.style.backgroundImage = `
            linear-gradient(to right, rgba(${r}, ${g}, ${b}, 0.2) 1px, transparent 1px),
            linear-gradient(to bottom, rgba(${r}, ${g}, ${b}, 0.2) 1px, transparent 1px)
        `;

        paperGrid.value.style.backgroundPositionY = `${window.scrollY * 0.3}px`;
    }
}

onMounted(() => {
    const interval = setInterval(() => {
        currentIndex.value = (currentIndex.value + 1) % home_images.length;
    }, 2000);
    parallaxWhenScrolling();
    window.addEventListener("scroll", parallaxWhenScrolling);

    onUnmounted(() => {
        clearInterval(interval);
        window.addEventListener("scroll", parallaxWhenScrolling);
    });
});
</script>

<style scoped>
.paper-grid {
    background-size: 92px 92px;
    background-attachment: fixed;
    transition: background-image 0.2s ease;
}

.tilted-left {
    clip-path: polygon(0% 0%, 100% 10%, 100% 100%, 0% 90%);
}
.tilted-right {
    clip-path: polygon(0% 10%, 100% 0%, 100% 90%, 0% 100%);
}

.vertical-carousel-left-enter-active,
.vertical-carousel-left-leave-active {
    transition: transform 1s ease;
}
.vertical-carousel-left-enter-from {
    transform: translateY(-100%);
}
.vertical-carousel-left-leave-to {
    transform: translateY(100%);
}

.vertical-carousel-right-enter-active,
.vertical-carousel-right-leave-active {
    transition: transform 2s ease;
}
.vertical-carousel-right-enter-from {
    transform: translateY(-100%);
}
.vertical-carousel-right-leave-to {
    transform: translateY(100%);
}
</style>
