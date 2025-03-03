import { defineStore } from "pinia";

export const useZoneStore = defineStore("zone", {
  state: () => ({
    zoneName: "",
    rootPath: "",
  }),
  actions: {
    setZone(zoneName, path) {
      this.zoneName = zoneName;
      this.rootPath = path;
    },
    setZoneName(name) {
      this.zoneName = name;
    },
    setPath(path) {
      this.rootPath = path;
    },
    resetZone() {
      this.zoneName = "";
      this.rootPath = "";
    },
  },
});
