<template>
    <TooltipProvider :delay-duration="delay" >
        <Tooltip>
            <TooltipTrigger v-if="$slots['trigger']">
                <slot name="trigger" />
            </TooltipTrigger>
            <TooltipContent v-if="props.text">
                <span>{{ props.text }}</span>
            </TooltipContent>
        </Tooltip>
    </TooltipProvider>
</template>

<script setup lang="ts">
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@components/ui/tooltip";

const props = defineProps<{
    delay?: number; 
    text: string;
}>();
</script>
