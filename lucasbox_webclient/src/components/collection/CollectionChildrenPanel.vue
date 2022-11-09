<script setup lang="ts">
import { useCollectionById } from "@/stores/testData2";
import { useRouteParams } from "@vueuse/router";
import { computed } from "vue";

const childIdRaw = useRouteParams<string>("childId");
const childId = computed(() => parseInt(childIdRaw.value));

const collection = useCollectionById(childId.value);

//const children = useArrayMap(collection.value?.children)
</script>

<template>
  <div class="menu">
    <li v-for="child in collection?.children ?? []" :key="child">
      <a>{{ child }}</a>
    </li>
    <li v-for="item in collection?.items ?? []" :key="item.id">
      <a>{{ item.title }}</a>
    </li>
  </div>
  {{ collection }}
</template>
