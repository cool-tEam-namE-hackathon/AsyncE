<template>
    <DropdownMenu v-bind="attrs">
        <DropdownMenuTrigger>
            <slot name="trigger" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
            <DropdownMenuLabel v-if="$slots['label']">
                <span v-if="props.label">{{ props.label }}</span>
            </DropdownMenuLabel>
            <DropdownMenuSeparator v-if="$slots['label']" />
            <DropdownMenuItem
                v-for="(option, index) in props.options"
                :key="index"
                @click="handleOptionClick(option.name)"
            >
                <span>
                    {{ option.name }}
                </span>
            </DropdownMenuItem>

            <!-- NO DATA FOUND -->
            <DropdownMenuItem v-if="props.options.length === 0" disabled>
                <span>{{ props.emptyMessage }}</span>
            </DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenu>
</template>

<script setup lang="ts">
import { useAttrs } from "vue";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { BaseDropdownProps } from "@/types/api/model";

const attrs = useAttrs();

const props = defineProps<BaseDropdownProps>();
const emits = defineEmits<{
    (e: "on-option-click", payload: string): void;
}>();

function handleOptionClick(option: string) {
    emits("on-option-click", option);
}
</script>
