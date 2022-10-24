<script setup lang="ts">
import { DownloadIcon, InfoCircleIcon } from "vue-tabler-icons";

withDefaults(
  defineProps<{
  title: string,
  description: string,
  cover?: string,
  badges: string[],
}>(),
  { cover: import.meta.env.VITE_DEFAULT_COVER },
);

</script>

<template>
  <a class="movie card border image-full cursor-pointer">
    <figure><img class="!h-60 w-full object-cover" :src="cover" /></figure>
    <div class="card-body">
      <div class="card-title">{{ title }}</div>
      <div class="flex flex-wrap gap-1">
        <slot name="featured"></slot>
        <span v-for="(content, index) in badges" :key="index" class="movie-details badge opacity-0 transition-opacity">{{ content }}</span>
      </div>
      <p class="movie-details text-ellipsis opacity-0 transition-opacity">{{ description }}</p>
      <div class="movie-details card-actions justify-end opacity-0 transition-opacity">
        <button class="btn btn-info"><InfoCircleIcon size="20"/>&nbsp;Details</button>
        <button class="btn btn-accent"><DownloadIcon size="20" />&nbsp;Download</button>
      </div>
    </div>
  </a>
</template>

<style scoped lang="postcss">
.card-title {
  text-shadow: 2px 2px 4px black;
}

.movie::before {
  @apply transition-all;
  background-color: transparent !important;
}

.movie:hover::before {
  background-color: hsl(var(--n) / var(--tw-bg-opacity)) !important;
}

.movie:hover .movie-details {
  opacity: 100;
}
</style>
