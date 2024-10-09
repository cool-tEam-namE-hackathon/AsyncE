<template>
    <!-- ONE -->
    <div className="w-full py-12 md:py-24 lg:py-32 xl:py-48">
        <div className="px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
                <div className="space-y-2">
                    <h1
                        className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none"
                    >
                        Welcome to AsyncE
                    </h1>
                    <p
                        className="mx-auto max-w-[700px] text-gray-500 md:text-xl dark:text-gray-400"
                    >
                        Connect, collaborate, and celebrate at your own pace.
                        Join our asynchronous gatherings and be part of a global
                        community.
                    </p>
                </div>
                <div className="space-x-4">
                    <button>Get Started</button>
                    <button>Learn More</button>
                </div>
            </div>
        </div>
    </div>

    <!-- TWO -->
    <section
        className="flex flex-col items-center w-full py-12 md:py-24 lg:py-32 bg-gray-100 dark:bg-gray-200"
    >
        <div className="px-4 md:px-6">
            <h2
                className="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-12"
            >
                Key Features
            </h2>
            <div
                className="inline-grid grid-cols-1 md:grid-cols-2 gap-4 mx-auto"
            >
                <!-- CARD 1 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Flexible Timing</span>
                    </div>
                    <span class="text-gray-600">
                        Participate whenever it suits you, across all time
                        zones.
                    </span>
                </div>

                <!-- CARD 2 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Rich Discussions</span>
                    </div>
                    <span class="text-gray-600">
                        Engage in thoughtful conversations without the pressure
                        of real-time responses.
                    </span>
                </div>

                <!-- CARD 3 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">Global Networking</span>
                    </div>
                    <span class="text-gray-600">
                        Connect with professionals and enthusiasts from around
                        the world.
                    </span>
                </div>

                <!-- CARD 4 -->
                <div
                    class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-md"
                >
                    <div class="flex items-center mb-4 text-gray-700">
                        <span class="text-xl font-bold">
                            Knowledge Repository
                        </span>
                    </div>
                    <span class="text-gray-600">
                        Access and contribute to a growing library of
                        discussions, resources, and insights from past
                        gatherings.
                    </span>
                </div>
            </div>
        </div>
    </section>
</template>
<script setup lang="ts">
import {
    createActor,
    AsyncE_backend,
} from "@declarations/AsyncE_backend/index";
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";

let be = AsyncE_backend;
async function login() {
    let authClient = await AuthClient.create();

    const network =
        process.env.DFX_NETWORK ||
        (process.env.NODE_ENV === "production" ? "ic" : "local");
    const internetIdentityUrl =
        network === "local"
            ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`
            : `https://identity.ic0.app`;

    await new Promise((resolve) => {
        authClient.login({
            identityProvider: internetIdentityUrl,
            onSuccess: resolve,
        });
    });

    const identity = authClient.getIdentity();
    const agent = HttpAgent.createSync({ identity });

    const canisterId = process.env.CANISTER_ID_ASYNCE_BACKEND || "";
    be = createActor(canisterId, {
        agent,
    });

    await be.login().then((response) => {
        console.log(response);
    });
}
</script>
