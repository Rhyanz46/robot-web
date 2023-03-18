<template>
  <v-container class="fill-height">
    <v-responsive class="d-flex align-center text-center fill-height">
      <h1 class="text-h2 font-weight-bold">Beli PUBG UC</h1>

      <div class="py-10" />
      <v-row class="d-flex align-center justify-center" v-if="pubgName">
        <v-col cols="12">
          <v-text-field label="Player Name" v-model="pubgName" disabled></v-text-field>
        </v-col>
      </v-row>
      <v-row class="d-flex align-center justify-center">
        <v-col cols="12">
          <v-text-field label="PUBG ID" v-model="pubgId" v-on:blur="checkId" :loading="pubgLoading"></v-text-field>
        </v-col>
        <v-col cols="12">
          <v-select
            :disabled="pubgName == ''"
            v-model="diamonSelected"
            :items="diamon_list"
            :rules="[v => !!v || 'Item is required']"
            label="Pilih Jumlah Diamon"
            required
          ></v-select>
        </v-col>
        <v-col cols="12">
          <v-select
            :disabled="diamonSelected == ''"
            v-model="hpSelected"
            :items="hp_list"
            :rules="[v => !!v || 'Item is required']"
            label="Pilih Nomor HP"
            required
          ></v-select>
        </v-col>
        <v-col cols="12">
          <v-btn
            :disabled="hpSelected == ''"
            color="primary"
            @click="beli"
            min-width="228"
            rel="noopener noreferrer"
            size="x-large"
            target="_blank"
            variant="flat"
          >
            <v-icon
              icon="mdi-cart-variant"
              size="large"
              start
            />

            Beli
          </v-btn>
        </v-col>
        <v-col cols="12">
          <v-textarea label="Message Result" v-if="result.finish" disabled v-model="result.message"/>
        </v-col>
      </v-row>

      <v-dialog v-model="dialog">
        <v-card>
          <v-card-title>
            <span class="headline">Alert</span>
          </v-card-title>
          <v-card-text>
            Dalam Proses
          </v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="primary" @click="dialog = false">OK</v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>

    </v-responsive>
  </v-container>
</template>

<script lang="ts" setup>
  import { ref, reactive, watch } from 'vue'
  import axios from "axios";

  const dialog = ref(false);
  const pubgId = ref("");
  const pubgLoading = ref(false);
  const pubgName = ref("");
  const result = reactive({message: "", status: 200, finish: false});
  const diamonSelected = ref("");
  const hpSelected = ref("");
  const hp_list = ref([
    '085337949499',
    '085238138513',
  ])
  const diamon_list = ref([
    '1 Diamon',
    '2 Diamon',
    '5 Diamon',
    '10 Diamon',
    '25 Diamon',
    '50 Diamon',
    '100 Diamon',
    '1000 Diamon',
  ])

  function beli(){
    console.log(result)
    result.finish = true
    result.message = "Waspada PENIPUAN! Anda akan membeli PUBG 125 Unknown Cash dengan tarif 25.227 (termasuk ppn 11%). Balas MDTPBG 087013. Abaikan jika tdk membeli.CS:cs.upoint.id"

    if (pubgId.value == ""){
      return
    }
    axios.post("http://localhost:8888/buy", {
      "pubg_id": pubgId.value,
      "hp": hpSelected.value,
    }	)
    .then(response => {
      // handle response
      console.log(response.data.data)
      dialog.value = true
    })
    .catch(error => {
      console.log(error)
    });
  }

  function checkId(){
    if (pubgId.value == ""){
      return
    }
    pubgLoading.value = true
    axios.post("http://localhost:8888/check-id", {
      "pubg_id": pubgId.value
    }	)
    .then(response => {
      // handle response
      pubgName.value = response.data.data.player_name
      pubgLoading.value = false
    })
    .catch(error => {
      // handle error
      pubgId.value = ""
      pubgName.value = ""
      pubgLoading.value = false
      diamonSelected.value = ""
      hpSelected.value = ""
    });
  }

  watch(pubgId, (val, _) => {
    if (val == ""){
      pubgName.value = ""
      diamonSelected.value = ""
      hpSelected.value = ""
    }
  })


  
</script>
