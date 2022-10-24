import { defineStore } from "pinia";
import { reactive } from "vue";

export const useTagStore = defineStore("tags", () => {
  const fileTags = reactive(new Map());

  fileTags.set(0, "1080p");
  fileTags.set(1, "720p");
  fileTags.set(2, "2160p");
  fileTags.set(100, "H264");
  fileTags.set(101, "H265");
  fileTags.set(102, "AV1");

  const franchiseTags = reactive(new Map());
  franchiseTags.set(0, "Anime");
  franchiseTags.set(1, "Very good");

  return { fileTags, franchiseTags };
});
