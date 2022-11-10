<script setup lang="ts">
import { routeLogin } from "@/router";
import { useRegisterMutation } from "@/stores/auth";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { CircleCheckIcon, CircleXIcon } from "vue-tabler-icons";

const router = useRouter();

const { mutate: register, loading: registerLoading, error: registerError } = useRegisterMutation();

const username = ref("");
const password = ref("");
const inviteCode = ref("");

const success = ref(false);

const handleRegister = async () => {
  try {
    // Don't really care about the result
    await register({
      username: username.value,
      password: password.value,
      inviteCode: inviteCode.value,
    });

    success.value = true;
    setTimeout(() => {
      router.replace({ name: routeLogin });
    }, 1000);
  } catch (e) {
    console.warn(e);
  }
};
</script>

<template>
  <form @submit.prevent="handleRegister">
    <div class="h-screen flex justify-center items-center">
      <div class="card card-bordered w-1/3 h-fit">
        <div class="card-body">
          <p class="text-center text-xl">Login</p>
          <input 
            v-model="username" 
            :disabled="registerLoading"
            type="text"
            required
            minlength="3"
            placeholder="Username"
            class="input input-bordered w-full"
          >
          <input
            v-model="password" 
            :disabled="registerLoading"
            type="password"
            required
            minlength="3"
            placeholder="Password"
            class="input input-bordered w-full"
          >
          <input
            v-model="inviteCode"
            :disabled="registerLoading"
            type="text"
            placeholder="Invite code"
            class="input input-bordered w-full"
          >

          <div v-if="registerError" class="alert alert-error justify-start">
            <CircleXIcon />
            <span>{{ registerError.message }}</span>
          </div>

          <div v-if="success" class="alert alert-success justify-start">
            <CircleCheckIcon />
            <span>Sucess ! Redirecting to login page...</span>
          </div>

          <input
            :disabled="registerLoading"
            type="submit"
            value="Register"
            class="btn btn-primary w-full"
          >
        </div>
      </div>
    </div>
  </form>
</template>
