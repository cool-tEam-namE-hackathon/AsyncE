<template>
    <div class="flex flex-col bg-white border rounded-lg mb-3 h-full">
        <div class="flex items-center p-4 gap-1">
            <Icon icon="ci:chat" width="24" height="24" class="text-black" />
            <h1>Chat</h1>
        </div>
        <hr />
        <div ref="chatRef" class="flex-1 overflow-auto px-4">
            <div
                v-for="(message, index) in messages"
                :key="index"
                :class="[
                    message.username === username ? 'text-right' : 'text-left',
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
            <Button class="p-2" :disabled="!message || !message.trim().length" @click="handleChatSend">
                <Icon
                    icon="lucide:send"
                    width="24"
                    height="24"
                    class="text-white"
                />
            </Button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { nextTick, ref } from "vue";

import { storeToRefs } from "pinia";

import { useRoute } from "vue-router";

import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";
import { useGroupStore } from "@stores/group-store";

import { Icon } from "@iconify/vue";

import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";

import { Chat } from "@declarations/AsyncE_backend/AsyncE_backend.did";

const route = useRoute();
const websocketStore = useWebsocketStore();
const groupStore = useGroupStore();
const { username } = storeToRefs(useUserStore());

const message = ref<string>("");
const messages = ref<Chat[]>([]);
const chatRef = ref<HTMLDivElement>();
const inputRef = ref<InstanceType<typeof Input>>();

async function scrollToBottom() {
    if (!chatRef.value) return;

    await nextTick();

    chatRef.value.scrollTo({
        top: chatRef.value.scrollHeight,
        behavior: "smooth",
    });
}

async function handleChatSend() {
    if (!message.value || !message.value.trim().length) {
        return;
    }

    const payload: Chat = {
        id: BigInt(0),
        uuid: crypto.randomUUID(),
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

    const index = messages.value.findIndex((x) => x.uuid === chat.uuid);
    messages.value.splice(index, 1);
    messages.value.push(chat);

    scrollToBottom();
}

websocketStore.setOnChatReceive(handleIncomingChat);

async function init() {
    const messageHistory = await groupStore.getChats(route.params.id as string);

    messages.value = [...messageHistory, ...messages.value];
    
    scrollToBottom();
}

init();
</script>
