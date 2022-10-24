import { defineStore } from "pinia";
import { reactive } from "vue";

interface Franchise {
  title: string,
  description?: string,
  tags: number[],
  movies?: Movie[],
  series?: Serie[],
  files?: DownloadableFile[],
}

interface Movie {
  title: string,
  description: string,
  cover?: string,
  files: DownloadableFile[],
}

interface Serie {
  title: string,
  description: string,
  cover?: string,
  seasons: {
    title: string,
    cover?: string,
    episodes: {
      title: string,
      description?: string,
      files: DownloadableFile[],
    }[]
  }[]
}

interface DownloadableFile {
  name: string,
  tags: number[],
  size: number,
  downloadLink: string,
  streamLink: string,
}

interface Indicator {
  name: string,
  value: string,
  percent?: number,
  critical?: boolean,
}

export const useTestDataStore = defineStore("testData", () => {
  const sw1 = {
    title: "Episode I - The Phantom Menace",
    description: "Two Jedi escape a hostile blockade to find allies and come across a young boy who may bring balance to the Force, but the long dormant Sith resurface to claim their original glory.",
    cover: "https://static.wikia.nocookie.net/frstarwars/images/e/e0/Lundi.png/revision/latest?cb=20151011153017",
    files: [
      {
        name: "StarWarsEpisode1-1080p-x265.mkv",
        tags: [0, 101],
        size: 3000000000,
        downloadLink: "",
        streamLink: "",
      },
    ],
  } as Movie;

  const sw2 = {
    title: "Episode II - The Clone Wars",
    description: `Depuis le blocus de la planète Naboo par la Fédération du commerce, la République, gouvernée par le Chancelier Palpatine, connaît une véritable crise. Un groupe de dissidents, mené par le sombre Jedi comte Dooku, manifeste son mécontentement envers le fonctionnement du régime. Le Sénat et la population intergalactique se montrent pour leur part inquiets face à l'émergence d'une telle menace.
    Certains sénateurs demandent à ce que la République soit dotée d'une solide armée pour empêcher que la situation ne se détériore davantage. Parallèlement, Padmé Amidala, devenue sénatrice, est menacée par les séparatistes et échappe de justesse à un attentat. Le Padawan Anakin Skywalker est chargé de sa protection. Son maître, Obi-Wan Kenobi, part enquêter sur cette tentative de meurtre et découvre la constitution d'une mystérieuse armée de clones...`,
    cover: "https://fr.web.img3.acsta.net/c_310_420/medias/nmedia/00/02/34/81/affclones.jpg",
    files: [
      {
        name: "StarWars-Episode2-2160p-x264.mkv",
        tags: [2, 100],
        size: 8500000000,
        downloadLink: "",
        streamLink: "",
      },
      {
        name: "StarWars-Episode2-720p-av1.mkv",
        tags: [1, 102],
        size: 200000,
        downloadLink: "",
        streamLink: "",
      },
    ],
  } as Movie;

  const cloneWars = {
    title: "The Clone Wars (2008)",
    description: "La galaxie est en proie à la Guerre des Clones qui oppose les maléfiques Séparatistes et leurs immenses armées d'androïdes à la République. Les Chevaliers Jedi, protecteurs de la République, luttent pour maintenir l'ordre et restaurer la paix tandis que de nouvelles planètes succombent chaque jour aux puissances du mal. Pour prendre l'avantage, le Chevalier Jedi Anakin Skywalker et sa jeune apprentie Padawan, Ahsoka Tano, sont chargés d'une mission capitale qui va les confronter au redoutable \"parrain\" de Tatooine, Jabba the Hutt.",
    cover: "https://fr.web.img3.acsta.net/c_310_420/pictures/20/03/18/11/59/4335871.jpg",
    seasons: [
      {
        title: "Season 1",
        episodes: [
          {
            title: "Episode 1",
            files: [
              {
                name: "CloneWars-S01E01.mkv",
                tags: [0, 100],
                size: 200000,
                downloadLink: "",
                streamLink: "",
              },
            ],
          },
          {
            title: "Episode 2",
            files: [
              {
                name: "CloneWars-S01E02.mkv",
                tags: [0, 100],
                size: 200000,
                downloadLink: "",
                streamLink: "",
              },
            ],
          },
        ],
      },
      {
        title: "Season 1",
        episodes: [
          {
            title: "Episode 1",
            files: [
              {
                name: "CloneWars-S02E01.mkv",
                tags: [0, 100],
                size: 200000,
                downloadLink: "",
                streamLink: "",
              },
            ],
          },
          {
            title: "Episode 2",
            files: [
              {
                name: "CloneWars-S02E02.mkv",
                tags: [0, 100],
                size: 200000,
                downloadLink: "",
                streamLink: "",
              },
            ],
          },
          {
            title: "Episode 3",
            files: [
              {
                name: "CloneWars-S02E03.mkv",
                tags: [0, 100],
                size: 200000,
                downloadLink: "",
                streamLink: "",
              },
            ],
          },
        ],
      },
    ],
  } as Serie;

  const sw = {
    title: "Star Wars",
    description: "You know what it is",
    tags: [1],
    movies: [sw1, sw2],
    series: [cloneWars],
  } as Franchise;

  const inception = {
    title: "Inception",
    movies: [{
      title: "Inception",
      description: "Dom Cobb est un voleur expérimenté – le meilleur qui soit dans l’art périlleux de l’extraction : sa spécialité consiste à s’approprier les secrets les plus précieux d’un individu, enfouis au plus profond de son subconscient, pendant qu’il rêve et que son esprit est particulièrement vulnérable. Très recherché pour ses talents dans l’univers trouble de l’espionnage industriel, Cobb est aussi devenu un fugitif traqué dans le monde entier qui a perdu tout ce qui lui est cher. Mais une ultime mission pourrait lui permettre de retrouver sa vie d’avant – à condition qu’il puisse accomplir l’impossible : l’inception. Au lieu de subtiliser un rêve, Cobb et son équipe doivent faire l’inverse : implanter une idée dans l’esprit d’un individu. S’ils y parviennent, il pourrait s’agir du crime parfait. Et pourtant, aussi méthodiques et doués soient-ils, rien n’aurait pu préparer Cobb et ses partenaires à un ennemi redoutable qui semble avoir systématiquement un coup d’avance sur eux. Un ennemi dont seul Cobb aurait pu soupçonner l’existence.",
      cover: "https://fr.web.img6.acsta.net/c_310_420/medias/nmedia/18/72/34/14/19476654.jpg",
      files: [{
        name: "Inception_1080p.mkv",
        tags: [0],
        downloadLink: "",
        streamLink: "",
      }],
    }],
  } as Franchise;

  const witcher = {
    title: "The Witcher",
    tags: [1],
    series: [{
      title: "The Witcher",
      description: "Le sorceleur Geralt, un chasseur de monstres mutant, se bat pour trouver sa place dans un monde où les humains se révèlent souvent plus vicieux que les bêtes.",
      cover: "https://fr.web.img5.acsta.net/c_310_420/pictures/19/12/12/12/13/2421997.jpg",
      seasons: [{
        title: "Season 1",
        episodes: [{
          title: "Episode 1",
          files: [{
            name: "Witcher-S01E01-1080p.mkv",
            tags: [0],
            downloadLink: "",
            streamLink: "",
          }],
        }],
      }],
    }],
  } as Franchise;

  const franchises = reactive([sw, inception, witcher]);

  const indicators = [
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
  ] as Indicator[];

  return { franchises, indicators };
});
