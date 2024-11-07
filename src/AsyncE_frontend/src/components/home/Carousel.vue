<template>
    <div
        class="flex h-full items-center justify-center overflow-hidden border-2 border-black"
    >
        <div class="carousel-container relative h-[60vh] w-96 overflow-hidden">
            <transition-group name="slide-vertical" tag="div" class="h-full">
                <img
                    v-for="(image, index) in images"
                    :key="index"
                    :src="image"
                    v-show="currentIndex === index"
                    class="absolute inset-0 h-full w-full object-cover"
                    alt="Carousel Image"
                />
            </transition-group>
        </div>
    </div>
</template>

<script lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

export default {
    setup() {
        // Define images and initial index
        const images = [
            "/images/home_1.jpeg",
            "/images/home_2.jpeg",
            // Add more images if needed
        ];
        const currentIndex = ref(0);

        // Automatically update the index every second
        onMounted(() => {
            const interval = setInterval(() => {
                currentIndex.value = (currentIndex.value + 1) % images.length;
            }, 1000);

            onUnmounted(() => clearInterval(interval));
        });

        return { images, currentIndex };
    },
};
</script>

<style scoped>
/* Transition for vertical sliding effect */
.slide-vertical-enter-active,
.slide-vertical-leave-active {
    transition: transform 0.5s ease;
}

.slide-vertical-enter-from {
    transform: translateY(100%);
}
.slide-vertical-leave-to {
    transform: translateY(-100%);
}
</style>
