<script setup lang="ts">
import { route404 } from "@/router";
</script>

<template>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: route404, params: { movieId: 1 } }">
        Episode I: Phantom Menace
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: route404, params: { movieId: 2 } }">
        Episode II: Clone wars
      </RouterLink>
    </li>
  </ul>
</template>
