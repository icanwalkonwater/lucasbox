<script setup lang="ts">
import { route404 } from "@/router";
</script>

<template>
  <div class="grid grid-cols-5">
    <div class="col-span-2 border-r">
      <div class="m-2 md:m-5">
        <h2 class="text-3xl mb-4">The Clone Wars</h2>
        <span>Ipsum velit esse enim qui aute esse est amet labore minim et laborum officia excepteur. Officia voluptate nulla reprehenderit tempor cillum mollit commodo culpa ex. Dolore ut aliquip est cillum in et officia aute aute. Irure cillum ea qui cupidatat incididunt adipisicing nisi ipsum exercitation nostrud ipsum voluptate consectetur voluptate.</span>

      </div>
      <div class="mt-2">
        <select class="select w-full mb-2">
          <option value="1" selected>Season 1</option>
          <option value="2">Season 2</option>
        </select>

        <ul class="menu">
          <li v-for="i in 10" :key="i">
            <RouterLink :to="{ name: route404, params: { ssid: 1, eid: i } }" active-class="active">Episode {{ i }}</RouterLink>
          </li>
        </ul>
      </div>
    </div>
    <div class="col-span-3 m-2 md:m-5">
      <RouterView />
    </div>
  </div>
</template>
