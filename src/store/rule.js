import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useRuleStore = defineStore("rule", {
  state: () => ({
    rule: {},
  }),
  actions: {
    setRule(new_rule) {
      this.rule = new_rule;
    },
    async loadRule() {
      // TODO: Implement this by invoking the Tauri API
    },
  },
});
