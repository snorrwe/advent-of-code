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

let _print_graph graph =
  Hashtbl.to_seq graph
  |> Seq.iter (fun (k, connections) ->
      let () = Printf.printf "%s\n" k in
      let () =
        Hashtbl.to_seq connections
        |> Seq.iter (fun (k, v) -> Printf.printf "\t%s -> %d\n" k v)
      in
      print_endline "")

let insert_distance_impl table a b dist =
  let f = try a |> Hashtbl.find table with Not_found -> Hashtbl.create 10 in
  let () = Hashtbl.replace f b dist in
  Hashtbl.replace table a f

let insert_distance table a b (dist : int) =
  let () = insert_distance_impl table a b dist in
  let () = insert_distance_impl table b a dist in
  ()

let parse_graph lines =
  let table = Hashtbl.create 10 in
  let re = Re2.create_exn {|(.*) to (.*) = (\d+)|} in
  List.iter
    (fun line ->
      let matches = Re2.find_submatches_exn re line in
      insert_distance table
        (Option.get matches.(1))
        (Option.get matches.(2))
        (Option.get matches.(3) |> int_of_string))
    lines;
  table

let rec traverse graph visited start reducer init =
  let () = assert (Hashtbl.find_opt visited start |> Option.is_none) in
  let () = Hashtbl.add visited start 0 in
  let connections = Hashtbl.find graph start in
  let result =
    let valid_conn =
      Hashtbl.to_seq connections
      |> Seq.filter (fun (k, _) -> Hashtbl.find_opt visited k |> Option.is_none)
    in
    if Seq.length valid_conn != 0 then
      Seq.map
        (fun (k, v) -> v + traverse graph visited k reducer init)
        valid_conn
      |> Seq.fold_left reducer init
    else 0
  in
  let () = Hashtbl.remove visited start in
  result

let solve graph reducer init =
  let visited = Hashtbl.create 10 in
  Hashtbl.to_seq_keys graph
  |> Seq.map (fun key -> traverse graph visited key reducer init)
  |> Seq.fold_left (fun a b -> reducer a b) init

let part1 graph = solve graph min (1 lsl 32)
let part2 graph = solve graph max 0
let lines = read_lines
let graph = parse_graph lines
let () = Printf.printf "part1 = %d\n" (part1 graph)
let () = Printf.printf "part2 = %d\n" (part2 graph)
