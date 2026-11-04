type t = int array

val make : int array
val get : t -> int -> int -> int
val set : t -> int -> int -> int -> unit
val zero : t -> unit
val sum : t -> int
val toggle : t -> int -> int -> unit
val inc : t -> int -> int -> int -> unit
