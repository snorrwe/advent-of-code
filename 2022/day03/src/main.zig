const std = @import("std");

fn char_to_prio(c: u8) i32 {
    if ('a' <= c and c <= 'z') {
        return (c - 'a') + 1;
    }
    return 27 + (c - 'A');
}

fn part1(input: []const u8) anyerror!i32 {
    var it = std.mem.split(u8, input, "\n");
    const allocator = std.heap.page_allocator;
    var sacka = std.AutoHashMap(u8, void).init(allocator);
    var sackb = std.AutoHashMap(u8, void).init(allocator);
    var result: i32 = 0;
    while (it.next()) |line| {
        sacka.clearRetainingCapacity();
        sackb.clearRetainingCapacity();

        const len = line.len;
        const l2 = len / 2;
        for (line[0..l2]) |c| {
            try sacka.put(c, void{});
        }
        for (line[l2..len]) |c| {
            try sackb.put(c, void{});
        }
        var jt = sacka.keyIterator();
        while (jt.next()) |a| {
            if (sackb.contains(a.*)) {
                result += char_to_prio(a.*);
            }
        }
    }
    return result;
}

fn part2(input: []const u8) anyerror!i32 {
    var it = std.mem.split(u8, input, "\n");
    const allocator = std.heap.page_allocator;
    var sacka = std.AutoHashMap(u8, void).init(allocator);
    var sackb = std.AutoHashMap(u8, void).init(allocator);
    var sackc = std.AutoHashMap(u8, void).init(allocator);
    var result: i32 = 0;
    while (it.next()) |linea| {
        if (linea.len == 0) {
            break;
        }
        sacka.clearRetainingCapacity();
        sackb.clearRetainingCapacity();
        sackc.clearRetainingCapacity();

        const lineb = it.next().?;
        const linec = it.next().?;

        for (linea) |c| {
            try sacka.put(c, void{});
        }
        for (lineb) |c| {
            try sackb.put(c, void{});
        }
        for (linec) |c| {
            try sackc.put(c, void{});
        }
        var jt = sacka.keyIterator();
        while (jt.next()) |a| {
            if (sackb.contains(a.*) and sackc.contains(a.*)) {
                result += char_to_prio(a.*);
            }
        }
    }
    return result;
}

pub fn main() anyerror!void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var a = gpa.allocator();

    var input = try file.readToEndAlloc(a, 32000);

    var result = try part1(input);
    std.log.info("part1: {}", .{result});

    result = try part2(input);
    std.log.info("part2: {}", .{result});
}

test "part1 test" {
    const result = try part1(
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
    );
    try std.testing.expectEqual(result, 157);
}

test "part2 test" {
    const result = try part2(
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
    );
    try std.testing.expectEqual(result, 70);
}
