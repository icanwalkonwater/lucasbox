<script setup lang="ts">
import { routeHome, routeRegister } from "@/router";
import { useAuthStore, useLoginMutation } from "@/stores/auth";
import { ref } from "vue";
import { RouterLink, useRouter } from "vue-router";
import { CircleXIcon } from "vue-tabler-icons";

const authStore = useAuthStore();
const router = useRouter();

const { mutate: login, loading: loginLoading, error: loginError } = useLoginMutation();

const username = ref("");
const password = ref("");

const handleLogin = async () => {
  try {
    const res = await login({
      username: username.value,
      password: password.value,
    });

    if (res?.data) {
      authStore.setTokens(res.data.login.accessToken, res.data.login.refreshToken);
    }

    router.replace({ name: routeHome });
  } catch (e) {
    console.warn(e);
  }
};
</script>

<template>
  <form @submit.prevent="handleLogin">
    <div class="h-screen flex justify-center items-center">
      <div class="card card-bordered md:w-1/3 h-fit">
        <div class="card-body">
          <p class="text-center text-xl">Login</p>
          <input 
            v-model="username" 
            :disabled="loginLoading"
            type="text"
            required
            minlength="3"
            placeholder="Username"
            class="input input-bordered w-full"
          >
          <input
            v-model="password" 
            :disabled="loginLoading"
            type="password"
            required
            minlength="3"
            placeholder="Password"
            class="input input-bordered w-full"
          >

          <div v-if="loginError" class="alert alert-error justify-start">
            <CircleXIcon />
            <span>{{ loginError.message }}</span>
          </div>

          <input
            :disabled="loginLoading"
            type="submit"
            value="Login"
            class="btn btn-primary w-full"
          >
          <RouterLink
            :to="{ name: routeRegister }"
            :class="{'btn': true, 'btn-secondary': true, 'w-full': true, 'btn-disabled': loginLoading}"
          >
            Register
          </RouterLink>
        </div>
      </div>
    </div>
  </form>
</template>
