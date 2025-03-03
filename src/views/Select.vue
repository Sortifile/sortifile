<template>
  <div style="max-width: 800px; margin: 0 auto">
    <h1>選擇最近的 Zone</h1>
    <div class="zone-list">
      <el-table
        :data="zones"
        width="100%"
        @row-click="handleRowClick"
        highlight-current-row
      >
        <el-table-column prop="zone_name" label="Zone" width="200" />
        <el-table-column prop="root_path" label="路徑" />
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useZoneStore } from "../store/zone";
import { useFormStore } from "../store/form";
import { useRuleStore } from "../store/rule";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";

const zones = ref([]);
const zoneStore = useZoneStore();
const formStore = useFormStore();
const ruleStore = useRuleStore();
const router = useRouter();

const navigateTo = (path) => {
  router.push(path);
};

const handleRowClick = (row) => {
  zoneStore.setZone(row.zone_name, row.root_path);
  console.log("selectedZone", zoneStore.zoneName, zoneStore.rootPath);
  navigateTo("/zone");
};

onMounted(async () => {
  zoneStore.resetZone();

  try {
    const zonesStr = await invoke("get_zone_list");
    zones.value = JSON.parse(zonesStr);
  } catch (error) {
    console.error(error);
    ElMessage.error("無法取得 Zone 列表");
    zones.value = [
      { zone_name: "Zone 1", root_path: "Path 1" },
      { zone_name: "Zone 2", root_path: "Path 2" },
      { zone_name: "Zone 3", root_path: "Path 3" },
    ];
  }
});
</script>

<style scoped>
.zone-list {
  padding: 0px;
}
</style>
