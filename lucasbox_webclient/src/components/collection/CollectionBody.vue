<script setup lang="ts">
import { useArrayFilter } from "@vueuse/shared";
import CollectionChildrenList from "./CollectionChildrenList.vue";
import InlineChildrenList from "./InlineChildrenList.vue";

const props = defineProps<{
  collection: {
    name: string,
    description: string,
    children: { id: number, name: string, inline: boolean, items: { id: number, name: string }[] }[],
  },
}>();

const children = useArrayFilter(props.collection.children, (c) => !c.inline);
const inlineChildren = useArrayFilter(props.collection.children, (c) => c.inline);
</script>

<template>
  <!-- Header -->
  <div class="m-2 md:m-5">
    <h2 class="text-3xl mb-4">{{ collection.name }}</h2>
    <span>{{ collection.description }}</span>
  </div>

  <!-- Nested collections -->
  <div v-if="children.length > 0" class="my-2">
    <CollectionChildrenList :children="children" />
  </div>

  <!-- Inlined children -->
  <div v-if="inlineChildren.length > 0" class="m-2">
    <InlineChildrenList :children="inlineChildren" />
  </div>
</template>
