<script setup lang="ts">
defineProps<{
  variant?: 'primary' | 'secondary' | 'tertiary'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
}>()
</script>

<template>
  <button
    :class="[
      'inline-flex items-center justify-center gap-2 font-headline font-semibold transition-all',
      'hover:scale-[1.02] active:scale-[0.98]',
      { 'opacity-50 cursor-not-allowed': disabled },
      variant === 'primary' || !variant ? 'btn-primary' : '',
      variant === 'secondary' ? 'btn-secondary' : '',
      variant === 'tertiary' ? 'btn-tertiary' : '',
      size === 'sm' ? 'px-4 py-2 text-xs' : '',
      size === 'md' || !size ? 'px-6 py-3 text-sm' : '',
      size === 'lg' ? 'px-8 py-4 text-base' : '',
    ]"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>
