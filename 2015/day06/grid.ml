type t = char array

let make = Array.make (1000 * 1000) (Char.chr 0)
let idx x y = (1000 * y) + x
let get (grid : t) x y = grid.(idx x y)
let set (grid : t) x y (value : int) = grid.(idx x y) <- Char.chr value
let zero (grid : t) = Array.fill grid 0 (Array.length grid) (Char.chr 0)
let sum (grid : t) = Array.fold_left (fun acc i -> int_of_char i + acc) 0 grid
let toggle (grid : t) x y = set grid x y (1 - int_of_char (get grid x y))
