const path = require("path");
require("dotenv").config({ path: path.join(__dirname, "../.env") });

// Import tests
require("./pocket_registry.spec");
require("./pocket.spec");
require("./assets.spec");
