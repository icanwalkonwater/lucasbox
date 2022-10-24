<script setup lang="ts">
import { routeDetailMovie } from "@/router";
</script>

<template>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: routeDetailMovie, params: { mid: 1 } }">
        Episode I: Phantom Menace
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailMovie, params: { mid: 2 } }">
        Episode II: Clone wars
      </RouterLink>
    </li>
  </ul>
</template>
