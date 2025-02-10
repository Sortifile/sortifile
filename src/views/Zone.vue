<template>
  <div class="zone-container">
    <!-- 左側檔案樹 -->
    <div class="sidebar">
      <h2>檔案瀏覽</h2>
      <el-scrollbar class="zone-scrollbar">
        <el-tree
          :data="fileTree"
          :props="defaultProps"
          @node-click="handleNodeClick"
          highlight-current
          default-expand-all
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

const projectName = "Sortifile"; // 專案名稱
const fileTree = ref([]);
const defaultProps = {
  children: "children",
  label: "name",
};

const selectedKey = ref("/"); // 預設選中的 Key（根目錄）
const selectedTitle = ref(projectName); // 預設顯示專案名稱
const fileInfo = ref(""); // 存放檔案內容
const loading = ref(false);

// 模擬的檔案結構（子目錄）
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
  {
    name: "package.json",
    path: "/package.json",
    isDirectory: false,
  },
  {
    name: "README.md",
    path: "/README.md",
    isDirectory: false,
  },
];

// 模擬的檔案資訊
const mockFileData = {
  "/src/main.js": `{ "type": "JavaScript", "size": "2KB", "modified": "2025-02-10" }`,
  "/src/App.vue": `{ "type": "Vue Component", "size": "3KB", "modified": "2025-02-09" }`,
  "/src/components/Header.vue": `{ "type": "Vue Component", "size": "1KB", "modified": "2025-02-08" }`,
  "/src/components/Footer.vue": `{ "type": "Vue Component", "size": "1KB", "modified": "2025-02-08" }`,
  "/package.json": `{ "name": "sortifile", "version": "1.0.0", "dependencies": { "vue": "^3.5.0" } }`,
  "/README.md": `# Sortifile\n\nA file sorting app built with Vue and Rust.`,
};

// 處理節點點擊事件
const handleNodeClick = (node) => {
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
};

// 初始化獲取檔案結構並預設選擇根目錄
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

  // 預設載入根目錄資訊（避免右側沒資料）
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
  overflow: none;
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
