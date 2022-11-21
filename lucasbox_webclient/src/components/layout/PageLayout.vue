<script setup lang="ts">
import MainSidebar from "./MainSidebar.vue";
import MainNavbar from "./MainNavbar.vue";
import { computed } from "vue";

const appName = import.meta.env.VITE_APP_NAME;

const props = defineProps<{ class?: string, hasHeader?: boolean }>();

const mainClasses = computed(() => `m-2 md:m-5 ${props.class}`);
const showHeader = computed(() => props.hasHeader ?? false);
</script>

<template>
  <div class="flex">
    <MainSidebar />

    <!-- Main block -->
    <div class="w-full">
      <MainNavbar :title="appName" />

      <header v-if="showHeader" :class="mainClasses">
        <slot name="header"></slot>
      </header>
      <div v-if="showHeader" class="divider"></div>
      <main :class="mainClasses">
        <slot></slot>
      </main>
    </div>
  </div>
</template>
