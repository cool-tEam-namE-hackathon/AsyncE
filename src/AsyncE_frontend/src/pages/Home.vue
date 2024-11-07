<template>
    <div
        class="paper-grid flex w-full flex-col items-center justify-center gap-8 bg-[#FAFAFA] px-8 py-16 sm:px-16 md:gap-20 md:px-24 md:py-24 lg:flex-row lg:px-32 lg:py-40"
        ref="paperGrid"
    >
        <div class="space-y-6">
            <div class="space-y-6">
                <h1
                    class="text-3xl font-medium tracking-tighter sm:text-4xl sm:font-light md:text-5xl lg:text-7xl/none"
                >
                    Empower<br />your communications<br />using AsyncE
                </h1>
                <p
                    class="max-w-[700px] text-gray-500 dark:text-gray-400 md:text-xl"
                >
                    <b class="font-semibold">Connect</b>,
                    <b class="font-semibold">collaborate</b>, and
                    <b class="font-semibold">celebrate</b> at your own pace
                    through <b class="font-semibold">asynchronous meetings</b>,
                    with both <b class="font-semibold">instant</b> and
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
                        v-show="currentVerticalCarouselIndex === index"
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
                        v-show="currentVerticalCarouselIndex === index"
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

    <div
        class="flex w-full flex-col items-center justify-center gap-16 bg-gradient-to-br from-slate-100 via-slate-200 to-slate-300 px-8 py-24 md:px-12 lg:py-40"
    >
        <div class="flex flex-col">
            <h2
                class="absolute right-1/2 translate-x-1/2 whitespace-nowrap text-3xl tracking-tighter sm:text-4xl md:text-5xl lg:text-5xl/none"
            >
                Meet each other at
            </h2>
            <h2
                class="mt-12 rounded-xl bg-slate-100 px-4 py-3 text-center text-3xl font-light tracking-tighter duration-100 sm:text-4xl md:mt-16 md:text-5xl lg:text-5xl/none"
                :class="color"
                v-motion-slide-visible-left
                v-for="({ text, color }, index) in meet_texts"
                :key="index"
                v-show="currentVerticalCarouselIndex === index"
            >
                {{ text }}.
            </h2>
        </div>
        <div
            class="mx-auto inline-grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-6"
        >
            <Card
                title="Connect on Your Schedule"
                description="Add your thoughts and updates whenever it fits your day, allowing for true flexibility."
            />
            <Card
                title="Engaging Video Conversations"
                description="Build on existing conversations by adding video responses, making discussions more interactive."
            />
            <Card
                title="Seamless Global Collaboration"
                description="Stay connected with family, friends, or colleagues worldwide, without the need for matching schedules."
            />
            <Card
                title="Long-Term Memory Hub"
                description="Save and revisit past conversations, creating a lasting archive of shared insights and memories."
            />
            <Card
                title="Asynchronous Brainstorming"
                description="Generate and refine ideas over time, giving everyone the chance to think, reflect, and respond thoughtfully."
            />
            <Card
                title="Enhanced Team Productivity"
                description="Empower teams to contribute from any location and at any time, making teamwork more inclusive and efficient."
            />
            <Card
                title="Personalized Participation"
                description="Join conversations when you’re ready, without the constraints of live meeting attendance."
            />
            <Card
                title="Streamlined Knowledge Sharing"
                description="Build a collective resource where everyone can access valuable insights and information, even long after discussions take place."
            />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import Card from "@/components/home/Card.vue";
import { home_images, meet_texts } from "@/data/home_page";

const currentVerticalCarouselIndex = ref(0);
const currentMeetTextsIndex = ref(0);
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
    const verticalCarouselInterval = setInterval(() => {
        currentVerticalCarouselIndex.value =
            (currentVerticalCarouselIndex.value + 1) % home_images.length;
    }, 2000);
    const meetTextsInterval = setInterval(() => {
        currentMeetTextsIndex.value =
            (currentMeetTextsIndex.value + 1) % meet_texts.length;
    }, 10000);
    parallaxWhenScrolling();
    window.addEventListener("scroll", parallaxWhenScrolling);

    onUnmounted(() => {
        clearInterval(verticalCarouselInterval);
        clearInterval(meetTextsInterval);
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
