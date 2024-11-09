<template>
  <ContextMenu>
    <ContextMenuTrigger v-if="$slots['trigger']">
      <slot name="trigger" />
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem 
          v-for="option in options" 
          :key="option.name" 
          @click="emit('on-option-click', option.name)"
      >
        <span :class="option.class">{{ option.name }}</span>
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
</template>

<script setup lang="ts">
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { DropdownOptions } from '@/types/api/model';

defineProps<{
    options: DropdownOptions[];
}>()

const emit = defineEmits<{
    (e: "on-option-click", payload: string): void;
}>();

</script>
