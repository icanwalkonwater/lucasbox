<script setup lang="ts">
import { useArrayFindNotNull } from "@/composables";
import { useCollectionById } from "@/stores/testData2";
import { useArrayMap } from "@vueuse/shared";
import { ref, toRefs } from "vue";

const props = defineProps<{
  childrenIds: number[],
}>();

const { childrenIds } = toRefs(props);
const children = useArrayMap(childrenIds, (el) => useCollectionById(el).value);

const selected = ref(props.childrenIds[0]);
const selectedChild = useArrayFindNotNull(children.value, ({ id }) => id === selected.value);
</script>

<template>
  <table class="table table-zebra w-full">
    <thead>
      <tr>
        <th colspan="2">
          <select v-model="selected" class="select w-full">
            <option v-for="child in children" :key="child.id" :value="child.id">{{ child.name }}</option>
          </select>
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(child, index) in selectedChild.items ?? []" :key="child.id" class="hover">
        <td>#{{ index }}</td>
        <td>{{ child.title }}</td>
      </tr>
    </tbody>
  </table>
</template>
