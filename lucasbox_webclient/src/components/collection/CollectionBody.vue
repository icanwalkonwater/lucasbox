<script setup lang="ts">
import { useArrayEmpty } from "@/hooks";
import type { Collection } from "@/stores/testData2";
import { ChevronRightIcon } from "vue-tabler-icons";
import CollectionChildrenList from "./CollectionChildrenList.vue";
import InlineChildrenList from "./InlineChildrenList.vue";

const props = defineProps<{
  collection: Collection,
}>();

console.log(props.collection);
console.log(props.collection.children);

const hasChildren = useArrayEmpty(props.collection.children);
const hasInlineChildren = useArrayEmpty(props.collection.inlineChildren);
</script>

<template>
  <!-- Header -->
  <div class="m-2 md:m-5">
    <h2 class="text-3xl mb-4">{{ collection.name }}</h2>
    <span>{{ collection.description }}</span>
  </div>

  <!-- Nested collections -->
  <div v-if="hasChildren" class="my-2">
    <CollectionChildrenList :children-ids="collection.children!" />
  </div>

  <!-- Inlined children -->
  <div v-if="hasInlineChildren" class="m-2">
    <InlineChildrenList :children-ids="collection.inlineChildren!" />
  </div>
</template>
