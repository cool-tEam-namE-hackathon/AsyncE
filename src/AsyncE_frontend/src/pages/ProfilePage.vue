<template>
    <div class="container py-6">
        <section class="bg-card rounded-lg border p-6">
            <div class="mb-6 flex items-center gap-6">
                <div class="flex items-center">
                    <img
                        :src="profilePicture"
                        :alt="userCredentials?.username"
                        width="100"
                        height="100"
                        class="rounded-full"
                    />
                </div>
                <div>
                    <h2 class="mb-1 text-2xl font-bold">
                        {{ userCredentials?.username }}
                    </h2>
                    <div class="flex items-center gap-1 text-sm">
                        <Icon
                            icon="akar-icons:calendar"
                            width="16"
                            height="16"
                            class="text-black"
                        />
                        {{
                            convertDateToReadableFormat(
                                userCredentials?.created_time_unix,
                            )
                        }}
                    </div>
                </div>
            </div>
            <div class="mb-6 flex items-center justify-between">
                <span class="text-lg font-medium">Balance:</span>
                <div class="flex items-center gap-2">
                    <Icon
                        icon="subway:coin"
                        width="16"
                        height="16"
                        class="text-black"
                    />
                    <span class="text-2xl font-bold">
                        {{ userCredentials?.balance }}
                    </span>
                </div>
            </div>
            <Button
                class="w-full"
                :disabled="isLoading || userCredentials?.balance === 0n"
                :is-loading="isLoading"
                @click="buySubscription"
            >
                <template #default> Buy Subscription </template>

                <template #loading>
                    <Icon
                        icon="prime:spinner"
                        width="16"
                        height="16"
                        class="mr-1 animate-spin text-white"
                    />
                    Buying Subscription...
                </template>
            </Button>
        </section>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

import { storeToRefs } from "pinia";

import { Icon } from "@iconify/vue";

import { useUserStore } from "@stores/user-store";

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
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isLoading.value = false;
    }
}
</script>
