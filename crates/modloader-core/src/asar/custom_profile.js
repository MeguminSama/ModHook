const { app } = require("electron");
const customAppDir = "$PROFILE";

const _setPath = app.setPath;

app.setPath = function (name, path) {
    if (name === "userData") {
        _setPath.call(app, name, customAppDir);
    } else {
        _setPath.call(app, name, path);
    }
};

app.setPath("userData", customAppDir);
