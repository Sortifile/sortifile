<template>
  <div>
    <el-row :gutter="20" align="middle">
      <el-col :span="6"> <h3>Zone Settings</h3> </el-col>
      <el-col :span="18">
        <pre class="rootPath">{{ rootPath }}</pre>
      </el-col>
    </el-row>

    <el-collapse v-model="activeName" accordion>
      <el-collapse-item title="偏好表單" name="1">
        <el-row :gutter="40" align="middle">
          <el-col :span="2">
            <el-button disabled @click="handleEditForm">
              <el-icon> <Edit /> </el-icon>
            </el-button>
          </el-col>
          <el-col :span="22">
            <pre>{{ JSON.stringify(formResponse, null, 4) }}</pre>
          </el-col>
        </el-row>
      </el-collapse-item>
      <el-collapse-item title="分類規則 (Rule.json)" name="2">
        <el-row :gutter="40" align="middle">
          <el-col :span="2">
            <el-button disabled @click="handleEditRule">
              <el-icon> <Edit /> </el-icon>
            </el-button>
          </el-col>
          <el-col :span="22">
            <pre>{{ JSON.stringify(rule, null, 4) }}</pre>
          </el-col>
        </el-row>
      </el-collapse-item>
    </el-collapse>
  </div>

  <!-- TODO: Edit Form-->
  <!-- TODO: Edit Rule-->
</template>

<script setup>
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useZoneStore } from "../../store/zone";
import { useFormStore } from "../../store/form";
import { useRuleStore } from "../../store/rule";
import { Edit } from "@element-plus/icons-vue";

const zoneStore = useZoneStore();
const formStore = useFormStore();
const ruleStore = useRuleStore();

const { zoneName, rootPath } = storeToRefs(zoneStore);
const { formResponse, formQuestion } = storeToRefs(formStore);
const { rule } = storeToRefs(ruleStore);

const activeName = ref("");

const editFormVisible = ref(false);
const editRuleVisible = ref(false);

const handleEditForm = () => {
  editFormVisible.value = true;
};

const handleEditRule = () => {
  editRuleVisible.value = true;
};
</script>

<style scoped>
.rootPath {
  padding-left: 8px;
  padding-right: 8px;
  padding-top: 5px;
  padding-bottom: 5px;
  background-color: #f0f0f0;
  border-radius: 3px;
  width: fit-content;
  margin-top: 0;
}

h3 {
  margin-top: 0;
  font-weight: bold;
}

pre {
  background: #f5f5f5;
  padding: 10px;
  border-radius: 5px;
  overflow: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
