(module
	(import "env" "fetch_input" (func $fetch_input (param i32)))
	(import "env" "input_length" (func $input_length (result i32)))


	(func (export "call")
		;; Assert that input_length is exactly 4 bytes long.
		(if
			(i32.ne
				(call $input_length)
				(i32.const 4)
			)
			(unreachable)
		)

		;; Load input data at the address 0.
		;;
		;; It contains only 1 word that represents an iteration count.
		(call $fetch_input
			(i32.const 0)
		)

		;; Load the iteration count from the address 0 and then
		;; call $recursive with this number.
		;; Drop the result (since it's always zero).
		(drop
			(call $recursive
				(i32.load
					(i32.const 0)
				)
			)
		)
	)

	(func $recursive (param i32) (result i32)
		block $out (result i32)
			get_local 0
			get_local 0
			i32.eqz
			br_if $out

			i32.const 1
			i32.sub
			call $recursive
		end
	)

	(table 0 anyfunc)
	(memory 1)
	(export "memory" (memory 0))
)
