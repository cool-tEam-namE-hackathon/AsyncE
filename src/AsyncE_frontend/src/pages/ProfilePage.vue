<template>
    <div class="container w-full py-8">
        <div class="mx-auto min-w-max max-w-4xl">
            <div
                class="overflow-hidden rounded-3xl bg-white px-8 py-8 shadow-xl"
            >
                <div
                    class="relative h-48 bg-gradient-to-r from-gray-200 to-gray-300"
                >
                    <img
                        :src="profilePicture"
                        :alt="userCredentials?.username"
                        class="absolute bottom-0 left-1/2 h-32 w-32 -translate-x-1/2 translate-y-1/2 transform rounded-full border-4 border-white object-cover shadow-md"
                    />
                </div>
                <div class="flex flex-col items-center pb-8 pt-20">
                    <h2
                        class="mb-2 text-center text-3xl font-bold text-gray-800"
                    >
                        {{ userCredentials?.username }}
                    </h2>
                    <div
                        class="mb-3 flex items-center justify-center text-gray-600"
                    >
                        <Icon
                            icon="akar-icons:calendar"
                            width="16"
                            height="16"
                            class="mr-2 text-black"
                        />
                        <span
                            >Joined
                            {{
                                convertDateToReadableFormat(
                                    userCredentials?.created_time_unix,
                                )
                            }}</span
                        >
                    </div>

                    <span
                        v-if="userCredentials?.subscription[0]"
                        class="mb-6 text-center text-gray-600"
                        >Subscription ends in:
                        <strong>
                            {{
                                userCredentials?.subscription[0]
                                    ?.duration_in_days
                            }}</strong
                        >
                        day(s)</span
                    >

                    <span v-else class="mb-6 text-center text-gray-600">
                        You are not currently subscribed
                    </span>
                    <div class="mb-8 self-stretch rounded-2xl bg-gray-100 p-6">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <span class="text-lg text-gray-600">
                                    Balance
                                </span>
                                <base-tooltip
                                    text="Subscription costs 5 coins"
                                    :delay="200"
                                >
                                    <template #trigger>
                                        <Icon
                                            icon="fe:info"
                                            width="20"
                                            height="20"
                                            class="text-black"
                                        />
                                    </template>
                                </base-tooltip>
                            </div>
                            <div class="flex items-center gap-3">
                                <Icon
                                    icon="lucide:coins"
                                    width="24"
                                    height="24"
                                    class="text-black"
                                />
                                <span
                                    class="text-3xl font-bold text-gray-800"
                                    >{{ userCredentials?.balance }}</span
                                >
                            </div>
                        </div>
                    </div>

                    <hr class="mb-8 w-full border-2 border-b-gray-100" />

                    <h1 class="mb-8 text-2xl font-bold">Subscription Plans</h1>

                    <div class="mb-8 flex flex-col gap-8 lg:flex-row">
                        <div class="h-full w-96 rounded-3xl shadow-lg">
                            <div
                                class="flex flex-col items-center justify-center rounded-t-3xl bg-[#211253] py-12"
                            >
                                <h4 class="text-2xl font-bold text-white">
                                    BASIC PLAN
                                </h4>
                                <p class="text-slate-100">PER MONTH</p>
                            </div>

                            <div
                                class="flex h-28 items-center justify-center bg-gradient-to-r from-[#2C8DFF] to-[#48E1FD]"
                            >
                                <div
                                    class="flex h-36 w-36 items-center justify-center rounded-full bg-[#2CB9FF] p-5 shadow-xl"
                                >
                                    <p
                                        class="text-3xl font-semibold text-white"
                                    >
                                        FREE
                                    </p>
                                </div>
                            </div>
                            <div class="mb-6 mt-10 flex flex-col gap-4 px-5">
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Create unlimited amount of meetings
                                </div>
                                <div class="flex gap-3">
                                    <X class="text-red-500" />
                                    Create more than 5 groups
                                </div>
                                <div class="flex gap-3">
                                    <X class="text-red-500" />
                                    Have more than 10 members in your group
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Upload unlimited amount of videos
                                </div>
                                <div class="flex gap-3">
                                    <X class="text-red-500" />
                                    Upload videos with generated subtitles using
                                    AI
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Real-time group chat
                                </div>
                                <div class="flex gap-3">
                                    <X class="text-red-500" />
                                    Edit and delete chat
                                </div>
                            </div>
                            <p class="mb-6 text-center font-bold text-gray-400">
                                This is the default subscription plan
                            </p>
                            <p
                                class="rounded-b-2xl bg-green-400 py-3 text-center text-black"
                                v-if="
                                    !isLoading &&
                                    !userCredentials?.subscription.length
                                "
                            >
                                You are currently on this subscription plan
                            </p>
                        </div>

                        <div class="h-full w-96 rounded-3xl shadow-lg">
                            <div
                                class="flex flex-col items-center justify-center rounded-t-3xl bg-[#211253] py-12"
                            >
                                <h4 class="text-2xl font-bold text-white">
                                    ADVANCED PLAN
                                </h4>
                                <p class="text-slate-100">PER MONTH</p>
                            </div>

                            <div
                                class="flex h-28 items-center justify-center bg-gradient-to-r from-[#6900E8] to-[#9B51FE]"
                            >
                                <div
                                    class="flex h-36 w-36 items-center justify-center gap-1 rounded-full bg-[#8E32FF] p-5 shadow-xl"
                                >
                                    <p
                                        class="text-3xl font-semibold text-white"
                                    >
                                        5
                                    </p>
                                    <Icon
                                        icon="lucide:coins"
                                        width="28"
                                        height="28"
                                        class="text-white"
                                    />
                                </div>
                            </div>
                            <div class="mb-6 mt-10 flex flex-col gap-4 px-5">
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Create unlimited amount of meetings
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Create unlimited amount of groups
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Have unlimited amount of members in your
                                    group
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Upload unlimited amount of videos
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Upload videos with generated subtitles using
                                    AI
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Real-time group chat
                                </div>
                                <div class="flex gap-3">
                                    <Check class="text-green-500" />
                                    Edit and delete chat
                                </div>
                            </div>

                            <Button
                                class="w-full transform rounded-xl bg-gradient-to-r from-[#6900E8] to-[#9B51FE] px-4 py-5 font-bold text-white transition duration-300 ease-in-out hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-50"
                                v-if="
                                    !isLoading &&
                                    userCredentials &&
                                    !userCredentials.subscription.length &&
                                    userCredentials.balance > 5n
                                "
                                @click="buySubscription"
                            >
                                <template v-if="isLoading">
                                    <Icon
                                        icon="prime:spinner"
                                        width="16"
                                        height="16"
                                        class="mr-1 animate-spin text-white"
                                    />
                                    Buying Subscription...
                                </template>
                                <template v-else>Buy Subscription</template>
                            </Button>
                            <p
                                class="mt-6 rounded-b-2xl bg-green-400 py-3 text-center text-black"
                                v-if="
                                    !isLoading &&
                                    userCredentials?.subscription.length
                                "
                            >
                                You are currently on this subscription plan
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { Icon } from "@iconify/vue";
import { Check, X } from "lucide-vue-next";
import { useUserStore } from "@stores/user-store";
import BaseTooltip from "@components/shared/BaseTooltip.vue";
import Button from "@components/ui/button/Button.vue";
import { useToast } from "@components/ui/toast/use-toast";
import { convertDateToReadableFormat } from "@/utils/helpers";

const { userCredentials, profilePicture } = storeToRefs(useUserStore());

const userStore = useUserStore();
const { toast } = useToast();

const isLoading = ref<boolean>(false);

async function buySubscription() {
    isLoading.value = true;
    try {
        await userStore.buySubscription();
        toast({
            title: "Subscription purchased successfully!",
            description: "Your subscription is now active.",
            class: "flex items-center border border-green-500 bg-green-50 text-green-700 p-4 rounded-md shadow-md",
        });
        userStore.getUserCredentials();
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isLoading.value = false;
    }
}
</script>
