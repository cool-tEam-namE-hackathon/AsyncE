<template>
    <div class="container my-auto w-full">
        <div class="mx-auto max-w-2xl">
            <div class="overflow-hidden rounded-3xl bg-white shadow-xl">
                <div
                    class="relative h-48 bg-gradient-to-r from-gray-200 to-gray-300"
                >
                    <img
                        :src="profilePicture"
                        :alt="userCredentials?.username"
                        class="absolute bottom-0 left-1/2 h-32 w-32 -translate-x-1/2 translate-y-1/2 transform rounded-full border-4 border-white object-cover shadow-md"
                    />
                </div>
                <div class="px-8 pb-8 pt-20">
                    <h2
                        class="mb-2 text-center text-3xl font-bold text-gray-800"
                    >
                        {{ userCredentials?.username }}
                    </h2>
                    <div
                        class="mb-6 flex items-center justify-center text-gray-600"
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
                    <div class="mb-8 rounded-2xl bg-gray-100 p-6">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <span class="text-lg text-gray-600">
                                    Balance
                                </span>
                                <base-tooltip text="Subscription costs 5 coins">
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
                            <div class="flex items-center gap-2">
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
                    <Button
                        class="w-full transform rounded-xl bg-black px-4 py-4 font-bold text-white transition duration-300 ease-in-out hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-50"
                        :class="{
                            'cursor-not-allowed opacity-50':
                                isLoading || userCredentials?.balance === 0n,
                        }"
                        :disabled="isLoading || userCredentials?.balance === 0n"
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
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

import { storeToRefs } from "pinia";

import { Icon } from "@iconify/vue";

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
        });
        userStore.getUserCredentials();
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isLoading.value = false;
    }
}
</script>
