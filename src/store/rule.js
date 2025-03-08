import { defineStore } from "pinia";

const initialRuleState = () => ({
  index: {
    sorting_entropy: 8,
    naming_complexity: 6,
    archival_tendency: 10,
  },
  spec: {
    file_types: [],
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
});

export const useRuleStore = defineStore("rule", {
  state: () => ({
    rule: initialRuleState(),
  }),
  actions: {
    setRule(new_rule) {
      Object.assign(this.rule, new_rule);
      console.log(this.rule);
    },
    resetRule() {
      this.rule = initialRuleState();
    },
  },
});
