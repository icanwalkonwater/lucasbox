<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  tags: Array<{
    label: string,
    class?: string,
  }>,
  class?: string,
}>();

const clazz = computed(() => props.class ?? "");
</script>

<template>
  <div :class="`flex flex-wrap ${clazz}`">
    <div
      v-for="(tag, i) in tags"
      :key="i"
      :class="`badge ${tag.class ?? 'badge-info'} mr-1`"
    >
      {{ tag.label }}
    </div>
  </div>
</template>
