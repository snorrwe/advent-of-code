type counter = { map : (string, int) Hashtbl.t }

let make = { map = Hashtbl.create 1024 }

let add { map } s =
  try Hashtbl.replace map s (Hashtbl.find map s + 1)
  with Not_found -> Hashtbl.add map s 1

let clear { map } = Hashtbl.clear map

let pairs line f =
  Seq.iter2 f
    (Seq.zip (String.to_seq line)
       (String.to_seq (String.sub line 1 (String.length line - 1))))
    (* padd with 0 bytes so the last chars are also processed *)
    (Seq.append
       (String.to_seq (String.sub line 2 (String.length line - 2)))
       (Seq.repeat (char_of_int 0)))

let ingest t s =
  pairs s (fun (lhs, rhs) third ->
      if lhs != rhs || lhs != third then
        add t
          (let buf = Buffer.create 0 in
           let () = Buffer.add_char buf lhs in
           let () = Buffer.add_char buf rhs in
           Buffer.contents buf))
