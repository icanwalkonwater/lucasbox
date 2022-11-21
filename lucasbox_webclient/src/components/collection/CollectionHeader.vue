<script setup lang="ts">
import { computed } from "vue";
import TagList from "../TagList.vue";

const props = defineProps<{
  collection: {
    name: string,
    description: string,
    tags: {
      label: string,
    }[],
    coverUrl?: string,
  }
}>();

const coverSrc = computed(() => props.collection.coverUrl ?? import.meta.env.VITE_DEFAULT_COVER);
</script>

<template>
  <div class="flex">
    <div class="hidden md:block mr-4">
      <img class="rounded-xl aspect-poster object-cover w-80" :src="coverSrc">
    </div>
    <div>
      <h1 class="text-3xl">{{ collection.name }}</h1>
      <TagList :tags="collection.tags" class="my-2" />
      <p>{{ collection.description }}</p>
    </div>
  </div>
</template>
