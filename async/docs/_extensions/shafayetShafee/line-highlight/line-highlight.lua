local PATTERN = "#<<$"

local function ensureHtmlDeps()
  quarto.doc.add_html_dependency({
  name = "line-highlight",
  version = "1.0.0",
  scripts = {
    {
      path = "resources/js/line-highlight.js",
      attribs = {defer = "true"},
      afterBody = true

    }
  },
  stylesheets = {"resources/css/line-highlight.css"}
})
end


local function isEmpty(s)
  return s == nil or s == ''
end


function get_lines(text)
  local lines = {}
  local code = text .. "\n"
  for line in code:gmatch("([^\r\n]*)[\r\n]") do
    table.insert(lines, line)
  end
  return lines
end


function remove_pattern(lines, pattern)
  local code_lines = {}
  for _, line in ipairs(lines) do
    if line:match(pattern) then
      local cleaned_line = line:gsub(pattern, "")
      table.insert(code_lines, cleaned_line)
    else
      table.insert(code_lines, line)
    end
  end
  return table.concat(code_lines, "\n")
end


-- create escaped highlight_directive_pattern
function escape_pattern(s)
  local escaped = ""
  for c in s:gmatch(".") do
    escaped = escaped .. "%" .. c
  end
  return escaped
end


-- get line numbers for the lines that are marked with ht-pattern/PATTERN
function get_lines_to_ht(cb, pattern)
  local lines_to_ht = {}
  local code_lines = get_lines(cb.text)
  for i, line in ipairs(code_lines) do
    if line:match(pattern) then
      table.insert(lines_to_ht, i)
    end
  end
  return table.concat(lines_to_ht, ",")
end


-- add line numbers and ht-pattern attrs to codeblock
local function add_attrs_to_cb(source_line_numbers, output_line_numbers, ht_pat)
  -- adding line-number attrs for source and output code blocks
  local adder = {
    CodeBlock = function(cb)
      if cb.classes:includes('cell-code') then
        cb.attributes['source-line-numbers'] = source_line_numbers
        if not isEmpty(ht_pat) then
          cb.attributes['ht-pattern'] = tostring(ht_pat)
        end
      elseif cb.classes:includes('highlight') then
        cb.attributes['output-line-numbers'] = output_line_numbers
      end
      return cb
    end
    }
  return adder
end


-- pass the div attrs to CodeBlock
local function add_cb_attrs()
  local adder = {
    Div = function(el)
      if el.classes:includes('cell') then
        local source_line_numbers = tostring(el.attributes["source-line-numbers"])
        local output_line_numbers = tostring(el.attributes["output-line-numbers"])
        local ht_pat = el.attributes['ht-pattern']
        local div = el:walk(
          add_attrs_to_cb(source_line_numbers, output_line_numbers, ht_pat)
          )
        return div
      end
   end
  }
  return adder
end


-- highlight the code blocks
local function highlight_cb()
  local highlighter = {
    CodeBlock = function(cb)
      local pattern
      local pattern_attr = cb.attributes['ht-pattern']
      if not isEmpty(pattern_attr) then
        pattern = escape_pattern(pattern_attr) .. "$"
      else
        pattern = PATTERN
      end
      local lines_to_ht =  get_lines_to_ht(cb, pattern)
      local line_number
      local line_number_attr
      if cb.classes:includes('cell-code') then
        -- for executable code chunk
        line_number_attr = cb.attributes['source-line-numbers']
        line_number = isEmpty(lines_to_ht) and line_number_attr or lines_to_ht
        cb.attributes['data-code-line-numbers'] = line_number
      elseif cb.classes:includes('highlight') then
        -- for code output
        line_number_attr = cb.attributes['output-line-numbers']
        line_number = line_number_attr
        cb.attributes['data-code-line-numbers'] = line_number
      else
        -- for markdown CodeBlock
        if not isEmpty(lines_to_ht) then
          line_number = lines_to_ht
        elseif cb.attributes["source-line-numbers"] then
          line_number = cb.attributes["source-line-numbers"]
        else
          line_number = ""
        end
        cb.attributes["data-code-line-numbers"] = line_number
      end
      cb.text = remove_pattern(get_lines(cb.text), pattern)
      return cb
    end
  }
  return highlighter
end


if FORMAT == "html" then
  -- ensuring dependencies for line-highlighting
  ensureHtmlDeps()

  function Pandoc(doc)
    local doc = doc:walk(add_cb_attrs())
    return doc:walk(highlight_cb())
  end
end





