<script setup lang="ts">
import { route404 } from "@/router";
</script>

<template>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: route404, params: { serieId: 1 } }">
        Clone Wars
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: route404, params: { serieId: 2 } }">
        Rebels
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: route404, params: { serieId: 3 } }">
        The Mandalorian
      </RouterLink>
    </li>
  </ul>
</template>
