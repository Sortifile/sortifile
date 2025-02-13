import { defineStore } from "pinia";

export const useZoneStore = defineStore("zone", {
  state: () => ({
    zoneName: "",
    path: "",
  }),
  actions: {
    setZoneName(name) {
      this.zoneName = name;
    },
    setPath(path) {
      this.path = path;
    },
    resetZone() {
      this.zoneName = "";
      this.path = "";
    },
  },
});
