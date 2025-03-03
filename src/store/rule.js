import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

const initialRuleState = {
  index: {
    sorting_entropy: 8,
    naming_complexity: 6,
    archival_tendency: 10,
  },
  spec: {
    file_types: [],
    sort_struct: ["學期", "科目", "用途"],
    folder_depth: 5,
    capacity: 30,
    naming_style: ["name", "version"],
    date_format: "YYYYMMDD",
    filename_letter_rule: "none",
  },
  natural_language_rules: [
    "blah",
    "blah blah",
    "blah blah blah",
    "blah blah blah blah",
    "blah blah blah blah blah",
    "blah blah blah blah blah blah",
  ],
};

export const useRuleStore = defineStore("rule", {
  state: () => ({
    rule: initialRuleState,
  }),
  actions: {
    setRule(new_rule) {
      this.rule = new_rule;
    },
    resetRule() {
      this.rule = initialRuleState;
    },
  },
});
