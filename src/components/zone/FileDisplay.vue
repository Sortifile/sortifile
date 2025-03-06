<template>
  <div class="file-display">
    <h3>{{ name }}</h3>

    <!-- 檔案專屬畫面 -->
    <el-row class="command-bar">
      <el-col :span="12">
        <div>
          <el-switch
            :model-value="ignoreSwitch"
            active-text="已忽略"
            inactive-text="未忽略"
            :disabled="isInheritedIgnore"
            @change="onSwitchChange"
          />
        </div>
      </el-col>
      <el-col :span="12" style="display: flex; justify-content: flex-end">
        <el-button type="primary" @click="handleResummarize">
          Resummarize this file
        </el-button>
      </el-col>
    </el-row>

    <!-- Metadata -->
    <h4>Metadata</h4>
    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="File Name">
          <el-input disabled placeholder="filename.txt" v-model="nameValue" />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="File Path">
          <el-input
            disabled
            placeholder="/path/to/filename.txt"
            v-model="pathValue"
          />
        </el-form-item>
      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="12">
        <el-form-item label="Created Date">
          <el-input
            disabled
            v-model="metadata.created_date"
            placeholder="2025-02-20"
          />
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item label="Last Modified Date">
          <el-input
            disabled
            v-model="metadata.last_modified_date"
            placeholder="2025-02-25"
          />
        </el-form-item>
      </el-col>
    </el-row>
    <div class="ignore-unsee" v-if="!ignoreSwitch">
      <el-row :gutter="20">
        <el-col :span="12">
          <el-form-item label="Last Sort Date">
            <el-input
              disabled
              v-model="metadata.last_sorted_date"
              placeholder="2025-02-20"
            />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="Last Summarized Date">
            <el-input
              disabled
              v-model="metadata.last_summarized_date"
              placeholder="2025-02-20"
            />
          </el-form-item>
        </el-col>
      </el-row>

      <el-divider></el-divider>
      <!-- 摘要資訊 -->
      <el-row :gutter="20" align="middle" style="margin-bottom: 15px">
        <el-col :span="12">
          <h4>Summary (Gen by AI)</h4>
        </el-col>
        <el-col
          :span="12"
          style="display: flex; justify-content: flex-end; padding-right: 20px"
        >
          <el-button type="primary" @click="handleSave">Save</el-button>
        </el-col>
      </el-row>
      <el-form
        :model="summaryData"
        label-position="top"
        label-width="auto"
        width="90%"
      >
        <!-- 基本資訊 (title, version) -->
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="Title">
              <el-input
                v-model="summaryData.title"
                placeholder="Document title or main heading"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="Version">
              <el-input
                v-model="summaryData.version"
                placeholder="Version number, e.g. v1.0"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <!-- 作者資訊 -->
        <el-card shadow="hover" style="margin-top: 20px">
          <template #header>
            <span>Author Information</span>
          </template>
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="Name">
                <el-input
                  v-model="summaryData.author.name"
                  placeholder="作者姓名"
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="Organization">
                <el-input
                  v-model="summaryData.author.organization"
                  placeholder="作者隸屬機構"
                />
              </el-form-item>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="School/Class">
                <el-input
                  v-model="summaryData.author.school_class"
                  placeholder="學校/科系/年級/班級"
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="ID">
                <el-input
                  v-model="summaryData.author.id"
                  placeholder="學號、員工編號等"
                />
              </el-form-item>
            </el-col>
          </el-row>
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="Semester">
                <el-input
                  v-model="summaryData.author.semester"
                  placeholder="學期資訊，如 2025spring"
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="Team">
                <el-input
                  v-model="summaryData.author.team"
                  placeholder="小組或專案組"
                />
              </el-form-item>
            </el-col>
          </el-row>
        </el-card>

        <!-- 主題資訊 (subject, summary, intended_use, section_range, status, file_format, language) -->
        <el-row :gutter="20" style="margin-top: 20px">
          <el-col :span="12">
            <el-form-item label="Subject">
              <el-input
                v-model="summaryData.subject"
                placeholder="文件主要科目或領域"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="Status">
              <el-input
                v-model="summaryData.status"
                placeholder="文件狀態，如 '草稿'、'最終版'"
              />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="File Format">
              <el-input
                v-model="summaryData.file_format"
                placeholder="原始檔案格式，如 PDF、DOCX"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="Language">
              <el-input
                v-model="summaryData.language"
                placeholder="文件語言，如 EN、ZH-TW"
              />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="Summary">
          <el-input
            type="textarea"
            v-model="summaryData.summary"
            placeholder="文件內容的精簡摘要或重點說明"
          />
        </el-form-item>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="Intended Use">
              <el-input
                v-model="summaryData.intended_use"
                placeholder="文件的用途或使用情境"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="Section Range">
              <el-input
                v-model="summaryData.section_range"
                placeholder="文件涵蓋的章節範圍"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <!-- Topics (改用 el-tag) -->
        <el-card shadow="hover" style="margin-top: 20px">
          <template #header>
            <span>Topics</span>
          </template>
          <div>
            <!-- 以 el-tag 顯示已加入的 topic -->
            <el-tag
              v-for="(topic, index) in summaryData.topics"
              :key="index"
              closable
              @close="removeTopic(index)"
              style="margin: 4px"
            >
              {{ topic }}
            </el-tag>
          </div>
          <!-- 新增 topic 輸入 -->
          <div style="margin-top: 10px">
            <el-input
              v-model="newTopic"
              placeholder="Press Enter to add a topic"
              @keyup.enter="addTopic"
              style="width: 300px"
            />
            <el-button
              type="primary"
              size="small"
              style="margin-left: 6px"
              @click="addTopic"
              >Add</el-button
            >
          </div>
        </el-card>
        <el-card shadow="hover" style="margin-top: 20px">
          <template #header>
            <span>Tags</span>
          </template>
          <div>
            <el-tag
              v-for="(tag, index) in summaryData.tags"
              :key="index"
              closable
              @close="removeTag(index)"
              style="margin: 4px"
              type="info"
            >
              {{ tag }}
            </el-tag>
          </div>
          <div style="margin-top: 10px">
            <el-input
              v-model="newTag"
              placeholder="Press Enter to add a tag"
              @keyup.enter="addTag"
              style="width: 300px"
            />
            <el-button
              type="primary"
              size="small"
              style="margin-left: 6px"
              @click="addTag"
              >Add</el-button
            >
          </div>
        </el-card>

        <!-- 儲存按鈕 -->
        <div style="display: flex; justify-content: flex-end; margin-top: 20px">
          <el-button @click="handleReset">Reset</el-button>
          <el-button type="primary" @click="handleSave">Save</el-button>
        </div>
      </el-form>
    </div>
  </div>
</template>

<script setup>
import { onMounted, reactive, ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { useZoneStore } from "../../store/zone";
import { storeToRefs } from "pinia";
import { join } from "@tauri-apps/api/path";
import { emit } from "@tauri-apps/api/event";
import { invoke } from "lodash";

async function joinPaths(base, subpath) {
  return await join(base, subpath);
}

const zoneStore = useZoneStore();
const { zoneName, rootPath } = storeToRefs(zoneStore);

const props = defineProps({
  name: { type: String, default: "" },
  path: { type: String, default: "" },
  loading: { type: Boolean, default: false },
  ignoreSwitch: { type: Boolean, default: false },
  isInheritedIgnore: { type: Boolean, default: false },
});

const nameValue = ref(props.name);
const pathValue = ref("");

const emits = defineEmits(["toggle-ignore", "update:loading"]);

function onSwitchChange(val) {
  emits("toggle-ignore", props.path, val);
}

// Mock data
const summaryData = reactive({
  title: "Project Report for Tauri Integration",
  version: "v1.0",
  author: {
    name: "Alice Chen",
    organization: "Tauri Dev Team",
    school_class: "",
    id: "E12345",
    semester: "2025spring",
    team: "Integration Group",
  },
  subject: "Software Development",
  summary:
    "This report details the integration of Tauri within the Sortifile project...",
  intended_use:
    "Reference material for project documentation and future development",
  section_range: "Sections 1 to 5",
  status: "Final",
  topics: ["Integration Architecture", "Performance Benchmarks"],
  tags: ["Tauri", "Rust", "Vue", "Sortifile"],
  file_format: "DOCX",
  language: "EN",
});

const metadata = ref({
  created_date: "2025-02-20",
  last_modified_date: "2025-02-25",
  last_sorted_date: "2025-02-25",
  last_summarized_date: "2025-02-25",
});

// 新增或刪除 Topics
const newTopic = ref("");
function addTopic() {
  if (newTopic.value.trim()) {
    summaryData.topics.push(newTopic.value.trim());
    newTopic.value = "";
  }
}

function removeTopic(index) {
  summaryData.topics.splice(index, 1);
}

// 新增或刪除 Tags
const newTag = ref("");
function addTag() {
  if (newTag.value.trim()) {
    summaryData.tags.push(newTag.value.trim());
    newTag.value = "";
  }
}
function removeTag(index) {
  summaryData.tags.splice(index, 1);
}

// 儲存
function handleSave() {
  console.log("Saved data:", JSON.stringify(summaryData, null, 2));
  ElMessage.success("JSON data has been saved!");
}

// 重新生成摘要
function handleResummarize() {
  ElMessageBox.confirm(
    "Are you sure you want to resummarize this file?",
    "Warning",
    {
      confirmButtonText: "OK",
      cancelButtonText: "Cancel",
      type: "warning",
    },
  )
    .then(() => {
      // call API to resummarize the file
      invoke("ai_summarize_one_file", {
        zoneName: zoneName.value,
        path: props.path,
      });

      ElMessage({
        type: "success",
        message: "Resummarize completed",
      });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "Resummarize canceled",
      });
    });
}

async function loadFileSummary() {
  // 開始前先把 loading 狀態通知外層
  emits("update:loading", true);

  try {
    // 把根路徑 + 檔案相對路徑串起來
    pathValue.value = await join(rootPath.value, props.path);
    console.log("FileDisplay mounted");

    // 用 tauri 或任何 API 取得檔案的摘要資料 (此為示範)
    const data = await invoke("get_summary_of_one_file", {
      zoneName: zoneName.value,
      filePath: props.path,
    });

    // 取回後更新 reactive 物件
    Object.assign(summaryData, JSON.parse(data));
  } catch (err) {
    console.error("API call failed:", err);
    ElMessage.error("Failed to get summary data");
  } finally {
    // 總是要結束 loading
    emits("update:loading", false);
  }
}

onMounted(() => {
  loadFileSummary();
});

// 若需要在 path 改變時，再次抓取資料
watch(
  () => props.path,
  () => {
    if (props.path) {
      loadFileSummary();
    }
  },
);
</script>

<style scoped>
.ignore-toggle {
  margin-bottom: 10px;
}

.file-display {
  overflow-y: auto;
  max-height: 70vh;
  padding-left: 15px;
  padding-right: 15px;
  width: 100%;
  height: fit-content;
}

.command-bar {
  max-width: 100%;
  width: 100%;
  margin: 0px;
}
</style>
