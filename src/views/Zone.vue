<template>
  <div class="zone-container">
    <!-- 左側檔案樹 -->
    <div class="sidebar">
      <h2>檔案瀏覽</h2>
      <el-scrollbar class="zone-scrollbar">
        <el-tree
          ref="treeRef"
          :data="fileTree"
          :props="defaultProps"
          node-key="path"
          highlight-current
          default-expand-all
          draggable
          :allow-drag="allowDrag"
          :allow-drop="allowDrop"
          @node-drop="handleDrop"
          @node-click="handleNodeClick"
        >
          <template #default="{ node, data }">
            <!-- 若 data.ignored 為 true，代表被忽略（無論 explicit or inherited） -->
            <span
              :class="{
                'ignored-file': data.ignored,
                'inherited-file': data.ignoredType === 'inherited',
              }"
            >
              {{ data.name }}
            </span>
          </template>
        </el-tree>
      </el-scrollbar>
    </div>

    <!-- 右側資訊顯示 -->
    <div class="content-display">
      <h2>{{ zoneName }}</h2>
      <el-row
        :gutter="20"
        align="middle"
        justify="start"
        style="max-width: 100%"
      >
        <el-col :span="14" style="display: flex">
          <el-button type="warning" @click="handleDeleteZone">
            <el-icon><DeleteFilled /></el-icon>
          </el-button>
          <el-button @click="handleShowHistory">Show History</el-button>
          <el-button @click="handleSummarizeAll">Summarize All</el-button>
          <el-button @click="handleRenewRules">Renew Rules</el-button>
          <el-button type="primary" @click="handleSortAll">
            Sort All
          </el-button>
        </el-col>
      </el-row>

      <!-- 依據選擇的節點類型，動態切換不同的顯示元件 -->
      <div class="right-content">
        <el-skeleton v-if="loading" :rows="5" animated />
        <ZoneDisplay v-else-if="selectedPath === ''" />
        <FolderDisplay
          v-else-if="selectedNode && selectedNode.isDirectory"
          :name="selectedTitle"
          :path="selectedPath"
          :ignore-switch="ignoreSwitch"
          :is-inherited-ignore="isInheritedIgnore"
          @toggle-ignore="toggleIgnore"
          @sort-folder="handleSortFolder"
        />
        <FileDisplay
          v-else
          :name="selectedTitle"
          :path="selectedPath"
          :ignore-switch="ignoreSwitch"
          :is-inherited-ignore="isInheritedIgnore"
          v-model:loading="loading"
          @toggle-ignore="toggleIgnore"
        />
      </div>
    </div>
  </div>
  <SortConfirm
    v-model="isSortResultDialogVisible"
    :sortResult="sortResult"
    @confirm-selection="handleConfirmedMoves"
  />
  <History v-model="isHistoryDialogVisible" />
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { DeleteFilled } from "@element-plus/icons-vue";
import { ElMessageBox, ElMessage, ElLoading } from "element-plus";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";

// 右側區塊大元件
import ZoneDisplay from "../components/zone/ZoneDisplay.vue";
import FolderDisplay from "../components/zone/FolderDisplay.vue";
import FileDisplay from "../components/zone/FileDisplay.vue";

import SortConfirm from "../components/zone/SortConfirm.vue";
import History from "../components/zone/History.vue";

// Store
import { useZoneStore } from "../store/zone";
import { useRuleStore } from "../store/rule";
import { useFormStore } from "../store/form";
import { storeToRefs } from "pinia";

const router = useRouter();

const zoneStore = useZoneStore();
const ruleStore = useRuleStore();
const formStore = useFormStore();
const { zoneName, rootPath } = storeToRefs(zoneStore);

const fileTree = ref([]);

const selectedPath = ref("");
const selectedTitle = ref(zoneName.value);

const loading = ref(false);
const treeRef = ref(null);

// 用來儲存要被「顯式忽略」的路徑。
// 若其中包含資料夾路徑，則該資料夾及其子項目會是 "繼承忽略"。
const ignoredPaths = ref([]);

// router
const navigateTo = (path) => {
  router.push({ path });
};

// Tree 設定
const defaultProps = {
  children: "children",
  label: "name",
};

/**
 * Mock Data (暫時先用假資料)
 */
const mockFileTree = [
  {
    name: "src",
    path: "src",
    isDirectory: true,
    children: [
      { name: "main.js", path: "src/main.js", isDirectory: false },
      { name: "App.vue", path: "src/App.vue", isDirectory: false },
      {
        name: "components",
        path: "src/components",
        isDirectory: true,
        children: [
          {
            name: "Header.vue",
            path: "src/components/Header.vue",
            isDirectory: false,
          },
          {
            name: "Footer.vue",
            path: "src/components/Footer.vue",
            isDirectory: false,
          },
        ],
      },
    ],
  },
  { name: "package.json", path: "package.json", isDirectory: false },
  { name: "README.md", path: "README.md", isDirectory: false },
];

/**
 * 右上方按鈕
 */
async function handleDeleteZone() {
  try {
    await invoke("delete_zone", {
      zoneName: zoneName.value,
    });
    navigateTo("/home");
  } catch (error) {
    console.error("API call failed:", error);
    ElMessage.error("Failed to delete zone");
  }
}

const isHistoryDialogVisible = ref(false);
function handleShowHistory() {
  console.log("Show History");
  isHistoryDialogVisible.value = true;
}

async function handleSummarizeAll() {
  console.log("Summarize All");

  ElMessageBox.confirm(
    "All old summaries will be replaced. Continue?",
    "Are you sure to summarize all files?",
    {
      confirmButtonText: "OK",
      cancelButtonText: "Cancel",
      type: "warning",
    },
  )
    .then(async () => {
      await invoke("ai_summarize_all_files", {
        zoneName: zoneName.value,
        zonePath: rootPath.value,
      })
        .then(() => {
          ElMessage({
            type: "success",
            message: "summarize completed",
          });
        })
        .catch((error) => {
          console.error("API call failed:", error);
          ElMessage.error("Failed to summarize all files");
        });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "summarize canceled",
      });
    });
}

async function handleRenewRules() {
  console.log("Renew Rules");
  ElMessageBox.confirm(
    "All old rules will be replaced. Continue?",
    "Are you sure to renew all rules?",
    {
      confirmButtonText: "OK",
      cancelButtonText: "Cancel",
      type: "warning",
    },
  )
    .then(async () => {
      // call API to renew all the rules
      let loadingInstance;
      loadingInstance = ElLoading.service({
        lock: true,
        text: "Sorting...",
        background: "rgba(0, 0, 0, 0.7)",
      });

      // TODO: 這裡如果出事我明天再處理
      await invoke("ai_renew_rules", {
        zoneName: zoneName.value,
        zonePath: rootPath.value,
      })
        .then((result) => {
          if (loadingInstance) loadingInstance.close();
          // save the new rules to store
          const renewed_rules = JSON.parse(result);
          ruleStore.setRule(renewed_rules);

          // 重新導向至規則檢視頁面
          navigateTo("/checkRenewRule");
        })
        .catch((error) => {
          loadingInstance.close();
          console.error("API call failed:", error);
          ElMessage.error("Failed to renew rules");
        });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "renew canceled",
      });
    });
}

const sortResult = ref({});
const isSortResultDialogVisible = ref(false);

async function handleSortAll() {
  try {
    await ElMessageBox.confirm(
      "This will sort the entire zone folder. Continue?",
      {
        confirmButtonText: "OK",
        cancelButtonText: "Cancel",
        type: "info",
      },
    );

    let loadingInstance;
    try {
      loadingInstance = ElLoading.service({
        lock: true,
        text: "Sorting...",
        background: "rgba(0, 0, 0, 0.7)",
      });

      // 呼叫 Rust API
      // TODO: 這裡如果出事我明天再處理
      let result;
      try {
        result = await invoke("ai_sort", {
          zoneName: zoneName.value,
          zonePath: rootPath.value,
          pathToSort: ".",
        });
      } catch (error) {
        console.error("API call failed:", error);
        ElMessage({
          type: "error",
          message: error?.toString() || "Sorting failed.",
        });
        return;
      }
      // convert result to object
      result = JSON.parse(result);
      if (result !== null) {
        sortResult.value = result; // 確保 Vue 直接接收物件
      } else {
        console.error("Unexpected API response:", result);
        throw new Error("Invalid response format from backend");
      }
      isSortResultDialogVisible.value = true;
    } catch (error) {
      console.error("API call failed:", error);
      ElMessage({
        type: "error",
        message: error?.toString() || "Sorting failed.",
      });
    } finally {
      if (loadingInstance) loadingInstance.close();
    }
  } catch {
    ElMessage({
      type: "info",
      message: "Sort canceled",
    });
  }
}

async function handleSortFolder(folderPath, folderName) {
  console.log("Sort Folder:", folderPath, folderName);
  try {
    await ElMessageBox.confirm(
      "This will sort folder " + folderPath + ". Continue?",
      {
        confirmButtonText: "OK",
        cancelButtonText: "Cancel",
        type: "info",
      },
    );

    let loadingInstance;
    try {
      loadingInstance = ElLoading.service({
        lock: true,
        text: "Sorting...",
        background: "rgba(0, 0, 0, 0.7)",
      });

      // 呼叫 Rust API
      // TODO: 這裡如果出事我明天再處理
      let result;
      try {
        result = await invoke("ai_sort", {
          zoneName: zoneName.value,
          zonePath: rootPath.value,
          pathToSort: folderPath,
        });
      } catch (error) {
        console.error("API call failed:", error);
        ElMessage({
          type: "error",
          message: error?.toString() || "Sorting failed.",
        });
        return;
      }

      if (typeof result === "object" && result !== null) {
        sortResult.value = result; // 確保 Vue 直接接收物件
      } else {
        console.error("Unexpected API response:", result);
        throw new Error("Invalid response format from backend");
      }
      isSortResultDialogVisible.value = true;
    } catch (error) {
      console.error("API call failed:", error);
      ElMessage({
        type: "error",
        message: error?.toString() || "Sorting failed.",
      });
    } finally {
      if (loadingInstance) loadingInstance.close();
    }
  } catch {
    ElMessage({
      type: "info",
      message: "Sort canceled",
    });
  }
}

const handleConfirmedMoves = async (selectedMoves) => {
  loading.value = true;
  console.log("Confirmed Moves:", selectedMoves);
  for (const move of selectedMoves) {
    console.log("Move:", move.src_path, "=>", move.new_path);
    try {
      await invoke("move_file", {
        zonePath: rootPath.value,
        srcPath: move.src_path,
        newPath: move.new_path,
        movedBy: move.moved_by,
        reason: move.reason,
      });
    } catch {
      console.error("move_files failed:", err);
      ElMessage({
        type: "error",
        message: error?.toString() || "Move failed.",
      });
    }
  }
  loading.value = false;
};

/**
 * 左側檔案樹
 */
function findNodeByPath(treeData, path) {
  for (const node of treeData) {
    if (node.path === path) return node;
    if (node.children && node.children.length) {
      const childResult = findNodeByPath(node.children, path);
      if (childResult) return childResult;
    }
  }
  return null;
}

function removeNodeByPath(treeData, path) {
  for (let i = 0; i < treeData.length; i++) {
    const node = treeData[i];
    if (node.path === path) {
      return treeData.splice(i, 1)[0];
    }
    if (node.children && node.children.length) {
      const removed = removeNodeByPath(node.children, path);
      if (removed) return removed;
    }
  }
  return null;
}

function sortChildren(array) {
  array.sort((a, b) => {
    if (a.isDirectory && !b.isDirectory) return -1;
    if (!a.isDirectory && b.isDirectory) return 1;
    return a.name.localeCompare(b.name);
  });
}

// 以遞迴方式，將整棵樹的節點標記為：
//   node.ignored = true/false
//   node.ignoredType = 'explicit' | 'inherited' | ''
// parentIgnored 代表父層是否處於忽略狀態 (true = 繼承)
function applyIgnoreStatusToTree(treeData) {
  treeData.forEach((node) => {
    applyIgnoreStatus(node, false);
  });
}

function applyIgnoreStatus(node, parentIgnored) {
  // 檢查此節點是否為顯式忽略
  const isExplicit = ignoredPaths.value.includes(node.path);

  // 本節點的忽略狀態 = 父層已忽略 或 自己顯式忽略
  node.ignored = parentIgnored || isExplicit;

  if (node.ignored) {
    node.ignoredType = parentIgnored ? "inherited" : "explicit";
  } else {
    node.ignoredType = "";
  }

  // 遞迴處理子節點
  if (node.children) {
    node.children.forEach((child) => {
      applyIgnoreStatus(child, node.ignored);
    });
  }
}

// 取得目前選取的 node 物件 (若找不到則為 null)
const selectedNode = computed(() => {
  return findNodeByPath(fileTree.value, selectedPath.value);
});

// 若是繼承忽略 (inherited)，就應該 disable Switch
const isInheritedIgnore = computed(() => {
  return selectedNode.value?.ignoredType === "inherited";
});

// 右側檔案內容的 Switch 綁定
const ignoreSwitch = computed({
  get() {
    const node = selectedNode.value;
    return node ? !!node.ignored && node.ignoredType !== "" : false;
  },
  set(val) {
    toggleIgnore(selectedPath.value, val);
  },
});

// 切換某個 path 是否要被「顯式忽略」
async function toggleIgnore(path, shouldIgnore) {
  // 暫存舊的 ignoredPaths，用於發生錯誤時回復
  const oldIgnoredPaths = [...ignoredPaths.value];

  if (shouldIgnore) {
    // 加入到 ignoredPaths
    if (!ignoredPaths.value.includes(path)) {
      ignoredPaths.value.push(path);
    }
  } else {
    // 移除
    const index = ignoredPaths.value.indexOf(path);
    if (index !== -1) {
      ignoredPaths.value.splice(index, 1);
    }
  }

  // 重新套用 ignore 狀態
  applyIgnoreStatusToTree(fileTree.value);

  // 呼叫後端 API 更新後端 ignore 狀態
  const formattedOutput = ignoredPaths.value.join("\n");

  invoke("set_ignore_list", {
    zonePath: rootPath.value,
    ignoredList: formattedOutput,
  }).catch((err) => {
    console.error("Ignore 更新失敗", err);
    // 回復舊狀態
    ignoredPaths.value = oldIgnoredPaths;
    applyIgnoreStatusToTree(fileTree.value);
  });
}

// 點擊樹狀節點
function handleNodeClick(node) {
  selectedPath.value = node.path;
  selectedTitle.value = node.name;
}

// 拖曳/放下檔案或資料夾
async function handleDrop(draggingNode, dropNode, dropType, ev) {
  console.log("tree drop:", {
    dragging: draggingNode.data.path,
    droppingOn: dropNode.data.path,
    dropType,
  });

  const srcPath = draggingNode.data.path;
  const destDirectoryPath = dropNode.data.path;
  const fileName = draggingNode.data.name;

  // 新路徑
  const newPath =
    destDirectoryPath === "/"
      ? `/${fileName}`
      : `${destDirectoryPath}/${fileName}`;

  try {
    await invoke("move_file", {
      zonePath: rootPath.value,
      srcPath: srcPath,
      newPath: srcPath,
      movedBy: "user",
      reason: "User moved file",
    });
  } catch (error) {
    console.error("move_file failed:", error);
    return;
  }

  console.log("模擬移動檔案成功:", srcPath, "=>", newPath);

  // 前端即時更新檔案樹
  const removedNode = removeNodeByPath(fileTree.value, srcPath);
  if (!removedNode) return;
  removedNode.path = newPath;

  const destNode = findNodeByPath(fileTree.value, destDirectoryPath);
  if (!destNode) {
    console.warn("未找到目標節點:", destDirectoryPath);
    return;
  }

  if (!destNode.children) destNode.children = [];
  destNode.children.push(removedNode);
  sortChildren(destNode.children);
  destNode.children = [...destNode.children];

  // 移動後，需要重新套用 ignore 狀態
  applyIgnoreStatusToTree(fileTree.value);
}

// DnD Rules
function allowDrag(node) {
  // 根目錄 ("/") 不可被拖曳
  return node.data.path !== "/";
}

function allowDrop(draggingNode, dropNode, type) {
  // 只允許拖曳到資料夾或根目錄，並且是放入 (inner)
  return (
    type === "inner" &&
    (dropNode.data.isDirectory || dropNode.data.path === "/")
  );
}

/**
 * Lifecycle
 */
onMounted(async () => {
  /**
   * 呼叫後端 API，根據 zonePath 取得檔案樹資料
   */

  await invoke("get_file_tree", {
    zonePath: rootPath.value,
  })
    .then((treeData) => {
      fileTree.value = JSON.parse(treeData);
    })
    .catch((error) => {
      console.error("API call failed:", error);
      ElMessage.error("Failed to get file tree data");
      fileTree.value = [
        {
          name: zoneName,
          path: "",
          isDirectory: true,
          children: mockFileTree,
        },
      ];
    });

  /**
   * 呼叫後端 API，取得 rules 和 form data，並存到 store
   */
  await invoke("get_zone_rules", {
    zoneName: zoneName.value,
  })
    .then((rules_str) => {
      const rules = JSON.parse(rules_str);
      ruleStore.setRule(rules);
    })
    .catch((error) => {
      console.error("API call failed:", error);
      ElMessage.error("Failed to get rules or form data");
    });

  await invoke("get_project_file", {
    zonePath: rootPath.value,
  })
    .then((project_file_str) => {
      const project_file_data = JSON.parse(project_file_str);
      formStore.setFormResponse(project_file_data["form_response"]);
    })
    .catch((error) => {
      console.error("API call failed:", error);
      ElMessage.error("Failed to get project_file");
    });

  /**
   * 呼叫後端 API，取得該 zone 下的 ignore 清單
   */
  await invoke("get_ignore_list", {
    zonePath: rootPath.value,
  })
    .then((ignoreListStr) => {
      const ignoreList = ignoreListStr.split("\n").filter((x) => x);
      ignoredPaths.value = ignoreList;
    })
    .catch((error) => {
      console.error("API call failed:", error);
      ElMessage.error("Failed to get ignore list");
      ignoredPaths.value = ["src/components", "README.md"];
    });

  // 將 ignore 狀態套用到檔案樹
  applyIgnoreStatusToTree(fileTree.value);
});
</script>

<style scoped>
.zone-container {
  display: flex;
  gap: 20px;
  height: 100%;
  overflow: hidden;
}

.sidebar {
  width: 20vw;
  max-width: 250px;
  border-right: 1px solid #dcdcdc;
  padding: 10px;
  overflow: hidden;
  position: sticky;
  top: 0;
}

.zone-scrollbar {
  max-height: 100%;
  overflow-y: auto;
}

.content-display {
  flex: 1;
  padding: 10px;
  overflow: hidden;
}

pre {
  background: #f5f5f5;
  padding: 10px;
  border-radius: 5px;
  overflow: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}

/* 被忽略的項目給予不同外觀 */
.ignored-file {
  color: #aaa; /* 偏灰 */
  text-decoration: line-through;
}

/* 若是繼承而來的忽略，可再額外加底色或其他標示 (視需求) */
.inherited-file {
  background-color: rgba(0, 0, 0, 0.05);
}

.right-content {
  margin-top: 20px;
  width: 100%;
}
</style>
