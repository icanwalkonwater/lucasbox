<script setup lang="ts">
import { useArrayFilter } from "@vueuse/shared";
import CollectionCard from "./CollectionCard.vue";

const props = defineProps<{
  collection: {
    name: string,
    description: string,
    tags: { label: string }[],
    children: { id: number, name: string, description: string, tags: { label: string }[], inline: boolean, items: { id: number, name: string }[] }[],
  },
}>();

const children = useArrayFilter(props.collection.children, (c) => !c.inline);
const inlineChildren = useArrayFilter(props.collection.children, (c) => c.inline);
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
    <CollectionCard
      v-for="child in children"
      :key="child.id"
      :collection="child"
    />
  </div>
</template>
