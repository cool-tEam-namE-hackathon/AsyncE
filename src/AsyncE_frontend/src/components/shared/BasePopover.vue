<template>
    <Popover>
        <PopoverTrigger>
            <slot name="trigger" />
        </PopoverTrigger>
        <PopoverContent v-bind="attrs">
            <slot name="content" />
        </PopoverContent>
    </Popover>
</template>

<script setup lang="ts">
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@components/ui/popover";
import { useAttrs } from "vue";

const attrs = useAttrs();

console.log(attrs);
</script>
