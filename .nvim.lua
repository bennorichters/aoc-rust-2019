vim.keymap.set('i', '<F5>', '<ESC>:w<CR>:sp<CR>:term cargo run --bin %:t:r<CR>')
vim.keymap.set('i', '<F6>', '<ESC>:w<CR>:sp<CR>:term cargo test %:t:r_tests::<CR>')
vim.keymap.set('n', '<F5>', ':w<CR>:sp<CR>:term cargo run --bin %:t:r<CR>')
vim.keymap.set('n', '<F6>', ':w<CR>:sp<CR>:term cargo test %:t:r_tests::<CR>')
