const std = @import("std");

fn char_to_prio(c: u8) usize {
    if ('a' <= c and c <= 'z') {
        return (c - 'a') + 1;
    }
    return 27 + (c - 'A');
}

fn part1(input: []const u8) anyerror!usize {
    var it = std.mem.split(u8, input, "\n");
    var result: usize = 0;
    while (it.next()) |line| {
        var sack = std.mem.zeroes([27 * 2]u8);

        const len = line.len;
        const l2 = len / 2;
        for (line[0..l2]) |c| {
            sack[char_to_prio(c)] = 1;
        }
        for (line[l2..len]) |c| {
            const p = char_to_prio(c);
            if (sack[p] == 1) {
                sack[p] += 1;
                result += p;
            }
        }
    }
    return result;
}

fn part2(input: []const u8) anyerror!usize {
    var it = std.mem.split(u8, input, "\n");
    var result: usize = 0;
    while (it.next()) |linea| {
        if (linea.len == 0) {
            break;
        }
        // use a bitfields to mark what priority was seen
        // in which line
        var sack = std.mem.zeroes([27 * 2]u8);

        const lineb = it.next().?;
        const linec = it.next().?;

        for (linea) |c| {
            sack[char_to_prio(c)] |= 1;
        }
        for (lineb) |c| {
            sack[char_to_prio(c)] |= 2;
        }
        for (linec) |c| {
            const i = char_to_prio(c);
            if (sack[i] == (2 | 1)) {
                sack[i] |= 4;
                result += i;
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
