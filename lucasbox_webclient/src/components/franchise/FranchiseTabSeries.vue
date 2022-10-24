<script setup lang="ts">
import { routeDetailSerie } from "@/router";
</script>

<template>
  <ul class="menu">
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { sid: 1 } }">
        Clone Wars
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { sid: 2 } }">
        Rebels
      </RouterLink>
    </li>
    <li>
      <RouterLink :to="{ name: routeDetailSerie, params: { sid: 3 } }">
        The Mandalorian
      </RouterLink>
    </li>
  </ul>
</template>
