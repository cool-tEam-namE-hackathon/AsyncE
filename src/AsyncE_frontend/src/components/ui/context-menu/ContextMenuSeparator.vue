<script setup lang="ts">
import { cn } from '@/lib/utils'
import {
  ContextMenuSeparator,
  type ContextMenuSeparatorProps,
} from 'radix-vue'
import { computed, type HTMLAttributes } from 'vue'

const props = defineProps<ContextMenuSeparatorProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props

  return delegated
})
</script>

<template>
  <ContextMenuSeparator v-bind="delegatedProps" :class="cn('-mx-1 my-1 h-px bg-slate-200 dark:bg-slate-800', props.class)" />
</template>
