<template>
    <Dialog v-model:open="isOpen">
        <DialogContent
            v-bind="attrs"
            :hide-close-button="hideCloseButton"
            class="sm:max-w-md"
            @interact-outside="handleClickOutside"
        >
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
                <slot name="footer" />
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>

<script setup lang="ts">
import { computed, useAttrs } from "vue";
import { BaseDialogProps } from "@/types/api/model";

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@ui/dialog";
import { FocusOutsideEvent } from "radix-vue/dist/DismissableLayer";

const props = defineProps<BaseDialogProps>();
const emits = defineEmits(["on-close-dialog"]);

const isOpen = computed({
    get: () => props.open,
    set: (newVal) => emits("on-close-dialog", newVal),
});

const attrs = useAttrs();

function handleClickOutside(e: FocusOutsideEvent) {
    if (!props.isClosable) {
        e.preventDefault();
    }
}
</script>
