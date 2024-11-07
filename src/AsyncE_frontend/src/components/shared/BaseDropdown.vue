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

            <template v-for="(option, index) in props.options" :key="`option-${index}`">
                <DropdownMenuSeparator
                    v-if="option.hasSeparator" 
                    :key="`separator-${index}`"
                />
                <DropdownMenuItem
                    @click="handleOptionClick(option.name)"
                >
                    <span :class="option.class">
                        {{ option.name }}
                    </span>
                </DropdownMenuItem>
            </template>

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
