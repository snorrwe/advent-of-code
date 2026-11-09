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

let part1 (lines : string list) =
  let re = Re2.create_exn {|(\\\\|\\")|(\\x[a-zA-Z0-9]{2})|} in
  List.to_seq lines
  |> Seq.map (fun line ->
      let len = String.length line in
      let matches = Re2.find_all re line in
      let l =
        match matches with
        | Ok matches ->
            let sub =
              List.to_seq matches
              |> Seq.map (fun m -> String.length m - 1)
              |> Seq.fold_left ( + ) 0
            in
            len - sub
        | _ -> len
      in
      len - (l - 2))
  |> Seq.fold_left ( + ) 0

let part2 (lines : string list) =
  let re = Re2.create_exn {|(\\\\|\\")|(\\x[a-zA-Z0-9]{2})|} in
  List.to_seq lines
  |> Seq.map (fun line ->
      let len = String.length line in
      let matches = Re2.find_all re line in
      let l =
        match matches with
        | Ok matches ->
            let delta =
              List.to_seq matches
              |> Seq.map (fun m -> match m with "\\\\" | "\\\"" -> 2 | _ -> 1)
              |> Seq.fold_left ( + ) 0
            in
            len + delta
        | _ -> len
      in
      l + 4 - len)
  |> Seq.fold_left ( + ) 0

let lines = read_lines
let () = Printf.printf "part1 = %d\n" (part1 lines)
let () = Printf.printf "part2 = %d\n" (part2 lines)
