import { useQuery } from "@vue/apollo-composable";
import gql from "graphql-tag";

export const useQueryRootCollections = () => useQuery<{ rootCollections: { id: number, name: string, description: string, tags: Array<{ label: string }> } }>(gql`
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
