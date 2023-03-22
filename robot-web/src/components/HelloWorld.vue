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
            v-model="ucSelected"
            :items="uc_list"
            :rules="[v => !!v || 'Item is required']"
            label="Pilih Jumlah UC"
            required
          ></v-select>
        </v-col>
        <v-col cols="12">
          <!-- <v-select
            :disabled="ucSelected == ''"
            v-model="hpSelected"
            :items="hp_list"
            :rules="[v => !!v || 'Item is required']"
            label="Pilih Nomor HP"
            required
          ></v-select> -->
          <v-text-field label="Masukkan Nomor HP" :disabled="ucSelected == ''" v-model="hpSelected"></v-text-field>
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
  const ucSelected = ref("");
  const hpSelected = ref("");
  const hp_list = ref([
    '085337949499',
    '085238138513',
  ])
  const uc_list = ref([
    '15+1 UC',
    '25+1 UC',
    '50+2 UC',
    '100+5 UC',
    '125+6 UC',
    '250+13 UC',
    '500+30 UC',
    '750+75 UC',
    '1000+100 UC',
  ])

  function beli(){
    console.log(result)
    result.finish = true
    result.message = "Waspada PENIPUAN! Anda akan membeli PUBG 125 Unknown Cash dengan tarif 25.227 (termasuk ppn 11%). Balas MDTPBG 087013. Abaikan jika tdk membeli.CS:cs.upoint.id"

    if (pubgId.value == ""){
      return
    }
    axios.post("http://103.150.116.205:8888/buy", {
      "pubg_id": pubgId.value,
      "hp": hpSelected.value,
      "uc": ucSelected.value
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
    axios.post("http://103.150.116.205:8888/check-id", {
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
      ucSelected.value = ""
      hpSelected.value = ""
    });
  }

  watch(pubgId, (val, _) => {
    if (val == ""){
      pubgName.value = ""
      ucSelected.value = ""
      hpSelected.value = ""
    }
  })


  
</script>
