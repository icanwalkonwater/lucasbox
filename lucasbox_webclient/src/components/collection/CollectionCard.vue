<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  collection: {
    name: string,
    description: string,
    coverUrl?: string,
    tags: {
      label: string,
    }[],
  }
}>();

const coverSrc = computed(() => props.collection.coverUrl ?? import.meta.env.VITE_DEFAULT_COVER);
</script>

<template>
  <div class="card image-full object-contain">
    <figure><img :src="coverSrc" class="aspect-video"></figure>
    <div class="card-body">
      <h2 class="card-title">{{ collection.name }}</h2>
      <div class="flex flex-wrap">
        <TagList :tags="collection.tags"/>
      </div>
      <p class="text-ellipsis">{{ collection.description }}</p>
    </div>
  </div>
</template>
