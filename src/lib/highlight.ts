// Using ES6 import syntax
import hljs from "highlight.js/lib/core";
import shell from "highlight.js/lib/languages/shell";
import rust from "highlight.js/lib/languages/rust";

hljs.registerLanguage("shell", shell);
hljs.registerLanguage("rust", rust);

export default hljs;
