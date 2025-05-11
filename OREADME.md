# Orustfmt

## Status: Highly Experimental

Tool designed to format specific rust syntax.
Example:

```rust
fn test( param_1 : u32, param_2 : u32 )
{
  let hello = call( param_1.max( param_2 ) );
  hello
}
```

## Installation

To install run:

```bash
git clone https://github.com/Anoromi-org/rustfmt
cd rustfmt
cargo install --bin orustfmt --path .
```

Before doing **anything** with binary run:

```bash
orustfmt --help
```

If no message was outputted try:

```bash
rustup run nightly-2025-04-02-x86_64 orustfmt --help
```

Use the command that worked for you.

## Usage

Currently orustfmt requires a specific use import configuration. Ensure `rustfmt.toml` includes it, otherwise you'll get errors.

```toml
imports_layout = "HorizontalVertical"
```

Now you can use orustfmt from cli.

```bash
orustfmt ./src/main.rs
```

## Vscode

Install `Custom Local Formatters` by jkillian.

```json
"customLocalFormatters.formatters": [
	{
		"command": "rustup run nightly-2025-04-02-x86_64 orustfmt",
		"languages": ["rust"]
	}
],
```

Now you can `Format document with...` orustfmt through local formatter.

## Neovim

If you use `conform.nvim` you can remap formatting for rust files for a specific project.

First configure custom formatter in neovim

```lua
conform.setup({
	...
	formatters = {
		orustfmt = {
			command = "rustup",
			options = {
			},
			args = function(self, ctx)
				local args = { "run", "nightly-2025-04-02", "orustfmt", "--emit=stdout" }
				 local edition = require("conform.util").parse_rust_edition(ctx.dirname) or self.options.default_edition
				 table.insert(args, "--edition=" .. edition)

				return args
			end,
			cwd = require("conform.util").root_file({
				"rustfmt.toml",
				".rustfmt.toml",
			}),
		},
	},
	...
)
```

Then in project that uses orustfmt create `.nvim.lua` config file in root.

```lua
if not vim.g.vscode then
	local conform = require("conform")

	vim.keymap.set({ "n", "v" }, "HERE YOUR KEYBINDING FOR FORMATTING CODE", function()
		local formatters = vim.bo.filetype == "rust" and { "orustfmt" } or nil
		conform.format({
			lsp_fallback = true,
			async = false,
			timeout_ms = 10000,
			formatters = formatters
		})
	end)
end

```
