let fname =
  try Sys.argv.(1)
  with Invalid_argument _err ->
    let () = Printf.eprintf "One argument (FILE) is required\n" in
    exit 1

let read_lines =
  let file = open_in fname in
  let result =
    try
      let try_read () = try Some (input_line file) with End_of_file -> None in
      let rec loop acc =
        match try_read () with
        | Some s -> loop (s :: acc)
        | None -> List.rev acc
      in
      loop []
    with e ->
      close_in_noerr file;
      raise e
  in
  close_in file;
  result

type op =
  (* number or register *)
  | Signal of string
  | AND of (string * string)
  | OR of (string * string)
  | LSHIFT of (string * int)
  | RSHIFT of (string * int)
  | NOT of string

type command = { src : string array; dst : string; op : op }
type signal = Cmd of command | Int of Stdint.uint16

let parse_command line =
  let src_rx =
    Re2.create_exn
      {|([a-zA-Z0-9]+) ([a-zA-Z0-9]+) ([a-zA-Z0-9]+)|(NOT) ([a-zA-Z0-9]+)|([a-zA-Z0-9]+)|}
  in
  let src, dst = Option.get (String.split_first ~sep:" -> " line) in
  let matches = Re2.find_submatches_exn src_rx src in
  let valid =
    Array.of_seq
      (Seq.map Option.get
         (Seq.filter
            (fun m -> Option.is_some m)
            (* skip the first item which is just the whole string *)
            (Array.to_seq (Array.sub matches 1 (Array.length matches - 1)))))
  in
  {
    op =
      (match valid with
      | [| x |] -> Signal x
      | [| "NOT"; x |] -> NOT x
      | [| x; "AND"; y |] -> AND (x, y)
      | [| x; "OR"; y |] -> OR (x, y)
      | [| x; "LSHIFT"; y |] -> LSHIFT (x, int_of_string y)
      | [| x; "RSHIFT"; y |] -> RSHIFT (x, int_of_string y)
      | _ ->
          raise (Invalid_argument (Printf.sprintf "Unimplemented dst: %s" dst)));
    dst;
    src = valid;
  }

let rec resolve table key =
  let exception I of int in
  try
    let () =
      match int_of_string_opt key with Some i -> raise (I i) | _ -> ()
    in
    match Hashtbl.find table key with
    | Int i -> i
    | Cmd cmd ->
        let result =
          match cmd.op with
          | Signal x -> (
              match int_of_string_opt x with
              | Some x -> Stdint.Uint16.of_int x
              | None -> resolve table x)
          | AND (kx, ky) ->
              let y = resolve table ky in
              let x = resolve table kx in
              Stdint.Uint16.logand x y
          | OR (kx, ky) ->
              let y = resolve table ky in
              let x = resolve table kx in
              Stdint.Uint16.logor x y
          | LSHIFT (kx, n) ->
              let x = resolve table kx in
              Stdint.Uint16.shift_left x n
          | RSHIFT (kx, n) ->
              let x = resolve table kx in
              Stdint.Uint16.shift_right x n
          | NOT kx ->
              let x = resolve table kx in
              Stdint.Uint16.lognot x
        in
        let () = Hashtbl.replace table key (Int result) in
        result
  with I i -> Stdint.Uint16.of_int i

let part1 lines =
  let commands =
    Hashtbl.of_seq
      (Seq.map parse_command (List.to_seq lines)
      |> Seq.map (fun cmd -> (cmd.dst, Cmd cmd)))
  in
  Stdint.Uint16.to_int (resolve commands "a")

let lines = read_lines
let () = Printf.printf "part1 = %d\n" (part1 lines)
