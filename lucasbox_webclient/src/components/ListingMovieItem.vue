<script setup lang="ts">
withDefaults(
  defineProps<{
    name: string,
    description: string,
    movies: number,
    series: number,
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
      <div class="flex flex-wrap gap-2">
        <div v-if="movies > 0" class="badge badge-secondary">{{ movies }}&nbsp;movies</div>
        <div v-if="series > 0" class="badge badge-secondary">{{ series }}&nbsp;serie</div>
      </div>
      <p>{{ description }}</p>
    </div>
  </div>
</template>
