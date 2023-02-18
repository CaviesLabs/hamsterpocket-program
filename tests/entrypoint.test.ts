const path = require('path');
require('dotenv').config({path: path.join(__dirname, '../.env')});

// Import tests
require("./initialize_pocket_program.spec");
require("./create_pocket.spec");