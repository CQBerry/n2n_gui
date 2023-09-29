<script setup lang="ts">
import { useStore } from '@/stores/properties'
import { customAlphabet } from 'nanoid'
import { computed } from 'vue'

const props = useStore()

const communityNameNotLegal = computed(() => !/^[A-Za-z0-9]+$/.test(props.communityName))

const keyRules = [() => (/^[A-Za-z0-9]+$/.test(props.encryptionKey) || props.encryptionKey === "") || 'Encryption Key should only contains English characters and numbers.']

</script>

<template>
    <div id="encryption_key">
        <v-text-field :rules="keyRules" label="Encryption Key" variant="outlined"
            v-model="props.encryptionKey"></v-text-field>
        <!-- Regex check -->
        <v-btn :disabled="communityNameNotLegal" size="x-large" variant="outlined"
            @click="props.encryptionKey = customAlphabet(props.communityName, 16)()">Random</v-btn>
    </div>
</template>

<style scoped lang="scss">
#encryption_key {
    display: flex;
    gap: 2vw
}
</style>