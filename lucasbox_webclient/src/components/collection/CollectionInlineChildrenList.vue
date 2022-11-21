<script setup lang="ts">
import { routeDetailCollectionItem } from "@/router";
import { assert } from "@vueuse/shared";
import { computed, ref } from "vue";
import MenuLinkChevron from "../MenuLinkChevron.vue";

const props = defineProps<{
  children: { id: number, name: string, items: { id: number, name: string }[] }[],
}>();

assert(props.children.length > 0);
const selected = ref(0);
const selectedChild = computed(() => props.children[selected.value]);
</script>

<template>
  <select v-model="selected" class="select w-full">
    <option
      v-for="(child, i) in children"
      :key="i"
      :value="i"
    >
      {{ child.name }} ({{ child.items.length }} element)
    </option>
  </select>
  <ul class="menu">
    <li
      v-for="item in selectedChild.items"
      :key="item.id"
    >
      <MenuLinkChevron 
        :route-name="routeDetailCollectionItem" 
        :params="{ itemId: item.id }" 
      >
        {{ item.name }}
      </MenuLinkChevron>
    </li>
  </ul>
</template>
