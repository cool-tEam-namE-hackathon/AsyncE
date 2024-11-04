<template>
    <div class="mb-3 flex h-full flex-col rounded-lg border bg-white">
        <div class="flex items-center gap-1 p-4">
            <Icon icon="ci:chat" width="24" height="24" class="text-black" />
            <h1>Chat</h1>
        </div>
        <hr />
        <div ref="chatRef" class="flex-1 overflow-y-auto px-4">
            <div
                v-for="(message, index) in messages"
                :key="index"
                :class="[
                    message.username === username ? 'text-right' : 'text-left',
                    'mb-2 space-y-1',
                ]"
            >
                <div class="mt-2 text-xs text-gray-400">
                    {{ message.username }}
                </div>
                <span
                    :class="[
                        message.username === username
                            ? 'bg-black text-white'
                            : 'bg-gray-200 text-black',
                        'inline-block max-w-[80%] break-words rounded-md p-2 text-sm',
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
            <Button
                class="p-2"
                :disabled="!message || !message.trim().length"
                @click="handleChatSend"
            >
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
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";
import { Chat } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";
import { storeToRefs } from "pinia";
import { nextTick, ref } from "vue";
import { useRoute } from "vue-router";

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
        const index = messages.value.findIndex((x) => x.uuid === chat.uuid);

        if (index !== -1) {
            messages.value[index] = chat;
        } else {
            messages.value.push(chat);
        }

        scrollToBottom();
    }
}

websocketStore.setOnChatReceive(handleIncomingChat);

async function init() {
    const messageHistory = await groupStore.getChats(route.params.id as string);

    messages.value = [...messageHistory, ...messages.value];

    scrollToBottom();
}

init();
</script>
