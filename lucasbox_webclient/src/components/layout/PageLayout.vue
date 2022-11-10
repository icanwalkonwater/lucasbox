<script setup lang="ts">
import MainSidebar from "./MainSidebar.vue";
import MainNavbar from "./MainNavbar.vue";
import { computed } from "vue";

const appName = import.meta.env.VITE_APP_NAME;

const props = defineProps<{ class?: string }>();

const mainClasses = computed(() => `m-2 md:m-5 ${props.class}`);
</script>

<template>
  <div class="flex">
    <MainSidebar />

    <!-- Main block -->
    <div class="w-full">
      <MainNavbar :title="appName" />

      <main :class="mainClasses">
        <slot></slot>
      </main>
    </div>
  </div>
</template>
