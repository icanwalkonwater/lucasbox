<script setup lang="ts">
import { routeDetailSerie } from "@/router";
</script>

<template>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { serieId: 1 } }">
        Clone Wars
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { serieId: 2 } }">
        Rebels
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { serieId: 3 } }">
        The Mandalorian
      </RouterLink>
    </li>
  </ul>
</template>
