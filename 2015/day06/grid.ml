type t = int array

let make = Array.make (1000 * 1000) 0
let idx x y = (1000 * y) + x
let get (grid : t) x y = grid.(idx x y)
let set (grid : t) x y value = grid.(idx x y) <- value
let zero (grid : t) = Array.fill grid 0 (Array.length grid) 0
let sum (grid : t) = Array.fold_left (fun acc i -> i + acc) 0 grid
let toggle (grid : t) x y = 1 - get grid x y |> set grid x y

let inc (grid : t) x y delta =
  let res = delta + get grid x y in
  max res 0 |> set grid x y
