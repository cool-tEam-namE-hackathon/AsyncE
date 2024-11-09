<template>
    <Progress :model-value="modelValue" />
</template>

<script setup lang="ts">
import { BaseProgress } from "@/types/api/model";
import { Progress } from "@components/ui/progress";
const props = defineProps<BaseProgress>();
</script>
