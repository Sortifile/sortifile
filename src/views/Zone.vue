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
        />
      </el-scrollbar>
    </div>

    <!-- 右側資訊顯示 -->
    <div class="content-display">
      <h2>{{ selectedTitle }}</h2>
      <el-skeleton v-if="loading" :rows="5" animated />
      <div v-else>
        <pre>{{ fileInfo }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";

const projectName = "Sortifile";
const fileTree = ref([]);
const selectedKey = ref("/");
const selectedTitle = ref(projectName);
const fileInfo = ref("");
const loading = ref(false);
const treeRef = ref(null);

const defaultProps = {
  children: "children",
  label: "name",
};

/**
 * Mock Data
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
  // 假設 Tauri API 名為 move_file，接收 srcPath 與 newPath
  // -------------------------------------------------------------
  /*
  try {
    const result = await invoke("move_file", {
      src: srcPath,
      dest: newPath,
    });
    console.log("move_file result:", result); // 假設一定為 success
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
}

/**
 * Drag / Drop Rules
 */
function allowDrag(node) {
  console.log("tree drag:", node.name);
  return node.data.path !== "/";
}

function allowDrop(draggingNode, dropNode, type) {
  return (
    type === "inner" &&
    (dropNode.data.isDirectory || dropNode.data.path === "/")
  );
}

/**
 * Lifecycle
 */
onMounted(() => {
  // TODO: 獲取真實檔案結構
  fileTree.value = [
    {
      name: projectName,
      path: "/",
      isDirectory: true,
      children: mockFileTree,
    },
  ];

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

pre {
  background: #f5f5f5;
  padding: 10px;
  border-radius: 5px;
  overflow: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
