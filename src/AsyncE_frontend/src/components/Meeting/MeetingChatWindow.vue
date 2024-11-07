<template>
    <div class="flex h-full flex-col rounded-lg border bg-white">
        <!-- Chat Header -->
        <div class="flex items-center gap-1 border-b p-4">
            <Icon icon="ci:chat" width="24" height="24" class="text-black" />
            <h1>Chat</h1>
            <base-tooltip text="Right click to edit or delete a chat">
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

        <!-- Chat Messages -->
        <div ref="chatRef" class="min-h-0 flex-1 overflow-y-auto p-4">
            <div
                v-motion-slide-right
                v-for="(message, index) in messages"
                :key="index"
                :class="[
                    message.username === userCredentials?.username
                        ? 'text-right'
                        : 'text-left',
                    'mb-2 space-y-1',
                ]"
            >
                <div class="mb-1 mt-2 text-xs text-gray-400">
                    {{ message.username }}
                </div>

                <span
                    v-if="message.username !== userCredentials?.username"
                    class="sm:max-w-[80%]' inline-block max-w-[90%] break-words rounded-md bg-black p-2 text-sm text-white"
                >
                    {{ message.content }}
                </span>

                <base-context-menu
                    v-else
                    :options="CHAT_OPTIONS"
                    @on-option-click="
                        (option) => handleOptionClick(option, message)
                    "
                >
                    <template #trigger>
                        <span
                            v-if="editingMessage !== message"
                            class="inline-block max-w-[90%] break-words rounded-md bg-gray-200 p-2 text-sm text-black sm:max-w-[80%]"
                        >
                            {{ message.content }}
                        </span>

                        <input
                            v-else
                            ref="editInputRef"
                            type="text"
                            v-model="editableContent"
                            :size="Math.max(editableContent.length, 1)"
                            @blur="editingMessage = undefined"
                            @keyup.enter="saveEdit(message.id)"
                            class="w-fit break-words rounded-md bg-gray-200 p-2 text-sm text-black"
                        />
                    </template>
                </base-context-menu>
            </div>
        </div>

        <!-- Chat Input -->
        <div class="flex items-center gap-3 border-t p-2">
            <Input
                ref="inputRef"
                v-model="message"
                placeholder="Type a message..."
                class="w-full"
                @keyup.enter="handleChatSend"
            />
            <Button
                class="shrink-0 p-2"
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
import { nextTick, ref } from "vue";
import { storeToRefs } from "pinia";
import { useRoute } from "vue-router";
import { CHAT_OPTIONS } from "@data/data-constants";
import { Chat } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { Icon } from "@iconify/vue";
import BaseTooltip from "@shared/BaseTooltip.vue";
import { useGroupStore } from "@stores/group-store";
import { useUserStore } from "@stores/user-store";
import { useWebsocketStore } from "@stores/websocket-store";
import BaseContextMenu from "@components/shared/BaseContextMenu.vue";
import { Button } from "@components/ui/button";
import { Input } from "@components/ui/input";
import { Message } from "@/types/api/model";

const route = useRoute();
const websocketStore = useWebsocketStore();
const groupStore = useGroupStore();
const { userCredentials } = storeToRefs(useUserStore());

const message = ref<string>("");
const editingMessage = ref<Message>();
const editableContent = ref<string>("");
const messages = ref<Chat[]>([]);
const chatRef = ref<HTMLDivElement>();
const editInputRef = ref<HTMLInputElement[]>([]);
const inputRef = ref<InstanceType<typeof Input>>();

async function handleOptionClick(option: string, message: Message) {
    if (option === "Edit chat") {
        editingMessage.value = message;
        editableContent.value = message.content;

        await nextTick();

        editInputRef.value[0]?.focus();
    }

    if (option === "Delete chat") {
        deleteChat(message.id);
    }
}

async function scrollToBottom() {
    if (!chatRef.value) return;

    await nextTick();

    chatRef.value.scrollTo({
        top: chatRef.value.scrollHeight,
        behavior: "smooth",
    });
}

async function deleteChat(id: bigint) {
    try {
        await groupStore.deleteChat(route.params.id as string, id);

        messages.value = messages.value.filter((msg) => msg.id !== id);
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function saveEdit(id: bigint) {
    try {
        await groupStore.editChat(
            route.params.id as string,
            id,
            editableContent.value,
        );

        const index = messages.value.findIndex((msg) => msg.id === id);
        if (index !== -1) {
            messages.value[index] = {
                ...messages.value[index],
                content: editableContent.value,
            };
        }

        editingMessage.value = undefined;
        editableContent.value = "";
    } catch (e) {
        console.log((e as Error).message);
    }
}

async function handleChatSend() {
    if (
        !message.value ||
        !message.value.trim().length ||
        !userCredentials.value
    ) {
        return;
    }

    const payload: Chat = {
        id: BigInt(0),
        uuid: crypto.randomUUID(),
        content: message.value,
        group_id: BigInt(route.params.id as string),
        created_time_unix: BigInt(0),
        username: userCredentials.value.username,
    };
    websocketStore.sendMessage(payload);

    messages.value.push(payload);

    message.value = "";

    scrollToBottom();
}

function handleIncomingChat(chat: Chat) {
    if (chat.username !== userCredentials.value?.username) {
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
