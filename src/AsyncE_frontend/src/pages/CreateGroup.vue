<template>
    <div class="container m-auto my-8 px-4">
        <div class="mx-auto w-full max-w-2xl rounded-lg bg-white p-6 shadow-md">
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-gray-900">
                    Create a New Group
                </h1>
                <p class="text-sm text-gray-500">
                    Set up a group for your asynchronous meetings / gatherings
                </p>
            </div>
            <div class="space-y-4">
                <div class="space-y-2">
                    <Label
                        >Group Name<span class="text-red-500"> *</span></Label
                    >
                    <Input v-model="groupName" placeholder="Enter group name" />
                </div>
                <div class="space-y-2">
                    <Label
                        >Group Image<span class="text-red-500"> *</span></Label
                    >
                    <div class="flex items-center space-x-2">
                        <Input
                            type="file"
                            accept="image/*"
                            @on-file-change="onFileInput"
                        />
                    </div>
                </div>

                <template v-if="!imageUrl">
                    <img
                        src="/images/placeholder.webp"
                        class="max-h-80 w-full rounded-md object-cover"
                        alt="placeholder"
                    />
                </template>

                <template v-else>
                    <img
                        :src="imageUrl"
                        class="h-full w-full rounded-md object-contain"
                        alt="placeholder"
                    />
                </template>
            </div>
            <div class="mt-6">
                <Button
                    class="w-full"
                    :disabled="isFormValid || isLoading"
                    :is-loading="isLoading"
                    @click="createGroup"
                >
                    <template #default> Create Group </template>

                    <template #loading>
                        <Icon
                            icon="prime:spinner"
                            width="16"
                            height="16"
                            class="mr-1 animate-spin text-white"
                        />
                        Creating Group...
                    </template>
                </Button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import { useGroupStore } from "@stores/group-store";
import Button from "@components/ui/button/Button.vue";
import Input from "@components/ui/input/Input.vue";
import Label from "@components/ui/label/Label.vue";
import { fileToBlob } from "@/utils/helpers";

const router = useRouter();
const groupStore = useGroupStore();

const isLoading = ref<boolean>(false);

const groupName = ref<string>("");
const imageUrl = ref<string>("");
const groupPicture = ref<Blob | null>();

const isFormValid = computed(() => {
    return !(groupName.value && groupPicture.value?.size);
});

function onFileInput(e: Event) {
    const file = (e.target as HTMLInputElement)?.files?.[0];

    if (file) {
        imageUrl.value = URL.createObjectURL(file);
        groupPicture.value = fileToBlob(file);
    }
}

async function createGroup() {
    if (!groupPicture.value) {
        return;
    }

    isLoading.value = true;

    const payload = {
        name: groupName.value,
        picture: new Uint8Array(await groupPicture.value?.arrayBuffer()),
    };

    try {
        await groupStore.createGroup(payload);
        router.push("/groups");
    } catch (e) {
        console.log((e as Error).message);
    } finally {
        isLoading.value = false;
    }
}
</script>
