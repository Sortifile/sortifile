import { defineStore } from "pinia";

const initialFormState = {
  file_types: [],
  logic_habit: "",
  sort_struct: [],
  folder_depth: 5,
  capacity: 30,
  naming: [],
  date_format: "YYYYMMDD",
  filename_letter_rule: "",
  archival_tendency: "",
};

export const useFormStore = defineStore("form", {
  state: () => ({
    formResponse: initialFormState,
    formQuestion: {}, // TODO: Implement this by invoking the Tauri resource API
  }),
  actions: {
    setFormResponse(response) {
      this.formResponse = response;
    },
    resetFormResponse() {
      this.formResponse = initialFormState;
    },
  },
});
