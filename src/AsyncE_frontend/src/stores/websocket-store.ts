import {
    _SERVICE,
    Chat,
    GroupInviteResponse,
    WebsocketEventMessage,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { canisterId as backendCanisterId } from "@declarations/AsyncE_backend/index";
import { SignIdentity } from "@dfinity/agent";
import IcWebSocket, { createWsConfig } from "ic-websocket-js";
import { defineStore, storeToRefs } from "pinia";
import { ref } from "vue";
import { useUserStore } from "@/stores/user-store";

export const useWebsocketStore = defineStore("websocket", () => {
    const { actor, identity } = storeToRefs(useUserStore());

    const ws = ref<IcWebSocket<_SERVICE, WebsocketEventMessage>>();

    let onGroupInvited = (group: GroupInviteResponse) => {};
    let onChatReceive = (chat: Chat) => {};

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

                case "Ping" in message:
                    console.log("Received a Ping");
                    break;

                case "GroupInvited" in message:
                    onGroupInvited(message.GroupInvited);
                    console.log(`Group invited: ${message.GroupInvited}`);

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

            setTimeout(resolve, 3000);
        });
    }

    return {
        ws,
        sendMessage,
        setWebsockets,

        setOnChatReceive: (callback: (chat: Chat) => void) =>
            (onChatReceive = callback),
        setOnGroupInvited: (callback: (group: GroupInviteResponse) => void) =>
            (onGroupInvited = callback),
    };
});
