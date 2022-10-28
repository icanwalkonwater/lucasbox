import { get, type MaybeRef } from "@vueuse/shared";
import { computed } from "vue";

export function useArrayEmpty<T>(array: MaybeRef<T[] | undefined>) {
  return computed(() => (get(array)?.length ?? 0) === 0);
}

export function useArrayNotEmpty<T>(array: MaybeRef<T[] | undefined>) {
  return computed(() => (get(array)?.length ?? 0) > 0);
}

export function useArrayFindNotNull<T>(array: T[], predicate: (t: T) => boolean) {
  return computed(() => array.find(predicate)!);
}
