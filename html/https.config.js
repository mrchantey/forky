var fs = require("fs");

module.exports = {
	cert: fs.readFileSync(__dirname + "/../target/client-cert.pem"),
	key: fs.readFileSync(__dirname + "/../target/client-key.pem"),
	passphrase: "12345"
};