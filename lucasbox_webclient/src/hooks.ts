import { computed } from "vue";

export function useArrayEmpty<T>(array: T[] | undefined) {
  return computed(() => (array?.length ?? 0) !== 0);
}

export function useArrayFindNotNull<T>(array: T[], predicate: (t: T) => boolean) {
  return computed(() => array.find(predicate)!);
}
