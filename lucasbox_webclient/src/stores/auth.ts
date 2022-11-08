import { defineStore } from "pinia";
import { useStorage } from '@vueuse/core'

export const useAuthStore = defineStore("auth", () => {
    const accessToken = useStorage<string | undefined>("accessToken", undefined);
    const refreshToken = useStorage<string | undefined>("refreshToken", undefined);

    return { accessToken, refreshToken };
});
