<template>
    <div class="h-dvh flex flex-col">
        <Navbar />
        <RouterView />
        <Footer />
    </div>
</template>

<script setup lang="ts">
import Navbar from "@components/layout/Navbar.vue";
import Footer from "@components/layout/Footer.vue";
// let greeting = ref("");

// async function handleSubmit(e) {
//     e.preventDefault();
//     const target = e.target;
//     const name = target.querySelector("#name").value;
//     await be.login().then((response) => {
//         greeting.value = response;
//     });
// }
</script>
