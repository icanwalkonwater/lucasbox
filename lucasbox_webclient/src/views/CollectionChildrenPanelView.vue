<script setup lang="ts">
import MenuLinkChevron from "@/components/MenuLinkChevron.vue";
import { routeDetailCollection } from "@/router";
import { useCollection } from "@/stores/collections";
import { logErrorMessages } from "@vue/apollo-util";
import { useRouteParams } from "@vueuse/router";
import { computed } from "vue";

const childIdRaw = useRouteParams<string>("childId");
const childId = computed(() => parseInt(childIdRaw.value));

const { result: queryRes, loading, onError } = useCollection(childId);
onError((err) => logErrorMessages(err));
</script>

<template>
  <div class="menu">
    <template v-if="!loading">
      <li v-for="child in queryRes?.collection?.children" :key="child.id">
        <MenuLinkChevron :route-name="routeDetailCollection" :params="{ collectionId: child.id }">
          {{ child.name }}
        </MenuLinkChevron>
      </li>
      <li v-for="item in queryRes?.collection?.items" :key="item.id">
        <a>{{ item.name }}</a>
      </li>
    </template>
  </div>
</template>
