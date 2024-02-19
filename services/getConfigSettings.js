import { invoke } from "@tauri-apps/api/tauri";


export class Config {
    constructor() {
        this.hash_count = 0;
        this.updated_date = "Never";
        this.auto_time = '22:00';
        this.selectedWeekday = -1;
        this.logging = false;
        this.obfuscated = false;
        this.use_db_path = false;
        this.custom_db_path = "";
        this.scan_dir = false;
        this.ignored_hashes = [];
    }

    loadConfig() {
        invoke("create_config", {})
        .then((output) => {
          const parsedData = JSON.parse(output);
          console.log("Loaded config: ", parsedData);
          this.hash_count = parsedData.hashes_in_db;
          if (parsedData.last_db_update != "Never") {
            this.updated_date = parsedData.last_db_update;
          }
            this.logging = parsedData.logging_is_active;
            this.obfuscated = parsedData.obfuscated_is_active;
            this.selectedWeekday = parsedData.db_update_weekday;
            this.auto_time = parsedData.db_update_time;
            this.use_db_path = parsedData.db_location.length > 0;
            this.custom_db_path = parsedData.db_location;
            this.scan_dir = parsedData.scan_dir;
            this.ignored_hashes = parsedData.ignored_hashes;
        })
        .catch((err) => console.error(err))
    }

    saveConfig(jsonData) {
        const jsonString = JSON.stringify(jsonData);
        console.log("Client sends: ", jsonData);

        invoke("create_config", { contents: jsonString })
        .then((output) => {
          const parsedData = JSON.parse(output);
          console.log("Server answer: ", parsedData);
        })
        .catch((err) => console.error(err))
    }

}

