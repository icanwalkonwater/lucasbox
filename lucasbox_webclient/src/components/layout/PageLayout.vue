<script setup lang="ts">
import MainSidebar from "./MainSidebar.vue";
import MainNavbar from "./MainNavbar.vue";

const appName = import.meta.env.VITE_APP_NAME;
</script>

<template>
  <div class="flex">
    <MainSidebar />

    <!-- Main block -->
    <div class="w-full overflow-y-scroll">
      <MainNavbar :title="appName" />
      
      <main class="m-2 md:m-5">
        <slot></slot>
      </main>
    
    </div>
  </div>
</template>
