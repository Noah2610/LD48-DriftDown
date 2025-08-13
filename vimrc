nmap <leader>r :!bin/run --features dev<CR>

autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml
autocmd BufNewFile,BufRead *.yml       setlocal shiftwidth=2 softtabstop=2 tabstop=2
