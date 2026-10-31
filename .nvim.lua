vim.lsp.config("ocamllsp", {})
vim.lsp.enable({ "ocamllsp" })

local conform = require("conform")

conform.setup({
    formatters_by_ft = {
        ocaml = { "ocamlformat" },
    },
})
