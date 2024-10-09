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
