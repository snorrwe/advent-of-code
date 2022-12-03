const std = @import("std");

fn char_to_prio(c: u8) i32 {
    if ('a' <= c and c <= 'z') {
        return (c - 'a') + 1;
    }
    return 27 + (c - 'A');
}

fn solve(input: []const u8) anyerror!i32 {
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

pub fn main() anyerror!void {
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var a = gpa.allocator();

    var input = try file.readToEndAlloc(a, 32000);
    var result = try solve(input);

    std.log.info("part1: {}", .{result});
}

test "basic test" {
    const result = try solve(
        \\vJrwpWtwJgWrhcsFMMfFFhFp
        \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        \\PmmdzqPrVvPwwTWBwg
        \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        \\ttgJtRGJQctTZtZT
        \\CrZsJsPPZsGzwwsLwLmpwMDw
    );
    try std.testing.expectEqual(result, 157);
}
