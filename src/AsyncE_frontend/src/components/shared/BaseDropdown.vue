<template>
    <DropdownMenu>
        <DropdownMenuTrigger>
            <slot name="trigger" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
            <DropdownMenuLabel>
                <span v-if="props.label">{{ props.label }}</span>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
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
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@ui/dropdown-menu";

import { BaseDropdownProps } from "@/types/api/model";

const props = defineProps<BaseDropdownProps>();
const emits = defineEmits<{
    (e: "on-option-click", payload: string): void;
}>();

function handleOptionClick(option: string) {
    emits("on-option-click", option);
}
</script>
