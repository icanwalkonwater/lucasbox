<script setup lang="ts">
defineProps<{ title: string }>();
</script>

<template>
  <nav class="navbar border-b">
    <div class="navbar-start">
      <h1 class="text-lg font-bold mx-5">{{ title }}</h1>
    </div>
  </nav>
</template>
