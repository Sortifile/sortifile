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
          <!-- slot 自訂 Tree 節點 -->
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
      <h2>{{ selectedTitle }}</h2>
      <el-skeleton v-if="loading" :rows="5" animated />
      <div v-else>
        <!-- 切換 ignore 狀態的開關 -->
        <div class="ignore-toggle">
          <!-- 根目錄不顯示 Switch -->
          <el-switch
            v-if="selectedKey !== '/'"
            v-model="ignoreSwitch"
            active-text="已忽略"
            inactive-text="未忽略"
            :disabled="isInheritedIgnore"
          />
        </div>

        <pre>{{ fileInfo }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
// import { invoke } from "@tauri-apps/api"; // 若需呼叫後端 API，可解除註解

const projectName = "Sortifile";
const fileTree = ref([]);
const selectedKey = ref("/");
const selectedTitle = ref(projectName);
const fileInfo = ref("");
const loading = ref(false);
const treeRef = ref(null);

/**
 * 用來儲存要被「顯式忽略」的路徑。
 * 若其中包含資料夾路徑，則該資料夾及其子項目會是 "繼承忽略"。
 */
const ignoredPaths = ref([]);

/**
 * Tree 設定
 */
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
    path: "/src",
    isDirectory: true,
    children: [
      { name: "main.js", path: "/src/main.js", isDirectory: false },
      { name: "App.vue", path: "/src/App.vue", isDirectory: false },
      {
        name: "components",
        path: "/src/components",
        isDirectory: true,
        children: [
          {
            name: "Header.vue",
            path: "/src/components/Header.vue",
            isDirectory: false,
          },
          {
            name: "Footer.vue",
            path: "/src/components/Footer.vue",
            isDirectory: false,
          },
        ],
      },
    ],
  },
  { name: "package.json", path: "/package.json", isDirectory: false },
  { name: "README.md", path: "/README.md", isDirectory: false },
];

const mockFileData = {
  "/src/main.js": `{ "type": "JavaScript", "size": "2KB", "modified": "2025-02-10" }`,
  "/src/App.vue": `{ "type": "Vue Component", "size": "3KB", "modified": "2025-02-09" }`,
  "/src/components/Header.vue": `{ "type": "Vue Component", "size": "1KB", "modified": "2025-02-08" }`,
  "/src/components/Footer.vue": `{ "type": "Vue Component", "size": "1KB", "modified": "2025-02-08" }`,
  "/package.json": `{ "name": "sortifile", "version": "1.0.0", "dependencies": { "vue": "^3.5.0" } }`,
  "/README.md": `# Sortifile\n\nA file sorting app built with Vue and Rust.`,
};

/**
 * Utility Functions
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

/**
 * 以遞迴方式，將整棵樹的節點標記為：
 *   node.ignored = true/false
 *   node.ignoredType = 'explicit' | 'inherited' | ''
 *
 * parentIgnored 代表父層是否處於忽略狀態 (true = 繼承)
 */
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

  // 設定 ignoredType
  if (node.ignored) {
    node.ignoredType = parentIgnored ? "inherited" : "explicit";
  } else {
    node.ignoredType = "";
  }

  // 如果有子節點，遞迴下去
  if (node.children) {
    node.children.forEach((child) => {
      applyIgnoreStatus(child, node.ignored);
    });
  }
}

/**
 * 右側檔案內容的 Switch，受 selectedKey (點選檔案/資料夾) 控制
 */
const ignoreSwitch = computed({
  get() {
    const node = selectedNode.value;
    return node ? !!node.ignored && node.ignoredType !== "" : false;
  },
  set(val) {
    toggleIgnore(selectedKey.value, val);
  },
});

/**
 * 回傳目前選取的 node 物件 (若找不到則為 null)
 */
const selectedNode = computed(() => {
  return findNodeByPath(fileTree.value, selectedKey.value);
});

/**
 * 若是繼承忽略 (inherited)，就應該 disable 開關
 */
const isInheritedIgnore = computed(() => {
  return selectedNode.value?.ignoredType === "inherited";
});

/**
 * 切換某個 path 是否要被「顯式忽略」
 */
function toggleIgnore(path, shouldIgnore) {
  // 如果想改成 "忽略 = true"
  if (shouldIgnore) {
    // 放進 ignoredPaths（但若已在內則跳過）
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

  // TODO: 呼叫後端 API 更新 ignore 內容
  /*
  invoke("update_ignore_file", {
    path,
    ignore: shouldIgnore,
  })
    .then((res) => {
      console.log("Ignore 更新成功", res);
    })
    .catch((err) => {
      console.error("Ignore 更新失敗", err);
    });
  */
}

/**
 * Event Handlers
 */
function handleNodeClick(node) {
  selectedKey.value = node.path;
  selectedTitle.value = node.name;
  loading.value = true;

  setTimeout(() => {
    if (node.path === "/") {
      fileInfo.value = `{ "project": "${projectName}", "version": "1.0.0", "description": "A file sorting app built with Vue and Rust." }`;
    } else if (node.isDirectory) {
      fileInfo.value = "這是資料夾，請選擇一個檔案來查看內容";
    } else {
      fileInfo.value = mockFileData[node.path] || "無法獲取檔案資訊";
    }
    loading.value = false;
  }, 50);
}

async function handleDrop(draggingNode, dropNode, dropType, ev) {
  console.log("tree drop:", {
    dragging: draggingNode.data.path,
    droppingOn: dropNode.data.path,
    dropType,
  });

  const srcPath = draggingNode.data.path;
  const destDirectoryPath = dropNode.data.path;
  const fileName = draggingNode.data.name;

  // Construct new path
  const newPath =
    destDirectoryPath === "/"
      ? `/${fileName}`
      : `${destDirectoryPath}/${fileName}`;

  // TODO: 呼叫 Tauri API 進行檔案移動
  // -------------------------------------------------------------
  /*
  try {
    const result = await invoke("move_file", {
      src: srcPath,
      dest: newPath,
    });
    console.log("move_file result:", result);
  } catch (error) {
    console.error("move_file failed:", error);
    return;
  }
  */
  // -------------------------------------------------------------

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

/**
 * Drag / Drop Rules
 */
function allowDrag(node) {
  console.log("tree drag:", node.name);
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
  // 先模擬讀取 ignore 檔案
  // TODO: 改成呼叫 Tauri 後端 API 來拿真正的 ignore 設定
  // ------------------------------------
  // const ignoreContent = await invoke("get_ignore_file");
  // ignoredPaths.value = parseIgnoreToArray(ignoreContent);
  // ------------------------------------
  // 暫用靜態範例：假設忽略清單包含資料夾與檔案
  ignoredPaths.value = ["/src", "/README.md"];

  // 初始化檔案樹
  fileTree.value = [
    {
      name: projectName,
      path: "/",
      isDirectory: true,
      children: mockFileTree,
    },
  ];

  // 將 ignore 狀態套用到檔案樹
  applyIgnoreStatusToTree(fileTree.value);

  // 預設顯示根目錄資訊
  fileInfo.value = `{ "project": "${projectName}", "version": "1.0.0", "description": "A file sorting app built with Vue and Rust." }`;
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
  width: 250px;
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

.ignore-toggle {
  margin-bottom: 10px;
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
  /* 例如：加上一點淡色背景或特殊提示 */
  background-color: rgba(0, 0, 0, 0.05);
}
</style>
