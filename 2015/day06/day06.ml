let grid = Grid.make

type command = {
  start_x : int;
  end_y : int;
  start_y : int;
  end_x : int;
  op : string;
}

let _print_command ch cmd =
  Printf.fprintf ch {|op=%s
start_x=%d
end_y=%d
start_y=%d
end_x=%d|} cmd.op
    cmd.start_x cmd.end_y cmd.start_y cmd.end_x

let parse_command line =
  let regex =
    Re2.create_exn {|(turn (on|off)|toggle) (\d+),(\d+) through (\d+),(\d+)|}
  in
  let matches = Re2.find_submatches_exn regex line in
  {
    start_x = int_of_string (Option.get matches.(3));
    start_y = int_of_string (Option.get matches.(4));
    end_x = int_of_string (Option.get matches.(5));
    end_y = int_of_string (Option.get matches.(6));
    op = Option.get matches.(1);
  }

let exec_fn cmd =
  match cmd.op with
  | "turn on" -> fun (y, x) -> Grid.set grid x y 1
  | "turn off" -> fun (y, x) -> Grid.set grid x y 0
  | "toggle" -> fun (y, x) -> Grid.toggle grid x y
  | _ -> raise Not_found

let exec_fn_v2 cmd =
  match cmd.op with
  | "turn on" -> fun (y, x) -> Grid.inc grid x y 1
  | "turn off" -> fun (y, x) -> Grid.inc grid x y (-1)
  | "toggle" -> fun (y, x) -> Grid.inc grid x y 2
  | _ -> raise Not_found

let part_1 (lines : string list) =
  let () = Grid.zero grid in
  let () =
    List.iter
      (fun line ->
        let command = parse_command line in
        let apply = exec_fn command in
        for y = command.start_y to command.end_y do
          for x = command.start_x to command.end_x do
            apply (x, y)
          done
        done)
      lines
  in
  Grid.sum grid

let part_2 (lines : string list) =
  let () = Grid.zero grid in
  let () =
    List.iter
      (fun line ->
        let command = parse_command line in
        let apply = exec_fn_v2 command in
        for y = command.start_y to command.end_y do
          for x = command.start_x to command.end_x do
            apply (x, y)
          done
        done)
      lines
  in
  Grid.sum grid

let fname =
  try Sys.argv.(1)
  with Invalid_argument _err ->
    let () = Printf.eprintf "One argument (FILE) is required\n" in
    exit 1

let lines =
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

let () = Printf.printf "part1: %d\n" (part_1 lines)
let () = Printf.printf "part2: %d\n" (part_2 lines)
