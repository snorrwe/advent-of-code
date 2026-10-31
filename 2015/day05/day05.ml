let contains s1 s2 =
  try
    let len = String.length s2 in
    for i = 0 to String.length s1 - len do
      if String.sub s1 i len = s2 then raise Exit
    done;
    false
  with Exit -> true

let part_1 (lines : string list) =
  let count_vowels line =
    String.fold_left
      (fun acc c -> if String.contains "aeiou" c then 1 + acc else acc)
      0 line
  in

  let rec is_dupe_impl line i =
    if i + 1 >= String.length line then false
    else
      String.get line i == String.get line (i + 1) || is_dupe_impl line (i + 1)
  in

  let has_dupe line = is_dupe_impl line 0 in

  let has_illegal line =
    not
      (List.for_all (fun s -> not (contains line s)) [ "ab"; "cd"; "pq"; "xy" ])
  in

  let is_nice (line : string) =
    let vowels = count_vowels line in
    let d = has_dupe line in
    let has_illegal = has_illegal line in
    let is_nice = vowels >= 3 && d && not has_illegal in
    (* let () = *)
    (*   Printf.printf "%s\tvowels=%d has_dupe=%b has_illegal=%b is_nice=%b\n" line *)
    (*     vowels d has_illegal is_nice *)
    (* in *)
    is_nice
  in

  List.fold_left (fun c l -> if is_nice l then 1 + c else c) 0 lines

let fname = Sys.argv.(1)
let file = open_in fname

let lines =
  let try_read () = try Some (input_line file) with End_of_file -> None in
  let rec loop acc =
    match try_read () with Some s -> loop (s :: acc) | None -> List.rev acc
  in
  loop []

let () = close_in file
let () = Printf.printf "part1: %d\n" (part_1 lines)
