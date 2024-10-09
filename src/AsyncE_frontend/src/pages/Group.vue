<template>
    <button @click="login">Login</button>
    <div v-if="isUsernameExist">
        <label for="">Username</label>
        <input v-model="username" type="text" />
        <label for="">File</label>
        <input type="file" accept="image/*" @change="onFileChange" />
        <button @click="register">Register</button>
    </div>
</template>
<script setup>
import { ref } from "vue";
import { useUserStore } from "@/stores/user-store";
import IcWebSocket, { generateRandomIdentity, createWsConfig } from "ic-websocket-js";

const userStore = useUserStore();

const userCredentials = ref([]);
const username = ref("");
const isUsernameExist = ref(false);
const imageBlob = ref(null);

async function register() {
    const payload = {
        username: [username.value],
        profile_picture_blob: new Uint8Array(
            await imageBlob.value.arrayBuffer(),
        ),
    };
    await userStore.register(payload);
}

async function login() {
    userCredentials.value = await userStore.login();
    if (!userCredentials.value.length) {
        isUsernameExist.value = true;

        const gatewayUrl = "ws://127.0.0.1:8080";
        const icUrl = "http://127.0.0.1:4943";

        const wsConfig = createWsConfig({
            canisterId: backendCanisterId,
            canisterActor: ic_websocket_example_backend,
            identity: generateRandomIdentity(),
            networkUrl: icUrl,
        });

        const ws = new IcWebSocket(gatewayUrl, undefined, wsConfig);

        ws.onopen = () => {
            console.log("Connected to the canister");
        };

        ws.onmessage = async (event) => {
            console.log("Received message:", event.data);
        };

        ws.onclose = () => {
            console.log("Disconnected from the canister");
        };

        ws.onerror = (error) => {
            console.log("Error:", error);
        };
    }
}

function onFileChange(e) {
    const file = e.target.files[0];

    if (file) {
        imageBlob.value = new Blob([file], {
            type: file.type,
        });
    }
}
</script>
