<script setup lang="ts">
import { routeDetailMovie } from "@/router";
import { ChevronLeftIcon, ChevronRightIcon, FileDownloadIcon, TrafficConeIcon } from "vue-tabler-icons";
</script>

<template>
  <div class="m-2 md:m-5">
    <div class="grid grid-cols-1 md:grid-cols-4">
      <div class="md:mr-5">
        <img class="rounded shadow" src="https://placeimg.com/1080/1920/nature" alt="">
      </div>
      <div class="md:col-span-3">
        <h2 class="text-3xl mb-4">Episode I: The Phantom Menace</h2>
        <span>Do ex eiusmod commodo laboris aliqua consectetur incididunt tempor incididunt esse aliquip. Irure cupidatat et laborum cillum aute qui enim. Non eiusmod enim dolor excepteur do minim. Consectetur proident amet et pariatur eiusmod fugiat culpa minim nostrud mollit consectetur.</span>
        <table class="table table-zebra hover w-full mt-4">
          <thead>
            <tr>
              <th>#</th>
              <th>Filename</th>
              <th>Quality</th>
              <th>Uploader</th>
              <th>Download</th>
              <th>Stream in VLC</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="i in 3" :key="i" class="hover">
              <td>{{ i }}</td>
              <td class="text-ellipsis">StarWars-Episode1-x265-1080p.mkv</td>
              <td>
                <div class="badge mr-1">1080p</div>
                <div class="badge">H265</div>
              </td>
              <td>admin</td>
              <td class="text-primary"><a><FileDownloadIcon /></a></td>
              <td class="text-orange-400"><a><TrafficConeIcon /></a></td>
            </tr>
          </tbody>
        </table>

        <div class="flex justify-between w-full mt-10">
          <RouterLink :to="{ name: routeDetailMovie }" class="btn btn-primary btn-disabled">
            <ChevronLeftIcon />&nbsp;None
          </RouterLink>
          <RouterLink :to="{ name: routeDetailMovie }" class="btn btn-primary">
            Episode II: The Clone Wars&nbsp;<ChevronRightIcon />
          </RouterLink>
        </div>
      </div>
    </div>
  </div>
</template>
