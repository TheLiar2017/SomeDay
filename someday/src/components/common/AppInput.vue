<script setup lang="ts">
defineProps<{
  modelValue?: string
  placeholder?: string
  icon?: string
  type?: string
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="relative">
    <span
      v-if="icon"
      class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-on-surface-variant"
    >
      {{ icon }}
    </span>
    <input
      :type="type || 'text'"
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :placeholder="placeholder"
      :class="['w-full input-soft', icon ? 'pl-10' : '']"
    />
  </div>
</template>
