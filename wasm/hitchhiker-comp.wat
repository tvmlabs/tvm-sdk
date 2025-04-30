(component
  (core module $A
    (func (export "answer") (result i32)
      i32.const 42
    )
    (memory (export "mem") 1)
    (func (export "realloc") (param i32 i32 i32 i32) (result i32)
      i32.const 0
    )
  )
  (core instance $a (instantiate $A))
  (func (export "answer") (result s32)
    (canon lift
      (core func $a "answer")
      (memory $a "mem")
      (realloc (func $a "realloc"))
    )
  )
)