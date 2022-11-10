import { useQuery } from "@vue/apollo-composable";
import type { MaybeRef } from "@vueuse/shared";
import gql from "graphql-tag";

export const useQueryRootCollections = () => useQuery<{ rootCollections: Array<{ id: number, name: string, description: string, tags: Array<{ label: string }> }> }>(gql`
  query RootCollections {
    rootCollections {
      id
      name
      description
      tags {
        label
      }
    }
  }
`);

export const useCollection = (id: MaybeRef<number>) => useQuery<{
  collection: {
    id: number,
    name: string,
    description: string,
    tags: { label: string }[],
    children: { id: number, name: string, inline: boolean, items: { id: number, name: string }[] }[],
    items: { id: number, name: string }[],
  } | null
}, { id: MaybeRef<number> }>(gql`
  query CollectionById($id: Int!) {
    collection(id: $id) {
      id
      name
      description
      tags { label }
      children { id name inline items { id name } }
      items { id name }
    }
  }
`, { id });
