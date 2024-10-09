<template>
    <Dialog v-model:open="isOpen">
        <DialogContent class="sm:max-w-md">
            <DialogHeader>
                <DialogTitle>
                    <slot name="title"></slot>
                </DialogTitle>
                <DialogDescription>
                    <slot name="description"></slot>
                </DialogDescription>
            </DialogHeader>
            <slot name="content"></slot>
            <DialogFooter>
                <DialogClose as-child>
                    <slot name="footer"></slot>
                </DialogClose>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>

<script setup lang="ts">
import { computed } from "vue";

import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "../ui/dialog";

const props = defineProps({
    open: {
        type: Boolean,
        default: false,
    },
});

const emits = defineEmits(["on-close-dialog"]);

const isOpen = computed({
    get: () => props.open,
    set: (newVal) => emits("on-close-dialog", newVal),
});
</script>
