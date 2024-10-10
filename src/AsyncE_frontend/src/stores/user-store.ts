import { ref, toRaw } from "vue";
import { defineStore } from "pinia";

import { createActor } from "@declarations/AsyncE_backend/index";

import { AuthClient } from "@dfinity/auth-client";
import { ActorSubclass, Identity } from "@dfinity/agent";
import { User } from "@/types/api/model";
import { _SERVICE } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { blobToURL } from "@/utils/helpers";

const client = ref<AuthClient | null>();
const isAuthenticated = ref<boolean>();
const identity = ref<Identity | null>();
const actor = ref<ActorSubclass<_SERVICE> | null>();
const isReady = ref<boolean>(false);

const username = ref<string | null>();
const profilePicture = ref<string>("");

const groupPicture = ref<string>("");

export const getIdentityProvider = () => {
    let idpProvider;

    if (typeof window !== "undefined") {
        const isLocal = process.env.DFX_NETWORK !== "ic";
        // Safari does not support localhost subdomains
        const isSafari = /^((?!chrome|android).)*safari/i.test(
            navigator.userAgent,
        );
        if (isLocal && isSafari) {
            idpProvider = `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
        } else if (isLocal) {
            idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
        }
    }
    return idpProvider;
};

export const defaultOptions = {
    createOptions: {
        idleOptions: {
            disableIdle: true,
        },
    },
    loginOptions: {
        identityProvider: getIdentityProvider(),
    },
};

function actorFromIdentity(identity: Identity) {
    return createActor(process.env.CANISTER_ID_ASYNCE_BACKEND || "", {
        agentOptions: {
            identity,
        },
    });
}

export const useUserStore = defineStore("user", () => {
    async function init() {
        client.value = await AuthClient.create(defaultOptions.createOptions);
        isAuthenticated.value = await client.value.isAuthenticated();
        identity.value = isAuthenticated.value
            ? client.value.getIdentity()
            : null;
        actor.value = identity.value ? actorFromIdentity(identity.value) : null;

        isReady.value = true;
    }

    async function login() {
        const authClient = toRaw(client.value);

        await new Promise<void>((resolve) => {
            authClient?.login({
                ...defaultOptions.loginOptions,
                identityProvider: getIdentityProvider(),
                async onSuccess() {
                    isAuthenticated.value = await authClient.isAuthenticated();
                    identity.value = isAuthenticated.value
                        ? authClient.getIdentity()
                        : null;
                    actor.value = identity.value
                        ? actorFromIdentity(identity.value)
                        : null;
                    resolve();
                },
            });
        });
    }

    async function logout() {
        const authClient = toRaw(client.value);
        await authClient?.logout();
        isAuthenticated.value = false;
        identity.value = null;
        actor.value = null;
    }

    async function register(payload: User) {
        const response = await actor.value?.register(payload);

        return response;
    }

    async function getUserCredentials() {
        const response = await actor.value?.get_user_credentials();
        if (response) {
            username.value = response[0]?.username[0];

            if (response[0]?.profile_picture_blob) {
                profilePicture.value = blobToURL(
                    response[0]?.profile_picture_blob,
                );
            }
        }
        return response;
    }

    return {
        isAuthenticated,
        identity,
        actor,
        isReady,
        username,
        profilePicture,

        init,
        login,
        logout,
        register,
        getUserCredentials,
    };
});
