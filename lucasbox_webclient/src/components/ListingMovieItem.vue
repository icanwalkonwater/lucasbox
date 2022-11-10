<script setup lang="ts">
withDefaults(
  defineProps<{
    name: string,
    description: string,
    tags?: string[],
    cover?: string
  }>(),
  { cover: import.meta.env.VITE_DEFAULT_COVER },
);
</script>

<template>
  <div class="card image-full cursor-pointer">
    <figure><img class="!h-60 w-full object-contain" :src="cover"></figure>
    <div class="card-body">
      <div class="card-title">{{ name }}</div>
      <div
        v-for="tag in tags"
        :key="tag"
        class="flex flex-wrap"
      >
        <div class="badge badge-primary mr-1">{{ tag }}</div>
      </div>
      <p>{{ description }}</p>
    </div>
  </div>
</template>
