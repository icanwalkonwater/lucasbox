<script setup lang="ts">
import { assert } from "@vueuse/shared";
import { computed, ref } from "vue";

const props = defineProps<{
  children: { id: number, name: string, items: { id: number, name: string }[] }[],
}>();

assert(props.children.length > 0);
const selected = ref(0);
const selectedChild = computed(() => props.children[selected.value]);
</script>

<template>
  <table class="table table-zebra w-full">
    <thead>
      <tr>
        <th colspan="2">
          <select v-model="selected" class="select w-full">
            <option
              v-for="(child, i) in children"
              :key="i"
              :value="i"
            >
              {{ child.name }}
            </option>
          </select>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(child, index) in selectedChild.items ?? []"
        :key="child.id"
        class="hover"
      >
        <td>#{{ index }}</td>
        <td>{{ child.name }}</td>
      </tr>
    </tbody>
  </table>
</template>
