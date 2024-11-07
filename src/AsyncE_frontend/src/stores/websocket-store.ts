import { ref } from "vue";
import { defineStore, storeToRefs } from "pinia";
import {
    _SERVICE,
    Chat,
    GroupInviteResponse,
    WebsocketEventMessage,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { canisterId as backendCanisterId } from "@declarations/AsyncE_backend/index";
import { SignIdentity } from "@dfinity/agent";
import IcWebSocket, { createWsConfig } from "ic-websocket-js";
import { useUserStore } from "@/stores/user-store";
import { DeleteChat, EditChat } from "@/types/api/model";

export const useWebsocketStore = defineStore("websocket", () => {
    const { actor, identity } = storeToRefs(useUserStore());

    const ws = ref<IcWebSocket<_SERVICE, WebsocketEventMessage>>();

    let onGroupInvited = (group: GroupInviteResponse) => {};
    let onChatReceive = (chat: Chat) => {};
    let onChatEdit = (chat: EditChat) => {};
    let onChatDelete = (chat: DeleteChat) => {};

    function sendMessage(chat: Chat) {
        if (ws.value) {
            ws.value.send({
                AddChat: chat,
            });
            console.log("Message sent:", chat);
        }
    }

    async function setWebsockets() {
        if (!actor.value) return;

        const gatewayUrl = "ws://127.0.0.1:8080";
        const icUrl = "http://127.0.0.1:4943";

        const wsConfig = createWsConfig({
            canisterId: backendCanisterId,
            canisterActor: actor.value!,
            identity: identity.value! as SignIdentity,
            networkUrl: icUrl,
        });

        ws.value = new IcWebSocket(gatewayUrl, undefined, wsConfig);
        ws.value.onmessage = async (event) => {
            console.log("Received message:", event.data);

            const message = event.data;
            switch (true) {
                case "AddChat" in message:
                    onChatReceive(message.AddChat);
                    console.log(message.AddChat);
                    break;

                case "EditChat" in message:
                    onChatEdit(message.EditChat);
                    console.log(message.EditChat);
                    break;

                case "DeleteChat" in message:
                    onChatDelete(message.DeleteChat);
                    console.log(message.DeleteChat);
                    break;

                case "Ping" in message:
                    console.log("Received a Ping");
                    break;

                case "GroupInvited" in message:
                    onGroupInvited(message.GroupInvited);
                    break;

                default:
                    console.log("Unknown variant");
            }
        };

        await new Promise((resolve, reject) => {
            ws.value!.onopen = () => {
                console.log("Websocket is opened");
                resolve(null);
            };

            ws.value!.onerror = (error) => {
                console.log("Websocket error:", error);
                reject(error);
            };

            ws.value!.onclose = () => {
                console.log("Disconnected from the canister");
                reject(null);
            };
        });
    }

    return {
        ws,
        sendMessage,
        setWebsockets,

        setOnChatReceive: (callback: (chat: Chat) => void) =>
            (onChatReceive = callback),
        setOnChatEdit: (callback: (chat: EditChat) => void) =>
            (onChatEdit = callback),
        setOnChatDelete: (callback: (chat: DeleteChat) => void) =>
            (onChatDelete = callback),
        setOnGroupInvited: (callback: (group: GroupInviteResponse) => void) =>
            (onGroupInvited = callback),
    };
});
