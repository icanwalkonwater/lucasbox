import { defineStore } from "pinia";
import { reactive } from "vue";

export interface WithId {
  id: number,
}

export type CollectionRef = number;

export type Collection = {
  name: string,
  description: string,
  children?: CollectionRef[],
  items?: CollectionItem[],
} & WithId;

export type CollectionItem = {
  title: string,
  files: DownloadableFile[],
} & WithId;

export type DownloadableFile = {
  name: string,
} & WithId;

export interface Indicator {
  name: string,
  value: string,
  percent?: number,
  critical?: boolean,
}

export const useTestDataStore2 = defineStore("testData2", () => {
  let collId = 0;

  const swMovies: Collection = {
    id: collId++,
    name: "Movies",
    description: "oui",
    items: [{
      id: 0,
      title: "Episode I",
      files: [{
        id: 0,
        name: "sw1.mkv",
      }],
    }],
  };

  const swCloneWarsS01: Collection = {
    id: collId++,
    name: "S01",
    description: "oui",
    items: [
      {
        id: 0,
        title: "E01",
        files: [{
          id: 0,
          name: "cwS01E01.mkv",
        }],
      },
    ],
  };

  const swCloneWars: Collection = {
    id: collId++,
    name: "Clone wars",
    description: "oui",
    children: [swCloneWarsS01.id],
  };

  const swSeries: Collection = {
    // series
    id: collId++,
    name: "Series",
    description: "oui",
    children: [swCloneWars.id],
  };

  const sw: Collection = {
    // franchise
    id: collId++,
    name: "SW",
    description: "oui",
    children: [swMovies.id,swSeries.id],
  };

  const collections = reactive([sw]);

  const indicators: Indicator[] = [
    {
      name: "Uptime",
      value: "35 days",
    },
    {
      name: "CPU",
      value: "i9-9999Z",
      percent: 12,
    },
    {
      name: "RAM",
      value: "3.4/4 Go",
      percent: 85,
    },
    {
      name: "Used disk space",
      value: "3.9/4 To",
      percent: 97,
      critical: true,
    },
    {
      name: "Active downloads",
      value: "5",
    },
    {
      name: "Network usage down",
      value: "1000 Mbps",
    },
  ];

  return { indicators, collections };
});
