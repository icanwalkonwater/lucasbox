<script setup lang="ts">
defineProps<{ title: string }>();
</script>

<template>
  <nav class="navbar border-b">
    <div class="navbar-start">
      <h1 class="text-lg font-bold mx-5">{{ title }}</h1>
      <div class="w-5"></div>
      <div class="breadcrumbs">
        <ul>
          <li><a>Home</a></li>
          <li><a>Stats</a></li>
        </ul>
      </div>
    </div>
  </nav>
</template>