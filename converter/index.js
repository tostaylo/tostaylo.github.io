const fs = require("fs");
const showdown = require("showdown");

const mdPath = `./posts/md/`;
const htmlPath = `../injected-app/public/assets/posts/html/`;
const converter = new showdown.Converter();

fs.readdir(mdPath, (err, files) => {
  files.forEach((mdFile) => {
    const fileMinusExt = mdFile.split(".").slice(0, 2).join(".");

    fs.readFile(`${mdPath}${mdFile}`, "utf8", (err, data) => {
      if (err) {
        console.error(err);
        return;
      }

      const html = converter.makeHtml(data);

      fs.writeFile(`${htmlPath}${fileMinusExt}.html`, html, function (err) {
        if (err) {
          return console.log(err, mdFile);
        }
        console.log("The file was saved!", mdFile);
      });
    });
  });
});
