"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
function getPosts() {
    return fetch('./posts.json')
        .then((response) => response.json())
        .then((data) => data);
}
function render() {
    return __awaiter(this, void 0, void 0, function* () {
        const posts = yield getPosts();
        const listItems = posts.map((item) => `<li>${item.name}</li>`).join('');
        const postUL = document.getElementById('post-titles');
        if (postUL)
            postUL.innerHTML = listItems;
    });
}
render();
//# sourceMappingURL=posts.js.map