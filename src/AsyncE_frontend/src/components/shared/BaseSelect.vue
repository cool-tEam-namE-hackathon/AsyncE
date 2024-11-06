<template>
    <Select v-model="modelValue">
        <SelectTrigger>
            <SelectValue :placeholder="props.placeholder" />
        </SelectTrigger>
        <SelectContent>
            <SelectGroup>
                <SelectItem
                    v-for="(option, index) in props.options"
                    :key="index"
                    :value="option.deviceId"
                >
                    {{ option.name }}
                </SelectItem>
            </SelectGroup>
        </SelectContent>
    </Select>
</template>

<script setup lang="ts">
import { computed } from "vue";

import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@components/ui/select";

import { BaseSelectProps } from "@/types/api/model";

const props = defineProps<BaseSelectProps>();
const emits = defineEmits<{
    (e: "update:modelValue", payload: string): void;
}>();

const modelValue = computed({
    get: () => props.modelValue,
    set: (newVal) => emits("update:modelValue", newVal!),
});
</script>
