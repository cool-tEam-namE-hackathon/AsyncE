<template>
    <div class="container py-6">
        <section class="bg-card rounded-lg border p-6">
            <div class="mb-6 flex items-center gap-6">
                <div class="flex items-center">
                    <img
                        :src="profilePicture"
                        :alt="userCredentials.username"
                        width="100"
                        height="100"
                        class="rounded-full"
                    />
                </div>
                <div>
                    <h2 class="mb-1 text-2xl font-bold">
                        {{ userCredentials.username }}
                    </h2>
                    <div class="flex items-center gap-1 text-sm">
                        <Icon
                            icon="akar-icons:calendar"
                            width="16"
                            height="16"
                            class="text-black"
                        />
                        Member since: May 15, 2023
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
                    <span class="text-2xl font-bold">{{
                        userCredentials.balance
                    }}</span>
                </div>
            </div>
            <Button class="w-full">
                <Icon
                    icon="subway:coin"
                    width="16"
                    height="16"
                    class="text-black"
                />
                Deduct 5
            </Button>
        </section>
    </div>
</template>

<script setup>
import { storeToRefs } from "pinia";

import { Icon } from "@iconify/vue";

import { useUserStore } from "@stores/user-store";

import Button from "@components/ui/button/Button.vue";

const { userCredentials, profilePicture } = storeToRefs(useUserStore());
</script>
