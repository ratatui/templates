/*
modified and reduced from
https://github.com/quarto-dev/quarto-cli/tree/main/src/resources/formats/revealjs/plugins/line-highlight
*/

window.document.addEventListener("DOMContentLoaded", function (event) {
  const delimiters = {
    step: "|",
    line: ",",
    lineRange: "-",
  };

  const regex = new RegExp(
    "^[\\d" + Object.values(delimiters).join("") + "]+$"
  );

  function handleLinesSelector(attr) {
    if (regex.test(attr)) {
      return true;
    } else {
      return false;
    }
  }

  function highlightCodeBlock(codeBlock) {
    const highlightSteps = splitLineNumbers(
      codeBlock.getAttribute(kCodeLineNumbersAttr)
    );

    if (highlightSteps.length) {
      // If we have at least one step, we generate fragments
      highlightSteps[0].forEach((highlight) => {
        // Add expected class on <pre> for reveal CSS
        codeBlock.parentNode.classList.add("code-wrapper");

        // Select lines to highlight
        spanToHighlight = [];
        if (typeof highlight.last === "number") {
          spanToHighlight = [].slice.call(
            codeBlock.querySelectorAll(
              ":scope > span:nth-child(n+" +
                highlight.first +
                "):nth-child(-n+" +
                highlight.last +
                ")"
            )
          );
        } else if (typeof highlight.first === "number") {
          spanToHighlight = [].slice.call(
            codeBlock.querySelectorAll(
              ":scope > span:nth-child(" + highlight.first + ")"
            )
          );
        }
        if (spanToHighlight.length) {
          // Add a class on <code> and <span> to select line to highlight
          spanToHighlight.forEach((span) =>
            span.classList.add("highlight-line")
          );
          codeBlock.classList.add("has-line-highlights");
        }
      });
    }
  }



  function getHighlightedLineBounds(block) {
    const highlightedLines = block.querySelectorAll(".highlight-line");
    if (highlightedLines.length === 0) {
      return { top: 0, bottom: 0 };
    } else {
      const firstHighlight = highlightedLines[0];
      const lastHighlight = highlightedLines[highlightedLines.length - 1];

      return {
        top: firstHighlight.offsetTop,
        bottom: lastHighlight.offsetTop + lastHighlight.offsetHeight,
      };
    }
  }


  function splitLineNumbers(lineNumbersAttr) {
    // remove space
    lineNumbersAttr = lineNumbersAttr.replace("/s/g", "");
    // seperate steps (for fragment)
    lineNumbersAttr = lineNumbersAttr.split(delimiters.step);

    // for each step, calculate first and last line, if any
    return lineNumbersAttr.map((highlights) => {
      // detect lines
      const lines = highlights.split(delimiters.line);
      return lines.map((range) => {
        if (/^[\d-]+$/.test(range)) {
          range = range.split(delimiters.lineRange);
          const firstLine = parseInt(range[0], 10);
          const lastLine = range[1] ? parseInt(range[1], 10) : undefined;
          return {
            first: firstLine,
            last: lastLine,
          };
        } else {
          return {};
        }
      });
    });
  }


  const kCodeLineNumbersAttr = "data-code-line-numbers";

  const divSourceCode = document.querySelectorAll("div.sourceCode");

  divSourceCode.forEach((el) => {
    if (el.hasAttribute(kCodeLineNumbersAttr)) {
      const codeLineAttr = el.getAttribute(kCodeLineNumbersAttr);
      el.removeAttribute("data-code-line-numbers");
      if (handleLinesSelector(codeLineAttr)) {
        // Only process if attr is a string to select lines to highlights
        // e.g "1|3,6|8-11"
        const codeBlock = el.querySelectorAll("pre code");
        codeBlock.forEach((code) => {
          // move attributes on code block
          code.setAttribute(kCodeLineNumbersAttr, codeLineAttr);
          highlightCodeBlock(code);
        });
      }
    }
  });

});
