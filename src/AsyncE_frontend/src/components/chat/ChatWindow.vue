<template>
    <div class="fixed bottom-32 right-4 z-50">
        <div
            v-show="isChatWindowOpen"
            class="w-72 bg-white border rounded-lg shadow-lg mb-3 z-50"
        >
            <h1 class="p-4">Chat</h1>
            <hr />
            <div ref="chatRef" class="h-80 px-4 overflow-auto">
                <div
                    v-for="(message, index) in messages"
                    :key="index"
                    :class="[
                        message.username === username
                            ? 'text-right'
                            : 'text-left',
                        'space-y-1 mb-2',
                    ]"
                >
                    <div class="text-xs text-gray-400 mt-2">
                        {{ message.username }}
                    </div>
                    <span
                        :class="[
                            message.username === username
                                ? 'bg-black text-white'
                                : 'bg-gray-200 text-black',
                            'inline-block p-2 rounded-md text-sm',
                        ]"
                    >
                        {{ message.content }}
                    </span>
                </div>
            </div>
            <hr />
            <div class="flex items-center gap-3 p-2">
                <Input
                    ref="inputRef"
                    v-model="message"
                    placeholder="Type a message..."
                    @keyup.enter="handleChatSend"
                />
                <Button class="p-2" @click="handleChatSend">
                    <Icon
                        icon="lucide:send"
                        width="24"
                        height="24"
                        class="text-white"
                    />
                </Button>
            </div>
        </div>

        <Button
            class="absolute right-0 rounded-full px-2 py-6 shadow-lg z-50"
            @click="toggleChatWindow"
        >
            <Icon
                icon="heroicons-outline:chat"
                width="32"
                height="32"
                class="text-white"
            />
        </Button>
    </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watchEffect } from "vue";

import { storeToRefs } from "pinia";

import { useRoute } from "vue-router";

import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";

import { Icon } from "@iconify/vue";

import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";

import { Chat } from "@declarations/AsyncE_backend/AsyncE_backend.did";

const route = useRoute();
const websocketStore = useWebsocketStore();
const { username } = storeToRefs(useUserStore());

const message = ref<string>("");
const messages = ref<Chat[]>([]);
const isChatWindowOpen = ref<boolean>(false);
const chatRef = ref<HTMLDivElement>();
const inputRef = ref<InstanceType<typeof Input>>();

function toggleChatWindow() {
    isChatWindowOpen.value = !isChatWindowOpen.value;
}

async function scrollToBottom() {
    if (!chatRef.value) return;

    await nextTick();

    chatRef.value.scrollTo({
        top: chatRef.value.scrollHeight,
        behavior: "smooth",
    });
}

async function handleChatSend() {
    const payload: Chat = {
        id: BigInt(0),
        content: message.value,
        group_id: BigInt(route.params.id as string),
        created_time_unix: BigInt(0),
        username: username.value,
    };
    websocketStore.sendMessage(payload);

    messages.value.push(payload);

    message.value = "";

    scrollToBottom();
}

function handleIncomingChat(chat: Chat) {
    if (chat.username !== username.value) {
        messages.value.push(chat);
    }

    scrollToBottom();
}

websocketStore.setOnChatReceive(handleIncomingChat);
</script>
