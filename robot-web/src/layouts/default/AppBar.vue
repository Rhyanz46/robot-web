<template>
  <v-app-bar flat>
    <v-app-bar-title>
      <v-btn prepend-icon="mdi-circle-slice-4" @click="toggleTheme" size="x-large">Web Bot</v-btn>
    </v-app-bar-title>
  </v-app-bar>
</template>

<script lang="ts" setup>
import { useTheme } from 'vuetify'
const theme = useTheme()
let toggleTheme = () => theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
</script>
