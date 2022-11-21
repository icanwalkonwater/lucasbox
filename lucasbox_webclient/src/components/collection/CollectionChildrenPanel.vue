<script setup lang="ts">
import { routeDetailCollection } from "@/router";
import { useCollection } from "@/stores/collections";
import { logErrorMessages } from "@vue/apollo-util";
import { toRefs } from "vue";
import { InfoCircleIcon } from "vue-tabler-icons";
import MenuLinkChevron from "../MenuLinkChevron.vue";

const props = defineProps<{ childId: number }>();

const { childId } = toRefs(props);

const { result: queryRes, loading, onError } = useCollection(childId);
onError((err) => logErrorMessages(err));
</script>

<template>
  <template v-if="!loading">
    <div v-if="queryRes?.collection?.children.length === 0" class="alert alert-info justify-start m-2">
      <InfoCircleIcon />
      <span>Nothing in this collection</span>
    </div>
    <div class="menu">
      <li v-for="child in queryRes?.collection?.children" :key="child.id">
        <MenuLinkChevron :route-name="routeDetailCollection" :params="{ collectionId: child.id }">
          {{ child.name }}
        </MenuLinkChevron>
      </li>
      <li v-for="item in queryRes?.collection?.items" :key="item.id">
        <a>{{ item.name }}</a>
      </li>
    </div>
  </template>
  <span v-else>Loading...</span>
</template>
