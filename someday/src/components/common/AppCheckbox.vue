<script setup lang="ts">
defineProps<{
  checked?: boolean
}>()

defineEmits<{
  'update:checked': [value: boolean]
}>()
</script>

<template>
  <button
    type="button"
    role="checkbox"
    :aria-checked="checked"
    @click="$emit('update:checked', !checked)"
    :class="[
      'w-5 h-5 rounded-full flex items-center justify-center transition-all',
      checked ? 'bg-primary' : 'border-2 border-outline-variant hover:border-primary',
    ]"
  >
    <span
      v-if="checked"
      class="material-symbols-outlined text-on-primary text-sm"
      style="font-variation-settings: 'FILL' 1;"
    >
      check
    </span>
  </button>
</template>
