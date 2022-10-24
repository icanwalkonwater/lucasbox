<script setup lang="ts">
import FranchiseItem from "./FranchiseItem.vue";
</script>

<template>
  <div class="grid grid-cols-2 h-full">
    <div class="h-full border-r">
      <FranchiseItem />
    </div>
    <div>
      <RouterView />
    </div>
  </div>
</template>
