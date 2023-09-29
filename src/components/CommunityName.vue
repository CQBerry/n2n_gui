<script setup lang="ts">
import { useStore } from '@/stores/properties'

const props = useStore()

const nameRules = [
    () => props.communityName !== "" || "Community Name should not be empty", 
    () => /^[A-Za-z0-9]+$/.test(props.communityName) || "Community Name should only contain English characters and numbers"
]

</script>

<template>
    <v-text-field :rules="nameRules" label="Community Name" variant="outlined" v-model="props.communityName"></v-text-field>
</template>