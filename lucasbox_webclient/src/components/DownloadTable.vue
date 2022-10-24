<script setup lang="ts">
import { FileDownloadIcon, TrafficConeIcon } from "vue-tabler-icons";

const files = [
  {
    id: 1,
    name: "StarWars-Episode1-x265-1080p.mkv",
    tags: [
      "1080p",
      "H265",
    ],
    uploader: "admin",
    downloadLink: "",
    streamLink: "",
  },
];
</script>

<template>
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
      <tr v-for="file in files" :key="file.id" class="hover">
        <td>{{ file.id }}</td>
        <td class="text-ellipsis">{{ file.name }}</td>
        <td>
          <div v-for="(tag, index) in file.tags" :key="index" class="badge mr-1">{{ tag }}</div>
        </td>
        <td>{{ file.uploader }}</td>
        <td class="text-primary"><a :href="file.downloadLink"><FileDownloadIcon /></a></td>
        <td class="text-orange-400"><a :href="file.streamLink"><TrafficConeIcon /></a></td>
      </tr>
    </tbody>
  </table>
</template>
