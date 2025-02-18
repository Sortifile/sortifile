<template>
  <div>
    <h3>Zone Settings</h3>
    <pre>Root Path: {{ rootPath }}</pre>
    <div>這裡要放 rule</div>
    <div>這裡要放 form</div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useZoneStore } from "../../store/zone";
import { storeToRefs } from "pinia";

const zoneStore = useZoneStore();
const { zoneName, rootPath } = storeToRefs(zoneStore);
</script>

<style scoped>
/* 依需要調整樣式 */
</style>
