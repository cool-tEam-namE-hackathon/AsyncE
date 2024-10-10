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
            >
                <span @click="handleClick(option.name)"
                    >{{ option.name }}
                </span>
            </DropdownMenuItem>
        </DropdownMenuContent>
    </DropdownMenu>
</template>

<script setup lang="ts">
import { BaseDropdownProps } from "@/types/api/model";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@ui/dropdown-menu";

const props = defineProps<BaseDropdownProps>();
const emits = defineEmits<{
    (e: "on-logout-click"): void;
}>();

function handleClick(name: string) {
    if (name === "Logout") {
        emits("on-logout-click");
    }
    return;
}
</script>
