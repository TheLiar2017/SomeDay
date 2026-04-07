<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

defineProps<{
  title?: string
}>()

const emit = defineEmits<{
  close: []
}>()

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') emit('close')
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <Teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div
        class="absolute inset-0 bg-black/30 backdrop-blur-sm"
        @click="emit('close')"
      />
      <div class="relative bg-surface-container-lowest rounded-xl shadow-cloud max-w-lg w-full max-h-[90vh] flex flex-col">
        <header class="flex items-center justify-between p-6 border-b border-outline-variant/20">
          <h2 class="text-xl font-bold text-on-surface font-headline">{{ title }}</h2>
          <button
            @click="emit('close')"
            class="p-2 hover:bg-surface-container-high rounded-full transition-colors"
          >
            <span class="material-symbols-outlined text-on-surface-variant">close</span>
          </button>
        </header>
        <div class="p-6 overflow-y-auto flex-1">
          <slot />
        </div>
        <footer v-if="$slots.footer" class="p-6 border-t border-outline-variant/20">
          <slot name="footer" />
        </footer>
      </div>
    </div>
  </Teleport>
</template>
