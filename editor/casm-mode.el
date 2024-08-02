(eval-and-compile
  (defconst casm-keywords
    '("Add" "Sub" "Mul" "Div" "Imm" "Push" "Pop"
      "StackAdd" "StackSub" "StackMul" "StackDiv" "Cmp" "Jmp" "Je"
      "Jne" "Jg" "Jge" "Jl" "Jle" "Jz" "Jnz" "Ret" "Call" "Fn" "Syscall")))

(defconst casm-highlights
  `((,(regexp-opt casm-keywords 'symbols) . font-lock-keyword-face)))


;;;###autoload
(define-derived-mode casm-mode prog-mode "casm"
  "Major Mode for editing Casm source code."
  :syntax-table casm-mode-syntax-table
  (setq font-lock-defaults '(casm-highlights))
  (setq-local comment-start "; "))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.casm\\'" . casm-mode))

(provide 'casm-mode)
