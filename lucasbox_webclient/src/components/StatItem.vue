<script setup lang="ts">
import { AlertTriangleIcon } from "vue-tabler-icons";
withDefaults(defineProps<{
  title: string,
  value: string,
  percent?: number,
  critical: boolean,
}>(), {
  critical: false,
});
</script>

<template>
  <div class="stats border">
    <div :class="{ stat: true, 'bg-warning': critical }">
      <div v-if="percent !== undefined" class="stat-figure">
        <div class="radial-progress" :style="`--value: ${percent};`">{{ percent }}%</div>
      </div>
      <div class="stat-title">{{ title }}</div>
      <div class="stat-value flex">
        <AlertTriangleIcon
          v-if="critical"
          size="40"
          class="mr-2"
        />
        {{ value }}
      </div>
    </div>
  </div>
</template>
