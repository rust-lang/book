## Ownership Inventory #1

The Ownership Inventory is a series of quizzes that test your understanding of ownership in real-world scenarios. These scenarios are inspired by common StackOverflow questions about Rust.

### A new technology: the in-browser IDE

These questions will involve Rust programs which use functions you haven't seen before. Therefore we will use an experimental technology that supports IDE features in the browser. The IDE lets you get information about unfamiliar functions and types. For example, try doing the following actions in the program below:

* Hover your mouse over `replace` to see its type and description.
* Hover your mouse over `s2` to see its inferred type.

---------


<pre>
<code class="ide">
/// Turns a string into a far more exciting string
fn make_exciting(s: &str) -> String {
  let s2 = s.replace(".", "!");
  let s3 = s2.replace("?", "‽");
  s3
}
</code>
</pre>

---------

A few important caveats about this experimental technology:

**PLATFORM COMPATIBILITY:** the in-browser IDE does not work on touch-screens. The in-browser IDE has only been tested to work on Google Chrome 109 and Firefox 107. It definitely does not work in Safari.

**MEMORY USAGE:** the in-browser IDE uses a [WebAssembly](https://rustwasm.github.io/book/) build of [rust-analyzer](https://github.com/rust-lang/rust-analyzer), which can take up a fair amount of memory. Each instance of the IDE appears to take around ~300 MB. 

**SCROLLING:** the in-browser IDE will "eat" your cursor if your cursor intersects with the editor while scrolling. If you're having trouble scrolling the page, try moving your cursor onto the rightmost scrollbar.

**LOAD TIMES:** the IDE may take up to 15 seconds to initialize for a new program. It will say "Loading..." as you interact with code in the editor.

### The Quiz

{{#quiz ../quizzes/ch06-04-inventory.toml}}