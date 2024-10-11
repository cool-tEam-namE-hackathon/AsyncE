<template>
    <Dialog v-model:open="isOpen">
        <DialogContent :hide-close-button="hideCloseButton" class="sm:max-w-md">
            <DialogHeader>
                <DialogTitle>
                    <slot name="title" />
                </DialogTitle>
                <DialogDescription>
                    <slot name="description" />
                </DialogDescription>
            </DialogHeader>
            <slot name="content" />
            <DialogFooter>
                <DialogClose as-child>
                    <slot name="footer" />
                </DialogClose>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { BaseDialogProps } from "@/types/api/model";

import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@ui/dialog";

const props = defineProps<BaseDialogProps>();
const emits = defineEmits(["on-close-dialog"]);

const isOpen = computed({
    get: () => props.open,
    set: (newVal) => emits("on-close-dialog", newVal),
});
</script>
