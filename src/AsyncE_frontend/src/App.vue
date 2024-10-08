<template>
    <h1>AsyncE</h1>
    <RouterView />
</template>

<script setup>
// import { ref } from "vue";
// import { createActor, AsyncE_backend } from "declarations/AsyncE_backend/index";
// import { AuthClient } from "@dfinity/auth-client";
// import { HttpAgent } from "@dfinity/agent";

// let greeting = ref("");
// let be = AsyncE_backend;

// async function handleSubmit(e) {
//     e.preventDefault();
//     const target = e.target;
//     const name = target.querySelector("#name").value;
//     await be.greet(name).then((response) => {
//         greeting.value = response;
//     });
// }

// async function login() {
//     let authClient = await AuthClient.create();

//     const network =
//         process.env.DFX_NETWORK ||
//         (process.env.NODE_ENV === "production" ? "ic" : "local");
//     const internetIdentityUrl =
//         network === "local"
//             ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`
//             : `https://identity.ic0.app`;

//     await new Promise((resolve) => {
//         authClient.login({
//             identityProvider: internetIdentityUrl,
//             onSuccess: resolve,
//         });
//     });

//     const identity = authClient.getIdentity();
//     const agent = new HttpAgent({ identity });
//     be = createActor(process.env.CANISTER_ID_ASYNCE_BACKEND, {
//         agent,
//     });

//     await be.login().then((response) => {
//         console.log(response);
//     });
</script>
