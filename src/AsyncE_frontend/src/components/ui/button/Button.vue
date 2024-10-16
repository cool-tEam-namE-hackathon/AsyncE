<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { Primitive, type PrimitiveProps } from "radix-vue";
import { type ButtonVariants, buttonVariants } from ".";
import { cn } from "@/lib/utils";

interface Props extends PrimitiveProps {
    variant?: ButtonVariants["variant"];
    size?: ButtonVariants["size"];
    class?: HTMLAttributes["class"];
    isLoading?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    as: "button",
});
</script>

<template>
    <Primitive
        :as="as"
        :as-child="asChild"
        :class="cn(buttonVariants({ variant, size }), props.class)"
    >
        <div v-if="isLoading" class="flex items-center">
            <slot name="loading" />
        </div>
        <div v-else class="flex items-center">
            <slot />
        </div>
    </Primitive>
</template>
