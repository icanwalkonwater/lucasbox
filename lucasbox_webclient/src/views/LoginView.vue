<script setup lang="ts">
function handleSubmit() {
    console.log("uwu");
}
</script>

<template>
    <form @submit.prevent="handleSubmit">
        <div class="h-screen flex justify-center items-center">
            <div class="card card-bordered w-1/3 h-fit">
                <div class="card-body">
                    <p class="text-center text-xl">Login</p>
                    <input type="text" required minlength="3" placeholder="Username"
                        class="input input-bordered w-full">
                    <input type="text" required minlength="3" placeholder="Password"
                        class="input input-bordered w-full">
                    <input type="submit" value="Login" class="btn btn-primary w-full">
                    <a class="btn btn-secondary w-full">Register</a>
                </div>
            </div>
        </div>
    </form>
</template>